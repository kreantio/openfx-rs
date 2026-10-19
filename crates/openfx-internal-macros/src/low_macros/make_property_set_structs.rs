use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::quote;

use crate::{common::type_sys::OpenFXTypeSys, low_macros::utils};

mod inputs;
use inputs::*;

pub fn make_property_set_structs(attr: TokenStream, item: TokenStream) -> TokenStream {
    if let Err(error) = utils::err_if_attribute_is_not_empty(attr) {
        return error;
    }

    let mut output_inner = proc_macro2::TokenStream::new();

    let input = syn::parse_macro_input!(item as Input);

    let propsetdefs: HashMap<&syn::Ident, &InputPropsetdef> =
        input.defs.iter().map(|def| (&def.name, def)).collect();

    for set in &input.sets {
        match make_property_set_struct(&mut output_inner, &propsetdefs, set) {
            Ok(()) => {}
            Err(error) => return error.to_compile_error().into(),
        }
    }

    output_inner.into()
}

fn make_property_set_struct(
    output: &mut proc_macro2::TokenStream,
    propsetdefs: &HashMap<&syn::Ident, &InputPropsetdef>,
    set: &InputPropertySet,
) -> syn::Result<()> {
    let mut output_inner = proc_macro2::TokenStream::new();

    for (prop, write) in set.resolved_props(propsetdefs)? {
        make_property_accessors(&mut output_inner, prop, write);
    }

    let struct_name = &set.name;

    output.extend(quote! {
        pub struct #struct_name(crate::sys::generic::core::OfxPropertySetHandle);

        impl #struct_name {
            pub fn sys_handle(&self) -> crate::sys::generic::core::OfxPropertySetHandle {
                self.0
            }
        }

        impl From<crate::sys::generic::core::OfxPropertySetHandle> for #struct_name {
            fn from(handle: crate::sys::generic::core::OfxPropertySetHandle) -> Self {
                Self(handle)
            }
        }

        impl #struct_name {
            #output_inner
        }
    });

    Ok(())
}

fn make_property_accessors(
    output: &mut proc_macro2::TokenStream,
    prop: &InputPropertyItem,
    write: &InputWriteSide,
) {
    let tys: Vec<_> = prop.ty.possible_types().iter().collect();
    let is_ambiguous = tys.len() > 1;

    let cname = syn::Ident::new(
        &format!("k{}", prop.canonical_name),
        prop.canonical_name.span(),
    );

    for ty in prop.ty.possible_types().iter() {
        let sys_ty = ty.sys();
        let fn_name_suffix = if is_ambiguous {
            format!(
                "{}_{}",
                prop.simple_name.to_string().trim_start_matches("r#"),
                sys_ty.to_string().to_lowercase()
            )
        } else {
            prop.simple_name
                .to_string()
                .trim_start_matches("r#")
                .to_owned()
        };

        if write.is_plugin() {
            make_property_setter(output, prop, &cname, &ty, &fn_name_suffix);
        }
        if write.is_host() {
            make_property_getter(output, prop, &cname, &ty, &fn_name_suffix);
        }
    }

    if write.is_plugin() {
        make_property_resetter(output, prop, &cname);
    }
    if write.is_host() && matches!(prop.ty, InputContainerType::Slice(_)) {
        make_property_dimensions_getter(output, prop, &cname);
    }
}

fn make_property_setter(
    output: &mut proc_macro2::TokenStream,
    prop: &InputPropertyItem,
    cname: &syn::Ident,
    ty: &OpenFXTypeLow,
    fn_name_suffix: &str,
) {
    let fn_name = syn::Ident::new(&format!("set_{}", fn_name_suffix), prop.simple_name.span());
    let fn_name_on_suite = syn::Ident::new(
        &format!(
            "prop_set_{}{}",
            match ty.sys() {
                OpenFXTypeSys::String(_) => "string_ptr".to_owned(),
                other => other.name_snake_case().to_owned(),
            },
            prop.ty.as_name_suffix()
        ),
        proc_macro2::Span::call_site(),
    );

    let low_ty = ty.quote_for_low_setter_type();
    let low_container_ty = match &prop.ty {
        InputContainerType::Scalar(_) => low_ty.clone(),
        InputContainerType::Array(_, size) => {
            quote! { [#low_ty; #size] }
        }
        InputContainerType::Slice(_) => quote! { &[#low_ty] },
    };

    let low_to_sys_quote = ty.quote_for_low_setter_type_to_sys_type(quote! { value });
    let low_value_to_sys_quote = match &prop.ty {
        InputContainerType::Scalar(_) => low_to_sys_quote,
        InputContainerType::Array(_, _) if ty.is_low_setter_type_and_sys_type_identical() => {
            quote! { value }
        }
        InputContainerType::Array(_, _) => quote! { value.map(|value| #low_to_sys_quote) },
        InputContainerType::Slice(_) if ty.is_low_setter_type_and_sys_type_identical() => {
            quote! { value }
        }
        InputContainerType::Slice(_) => {
            quote! { &value.iter().map(|value| #low_to_sys_quote).collect::<Vec<_>>() }
        }
    };

    output.extend(
        quote! {
            /// ## Safety
            ///
            /// - `self.0` must be a valid handle of
            ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
            /// - `suite` must be valid.
            pub unsafe fn #fn_name(&self, suite: &crate::low_plugin::suites::PropertySuiteV1, value: #low_container_ty) -> crate::low::Result<()> {
                let value_simple = #low_value_to_sys_quote;
                suite.#fn_name_on_suite(self.sys_handle(), crate::sys_umbrella::#cname, value_simple)
            }
        }
    );
}

fn make_property_getter(
    output: &mut proc_macro2::TokenStream,
    prop: &InputPropertyItem,
    cname: &syn::Ident,
    ty: &OpenFXTypeLow,
    fn_name_suffix: &str,
) {
    let fn_name = syn::Ident::new(&format!("get_{}", fn_name_suffix), prop.simple_name.span());
    let fn_name_on_suite = syn::Ident::new(
        &format!(
            "prop_get_{}{}",
            match ty.sys() {
                OpenFXTypeSys::String(_) => "string_ptr".to_owned(),
                other => other.name_snake_case().to_owned(),
            },
            prop.ty.as_name_suffix()
        ),
        proc_macro2::Span::call_site(),
    );

    let low_ty = ty.quote_for_low_getter_type();
    let low_from_sys_quote = ty.quote_for_sys_type_to_low_getter_type(quote! { value_simple });
    let low_value_from_sys_quote = match &prop.ty {
        InputContainerType::Scalar(_) => quote! { #low_from_sys_quote },
        InputContainerType::Array(_, _) | InputContainerType::Slice(_)
            if ty.is_sys_type_and_low_getter_type_identical() =>
        {
            quote! { value_simple }
        }
        InputContainerType::Array(_, _) => {
            quote! { value_simple.into_iter().map(|value_simple| #low_from_sys_quote) }
        }
        InputContainerType::Slice(_) => {
            quote! { value_simple.into_iter().map(|value_simple| #low_from_sys_quote).collect() }
        }
    };

    let docs = quote! {
        /// ## Safety
        ///
        /// - `self.0` must be a valid handle of
        ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
        /// - `suite` must be valid.
    };

    match &prop.ty {
        InputContainerType::Scalar(_) => output.extend(
            quote! {
                #docs
                pub unsafe fn #fn_name<'a, 'b>(&'a self, suite: &'b crate::low_plugin::suites::PropertySuiteV1) -> crate::low::Result<#low_ty>
                    where 'b: 'a
                {
                    let value_simple = suite.#fn_name_on_suite(self.sys_handle(), crate::sys_umbrella::#cname)?;
                    let value = #low_value_from_sys_quote;
                    Ok(value)
                }
            },
        ),
        InputContainerType::Array(_, size) if ty.is_sys_type_and_low_getter_type_identical() => output.extend(
            quote! {
                #docs
                pub unsafe fn #fn_name<'a, 'b>(&'a self, suite: &'b crate::low_plugin::suites::PropertySuiteV1) -> crate::low::Result<[#low_ty; #size]>
                    where 'b: 'a
                {
                    let value = suite.#fn_name_on_suite::<#size>(self.sys_handle(), crate::sys_umbrella::#cname)?;
                    Ok(value)
                }
            }
        ),
        InputContainerType::Array(_, size) => output.extend(
            quote! {
                #docs
                pub unsafe fn #fn_name(&self, suite: &crate::low_plugin::suites::PropertySuiteV1) -> crate::low::Result<[#low_ty; #size]> {
                    let value_simple = suite.#fn_name_on_suite::<#size>(self.sys_handle(), crate::sys_umbrella::#cname)?;
                    let value = #low_value_from_sys_quote;
                    Ok(value)
                }
            },
        ),
        InputContainerType::Slice(_) => output.extend(
            quote! {
                #docs
                pub unsafe fn #fn_name(&self, suite: &crate::low_plugin::suites::PropertySuiteV1) -> crate::low::Result<::std::vec::Vec<#low_ty>> {
                    let dimensions = suite.prop_get_dimension(self.sys_handle(), crate::sys_umbrella::#cname)? as usize;
                    let mut value_simple = ::std::vec::Vec::with_capacity(dimensions);
                    unsafe { value_simple.set_len(dimensions); }
                    suite.#fn_name_on_suite(self.sys_handle(), crate::sys_umbrella::#cname, &mut value_simple)?;
                    let value = #low_value_from_sys_quote;
                    Ok(value)
                }
            },
        ),
    }
}

fn make_property_resetter(
    output: &mut proc_macro2::TokenStream,
    prop: &InputPropertyItem,
    cname: &syn::Ident,
) {
    let fn_name = syn::Ident::new(
        &format!(
            "reset_{}",
            prop.simple_name.to_string().trim_start_matches("r#")
        ),
        prop.simple_name.span(),
    );

    output.extend(
        quote! {
            /// ## Safety
            ///
            /// - `self.0` must be a valid handle of
            ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
            /// - `suite` must be valid.
            pub unsafe fn #fn_name(&self, suite: &crate::low_plugin::suites::PropertySuiteV1) -> crate::low::Result<()> {
                unsafe {
                    suite.prop_reset(self.sys_handle(), crate::sys_umbrella::#cname)
                }
            }
        }
    );
}

fn make_property_dimensions_getter(
    output: &mut proc_macro2::TokenStream,
    prop: &InputPropertyItem,
    cname: &syn::Ident,
) {
    let fn_name = syn::Ident::new(
        &format!(
            "len_{}",
            prop.simple_name.to_string().trim_start_matches("r#")
        ),
        prop.simple_name.span(),
    );

    output.extend(
        quote! {
            /// ## Safety
            ///
            /// - `self.0` must be a valid handle of
            ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
            /// - `suite` must be valid.
            pub unsafe fn #fn_name(&self, suite: &crate::low_plugin::suites::PropertySuiteV1) -> crate::low::Result<::std::os::raw::c_int> {
                unsafe {
                    suite.prop_get_dimension(self.sys_handle(), crate::sys_umbrella::#cname)
                }
            }
        }
    );
}

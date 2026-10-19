use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};

use crate::low_macros::utils;

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

    let preset = ty.make_impl_setter_preset();

    let fn_name_on_suite = syn::Ident::new(
        &format!(
            "prop_set_{}{}{}",
            ty.sys().name_snake_case().to_owned(),
            preset
                .sys_type_suffix()
                .map_or(String::new(), |s| format!("_{s}")),
            prop.ty.as_name_suffix(),
        ),
        proc_macro2::Span::call_site(),
    );

    let low_ty = preset.low_type(ty.ident());
    let low_container_ty = match &prop.ty {
        InputContainerType::Scalar(_) => low_ty.clone(),
        InputContainerType::Array(_, size) => {
            quote! { [#low_ty; #size] }
        }
        InputContainerType::Slice(_) => quote! { &[#low_ty] },
    };

    // let low_to_sys_quote = preset.low_type_to_sys_type(quote! { value });
    let low_value_to_sys_quote = match preset.low_type_to_sys_type(quote! { value }) {
        Some(low_to_sys_quote) => match &prop.ty {
            InputContainerType::Scalar(_) => low_to_sys_quote,
            InputContainerType::Array(_, _) => quote! { value.map(|value| #low_to_sys_quote) },
            InputContainerType::Slice(_) => {
                quote! { &value.iter().map(|value| #low_to_sys_quote).collect::<Vec<_>>() }
            }
        },
        _ => quote! { value },
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

    let preset = ty.make_impl_getter_preset();

    let fn_name_on_suite = syn::Ident::new(
        &format!(
            "prop_get_{}{}{}",
            ty.sys().name_snake_case().to_owned(),
            preset
                .sys_type_suffix()
                .map_or(String::new(), |s| format!("_{s}")),
            prop.ty.as_name_suffix(),
        ),
        proc_macro2::Span::call_site(),
    );

    let low_ty = preset.low_type(ty.ident());
    let sys_to_low_quote = preset.sys_type_to_low_type(quote! { value_sys });
    let is_low_type_and_sys_type_identical = sys_to_low_quote.is_none();
    let sys_value_to_low_quote = match sys_to_low_quote {
        Some(low_from_sys_quote) => match &prop.ty {
            InputContainerType::Scalar(_) => quote! { #low_from_sys_quote },
            InputContainerType::Array(_, _) => {
                quote! { value_sys.into_iter().map(|value_sys| #low_from_sys_quote) }
            }
            InputContainerType::Slice(_) => {
                quote! { value_sys.into_iter().map(|value_sys| #low_from_sys_quote).collect() }
            }
        },
        _ => quote! { value_sys },
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
                    let value_sys = suite.#fn_name_on_suite(self.sys_handle(), crate::sys_umbrella::#cname)?;
                    let value = #sys_value_to_low_quote;
                    Ok(value)
                }
            },
        ),
        InputContainerType::Array(_, size) if is_low_type_and_sys_type_identical => output.extend(
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
                    let value_sys = suite.#fn_name_on_suite::<#size>(self.sys_handle(), crate::sys_umbrella::#cname)?;
                    let value = #sys_value_to_low_quote;
                    Ok(value)
                }
            },
        ),
        InputContainerType::Slice(_) => output.extend(
            quote! {
                #docs
                pub unsafe fn #fn_name(&self, suite: &crate::low_plugin::suites::PropertySuiteV1) -> crate::low::Result<::std::vec::Vec<#low_ty>> {
                    let dimensions = suite.prop_get_dimension(self.sys_handle(), crate::sys_umbrella::#cname)? as usize;
                    let mut value_sys = ::std::vec::Vec::with_capacity(dimensions);
                    unsafe { value_sys.set_len(dimensions); }
                    suite.#fn_name_on_suite(self.sys_handle(), crate::sys_umbrella::#cname, &mut value_sys)?;
                    let value = #sys_value_to_low_quote;
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

impl OpenFXTypeLow {
    fn make_impl_setter_preset(&self) -> Box<dyn ImplSetterPreset> {
        match self {
            OpenFXTypeLow::Int(_) => Box::new(ImplAccessorsPresetInt),
            OpenFXTypeLow::Double(_) => Box::new(ImplAccessorsPresetDouble),
            OpenFXTypeLow::Enum(_, name) => {
                Box::new(ImplAccessorsPresetEnum { name: name.clone() })
            }
            OpenFXTypeLow::Bool(_) => Box::new(ImplAccessorsPresetBool),
            OpenFXTypeLow::String(_) => Box::new(ImplAccessorsPresetString),
            OpenFXTypeLow::Pointer(_) => Box::new(ImplAccessorsPresetPointer),
        }
    }
    fn make_impl_getter_preset(&self) -> Box<dyn ImplGetterPreset> {
        match self {
            OpenFXTypeLow::Int(_) => Box::new(ImplAccessorsPresetInt),
            OpenFXTypeLow::Double(_) => Box::new(ImplAccessorsPresetDouble),
            OpenFXTypeLow::Enum(_, name) => {
                Box::new(ImplAccessorsPresetEnum { name: name.clone() })
            }
            OpenFXTypeLow::Bool(_) => Box::new(ImplAccessorsPresetBool),
            OpenFXTypeLow::String(_) => Box::new(ImplAccessorsPresetString),
            OpenFXTypeLow::Pointer(_) => Box::new(ImplAccessorsPresetPointer),
        }
    }
}

trait ImplSetterPreset {
    fn sys_type_suffix(&self) -> Option<&'static str> {
        None
    }
    fn low_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream;
    /// `None` means that the low type and the sys type are identical.
    fn low_type_to_sys_type(
        &self,
        _low_val: proc_macro2::TokenStream,
    ) -> Option<proc_macro2::TokenStream> {
        None
    }
}

trait ImplGetterPreset {
    fn sys_type_suffix(&self) -> Option<&'static str> {
        None
    }
    fn low_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream;
    /// `None` means that the low type and the sys type are identical.
    fn sys_type_to_low_type(
        &self,
        _sys_val: proc_macro2::TokenStream,
    ) -> Option<proc_macro2::TokenStream> {
        None
    }
}

struct ImplAccessorsPresetInt;
impl ImplAccessorsPresetInt {
    fn low_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        let c_int = quote_spanned! { ident.span() => c_int };
        quote! { ::std::os::raw::#c_int }
    }
}
impl ImplSetterPreset for ImplAccessorsPresetInt {
    fn low_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        self.low_type(ident)
    }
}
impl ImplGetterPreset for ImplAccessorsPresetInt {
    fn low_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        self.low_type(ident)
    }
}

struct ImplAccessorsPresetDouble;
impl ImplAccessorsPresetDouble {
    fn low_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        let f64 = quote_spanned! { ident.span() => f64 };
        quote! { ::core::primitive::#f64 }
    }
}
impl ImplSetterPreset for ImplAccessorsPresetDouble {
    fn low_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        self.low_type(ident)
    }
}
impl ImplGetterPreset for ImplAccessorsPresetDouble {
    fn low_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        self.low_type(ident)
    }
}

struct ImplAccessorsPresetBool;
impl ImplAccessorsPresetBool {
    fn low_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        let bool = quote_spanned! { ident.span() => bool };
        quote! { ::core::primitive::#bool }
    }
}
impl ImplSetterPreset for ImplAccessorsPresetBool {
    fn low_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        self.low_type(ident)
    }
    fn low_type_to_sys_type(
        &self,
        low_val: proc_macro2::TokenStream,
    ) -> Option<proc_macro2::TokenStream> {
        Some(quote! { #low_val as ::std::os::raw::c_int })
    }
}
impl ImplGetterPreset for ImplAccessorsPresetBool {
    fn low_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        self.low_type(ident)
    }
    fn sys_type_to_low_type(
        &self,
        sys_val: proc_macro2::TokenStream,
    ) -> Option<proc_macro2::TokenStream> {
        Some(quote! { #sys_val != 0 })
    }
}

struct ImplAccessorsPresetPointer;
impl ImplAccessorsPresetPointer {
    fn low_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        let c_void = quote_spanned! { ident.span() => c_void };
        quote! { ::std::option::Option<::std::ptr::NonNull<::std::ffi::#c_void>> }
    }
}
impl ImplSetterPreset for ImplAccessorsPresetPointer {
    fn low_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        self.low_type(ident)
    }
}
impl ImplGetterPreset for ImplAccessorsPresetPointer {
    fn low_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        self.low_type(ident)
    }
}

struct ImplAccessorsPresetString;
impl ImplAccessorsPresetString {
    const SYS_TYPE_SUFFIX: Option<&'static str> = Some("ptr");
}
impl ImplSetterPreset for ImplAccessorsPresetString {
    fn sys_type_suffix(&self) -> Option<&'static str> {
        Self::SYS_TYPE_SUFFIX
    }
    fn low_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        let cstr = quote_spanned! { ident.span() => CStr };
        quote! { ::std::option::Option<&::std::ffi::#cstr> }
    }
    fn low_type_to_sys_type(
        &self,
        low_val: proc_macro2::TokenStream,
    ) -> Option<proc_macro2::TokenStream> {
        Some(quote! { #low_val.and_then(|s| ::std::ptr::NonNull::new(s.as_ptr() as *mut _)) })
    }
}
impl ImplGetterPreset for ImplAccessorsPresetString {
    fn sys_type_suffix(&self) -> Option<&'static str> {
        Self::SYS_TYPE_SUFFIX
    }
    fn low_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        let c_char = quote_spanned! { ident.span() => c_char };
        quote! { ::std::option::Option<::std::ptr::NonNull<::std::ffi::#c_char>> }
    }
}

struct ImplAccessorsPresetEnum {
    name: syn::Ident,
}
impl ImplAccessorsPresetEnum {
    const SYS_TYPE_SUFFIX: Option<&'static str> = ImplAccessorsPresetString::SYS_TYPE_SUFFIX;
}
impl ImplSetterPreset for ImplAccessorsPresetEnum {
    fn sys_type_suffix(&self) -> Option<&'static str> {
        Self::SYS_TYPE_SUFFIX
    }
    fn low_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        let name = &self.name;
        let known = quote_spanned! { ident.span() => crate::low::enums::#name };
        quote_spanned! { ident.span() => crate::low_plugin::property_sets::EnumPropertyPtr<#known> }
    }
    fn low_type_to_sys_type(
        &self,
        low_val: proc_macro2::TokenStream,
    ) -> Option<proc_macro2::TokenStream> {
        Some(quote! { #low_val.as_ptr() })
    }
}
impl ImplGetterPreset for ImplAccessorsPresetEnum {
    fn sys_type_suffix(&self) -> Option<&'static str> {
        Self::SYS_TYPE_SUFFIX
    }
    fn low_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        let name = &self.name;
        let known = quote_spanned! { ident.span() => crate::low::enums::#name };
        quote! { crate::low_plugin::property_sets::LosslessEnumProperty<#known> }
    }
    fn sys_type_to_low_type(
        &self,
        sys_val: proc_macro2::TokenStream,
    ) -> Option<proc_macro2::TokenStream> {
        let name = &self.name;
        let known = quote_spanned! { name.span() => crate::low::enums::#name };
        Some(
            quote! { unsafe { crate::low_plugin::property_sets::LosslessEnumProperty::<#known>::from_ptr(#sys_val) } },
        )
    }
}

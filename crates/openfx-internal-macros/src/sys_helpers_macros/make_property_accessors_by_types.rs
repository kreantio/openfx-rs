use proc_macro::TokenStream;
use quote::quote;

use crate::common::type_sys::OpenFXTypeSys;

pub fn make_property_accessors_by_types(tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(tokens as Input);

    let mut output: Vec<proc_macro2::TokenStream> = Vec::new();

    for ty in input.into_iter() {
        // array
        make_property_setter_for_type(&mut output, &ty, ContainerType::Array);
        make_property_getter_for_type(&mut output, &ty, ContainerType::Array);

        // single
        make_property_setter_for_type(&mut output, &ty, ContainerType::Single);
        make_property_getter_for_type(&mut output, &ty, ContainerType::Single);

        // fixed array
        make_property_setter_for_fixed_array(&mut output, &ty);
        make_property_getter_for_fixed_array(&mut output, &ty);
    }

    quote! {
        #(#output)*
    }
    .into()
}

enum ContainerType {
    Single,
    Array,
}

impl ContainerType {
    fn name_suffix(&self, ty: &OpenFXTypeSys) -> String {
        match self {
            ContainerType::Single => ty.to_string().to_lowercase(),
            ContainerType::Array => format!("{}s", ty.to_string().to_lowercase()),
        }
    }
}

fn make_property_setter_for_type(
    output: &mut Vec<proc_macro2::TokenStream>,
    ty: &OpenFXTypeSys,
    container_type: ContainerType,
) {
    let name_suffix = container_type.name_suffix(ty);
    let name = syn::Ident::new(&format!("set_{}", name_suffix), ty.span());

    let rust_ty_for_setter = ty.rust_type_quote_for_setter();
    let value_type = match container_type {
        ContainerType::Single => quote! { #rust_ty_for_setter },
        ContainerType::Array => quote! { &[#rust_ty_for_setter] },
    };
    let sys_setter_fn_ident = match container_type {
        ContainerType::Single => ty.sys_setter_fn_ident_single(),
        ContainerType::Array => ty.sys_setter_fn_ident_array(),
    };
    let suite_fn_args = match container_type {
        ContainerType::Single => quote! { 0, value },
        ContainerType::Array => quote! { value.len() as std::os::raw::c_int, value.as_ptr() },
    };

    output.push(quote! {
        /// ## SAFETY
        ///
        /// - `suite` must be a valid pointer to
        ///   [`crate::sys_umbrella::OfxPropertySuiteV1`].
        /// - `handle` must be a valid handle of
        ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
        /// - The type of `property`'s value must match the type this function
        ///   is specialized for.
        #[inline(always)]
        pub unsafe fn #name(
            suite: *const crate::sys_umbrella::OfxPropertySuiteV1,
            handle: crate::sys_umbrella::OfxPropertySetHandle,
            property: *const std::os::raw::c_char,
            value: #value_type,
        ) -> Result<(), crate::sys_umbrella::OfxStatus> {
            // SAFETY: granted by the standard
            let suite_fn = unsafe { (&*suite).#sys_setter_fn_ident.unwrap_unchecked() };
            if let s = unsafe { suite_fn(handle, property, #suite_fn_args) }
                && s != crate::sys_umbrella::kOfxStatOK {
                Err(s)
            } else {
                Ok(())
            }
        }
    });
}

fn make_property_setter_for_fixed_array(
    output: &mut Vec<proc_macro2::TokenStream>,
    ty: &OpenFXTypeSys,
) {
    let name = syn::Ident::new(
        &format!("set_{}s_n", ty.to_string().to_lowercase()),
        ty.span(),
    );

    let rust_ty_for_setter = ty.rust_type_quote_for_setter();
    let sys_setter_fn_ident = ty.sys_setter_fn_ident_array();

    output.push(quote! {
        /// ## SAFETY
        ///
        /// - `suite` must be a valid pointer to
        ///   [`crate::sys_umbrella::OfxPropertySuiteV1`].
        /// - `handle` must be a valid handle of
        ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
        /// - The type of `property`'s value must match the type this function
        ///   is specialized for.
        #[inline(always)]
        pub unsafe fn #name<const N: usize>(
            suite: *const crate::sys_umbrella::OfxPropertySuiteV1,
            handle: crate::sys_umbrella::OfxPropertySetHandle,
            property: *const std::os::raw::c_char,
            value: [#rust_ty_for_setter; N],
        ) -> Result<(), crate::sys_umbrella::OfxStatus> {
            // SAFETY: granted by the standard
            let suite_fn = unsafe { (&*suite).#sys_setter_fn_ident.unwrap_unchecked() };
            if let s = unsafe { suite_fn(handle, property, N as std::os::raw::c_int, value.as_ptr()) }
                && s != crate::sys_umbrella::kOfxStatOK {
                Err(s)
            } else {
                Ok(())
            }
        }
    });
}

fn make_property_getter_for_type(
    output: &mut Vec<proc_macro2::TokenStream>,
    ty: &OpenFXTypeSys,
    container_type: ContainerType,
) {
    let name_suffix = container_type.name_suffix(ty);
    let name = syn::Ident::new(&format!("get_{}", name_suffix), ty.span());

    let sys_getter_fn_ident = match container_type {
        ContainerType::Single => ty.sys_getter_fn_ident_single(),
        ContainerType::Array => ty.sys_getter_fn_ident_array(),
    };

    match container_type {
        ContainerType::Single => {
            let rust_ty_for_getter = ty.rust_type_quote_for_getter();
            let value_type = quote! { #rust_ty_for_getter };
            output.push(quote! {
                /// ## SAFETY
                ///
                /// - `suite` must be a valid pointer to
                ///   [`crate::sys_umbrella::OfxPropertySuiteV1`].
                /// - `handle` must be a valid handle of
                ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
                /// - The type of `property`'s value must match the type this function
                ///   is specialized for.
                #[inline(always)]
                pub unsafe fn #name(
                    suite: *const crate::sys_umbrella::OfxPropertySuiteV1,
                    handle: crate::sys_umbrella::OfxPropertySetHandle,
                    property: *const std::os::raw::c_char,
                ) -> Result<#value_type, crate::sys_umbrella::OfxStatus> {
                    // SAFETY: granted by the standard
                    let suite_fn = unsafe { (&*suite).#sys_getter_fn_ident.unwrap_unchecked() };
                    let mut value: #value_type = std::mem::zeroed();
                    if let s = unsafe { suite_fn(handle, property, 0, &mut value) }
                        && s != crate::sys_umbrella::kOfxStatOK {
                        Err(s)
                    } else {
                        Ok(value)
                    }
                }
            });
        }
        ContainerType::Array => {
            let rust_ty_for_getter = ty.rust_type_quote_for_getter();
            output.push(quote! {
                /// ## SAFETY
                ///
                /// - `suite` must be a valid pointer to
                ///   [`crate::sys_umbrella::OfxPropertySuiteV1`].
                /// - `handle` must be a valid handle of
                ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
                /// - The type of `property`'s value must match the type this function
                ///   is specialized for.
                #[inline(always)]
                pub unsafe fn #name(
                    suite: *const crate::sys_umbrella::OfxPropertySuiteV1,
                    handle: crate::sys_umbrella::OfxPropertySetHandle,
                    property: *const std::os::raw::c_char,
                    values: &mut [#rust_ty_for_getter],
                ) -> Result<(), crate::sys_umbrella::OfxStatus> {
                    // SAFETY: granted by the standard
                    let suite_fn = unsafe { (&*suite).#sys_getter_fn_ident.unwrap_unchecked() };
                    let count = values.len() as std::os::raw::c_int;
                    if let s = unsafe { suite_fn(handle, property, count, values.as_mut_ptr()) }
                        && s != crate::sys_umbrella::kOfxStatOK {
                        Err(s)
                    } else {
                        Ok(())
                    }
                }
            });
        }
    }
}

fn make_property_getter_for_fixed_array(
    output: &mut Vec<proc_macro2::TokenStream>,
    ty: &OpenFXTypeSys,
) {
    let name = syn::Ident::new(
        &format!("get_{}s_n", ty.to_string().to_lowercase()),
        ty.span(),
    );

    let sys_getter_fn_ident = ty.sys_getter_fn_ident_array();

    let rust_ty_for_getter = ty.rust_type_quote_for_getter();
    let value_type = quote! { [#rust_ty_for_getter; N] };
    output.push(quote! {
        /// ## SAFETY
        ///
        /// - `suite` must be a valid pointer to
        ///   [`crate::sys_umbrella::OfxPropertySuiteV1`].
        /// - `handle` must be a valid handle of
        ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
        /// - The type of `property`'s value must match the type this function
        ///   is specialized for.
        #[inline(always)]
        pub unsafe fn #name<const N: usize>(
            suite: *const crate::sys_umbrella::OfxPropertySuiteV1,
            handle: crate::sys_umbrella::OfxPropertySetHandle,
            property: *const std::os::raw::c_char,
        ) -> Result<#value_type, crate::sys_umbrella::OfxStatus> {
            // SAFETY: granted by the standard
            let suite_fn = unsafe { (&*suite).#sys_getter_fn_ident.unwrap_unchecked() };
            let mut value: #value_type = std::mem::zeroed();
            if let s = unsafe { suite_fn(handle, property, N as std::os::raw::c_int, value.as_mut_ptr()) }
                && s != crate::sys_umbrella::kOfxStatOK {
                Err(s)
            } else {
                Ok(value)
            }
        }
    });
}

mod kw {
    syn::custom_keyword!(Int);
    syn::custom_keyword!(Double);
    syn::custom_keyword!(String);
    syn::custom_keyword!(Pointer);
}

/// ## Examples
///
/// ```rust,ignore
/// openfx_internal_macros::sys_helpers_make_property_accessors_by_types! {
///     Int; Double; String; Pointer;
/// }
/// ```
struct Input {
    int: kw::Int,
    double: kw::Double,
    string: kw::String,
    pointer: kw::Pointer,
}

impl syn::parse::Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let int = input.parse::<kw::Int>()?;
        input.parse::<syn::Token![;]>()?;
        let double = input.parse::<kw::Double>()?;
        input.parse::<syn::Token![;]>()?;
        let string = input.parse::<kw::String>()?;
        input.parse::<syn::Token![;]>()?;
        let pointer = input.parse::<kw::Pointer>()?;
        input.parse::<syn::Token![;]>()?;
        Ok(Input {
            int,
            double,
            string,
            pointer,
        })
    }
}

impl std::iter::IntoIterator for Input {
    type Item = OpenFXTypeSys;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            OpenFXTypeSys::Int(syn::Ident::new("Int", self.int.span)),
            OpenFXTypeSys::Double(syn::Ident::new("Double", self.double.span)),
            OpenFXTypeSys::String(syn::Ident::new("String", self.string.span)),
            OpenFXTypeSys::Pointer(syn::Ident::new("Pointer", self.pointer.span)),
        ]
        .into_iter()
    }
}

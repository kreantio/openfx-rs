use std::collections::HashSet;

use proc_macro::TokenStream;
use quote::quote;

use crate::sys_helpers_macros::common::OpenFXTypeIdent;

pub fn make_property_accessors_by_types(tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(tokens as MakePropertyAccessorsByTypesInput);

    let mut output: Vec<proc_macro2::TokenStream> = Vec::new();

    for item in input.items {
        {
            // array
            make_property_setter_for_type(
                &mut output,
                &item.array.set_ident,
                &item.ty,
                ContainerType::Array,
                &item.array.vis,
            );
            make_property_getter_for_type(
                &mut output,
                &item.array.get_ident,
                &item.ty,
                ContainerType::Array,
                &item.array.vis,
            );
        }

        {
            // single
            make_property_setter_for_type(
                &mut output,
                &item.single.set_ident,
                &item.ty,
                ContainerType::Single,
                &item.single.vis,
            );
            make_property_getter_for_type(
                &mut output,
                &item.single.get_ident,
                &item.ty,
                ContainerType::Single,
                &item.single.vis,
            );
        }

        // fixed array
        if let Some(fixed_array) = item.fixed_array {
            let mut possible_sizes: Vec<usize> = fixed_array.possible_sizes.into_iter().collect();
            possible_sizes.sort();
            for size in possible_sizes {
                make_property_setter_for_type(
                    &mut output,
                    &fixed_array.set_ident,
                    &item.ty,
                    ContainerType::FixedArray(size),
                    &fixed_array.vis,
                );
                make_property_getter_for_type(
                    &mut output,
                    &fixed_array.get_ident,
                    &item.ty,
                    ContainerType::FixedArray(size),
                    &fixed_array.vis,
                );
            }
        }
    }

    quote! {
        #(#output)*
    }
    .into()
}

enum ContainerType {
    Single,
    Array,
    FixedArray(usize),
}

impl ContainerType {
    fn name_suffix(&self, ty: &OpenFXTypeIdent) -> String {
        match self {
            ContainerType::Single => ty.to_string().to_lowercase(),
            ContainerType::Array => format!("{}s", ty.to_string().to_lowercase()),
            ContainerType::FixedArray(size) => {
                format!("{}s_{}", ty.to_string().to_lowercase(), size)
            }
        }
    }
}

fn make_property_setter_for_type(
    output: &mut Vec<proc_macro2::TokenStream>,
    set_ident: &syn::Ident,
    ty: &OpenFXTypeIdent,
    container_type: ContainerType,
    vis: &syn::Visibility,
) {
    let name_suffix = container_type.name_suffix(ty);
    let name = syn::Ident::new(&format!("set_{}", name_suffix), set_ident.span());

    let rust_ty_for_setter = ty.rust_type_quote_for_setter();
    let value_type = match container_type {
        ContainerType::Single => quote! { #rust_ty_for_setter },
        ContainerType::Array => quote! { &[#rust_ty_for_setter] },
        ContainerType::FixedArray(size) => quote! { [#rust_ty_for_setter; #size] },
    };
    let sys_setter_fn_ident = match container_type {
        ContainerType::Single => ty.sys_setter_fn_ident_single(),
        ContainerType::Array | ContainerType::FixedArray(_) => ty.sys_setter_fn_ident_array(),
    };
    let suite_fn_args = match container_type {
        ContainerType::Single => quote! { 0, value },
        ContainerType::Array => quote! { value.len() as std::os::raw::c_int, value.as_ptr() },
        ContainerType::FixedArray(size) => quote! { #size as std::os::raw::c_int, value.as_ptr() },
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
        #vis unsafe fn #name(
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

fn make_property_getter_for_type(
    output: &mut Vec<proc_macro2::TokenStream>,
    get_ident: &syn::Ident,
    ty: &OpenFXTypeIdent,
    container_type: ContainerType,
    vis: &syn::Visibility,
) {
    let name_suffix = container_type.name_suffix(ty);
    let name = syn::Ident::new(&format!("get_{}", name_suffix), get_ident.span());

    let sys_getter_fn_ident = match container_type {
        ContainerType::Single => ty.sys_getter_fn_ident_single(),
        ContainerType::Array | ContainerType::FixedArray(_) => ty.sys_getter_fn_ident_array(),
    };

    if matches!(container_type, ContainerType::Array) {
        let rust_ty_for_getter = ty.rust_type_quote_for_getter();
        let value_type = quote! { &mut [#rust_ty_for_getter] };
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
            #vis unsafe fn #name(
                suite: *const crate::sys_umbrella::OfxPropertySuiteV1,
                handle: crate::sys_umbrella::OfxPropertySetHandle,
                property: *const std::os::raw::c_char,
                values: #value_type,
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
    } else {
        let rust_ty_for_getter = ty.rust_type_quote_for_getter();
        let value_type = match container_type {
            ContainerType::Single => quote! { #rust_ty_for_getter },
            ContainerType::FixedArray(size) => quote! { [#rust_ty_for_getter; #size] },
            _ => unreachable!(),
        };
        let suite_fn_args = match container_type {
            ContainerType::Single => quote! { 0, &mut value },
            ContainerType::FixedArray(size) => {
                quote! { #size as std::os::raw::c_int, value.as_mut_ptr() }
            }
            ContainerType::Array => unreachable!(),
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
            #vis unsafe fn #name(
                suite: *const crate::sys_umbrella::OfxPropertySuiteV1,
                handle: crate::sys_umbrella::OfxPropertySetHandle,
                property: *const std::os::raw::c_char,
            ) -> Result<#value_type, crate::sys_umbrella::OfxStatus> {
                // SAFETY: granted by the standard
                let suite_fn = unsafe { (&*suite).#sys_getter_fn_ident.unwrap_unchecked() };
                let mut value: #value_type = std::mem::zeroed();
                if let s = unsafe { suite_fn(handle, property, #suite_fn_args) }
                    && s != crate::sys_umbrella::kOfxStatOK {
                    Err(s)
                } else {
                    Ok(value)
                }
            }
        });
    }
}

/// ## Examples
///
/// ```rust,ignore
/// openfx_internal_macros::sys_helpers_make_property_accessors_by_types! {
///     Double: ... pub { set get }, 1 pub { set get }, (2|3|4) pub(crate) { set get };
///     Int: ... pub { set get }, 1 pub { set get }, (2|4) pub(crate) { set get };
///     Pointer: ... pub { set get }, 1 pub { set get };
///     String: ... pub { set get }, 1 pub { set get }, 2 pub(crate) { set get };
/// }
/// ```
struct MakePropertyAccessorsByTypesInput {
    items: syn::punctuated::Punctuated<MakePropertyAccessorsByTypesInputItem, syn::Token![;]>,
}

impl syn::parse::Parse for MakePropertyAccessorsByTypesInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let items =
            input.parse_terminated(MakePropertyAccessorsByTypesInputItem::parse, syn::Token![;])?;
        Ok(MakePropertyAccessorsByTypesInput { items })
    }
}

struct MakePropertyAccessorsByTypesInputItem {
    ty: OpenFXTypeIdent,
    array: MakePropertyAccessorsByTypesInputItemArrayOrSingle,
    single: MakePropertyAccessorsByTypesInputItemArrayOrSingle,
    fixed_array: Option<MakePropertyAccessorsByTypesInputItemFixedArray>,
}

impl syn::parse::Parse for MakePropertyAccessorsByTypesInputItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ty: OpenFXTypeIdent = input.parse()?;
        input.parse::<syn::Token![:]>()?;
        input.parse::<syn::Token![...]>()?;
        let array: MakePropertyAccessorsByTypesInputItemArrayOrSingle = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        input.parse::<syn::LitInt>()?;
        let single: MakePropertyAccessorsByTypesInputItemArrayOrSingle = input.parse()?;
        let fixed_array = if input.peek(syn::Token![,]) {
            input.parse::<syn::Token![,]>()?;
            Some(input.parse::<MakePropertyAccessorsByTypesInputItemFixedArray>()?)
        } else {
            None
        };
        Ok(MakePropertyAccessorsByTypesInputItem {
            ty,
            array,
            single,
            fixed_array,
        })
    }
}

struct MakePropertyAccessorsByTypesInputItemArrayOrSingle {
    vis: syn::Visibility,
    set_ident: syn::Ident,
    get_ident: syn::Ident,
}

impl syn::parse::Parse for MakePropertyAccessorsByTypesInputItemArrayOrSingle {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let vis: syn::Visibility = input.parse()?;
        let content;
        syn::braced!(content in input);
        let set_ident: syn::Ident = content.parse()?;
        let get_ident: syn::Ident = content.parse()?;
        Ok(MakePropertyAccessorsByTypesInputItemArrayOrSingle {
            vis,
            set_ident,
            get_ident,
        })
    }
}

struct MakePropertyAccessorsByTypesInputItemFixedArray {
    possible_sizes: HashSet<usize>,
    vis: syn::Visibility,
    set_ident: syn::Ident,
    get_ident: syn::Ident,
}

impl syn::parse::Parse for MakePropertyAccessorsByTypesInputItemFixedArray {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut possible_sizes = HashSet::new();
        if input.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);
            while !content.is_empty() {
                let size: syn::LitInt = content.parse()?;
                possible_sizes.insert(size.base10_parse::<usize>()?);
                if content.peek(syn::Token![|]) {
                    content.parse::<syn::Token![|]>()?;
                } else if !content.is_empty() {
                    return Err(content.error("expected `|` between fixed array sizes"));
                }
            }
        } else {
            let size: syn::LitInt = input.parse()?;
            possible_sizes.insert(size.base10_parse::<usize>()?);
        }
        let vis: syn::Visibility = input.parse()?;
        let content;
        syn::braced!(content in input);
        let set_ident: syn::Ident = content.parse()?;
        let get_ident: syn::Ident = content.parse()?;
        Ok(MakePropertyAccessorsByTypesInputItemFixedArray {
            possible_sizes,
            vis,
            set_ident,
            get_ident,
        })
    }
}

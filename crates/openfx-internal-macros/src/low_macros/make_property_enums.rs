use proc_macro::TokenStream;
use quote::quote;

use crate::low_macros::utils;

pub fn make_property_enums(attr: TokenStream, item: TokenStream) -> TokenStream {
    if let Err(error) = utils::err_if_attribute_is_not_empty(attr) {
        return error;
    }

    let input = syn::parse_macro_input!(item as Input);

    let mut output = proc_macro2::TokenStream::new();

    for enum_item in input.items {
        let name = &enum_item.name;

        // enum
        {
            let mut variant_items: Vec<proc_macro2::TokenStream> = vec![];
            for variant in &enum_item.variants {
                let var_name = &variant.name;
                let var_doc = if let InputEnumVariantAttribute::Sys(sys) = &variant.value {
                    let docstr = format!("See: [`crate::sys_umbrella::{sys}`]");
                    Some(quote! { #[doc = #docstr] })
                } else {
                    None
                };
                variant_items.push(quote! {
                    #var_doc
                    #var_name,
                });
            }
            output.extend(quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq)]
                #[non_exhaustive]
                pub enum #name {
                    #(#variant_items)*
                }
            });
        }

        // impl
        {
            let vars = enum_item
                .variants
                .iter()
                .map(|v| &v.name)
                .collect::<Vec<_>>();
            let vals = enum_item
                .variants
                .iter()
                .map(|v| match &v.value {
                    InputEnumVariantAttribute::Sys(sys) => {
                        quote! { crate::sys_umbrella::#sys }
                    }
                    InputEnumVariantAttribute::SysLiteral(sys_literal) => {
                        quote! { #sys_literal }
                    }
                })
                .collect::<Vec<_>>();

            output.extend(quote! {
                impl crate::low_plugin::property_sets::KnownEnumProperty for #name {
                    /// ## Safety
                    ///
                    /// `ptr` must be valid and point to a null-terminated C
                    /// string.
                    ///
                    /// See the Safety section of [`Self`] for more information.
                    unsafe fn try_from_ptr(ptr: std::ptr::NonNull<std::os::raw::c_char>) -> Option<Self> {
                        let cstr = unsafe { std::ffi::CStr::from_ptr(ptr.as_ptr()) };
                        match true {
                            #( _ if cstr == #vals => Some(Self::#vars), )*
                            _ => None,
                        }
                    }

                    fn as_cstr(&self) -> &std::ffi::CStr {
                        match self {
                            #( Self::#vars => #vals, )*
                        }
                    }
                }

                impl From<#name> for crate::low_plugin::property_sets::EnumPropertyPtr<#name> {
                    fn from(value: #name) -> Self {
                        let ptr = crate::low_plugin::property_sets::KnownEnumProperty::as_cstr(&value).as_ptr();
                        Self::new(std::ptr::NonNull::new(ptr as *mut std::os::raw::c_char))
                    }
                }

                impl PartialEq<crate::low_plugin::property_sets::LosslessEnumProperty<#name>> for #name {
                    fn eq(&self, other: &crate::low_plugin::property_sets::LosslessEnumProperty<#name>) -> bool {
                        other == self
                    }
                }

                impl PartialEq<crate::low_plugin::property_sets::LosslessEnumPropertyOwned<#name>> for #name {
                    fn eq(&self, other: &crate::low_plugin::property_sets::LosslessEnumPropertyOwned<#name>) -> bool {
                        other == self
                    }
                }
            });
        }
    }

    output.into()
}

/// ## Examples
///
/// ```rust,ignore
/// #[make_property_enums]
/// mod __ {
///     enum ImageClipPropFieldOrder {
///         #[sys(kOfxImageFieldLower)]
///         Lower,
///         #[sys(kOfxImageFieldNone)]
///         None,
///         #[sys(kOfxImageFieldUpper)]
///         Upper,
///     }
///     enum ImageEffectPropCPURenderSupported {
///         #[sys_literal(c"false")]
///         False,
///         #[sys_literal(c"true")]
///         True,
///     }
///     // …
/// }
/// ```
struct Input {
    items: Vec<InputEnumItem>,
}

impl syn::parse::Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content = utils::extract_dummy_mod_content(input)?;

        let mut items = Vec::new();
        while !content.is_empty() {
            let item = content.parse::<InputEnumItem>()?;
            items.push(item);
        }
        Ok(Input { items })
    }
}

struct InputEnumItem {
    name: syn::Ident,
    variants: syn::punctuated::Punctuated<InputEnumVariant, syn::Token![,]>,
}

impl syn::parse::Parse for InputEnumItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _enum_token = input.parse::<syn::Token![enum]>()?;
        let name = input.parse::<syn::Ident>()?;
        let content;
        let _brace_token = syn::braced!(content in input);
        let variants = content.parse_terminated(InputEnumVariant::parse, syn::Token![,])?;
        Ok(InputEnumItem { name, variants })
    }
}

struct InputEnumVariant {
    value: InputEnumVariantAttribute,
    name: syn::Ident,
}

impl syn::parse::Parse for InputEnumVariant {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let value = input.parse::<InputEnumVariantAttribute>()?;
        let name = input.parse::<syn::Ident>()?;
        Ok(InputEnumVariant { value, name })
    }
}

enum InputEnumVariantAttribute {
    Sys(syn::Ident),
    SysLiteral(syn::LitCStr),
}

impl syn::parse::Parse for InputEnumVariantAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _pound_token = input.parse::<syn::Token![#]>()?;
        let content;
        let _bracket_token = syn::bracketed!(content in input);
        let attr_name = content.parse::<syn::Ident>()?;
        let inner;
        let _paren_token = syn::parenthesized!(inner in content);
        let result = if attr_name == "sys" {
            InputEnumVariantAttribute::Sys(inner.parse::<syn::Ident>()?)
        } else if attr_name == "sys_literal" {
            InputEnumVariantAttribute::SysLiteral(inner.parse::<syn::LitCStr>()?)
        } else {
            return Err(syn::Error::new_spanned(
                attr_name,
                "expected `sys` or `sys_literal`",
            ));
        };
        if !inner.is_empty() {
            return Err(inner.error("unexpected tokens in attribute"));
        }
        if !content.is_empty() {
            return Err(content.error("unexpected tokens in attribute"));
        }
        Ok(result)
    }
}

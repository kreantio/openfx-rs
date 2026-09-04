use proc_macro::TokenStream;
use quote::quote;

pub fn make_property_enums_from_c(tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(tokens as Input);

    let mut output = proc_macro2::TokenStream::new();

    for enum_item in input.items {
        let name = &enum_item.name;
        let sys_name = &enum_item.sys_name;

        // enum
        {
            let mut inner = proc_macro2::TokenStream::new();
            for variant in &enum_item.variants {
                let sys_variant_name = &variant.sys_name;
                let path_str = format!("crate::sys_umbrella::{}", sys_variant_name);
                let docstr = format!("See: [`{}`].", path_str);
                inner.extend(quote! { #[doc = #docstr] });
                let var_name = &variant.name;
                inner.extend(quote! { #var_name, });
            }
            inner.extend(quote! {
                /// Use [`Self::matches`] instead. Even if matching this variant
                /// for unrecognized values works now, it will break when this
                /// crate adds dedicated variants for those values in a future
                /// release.
                UnknownDontUseThisDirectly(crate::sys_umbrella::#sys_name),
            });
            output.extend(quote! {
                #[derive(Debug, Clone, PartialEq, Eq)]
                #[non_exhaustive]
                pub enum #name { #inner }
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
                .map(|v| {
                    let sys_variant_name = &v.sys_name;
                    quote! { crate::sys_umbrella::#sys_variant_name }
                })
                .collect::<Vec<_>>();

            output.extend(quote! {
                impl #name {
                    pub fn from_sys(val: crate::sys_umbrella::#sys_name) -> Self {
                        match val {
                            #( #vals => Self::#vars, )*
                            _ => Self::UnknownDontUseThisDirectly(val),
                        }
                    }

                    pub fn as_sys(&self) -> crate::sys_umbrella::#sys_name {
                        match self {
                            #( Self::#vars => #vals, )*
                            Self::UnknownDontUseThisDirectly(val) => *val,
                        }
                    }

                    pub fn matches(&self, val: crate::sys_umbrella::#sys_name) -> bool {
                        self.as_sys() == val
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
/// make_property_enums_from_c! {
///     #[sys(OfxDrawLineStipplePattern)]
///     enum DrawLineStipplePattern {
///         #[sys(kOfxDrawLineStipplePatternAltDash)]
///         AltDash,
///         #[sys(kOfxDrawLineStipplePatternDash)]
///         Dash,
///         // …
///     }
///     // …
/// }
/// ```
struct Input {
    items: Vec<InputEnumItem>,
}

impl syn::parse::Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut items = Vec::new();
        while !input.is_empty() {
            let item = input.parse::<InputEnumItem>()?;
            items.push(item);
        }
        Ok(Input { items })
    }
}

struct InputEnumItem {
    sys_name: syn::Ident,
    name: syn::Ident,
    variants: syn::punctuated::Punctuated<InputEnumVariant, syn::Token![,]>,
}

impl syn::parse::Parse for InputEnumItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let sys_name = input.parse::<InputEnumVariantAttributeSys>()?.sys;
        let _enum_token = input.parse::<syn::Token![enum]>()?;
        let name = input.parse::<syn::Ident>()?;
        let content;
        let _brace_token = syn::braced!(content in input);
        let variants = content.parse_terminated(InputEnumVariant::parse, syn::Token![,])?;
        Ok(InputEnumItem {
            sys_name,
            name,
            variants,
        })
    }
}

struct InputEnumVariant {
    sys_name: syn::Ident,
    name: syn::Ident,
}

impl syn::parse::Parse for InputEnumVariant {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let sys_name = input.parse::<InputEnumVariantAttributeSys>()?.sys;
        let name = input.parse::<syn::Ident>()?;
        Ok(InputEnumVariant { sys_name, name })
    }
}

struct InputEnumVariantAttributeSys {
    sys: syn::Ident,
}

impl syn::parse::Parse for InputEnumVariantAttributeSys {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _pound_token = input.parse::<syn::Token![#]>()?;
        let content;
        let _bracket_token = syn::bracketed!(content in input);
        let attr_name = content.parse::<syn::Ident>()?;
        if attr_name != "sys" {
            return Err(syn::Error::new_spanned(attr_name, "expected `sys`"));
        }
        let inner;
        let _paren_token = syn::parenthesized!(inner in content);
        let sys = inner.parse::<syn::Ident>()?;
        if !inner.is_empty() {
            return Err(inner.error("unexpected tokens in `sys` attribute"));
        }
        if !content.is_empty() {
            return Err(content.error("unexpected tokens in attribute"));
        }
        Ok(InputEnumVariantAttributeSys { sys })
    }
}

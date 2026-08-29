use proc_macro::TokenStream;
use quote::quote;

pub fn make_property_enums(tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(tokens as MakePropertyEnumsInput);

    let mut output = proc_macro2::TokenStream::new();

    for enum_item in input.items {
        let name = &enum_item.name;

        // enum
        {
            let mut inner = proc_macro2::TokenStream::new();
            for variant in &enum_item.variants {
                if let MakePropertyEnumsInputEnumVariant::WithPath { path, .. } = &variant {
                    let path_str = path
                        .segments
                        .iter()
                        .map(|s| s.ident.to_string())
                        .collect::<Vec<_>>()
                        .join("::");
                    let docstr = format!("See: [`{}`].", path_str);
                    inner.extend(quote! { #[doc = #docstr] });
                }
                let var_name = variant.name();
                inner.extend(quote! { #var_name, });
            }
            inner.extend(quote! { Other(*const std::os::raw::c_char), });
            output.extend(quote! {
                #[derive(Debug, Clone)] pub enum #name { #inner }
            });
        }

        // impl
        {
            let vars = enum_item
                .variants
                .iter()
                .map(|v| v.name())
                .collect::<Vec<_>>();
            let vals = enum_item
                .variants
                .iter()
                .map(|v| match v {
                    MakePropertyEnumsInputEnumVariant::WithPath { path, .. } => {
                        quote! { #path }
                    }
                    MakePropertyEnumsInputEnumVariant::WithCStrLiteral { cstr_literal, .. } => {
                        quote! { #cstr_literal }
                    }
                })
                .collect::<Vec<_>>();

            output.extend(quote! {
                impl #name {
                    /// ## SAFETY
                    ///
                    /// - The pointer must be valid and point to a null-
                    ///   terminated C string.
                    /// - The pointer must live at least as long as the returned
                    ///   [`Self`] value.
                    pub unsafe fn from_ptr(ptr: *const std::os::raw::c_char) -> Self {
                        let cstr = unsafe { std::ffi::CStr::from_ptr(ptr) };
                        match true {
                            #( _ if cstr == #vals => Self::#vars, )*
                            _ => Self::Other(cstr.as_ptr()),
                        }
                    }

                    /// ## SAFETY
                    ///
                    /// The returned pointer is valid as long as the
                    /// [`std::ffi::CString`] inside [`Self::Other`] is not dropped.
                    pub fn as_ptr(&self) -> *const std::os::raw::c_char {
                        match self {
                            #( Self::#vars => #vals.as_ptr(), )*
                            Self::Other(ptr) => *ptr,
                        }
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
/// make_property_enums! {
///     ImageClipPropFieldOrder {
///         Lower => crate::sys_umbrella::kOfxImageFieldLower,
///         None => crate::sys_umbrella::kOfxImageFieldNone,
///         Upper => crate::sys_umbrella::kOfxImageFieldUpper,
///     }
///     ImageEffectPropCPURenderSupported {
///         False : c"false",
///         True : c"true",
///     }
/// }
/// ```
struct MakePropertyEnumsInput {
    items: Vec<MakePropertyEnumsInputEnumItem>,
}

impl syn::parse::Parse for MakePropertyEnumsInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut items = Vec::new();
        while !input.is_empty() {
            let item = input.parse::<MakePropertyEnumsInputEnumItem>()?;
            items.push(item);
        }
        Ok(MakePropertyEnumsInput { items })
    }
}

struct MakePropertyEnumsInputEnumItem {
    name: syn::Ident,
    variants: syn::punctuated::Punctuated<MakePropertyEnumsInputEnumVariant, syn::Token![,]>,
}

impl syn::parse::Parse for MakePropertyEnumsInputEnumItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<syn::Ident>()?;
        let content;
        let _brace_token = syn::braced!(content in input);
        let variants =
            content.parse_terminated(MakePropertyEnumsInputEnumVariant::parse, syn::Token![,])?;
        Ok(MakePropertyEnumsInputEnumItem { name, variants })
    }
}

enum MakePropertyEnumsInputEnumVariant {
    WithPath {
        name: syn::Ident,
        path: syn::Path,
    },
    WithCStrLiteral {
        name: syn::Ident,
        cstr_literal: syn::LitCStr,
    },
}

impl syn::parse::Parse for MakePropertyEnumsInputEnumVariant {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<syn::Ident>()?;
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::Token![:]) {
            let _colon_token = input.parse::<syn::Token![:]>()?;
            let cstr_literal = input.parse::<syn::LitCStr>()?;
            Ok(MakePropertyEnumsInputEnumVariant::WithCStrLiteral { name, cstr_literal })
        } else if lookahead.peek(syn::Token![=>]) {
            let _arrow_token = input.parse::<syn::Token![=>]>()?;
            let path = input.parse::<syn::Path>()?;
            Ok(MakePropertyEnumsInputEnumVariant::WithPath { name, path })
        } else {
            Err(lookahead.error())
        }
    }
}

impl MakePropertyEnumsInputEnumVariant {
    fn name(&self) -> &syn::Ident {
        match self {
            MakePropertyEnumsInputEnumVariant::WithPath { name, .. } => name,
            MakePropertyEnumsInputEnumVariant::WithCStrLiteral { name, .. } => name,
        }
    }
}

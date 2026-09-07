use proc_macro::TokenStream;
use quote::quote;

pub fn make_property_enums(tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(tokens as Input);

    let mut output = proc_macro2::TokenStream::new();

    for enum_item in input.items {
        let name = &enum_item.name;

        // enum
        {
            let mut inner = proc_macro2::TokenStream::new();
            for variant in &enum_item.variants {
                if let InputEnumVariantAttribute::Sys(sys) = &variant.value {
                    let path_str = format!("crate::sys_umbrella::{}", sys);
                    let docstr = format!("See: [`{}`].", path_str);
                    inner.extend(quote! { #[doc = #docstr] });
                }
                let var_name = &variant.name;
                inner.extend(quote! { #var_name, });
            }
            inner.extend(quote! {
                /// Use [`Self::matches`] instead. Even if matching this variant
                /// for unrecognized values works now, it will break when this
                /// crate adds dedicated variants for those values in a future
                /// release.
                UnknownDontUseThisDirectly(*const std::os::raw::c_char),
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
                            _ => Self::UnknownDontUseThisDirectly(cstr.as_ptr()),
                        }
                    }

                    /// ## SAFETY
                    ///
                    /// - The pointer must be either null or valid and point to
                    ///   a null-terminated C string.
                    /// - The pointer must live at least as long as the returned
                    ///   [`Self`] value.
                    pub unsafe fn from_ptr_null_checked(ptr: *const std::os::raw::c_char) -> Self {
                        if ptr.is_null() {
                            Self::UnknownDontUseThisDirectly(std::ptr::null())
                        } else {
                            Self::from_ptr(ptr)
                        }
                    }

                    /// ## SAFETY
                    ///
                    /// The returned pointer is valid as long as the
                    /// [`std::ffi::CString`] inside
                    /// [`Self::UnknownDontUseThisDirectly`] is not dropped.
                    pub fn as_ptr(&self) -> *const std::os::raw::c_char {
                        match self {
                            #( Self::#vars => #vals.as_ptr(), )*
                            Self::UnknownDontUseThisDirectly(ptr) => *ptr,
                        }
                    }

                    /// ## SAFETY
                    ///
                    /// - The pointer must be either null or valid and point to
                    ///   a null-terminated C string.
                    /// - The pointer must live at least as long as the returned
                    ///   [`Self`] value.
                    pub unsafe fn matches(&self, val: &::std::ffi::CStr) -> bool {
                        let ptr = self.as_ptr();
                        if ptr.is_null() {
                            false
                        } else {
                            ::std::ffi::CStr::from_ptr(ptr) == val
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
        let mut items = Vec::new();
        while !input.is_empty() {
            let item = input.parse::<InputEnumItem>()?;
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

use quote::{quote, quote_spanned};

#[derive(Clone)]
pub enum SimpleType {
    Int(Option<syn::Ident>),
    Double(Option<syn::Ident>),
    String(Option<syn::Ident>),
    Pointer(Option<syn::Ident>),
}

impl syn::parse::Parse for SimpleType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: syn::Ident = input.parse()?;
        match ident.to_string().as_str() {
            "Int" => Ok(SimpleType::Int(Some(ident))),
            "Double" => Ok(SimpleType::Double(Some(ident))),
            "String" => Ok(SimpleType::String(Some(ident))),
            "Pointer" => Ok(SimpleType::Pointer(Some(ident))),
            _ => Err(syn::Error::new_spanned(
                ident,
                "Expected one of: Int, Double, String, Pointer",
            )),
        }
    }
}

impl std::fmt::Display for SimpleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimpleType::Int(_) => write!(f, "Int"),
            SimpleType::Double(_) => write!(f, "Double"),
            SimpleType::String(_) => write!(f, "String"),
            SimpleType::Pointer(_) => write!(f, "Pointer"),
        }
    }
}

impl SimpleType {
    pub fn new_int() -> Self {
        SimpleType::Int(None)
    }
    pub fn new_double() -> Self {
        SimpleType::Double(None)
    }
    pub fn new_string() -> Self {
        SimpleType::String(None)
    }
    pub fn new_pointer() -> Self {
        SimpleType::Pointer(None)
    }

    pub fn name_snake_case(&self) -> &str {
        match self {
            Self::Int(_) => "int",
            Self::Double(_) => "double",
            Self::String(_) => "string",
            Self::Pointer(_) => "pointer",
        }
    }

    pub fn quote_rust_type(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Int(ident) => {
                let c_int = if let Some(ident) = ident {
                    quote_spanned! { ident.span() => c_int }
                } else {
                    quote! { c_int }
                };
                quote! { ::std::ffi::#c_int }
            }
            Self::Double(ident) => {
                let f64 = if let Some(ident) = ident {
                    quote_spanned! { ident.span() => f64 }
                } else {
                    quote! { f64 }
                };
                quote! { ::std::primitive::#f64 }
            }
            Self::String(ident) => {
                let c_char = if let Some(ident) = ident {
                    quote_spanned! { ident.span() => c_char }
                } else {
                    quote! { c_char }
                };
                quote! { ::std::option::Option<::std::ptr::NonNull<::std::ffi::#c_char>> }
            }
            Self::Pointer(ident) => {
                let c_void = if let Some(ident) = ident {
                    quote_spanned! { ident.span() => c_void }
                } else {
                    quote! { c_void }
                };
                quote! { ::std::option::Option<::std::ptr::NonNull<::std::ffi::#c_void>> }
            }
        }
    }

    pub fn quote_rust_ffi_type(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Int(ident) => {
                let c_int = if let Some(ident) = ident {
                    quote_spanned! { ident.span() => c_int }
                } else {
                    quote! { c_int }
                };
                quote! { ::std::ffi::#c_int }
            }
            Self::Double(ident) => {
                let f64 = if let Some(ident) = ident {
                    quote_spanned! { ident.span() => f64 }
                } else {
                    quote! { f64 }
                };
                quote! { ::std::primitive::#f64 }
            }
            Self::String(ident) => {
                let c_char = if let Some(ident) = ident {
                    quote_spanned! { ident.span() => c_char }
                } else {
                    quote! { c_char }
                };
                quote! { *mut ::std::ffi::#c_char }
            }
            Self::Pointer(ident) => {
                let c_void = if let Some(ident) = ident {
                    quote_spanned! { ident.span() => c_void }
                } else {
                    quote! { c_void }
                };
                quote! { *mut ::std::ffi::#c_void }
            }
        }
    }
}

pub struct ParameterType {
    pub element_type: ParameterElementType,
    pub dimensions: usize,
}

pub enum ParameterElementType {
    Int(syn::Ident),
    Double(syn::Ident),
    String(syn::Ident),
    Pointer(syn::Ident),
    Bool(syn::Ident),
}

impl syn::parse::Parse for ParameterType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let type_content;
        syn::bracketed!(type_content in input);
        let element_type = type_content.parse::<syn::Ident>()?;
        let element_type = match element_type.to_string().as_str() {
            "Int" => ParameterElementType::Int(element_type.clone()),
            "Double" => ParameterElementType::Double(element_type.clone()),
            "String" => ParameterElementType::String(element_type.clone()),
            "Pointer" => ParameterElementType::Pointer(element_type.clone()),
            "Bool" => ParameterElementType::Bool(element_type.clone()),
            _ => {
                return Err(syn::Error::new_spanned(
                    element_type,
                    "expected `Int`, `Double`, `String`, `Pointer` or `Bool`",
                ));
            }
        };
        type_content.parse::<syn::Token![;]>()?;
        let dimensions = type_content.parse::<syn::LitInt>()?.base10_parse()?;
        if !type_content.is_empty() {
            return Err(type_content.error("unexpected tokens in monomorphization type"));
        }
        Ok(ParameterType {
            element_type,
            dimensions,
        })
    }
}

impl ParameterType {
    pub fn parse_option(input: syn::parse::ParseStream) -> syn::Result<Option<Self>> {
        if input.peek(syn::token::Bracket) {
            Ok(Some(input.parse()?))
        } else {
            let none = input.parse::<syn::Ident>()?;
            if none != "None" {
                return Err(syn::Error::new_spanned(
                    none,
                    "expected `[<type>; <dimensions>]` or `None`",
                ));
            }
            Ok(None)
        }
    }
}

impl std::fmt::Display for ParameterElementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(_) => write!(f, "Int"),
            Self::Double(_) => write!(f, "Double"),
            Self::String(_) => write!(f, "String"),
            Self::Pointer(_) => write!(f, "Pointer"),
            Self::Bool(_) => write!(f, "Bool"),
        }
    }
}

impl ParameterElementType {
    pub fn ident(&self) -> &syn::Ident {
        match self {
            Self::Int(ident) => ident,
            Self::Double(ident) => ident,
            Self::String(ident) => ident,
            Self::Pointer(ident) => ident,
            Self::Bool(ident) => ident,
        }
    }

    pub fn simple(&self) -> SimpleType {
        match self {
            Self::Int(ident) => SimpleType::Int(Some(ident.clone())),
            Self::Double(ident) => SimpleType::Double(Some(ident.clone())),
            Self::String(ident) => SimpleType::String(Some(ident.clone())),
            Self::Pointer(ident) => SimpleType::Pointer(Some(ident.clone())),
            Self::Bool(ident) => SimpleType::Int(Some(ident.clone())),
        }
    }
}

use quote::{quote, quote_spanned};

#[derive(Clone)]
pub enum OpenFXTypeSys {
    Int(syn::Ident),
    Double(syn::Ident),
    String(syn::Ident),
    Pointer(syn::Ident),
}

impl syn::parse::Parse for OpenFXTypeSys {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: syn::Ident = input.parse()?;
        match ident.to_string().as_str() {
            "Int" => Ok(OpenFXTypeSys::Int(ident)),
            "Double" => Ok(OpenFXTypeSys::Double(ident)),
            "String" => Ok(OpenFXTypeSys::String(ident)),
            "Pointer" => Ok(OpenFXTypeSys::Pointer(ident)),
            _ => Err(syn::Error::new_spanned(
                ident,
                "Expected one of: Int, Double, String, Pointer",
            )),
        }
    }
}

impl std::fmt::Display for OpenFXTypeSys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenFXTypeSys::Int(ident)
            | OpenFXTypeSys::Double(ident)
            | OpenFXTypeSys::String(ident)
            | OpenFXTypeSys::Pointer(ident) => {
                write!(f, "{}", ident)
            }
        }
    }
}

impl OpenFXTypeSys {
    pub fn span(&self) -> proc_macro2::Span {
        match self {
            OpenFXTypeSys::Int(ident)
            | OpenFXTypeSys::Double(ident)
            | OpenFXTypeSys::String(ident)
            | OpenFXTypeSys::Pointer(ident) => ident.span(),
        }
    }

    pub fn rust_type_quote_for_setter(&self) -> proc_macro2::TokenStream {
        match self {
            OpenFXTypeSys::Int(ident) => {
                let c_int = quote_spanned! { ident.span() => c_int };
                quote! { ::std::os::raw::#c_int }
            }
            OpenFXTypeSys::Double(ident) => {
                let f64 = quote_spanned! { ident.span() => f64 };
                quote! { ::core::primitive::#f64 }
            }
            OpenFXTypeSys::String(ident) => {
                let c_char = quote_spanned! { ident.span() => c_char };
                quote! { *const ::std::os::raw::#c_char }
            }
            OpenFXTypeSys::Pointer(ident) => {
                let c_void = quote_spanned! { ident.span() => c_void };
                quote! { *mut ::std::ffi::#c_void }
            }
        }
    }
    pub fn rust_type_quote_for_getter(&self) -> proc_macro2::TokenStream {
        match self {
            OpenFXTypeSys::Int(ident) => {
                let c_int = quote_spanned! { ident.span() => c_int };
                quote! { ::std::os::raw::#c_int }
            }
            OpenFXTypeSys::Double(ident) => {
                let f64 = quote_spanned! { ident.span() => f64 };
                quote! { ::core::primitive::#f64 }
            }
            OpenFXTypeSys::String(ident) => {
                let c_char = quote_spanned! { ident.span() => c_char };
                quote! { *mut ::std::os::raw::#c_char }
            }
            OpenFXTypeSys::Pointer(ident) => {
                let c_void = quote_spanned! { ident.span() => c_void };
                quote! { *mut ::std::ffi::#c_void }
            }
        }
    }
    pub fn sys_setter_fn_ident_single(&self) -> syn::Ident {
        let name = format!("propSet{}", self);
        syn::Ident::new(&name, self.span())
    }
    pub fn sys_setter_fn_ident_array(&self) -> syn::Ident {
        let name = format!("propSet{}N", self);
        syn::Ident::new(&name, self.span())
    }
    pub fn sys_getter_fn_ident_single(&self) -> syn::Ident {
        let name = format!("propGet{}", self);
        syn::Ident::new(&name, self.span())
    }
    pub fn sys_getter_fn_ident_array(&self) -> syn::Ident {
        let name = format!("propGet{}N", self);
        syn::Ident::new(&name, self.span())
    }
}

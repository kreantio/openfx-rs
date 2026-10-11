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
    pub fn name_snake_case(&self) -> &str {
        match self {
            OpenFXTypeSys::Int(_) => "int",
            OpenFXTypeSys::Double(_) => "double",
            OpenFXTypeSys::String(_) => "string",
            OpenFXTypeSys::Pointer(_) => "pointer",
        }
    }
}

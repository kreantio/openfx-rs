#[derive(Clone)]
pub enum OpenFXTypeSimple {
    Int(syn::Ident),
    Double(syn::Ident),
    String(syn::Ident),
    Pointer(syn::Ident),
}

impl syn::parse::Parse for OpenFXTypeSimple {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: syn::Ident = input.parse()?;
        match ident.to_string().as_str() {
            "Int" => Ok(OpenFXTypeSimple::Int(ident)),
            "Double" => Ok(OpenFXTypeSimple::Double(ident)),
            "String" => Ok(OpenFXTypeSimple::String(ident)),
            "Pointer" => Ok(OpenFXTypeSimple::Pointer(ident)),
            _ => Err(syn::Error::new_spanned(
                ident,
                "Expected one of: Int, Double, String, Pointer",
            )),
        }
    }
}

impl std::fmt::Display for OpenFXTypeSimple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenFXTypeSimple::Int(ident)
            | OpenFXTypeSimple::Double(ident)
            | OpenFXTypeSimple::String(ident)
            | OpenFXTypeSimple::Pointer(ident) => {
                write!(f, "{}", ident)
            }
        }
    }
}

impl OpenFXTypeSimple {
    pub fn name_snake_case(&self) -> &str {
        match self {
            OpenFXTypeSimple::Int(_) => "int",
            OpenFXTypeSimple::Double(_) => "double",
            OpenFXTypeSimple::String(_) => "string",
            OpenFXTypeSimple::Pointer(_) => "pointer",
        }
    }
}

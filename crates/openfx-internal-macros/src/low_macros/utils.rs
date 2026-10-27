use quote::quote;

pub fn err_if_attribute_is_not_empty(
    attr: proc_macro::TokenStream,
) -> Result<(), proc_macro::TokenStream> {
    if !attr.is_empty() {
        Err(syn::Error::new_spanned(
            proc_macro2::TokenStream::from(attr),
            "This attribute does not take any arguments.",
        )
        .to_compile_error()
        .into())
    } else {
        Ok(())
    }
}

pub fn extract_dummy_mod_content(
    input: syn::parse::ParseStream,
) -> syn::Result<syn::parse::ParseBuffer> {
    input.parse::<syn::Token![mod]>()?;
    let ident = input.parse::<syn::Ident>()?;
    if ident != "__" {
        return Err(syn::Error::new_spanned(ident, "expected module name `__`"));
    }

    let content;
    syn::braced!(content in input);

    Ok(content)
}

/// Returns whether the next tokens in `input` are a `#[<name> …]` attribute,
/// without consuming anything.
///
/// - Author: OpenCode 2 / GLM 5.3 (provider: either Z.AI or OpenCode Go,
///   thinking: Max)
/// - Reviewed-by: Umaĵo
pub fn peek_attribute(input: syn::parse::ParseStream, name: &str) -> bool {
    fn test(input: syn::parse::ParseStream, name: &str) -> syn::Result<bool> {
        input.parse::<syn::Token![#]>()?;
        let content;
        syn::bracketed!(content in input);
        let attr_name = content.parse::<syn::Ident>()?;
        Ok(attr_name == name)
    }

    let fork = input.fork();
    test(&fork, name).unwrap_or(false)
}

pub fn make_doc_see_sys(cname: &str) -> proc_macro2::TokenStream {
    let doc = format!("See: [`crate::sys_umbrella::{cname}`]");
    quote! {
        #[doc = #doc]
    }
}

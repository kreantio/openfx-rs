use proc_macro::TokenStream;

pub fn err_if_attribute_is_not_empty(attr: TokenStream) -> Result<(), TokenStream> {
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

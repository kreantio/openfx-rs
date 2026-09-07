use convert_case::Casing;
use proc_macro::TokenStream;
use quote::quote;

pub fn plugin_impl_host_for_fetch_suite(tokens: TokenStream) -> TokenStream {
    let Input {
        simple_ident,
        corrected_k_ident,
    } = syn::parse_macro_input!(tokens as Input);
    let simple_ident_str = simple_ident.to_string();
    let Some(v_index) = simple_ident_str.rfind('V') else {
        return syn::Error::new(simple_ident.span(), "expected identifier to contain 'V'")
            .to_compile_error()
            .into();
    };
    let Some(version_number) = simple_ident_str[v_index + 1..].parse::<i32>().ok() else {
        return syn::Error::new(
            simple_ident.span(),
            "expected a valid version number after 'V'",
        )
        .to_compile_error()
        .into();
    };
    let fn_ident = syn::Ident::new(
        &format!(
            "fetch_{}_v{}",
            simple_ident_str[..v_index]
                .to_owned()
                .to_case(convert_case::Case::Snake),
            version_number
        ),
        simple_ident.span(),
    );
    let k_ident = if let Some(corrected_k_ident) = &corrected_k_ident {
        corrected_k_ident.to_owned()
    } else {
        syn::Ident::new(
            &format!("kOfx{}", &simple_ident_str[..v_index]),
            simple_ident.span(),
        )
    };

    quote! {
        impl Host {
            pub unsafe fn #fn_ident(&self) -> Option<crate::low_plugin::suites::#simple_ident> {
                let sys_suite = self.sys_fetch_suite(crate::sys_umbrella::#k_ident, #version_number);
                crate::low_plugin::suites::#simple_ident::try_from(sys_suite)
            }
        }
    }.into()
}

struct Input {
    simple_ident: syn::Ident,
    corrected_k_ident: Option<syn::Ident>,
}

impl syn::parse::Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let simple_ident = input.parse()?;
        if input.peek(syn::Token![@]) {
            let _at_token: syn::Token![@] = input.parse()?;
            let corrected_k_ident = input.parse()?;
            Ok(Input {
                simple_ident,
                corrected_k_ident: Some(corrected_k_ident),
            })
        } else if !input.is_empty() {
            Err(input.error("unexpected input"))
        } else {
            Ok(Input {
                simple_ident,
                corrected_k_ident: None,
            })
        }
    }
}

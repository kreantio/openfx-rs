use proc_macro::TokenStream;
use quote::quote;

use crate::low_macros::utils::make_doc_see_sys;

/// This is copied from [`super::make_suite_struct::make_suite_struct`].
pub fn make_object_struct(tokens: TokenStream) -> TokenStream {
    let Input {
        simple_ident,
        full_ident,
    } = syn::parse_macro_input!(tokens as Input);

    let doc_see_sys = make_doc_see_sys(&full_ident.to_string());

    quote! {
        #doc_see_sys
        ///
        /// ## Safety
        ///
        /// This type is not `Send`, but it is `Sync`. For details, see the Safety
        /// section of [`crate::low_plugin::Host`], which has the same requirements.
        #[derive(Clone, Copy)]
        pub struct #simple_ident(crate::sys_umbrella::#full_ident);
        unsafe impl Sync for #simple_ident {}
        impl #simple_ident {
            pub fn from_sys_handle(sys_handle: crate::sys_umbrella::#full_ident) -> Self {
                Self(sys_handle)
            }
            pub fn from_sys_ptr(sys_ptr: *const ::std::os::raw::c_void) -> Self {
                Self(sys_ptr as crate::sys_umbrella::#full_ident)
            }
            pub fn sys_handle(&self) -> crate::sys_umbrella::#full_ident {
                self.0
            }
        }
        impl From<&#simple_ident> for crate::sys_umbrella::#full_ident {
            fn from(value: &#simple_ident) -> Self {
                value.sys_handle()
            }
        }
    }
    .into()
}

struct Input {
    simple_ident: syn::Ident,
    full_ident: syn::Ident,
}

impl syn::parse::Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let simple_ident: syn::Ident = input.parse()?;
        let _: syn::Token![,] = input.parse()?;
        let full_ident: syn::Ident = input.parse()?;
        Ok(Input {
            simple_ident,
            full_ident,
        })
    }
}

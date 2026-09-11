use proc_macro::TokenStream;
use quote::quote;

/// This is copied from [`super::make_suite_struct::make_suite_struct`].
pub fn make_object_struct(tokens: TokenStream) -> TokenStream {
    let Input {
        simple_ident,
        full_ident,
        fns: _,
    } = syn::parse_macro_input!(tokens as Input);

    quote! {
        /// ## Safety
        ///
        /// This type is not `Send`, but it is `Sync`. For details, see the Safety
        /// section of [`crate::low_plugin::Host`], which has the same requirements.
        #[derive(Clone, Copy)]
        pub struct #simple_ident(::std::ptr::NonNull<crate::sys_umbrella::#full_ident>);
        impl crate::low_plugin::HostOwned for #simple_ident {}
        unsafe impl Sync for #simple_ident {}
        impl #simple_ident {
            pub fn try_from_sys(ptr: *const ::std::os::raw::c_void) -> Option<Self> {
                ::std::ptr::NonNull::new(ptr as *mut crate::sys_umbrella::#full_ident).map(|v| Self(v))
            }
            pub unsafe fn from_sys_unchecked(ptr: *const ::std::os::raw::c_void) -> Self {
                Self::try_from_sys(ptr).unwrap_unchecked()
            }
            pub fn sys(&self) -> ::std::ptr::NonNull<crate::sys_umbrella::#full_ident> {
                self.0
            }
            pub fn sys_ptr(&self) -> *const crate::sys_umbrella::#full_ident {
                self.0.as_ptr()
            }
            pub unsafe fn sys_ref(&self) -> &crate::sys_umbrella::#full_ident {
                unsafe { self.0.as_ref() }
            }
        }
        impl From<&#simple_ident> for *const crate::sys_umbrella::#full_ident {
            fn from(value: &#simple_ident) -> Self {
                value.sys_ptr()
            }
        }

        // const _: () = {
        //     #(
        //         let _ = #simple_ident::#fns;
        //     )*
        // };
    }
    .into()
}

struct Input {
    simple_ident: syn::Ident,
    full_ident: syn::Ident,
    #[expect(dead_code)]
    fns: syn::punctuated::Punctuated<syn::Ident, syn::Token![,]>,
}

impl syn::parse::Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let simple_ident: syn::Ident = input.parse()?;
        let _: syn::Token![:] = input.parse()?;
        let full_ident: syn::Ident = input.parse()?;
        let _: syn::Token![:] = input.parse()?;
        let fns: syn::punctuated::Punctuated<syn::Ident, syn::Token![,]> =
            input.parse_terminated(syn::Ident::parse, syn::Token![,])?;
        Ok(Input {
            simple_ident,
            full_ident,
            fns,
        })
    }
}

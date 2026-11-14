use proc_macro::TokenStream;
use quote::quote;

use crate::low_macros::utils::{self, make_doc_see_sys};

pub fn impl_param_set_descriptor(attr: TokenStream, item: TokenStream) -> TokenStream {
    if let Err(error) = utils::err_if_attribute_is_not_empty(attr) {
        return error;
    }

    let input = syn::parse_macro_input!(item as Input);
    let param_set_descriptor_ident = input.param_set_descriptor_ident;

    let mut output_inner = proc_macro2::TokenStream::new();

    for item in input.items {
        match item {
            InputItem::ParamDefine(InputParamDefine {
                type_key,
                fn_name,
                prop_set_ty,
            }) => {
                let doc_see_sys =
                    make_doc_see_sys(&["OfxParameterSuiteV1::paramDefine", &type_key.to_string()]);
                let fn_name_no_output =
                    syn::Ident::new(&format!("{}_no_output", fn_name), fn_name.span());
                output_inner.extend(quote! {
                    #doc_see_sys
                    ///
                    /// ## Safety
                    ///
                    /// `self` and `parameter_suite` must be valid."
                    pub unsafe fn #fn_name(
                        &self,
                        parameter_suite: &crate::low_plugin::suites::ParameterSuiteV1,
                        name: &std::ffi::CStr,
                    ) -> crate::low::InboundResult<#prop_set_ty> {
                        unsafe {
                            self.param_define(parameter_suite, crate::sys_umbrella::#type_key, name)
                                .map_ok(<#prop_set_ty>::from)
                        }
                    }
                    #doc_see_sys
                    ///
                    /// ## Safety
                    ///
                    /// `self` and `parameter_suite` must be valid."
                    pub unsafe fn #fn_name_no_output(
                        &self,
                        parameter_suite: &crate::low_plugin::suites::ParameterSuiteV1,
                        name: &std::ffi::CStr,
                    ) -> crate::low::InboundResult<()> {
                        unsafe {
                            self.param_define_no_output(parameter_suite, crate::sys_umbrella::#type_key, name)
                        }
                    }
                });
            }
        }
    }

    quote! {
        impl crate::low_plugin::objects::#param_set_descriptor_ident {
            #output_inner
        }
    }
    .into()
}

/// ## Examples
///
/// ```rust,ignore
/// #[impl_param_set_descriptor]
/// impl ParamSetDescriptor {
///     #[param_define(kOfxParamTypeBoolean)]
///     fn param_define_boolean() -> ParamBooleanPropertySet;
///     // …    
/// }
/// ```
struct Input {
    param_set_descriptor_ident: syn::Ident,
    items: Vec<InputItem>,
}

impl syn::parse::Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: syn::Token![impl] = input.parse()?;
        let param_set_descriptor_ident: syn::Ident = input.parse()?;
        if param_set_descriptor_ident != "ParamSetDescriptor" {
            return Err(syn::Error::new_spanned(
                param_set_descriptor_ident,
                "expected `ParamSetDescriptor`",
            ));
        }

        let body;
        syn::braced!(body in input);

        let mut items = vec![];
        while !body.is_empty() {
            if utils::peek_attribute(&body, "param_define") {
                let item: InputParamDefine = body.parse()?;
                items.push(InputItem::ParamDefine(item));
            } else {
                return Err(syn::Error::new_spanned(
                    body.cursor().token_stream(),
                    "unexpected item",
                ));
            }
        }

        Ok(Self {
            param_set_descriptor_ident,
            items,
        })
    }
}

enum InputItem {
    ParamDefine(InputParamDefine),
}

struct InputParamDefine {
    type_key: syn::Ident,
    fn_name: syn::Ident,
    prop_set_ty: syn::Ident,
}

impl syn::parse::Parse for InputParamDefine {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: syn::Token![#] = input.parse()?;
        let meta;
        syn::bracketed!(meta in input);
        let meta_name: syn::Ident = meta.parse()?;
        if meta_name != "param_define" {
            return Err(syn::Error::new_spanned(
                meta_name,
                "expected `param_define`",
            ));
        }
        let meta_content;
        syn::parenthesized!(meta_content in meta);
        let type_key: syn::Ident = meta_content.parse()?;

        let _: syn::Token![fn] = input.parse()?;
        let fn_name: syn::Ident = input.parse()?;

        let parameters;
        syn::parenthesized!(parameters in input);
        if !parameters.is_empty() {
            return Err(syn::Error::new_spanned(
                parameters.cursor().token_stream(),
                "expected empty parameter list",
            ));
        }

        let _: syn::Token![->] = input.parse()?;

        let prop_set_ty: syn::Ident = input.parse()?;

        let _: syn::Token![;] = input.parse()?;

        Ok(Self {
            type_key,
            fn_name,
            prop_set_ty,
        })
    }
}

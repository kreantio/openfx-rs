use proc_macro::TokenStream;
use quote::quote;

use crate::low_macros::utils::{self, make_doc_see_sys};

pub fn impl_param_set_instance(attr: TokenStream, item: TokenStream) -> TokenStream {
    if let Err(error) = utils::err_if_attribute_is_not_empty(attr) {
        return error;
    }

    let input = syn::parse_macro_input!(item as Input);
    let param_set_instance_ident = input.param_set_instance_ident;

    let mut output_inner = proc_macro2::TokenStream::new();

    for item in input.items {
        match item {
            InputItem::ParamGetHandle(InputParamGetHandle {
                type_key,
                fn_name,
                object_ty,
                prop_set_ty,
            }) => {
                let doc_see_sys = make_doc_see_sys(&[
                    "OfxParameterSuiteV1::paramGetHandle",
                    &type_key.to_string(),
                ]);
                let fn_name_param_handle_only = {
                    let fn_name_string = fn_name.to_string();
                    let Some(prefix_stripped) = fn_name_string.strip_prefix("param_get_handle_")
                    else {
                        return syn::Error::new_spanned(
                            fn_name,
                            "expected function name starting with `param_get_handle_`",
                        )
                        .to_compile_error()
                        .into();
                    };
                    syn::Ident::new(
                        &format!("param_get_param_handle_{prefix_stripped}"),
                        fn_name.span(),
                    )
                };
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
                    ) -> crate::low::InboundResult<(#object_ty, #prop_set_ty)> {
                        unsafe {
                            self.param_get_handle(parameter_suite, name)
                                .map_ok(|(param, prop_set)| (#object_ty::from_sys_handle(param), #prop_set_ty::from(prop_set)))
                        }
                    }
                    #doc_see_sys
                    ///
                    /// ## Safety
                    ///
                    /// `self` and `parameter_suite` must be valid."
                    pub unsafe fn #fn_name_param_handle_only(
                        &self,
                        parameter_suite: &crate::low_plugin::suites::ParameterSuiteV1,
                        name: &std::ffi::CStr,
                    ) -> crate::low::InboundResult<#object_ty> {
                        unsafe {
                            self.param_get_param_handle(parameter_suite, name)
                                .map_ok(#object_ty::from_sys_handle)
                        }
                    }
                });
            }
        }
    }

    quote! {
        impl crate::low_plugin::objects::#param_set_instance_ident {
            #output_inner
        }
    }
    .into()
}

/// ## Examples
///
/// ```rust,ignore
/// #[impl_param_set_instance]
/// impl ParamSetInstance {
///     #[param_get_handle(kOfxParamTypeBoolean)]
///     fn param_get_handle_boolean() -> (ParamBoolean, ParamBooleanPropertySet);
///     // …    
/// }
/// ```
struct Input {
    param_set_instance_ident: syn::Ident,
    items: Vec<InputItem>,
}

impl syn::parse::Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: syn::Token![impl] = input.parse()?;
        let param_set_instance_ident: syn::Ident = input.parse()?;
        if param_set_instance_ident != "ParamSetInstance" {
            return Err(syn::Error::new_spanned(
                param_set_instance_ident,
                "expected `ParamSetInstance`",
            ));
        }

        let body;
        syn::braced!(body in input);

        let mut items = vec![];
        while !body.is_empty() {
            if utils::peek_attribute(&body, "param_get_handle") {
                let item: InputParamGetHandle = body.parse()?;
                items.push(InputItem::ParamGetHandle(item));
            } else {
                return Err(syn::Error::new_spanned(
                    body.cursor().token_stream(),
                    "unexpected item",
                ));
            }
        }

        Ok(Self {
            param_set_instance_ident,
            items,
        })
    }
}

enum InputItem {
    ParamGetHandle(InputParamGetHandle),
}

struct InputParamGetHandle {
    type_key: syn::Ident,
    fn_name: syn::Ident,
    object_ty: syn::Ident,
    prop_set_ty: syn::Ident,
}

impl syn::parse::Parse for InputParamGetHandle {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: syn::Token![#] = input.parse()?;
        let meta;
        syn::bracketed!(meta in input);
        let meta_name: syn::Ident = meta.parse()?;
        if meta_name != "param_get_handle" {
            return Err(syn::Error::new_spanned(
                meta_name,
                "expected `param_get_handle`",
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

        let ret_tuple_content;
        syn::parenthesized!(ret_tuple_content in input);
        let object_ty: syn::Ident = ret_tuple_content.parse()?;
        let _: syn::Token![,] = ret_tuple_content.parse()?;
        let prop_set_ty: syn::Ident = ret_tuple_content.parse()?;

        let _: syn::Token![;] = input.parse()?;

        Ok(Self {
            type_key,
            fn_name,
            object_ty,
            prop_set_ty,
        })
    }
}

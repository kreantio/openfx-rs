use proc_macro::TokenStream;
use quote::quote;

use crate::{common::ParameterType, low_macros::utils::make_doc_see_sys};

/// This is copied from [`super::make_object_struct::make_object_struct`].
pub fn make_parameter_struct(tokens: TokenStream) -> TokenStream {
    let Input { simple_ident, ty } = syn::parse_macro_input!(tokens as Input);

    let doc_see_sys =
        make_doc_see_sys(&["OfxParamHandle", &format!("kOfxParamType{simple_ident}")]);

    let mut output = quote! {
        #doc_see_sys
        ///
        /// ## Safety
        ///
        /// This type is not `Send`, but it is `Sync`. For details, see the Safety
        /// section of [`crate::low_plugin::Host`], which has the same requirements.
        #[derive(Clone, Copy)]
        pub struct #simple_ident(crate::sys_umbrella::OfxParamHandle);
        unsafe impl Sync for #simple_ident {}
        impl #simple_ident {
            pub fn from_sys_handle(sys_handle: crate::sys_umbrella::OfxParamHandle) -> Self {
                Self(sys_handle)
            }
            pub fn from_sys_ptr(sys_ptr: *const ::std::os::raw::c_void) -> Self {
                Self(sys_ptr as crate::sys_umbrella::OfxParamHandle)
            }
            pub fn sys_handle(&self) -> crate::sys_umbrella::OfxParamHandle {
                self.0
            }
        }
        impl From<&#simple_ident> for crate::sys_umbrella::OfxParamHandle {
            fn from(value: &#simple_ident) -> Self {
                value.sys_handle()
            }
        }
    };

    output.extend(quote! {
        impl #simple_ident {
            /// See: [`crate::sys_umbrella::OfxParameterSuiteV1::paramCopy`]
            ///
            /// ## Safety
            ///
            /// `self`, `parameter_suite` and `param_from` must be valid.
            pub unsafe fn param_copy_from(
                &self,
                parameter_suite: &crate::low_plugin::suites::ParameterSuiteV1,
                param_from: &Self,
                dst_offset: crate::sys_umbrella::OfxTime,
                frame_range: Option<&crate::sys_umbrella::OfxRangeD>,
            ) -> crate::low::InboundResult<()> {
                unsafe {
                    parameter_suite.param_copy(
                        self.sys_handle(),
                        param_from.sys_handle(),
                        dst_offset,
                        frame_range,
                    )
                }
            }

            /// See: [`crate::sys_umbrella::OfxParameterSuiteV1::paramDeleteAllKeys`]
            ///
            /// ## Safety
            ///
            /// `self` and `parameter_suite` must be valid.
            pub unsafe fn param_delete_all_keys(
                &self,
                parameter_suite: &crate::low_plugin::suites::ParameterSuiteV1,
            ) -> crate::low::InboundResult<()> {
                unsafe {
                    parameter_suite.param_delete_all_keys(self.sys_handle())
                }
            }

            /// See: [`crate::sys_umbrella::OfxParameterSuiteV1::paramDeleteKey`]
            ///
            /// ## Safety
            ///
            /// `self` and `parameter_suite` must be valid.
            pub unsafe fn param_delete_key(
                &self,
                parameter_suite: &crate::low_plugin::suites::ParameterSuiteV1,
                time: crate::sys_umbrella::OfxTime,
            ) -> crate::low::InboundResult<()> {
                unsafe {
                    parameter_suite.param_delete_key(self.sys_handle(), time)
                }
            }

            /// See: [`crate::sys_umbrella::OfxParameterSuiteV1::paramGetKeyIndex`]
            ///
            /// ## Safety
            ///
            /// `self` and `parameter_suite` must be valid.
            pub unsafe fn param_get_key_index(
                &self,
                parameter_suite: &crate::low_plugin::suites::ParameterSuiteV1,
                time: crate::sys_umbrella::OfxTime,
                direction: std::ffi::c_int,
            ) -> crate::low::InboundResult<std::ffi::c_int> {
                unsafe {
                    parameter_suite.param_get_key_index(self.sys_handle(), time, direction)
                }
            }

            /// See: [`crate::sys_umbrella::OfxParameterSuiteV1::paramGetKeyTime`]
            ///
            /// ## Safety
            ///
            /// `self` and `parameter_suite` must be valid.
            pub unsafe fn param_get_key_time(
                &self,
                parameter_suite: &crate::low_plugin::suites::ParameterSuiteV1,
                nth_key: std::ffi::c_uint,
            ) -> crate::low::InboundResult<crate::sys_umbrella::OfxTime> {
                unsafe {
                    parameter_suite.param_get_key_time(self.sys_handle(), nth_key)
                }
            }

            /// See: [`crate::sys_umbrella::OfxParameterSuiteV1::paramGetNumKeys`]
            ///
            /// ## Safety
            ///
            /// `self` and `parameter_suite` must be valid.
            pub unsafe fn param_get_num_keys(
                &self,
                parameter_suite: &crate::low_plugin::suites::ParameterSuiteV1,
            ) -> crate::low::InboundResult<std::ffi::c_uint> {
                unsafe {
                    parameter_suite.param_get_num_keys(self.sys_handle())
                }
            }

            /// See: [`crate::sys_umbrella::OfxParameterSuiteV1::paramGetPropertySet`]
            ///
            /// ## Safety
            ///
            /// `self` and `parameter_suite` must be valid.
            pub unsafe fn param_get_property_set(
                &self,
                parameter_suite: &crate::low_plugin::suites::ParameterSuiteV1,
            ) -> crate::low::InboundResult<crate::sys_umbrella::OfxPropertySetHandle> {
                unsafe {
                    parameter_suite.param_get_property_set(self.sys_handle())
                }
            }
        }
    });

    if let Some(ty) = ty {}

    output.into()
}

struct Input {
    simple_ident: syn::Ident,
    ty: Option<ParameterType>,
}

impl syn::parse::Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let simple_ident: syn::Ident = input.parse()?;
        let _: syn::Token![,] = input.parse()?;
        let _: syn::Token![type] = input.parse()?;
        let _: syn::Token![=] = input.parse()?;
        let ty = ParameterType::parse_option(input)?;
        Ok(Input { simple_ident, ty })
    }
}

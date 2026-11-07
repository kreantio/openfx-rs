use proc_macro::TokenStream;
use quote::{quote, quote_spanned};

use crate::{
    common::{ParameterElementType, ParameterType, SimpleType},
    low_macros::utils::{self, make_doc_see_sys},
};

pub fn make_parameter_types(attr: TokenStream, item: TokenStream) -> TokenStream {
    if let Err(error) = utils::err_if_attribute_is_not_empty(attr) {
        return error;
    }

    let input = syn::parse_macro_input!(item as Input);

    let mut output = proc_macro2::TokenStream::new();

    for input_param_type in &input.param_types {
        make_parameter_struct(&mut output, input_param_type);
    }

    output.into()
}

fn make_parameter_struct(output: &mut proc_macro2::TokenStream, input: &InputParamType) {
    let InputParamType { simple_ident, ty } = input;

    let doc_see_sys =
        make_doc_see_sys(&["OfxParamHandle", &format!("kOfxParamType{simple_ident}")]);

    output.extend(quote! {
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
            pub fn from_sys_ptr(sys_ptr: *const ::std::ffi::c_void) -> Self {
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
    });

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

    if let Some(ty) = ty {
        make_setters(output);
        make_getters(output, simple_ident, ty);
    }
}

fn make_setters(output: &mut proc_macro2::TokenStream) {
    // TODO!!
}

fn make_getters(
    output: &mut proc_macro2::TokenStream,
    parameter_type_name_ident: &syn::Ident,
    ty: &ParameterType,
) {
    struct Things {
        param_ty_quote: proc_macro2::TokenStream,
        simple_value_init_quote: proc_macro2::TokenStream,
        simple_value_mut_refs_quote: proc_macro2::TokenStream,
        param_value_from_simple_value_quote: proc_macro2::TokenStream,
    }

    let simple_type_quote = ty.element_type.simple().quote_rust_ffi_type();

    for preset in ty.element_type.make_impl_getter_presets() {
        let param_el_ty_quote = preset.parameter_element_type(ty.element_type.ident());

        let Things {
            param_ty_quote,
            simple_value_init_quote,
            simple_value_mut_refs_quote,
            param_value_from_simple_value_quote,
        } = if ty.dimensions == 1 {
            let mut param_value_from_simple_value_quote = quote! {
                let simple_value = unsafe { simple_value.assume_init() };
            };
            if let Some(q) = preset.simple_type_to_parameter_element_type(quote! { simple_value }) {
                param_value_from_simple_value_quote.extend(quote! {
                    let param_value = #q;
                });
            } else {
                param_value_from_simple_value_quote.extend(quote! {
                    let param_value = simple_value;
                });
            }

            Things {
                param_ty_quote: param_el_ty_quote,
                simple_value_init_quote: quote! {
                    let mut simple_value: std::mem::MaybeUninit<#simple_type_quote> = std::mem::MaybeUninit::uninit();
                },
                simple_value_mut_refs_quote: quote! {
                    &mut simple_value
                },
                param_value_from_simple_value_quote,
            }
        } else {
            let dimensions =
                syn::LitInt::new(&ty.dimensions.to_string(), proc_macro2::Span::call_site());
            let mut simple_value_init_quote = proc_macro2::TokenStream::new();
            let mut simple_value_mut_refs_quote = proc_macro2::TokenStream::new();
            let mut param_value_from_simple_value_quote = proc_macro2::TokenStream::new();
            let mut param_value_array_constructor_items: Vec<proc_macro2::TokenStream> = vec![];
            for i in 0..ty.dimensions {
                let simple_value_ident =
                    syn::Ident::new(&format!("simple_value_{i}"), proc_macro2::Span::call_site());
                simple_value_init_quote.extend(quote! {
                    let mut #simple_value_ident: std::mem::MaybeUninit<#simple_type_quote> = std::mem::MaybeUninit::uninit();
                });
                simple_value_mut_refs_quote.extend(quote! {
                    &mut #simple_value_ident
                });
                if i != ty.dimensions - 1 {
                    simple_value_mut_refs_quote.extend(quote! {, });
                }
                param_value_from_simple_value_quote.extend(quote! {
                    let #simple_value_ident = unsafe { #simple_value_ident.assume_init() };
                });
                if let Some(q) =
                    preset.simple_type_to_parameter_element_type(quote! { #simple_value_ident })
                {
                    param_value_array_constructor_items.push(q);
                } else {
                    param_value_array_constructor_items.push(quote! { #simple_value_ident });
                }
            }
            param_value_from_simple_value_quote.extend(quote! {
                let param_value = [#(#param_value_array_constructor_items),*];
            });
            Things {
                param_ty_quote: quote! { [#param_el_ty_quote; #dimensions] },
                simple_value_init_quote,
                simple_value_mut_refs_quote,
                param_value_from_simple_value_quote,
            }
        };

        {
            let fn_name = syn::Ident::new(
                &format!(
                    "param_get_value{}",
                    preset
                        .fn_name_suffix()
                        .map_or(String::new(), |s| format!("_{s}"))
                ),
                proc_macro2::Span::call_site(),
            );
            let mut doc = quote! {
                /// See: [`crate::sys_umbrella::OfxParameterSuiteV1::paramGetValue`]
                ///
                /// ## Safety
                ///
                /// `self` and `parameter_suite` must be valid.
            };
            if let Some(additional_safety_doc) = preset.additional_safety_doc() {
                doc.extend(quote! {
                    ///
                });
                doc.extend(additional_safety_doc);
            }

            output.extend(quote! {
                impl #parameter_type_name_ident {
                    #doc
                    pub unsafe fn #fn_name(
                        &self,
                        parameter_suite: &crate::low_plugin::suites::ParameterSuiteV1,
                    ) -> crate::low::InboundResult<#param_ty_quote> {
                        #simple_value_init_quote;
                        unsafe {
                            crate::low_plugin::suites::parameter_suite::param_get_value!(
                                parameter_suite, self, #simple_value_mut_refs_quote
                            ).map_ok(|()| {
                                #param_value_from_simple_value_quote
                                param_value
                            })
                        }
                    }
                }
            });
        }

        {
            let fn_name = syn::Ident::new(
                &format!(
                    "param_get_value_at_time{}",
                    preset
                        .fn_name_suffix()
                        .map_or(String::new(), |s| format!("_{s}"))
                ),
                proc_macro2::Span::call_site(),
            );
            let mut doc = quote! {
                /// See: [`crate::sys_umbrella::OfxParameterSuiteV1::paramGetValueAtTime`]
                ///
                /// ## Safety
                ///
                /// `self` and `parameter_suite` must be valid.
            };
            if let Some(additional_safety_doc) = preset.additional_safety_doc() {
                doc.extend(quote! {
                    ///
                });
                doc.extend(additional_safety_doc);
            }

            output.extend(quote! {
                impl #parameter_type_name_ident {
                    #doc
                    pub unsafe fn #fn_name(
                        &self,
                        parameter_suite: &crate::low_plugin::suites::ParameterSuiteV1,
                        time: crate::sys_umbrella::OfxTime,
                    ) -> crate::low::InboundResult<#param_ty_quote> {
                        #simple_value_init_quote;
                        unsafe {
                            crate::low_plugin::suites::parameter_suite::param_get_value_at_time!(
                                parameter_suite, self, time, #simple_value_mut_refs_quote
                            ).map_ok(|()| {
                                #param_value_from_simple_value_quote
                                param_value
                            })
                        }
                    }
                }
            })
        }
    }
}

impl ParameterElementType {
    fn make_impl_setter_presets(&self) -> Vec<Box<dyn ImplSetterPreset>> {
        match self {
            ParameterElementType::Int(_) => vec![Box::new(ImplAccessorsPresetInt)],
            ParameterElementType::Double(_) => vec![Box::new(ImplAccessorsPresetDouble)],
            ParameterElementType::Bool(_) => vec![Box::new(ImplAccessorsPresetBool)],
            ParameterElementType::String(_) => vec![Box::new(ImplAccessorsPresetString)],
            ParameterElementType::Pointer(_) => vec![Box::new(ImplAccessorsPresetPointer)],
        }
    }
    fn make_impl_getter_presets(&self) -> Vec<Box<dyn ImplGetterPreset>> {
        match self {
            ParameterElementType::Int(_) => vec![Box::new(ImplAccessorsPresetInt)],
            ParameterElementType::Double(_) => vec![Box::new(ImplAccessorsPresetDouble)],
            ParameterElementType::Bool(_) => vec![Box::new(ImplAccessorsPresetBool)],
            ParameterElementType::String(_) => vec![
                Box::new(ImplAccessorsPresetString),
                Box::new(ImplAccessorsPresetStringOwned),
            ],
            ParameterElementType::Pointer(_) => vec![Box::new(ImplAccessorsPresetPointer)],
        }
    }
}

trait ImplSetterPreset {
    /// Determines what the setter function added by this preset should be
    /// called. If the returned value is `Some`, a suffix is appended to the
    /// setter function's name.
    fn fn_name_suffix(&self) -> Option<&'static str> {
        None
    }
    fn additional_safety_doc(&self) -> Option<proc_macro2::TokenStream> {
        None
    }
    fn simple_type(&self) -> SimpleType;
    fn parameter_element_type_to_simple_type(
        &self,
        _param_val: proc_macro2::TokenStream,
    ) -> Option<proc_macro2::TokenStream> {
        None
    }
}
trait ImplGetterPreset {
    /// Determines what the setter function added by this preset should be
    /// called. If the returned value is `Some`, a suffix is appended to the
    /// setter function's name.
    fn fn_name_suffix(&self) -> Option<&'static str> {
        None
    }
    fn additional_safety_doc(&self) -> Option<proc_macro2::TokenStream> {
        None
    }
    fn simple_type(&self) -> SimpleType;
    fn parameter_element_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream;
    fn simple_type_to_parameter_element_type(
        &self,
        _simple_val: proc_macro2::TokenStream,
    ) -> Option<proc_macro2::TokenStream> {
        None
    }
}

struct ImplAccessorsPresetInt;
impl ImplSetterPreset for ImplAccessorsPresetInt {
    fn simple_type(&self) -> SimpleType {
        SimpleType::new_int()
    }
}
impl ImplGetterPreset for ImplAccessorsPresetInt {
    fn simple_type(&self) -> SimpleType {
        SimpleType::new_int()
    }
    fn parameter_element_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        let c_int = quote_spanned! { ident.span() => c_int };
        quote! { ::std::ffi::#c_int }
    }
}

struct ImplAccessorsPresetDouble;
impl ImplSetterPreset for ImplAccessorsPresetDouble {
    fn simple_type(&self) -> SimpleType {
        SimpleType::new_double()
    }
}
impl ImplGetterPreset for ImplAccessorsPresetDouble {
    fn simple_type(&self) -> SimpleType {
        SimpleType::new_double()
    }
    fn parameter_element_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        let f64 = quote_spanned! { ident.span() => f64 };
        quote! { ::std::primitive::#f64 }
    }
}

struct ImplAccessorsPresetBool;
impl ImplSetterPreset for ImplAccessorsPresetBool {
    fn simple_type(&self) -> SimpleType {
        SimpleType::new_int()
    }
    fn parameter_element_type_to_simple_type(
        &self,
        param_val: proc_macro2::TokenStream,
    ) -> Option<proc_macro2::TokenStream> {
        Some(quote! { #param_val as ::std::ffi::c_int })
    }
}
impl ImplGetterPreset for ImplAccessorsPresetBool {
    fn simple_type(&self) -> SimpleType {
        SimpleType::new_int()
    }
    fn parameter_element_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        let bool = quote_spanned! { ident.span() => bool };
        quote! { ::std::primitive::#bool }
    }
    fn simple_type_to_parameter_element_type(
        &self,
        simple_val: proc_macro2::TokenStream,
    ) -> Option<proc_macro2::TokenStream> {
        Some(quote! { #simple_val != 0 })
    }
}

struct ImplAccessorsPresetPointer;
impl ImplSetterPreset for ImplAccessorsPresetPointer {
    fn simple_type(&self) -> SimpleType {
        SimpleType::new_pointer()
    }
    fn parameter_element_type_to_simple_type(
        &self,
        param_val: proc_macro2::TokenStream,
    ) -> Option<proc_macro2::TokenStream> {
        Some(quote! { #param_val.as_mut_ptr() })
    }
}
impl ImplGetterPreset for ImplAccessorsPresetPointer {
    fn additional_safety_doc(&self) -> Option<proc_macro2::TokenStream> {
        Some(quote! {
            /// The lifetime of the retrieved pointer(s) is controlled by the
            /// host.
        })
    }
    fn simple_type(&self) -> SimpleType {
        SimpleType::new_pointer()
    }
    fn parameter_element_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        let c_void = quote_spanned! { ident.span() => c_void };
        quote! { ::std::option::Option<::std::ptr::NonNull<::std::ffi::#c_void>> }
    }
    fn simple_type_to_parameter_element_type(
        &self,
        simple_val: proc_macro2::TokenStream,
    ) -> Option<proc_macro2::TokenStream> {
        Some(quote! { ::std::ptr::NonNull::new(#simple_val) })
    }
}

struct ImplAccessorsPresetString;
impl ImplSetterPreset for ImplAccessorsPresetString {
    fn simple_type(&self) -> SimpleType {
        SimpleType::new_pointer()
    }
    fn parameter_element_type_to_simple_type(
        &self,
        param_val: proc_macro2::TokenStream,
    ) -> Option<proc_macro2::TokenStream> {
        Some(quote! { #param_val.as_ptr() })
    }
}
impl ImplGetterPreset for ImplAccessorsPresetString {
    fn simple_type(&self) -> SimpleType {
        SimpleType::new_pointer()
    }
    fn parameter_element_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        let c_char = quote_spanned! { ident.span() => c_char };
        quote! { ::std::option::Option<::std::ptr::NonNull<::std::ffi::#c_char>> }
    }
    fn simple_type_to_parameter_element_type(
        &self,
        simple_val: proc_macro2::TokenStream,
    ) -> Option<proc_macro2::TokenStream> {
        Some(quote! { ::std::ptr::NonNull::new(#simple_val) })
    }
}

struct ImplAccessorsPresetStringOwned;
impl ImplGetterPreset for ImplAccessorsPresetStringOwned {
    fn fn_name_suffix(&self) -> Option<&'static str> {
        Some("owned")
    }
    fn simple_type(&self) -> SimpleType {
        SimpleType::new_pointer()
    }
    fn parameter_element_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        let cstring = quote_spanned! { ident.span() => CString };
        quote! { ::std::option::Option<::std::ffi::#cstring> }
    }
    fn simple_type_to_parameter_element_type(
        &self,
        simple_val: proc_macro2::TokenStream,
    ) -> Option<proc_macro2::TokenStream> {
        Some(
            quote! { if #simple_val.is_null() { None } else { Some(::std::ffi::CStr::from_ptr(#simple_val).to_owned()) } },
        )
    }
}

/// ## Examples
///
/// ```rust,ignore
/// #[make_parameter_structs]
/// mod __ {
///     #[param_type]
///     struct ParamBoolean([Bool; 1]);
///     // …
///     #[param_type]
///     struct ParamGroup;
///     // …
/// }
/// ```
struct Input {
    param_types: Vec<InputParamType>,
}

impl syn::parse::Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: syn::Token![mod] = input.parse()?;
        let mod_name = input.parse::<syn::Ident>()?;
        if mod_name != "__" {
            return Err(syn::Error::new_spanned(
                mod_name,
                "expected module name `__`",
            ));
        }
        let body;
        syn::braced!(body in input);
        let mut param_types = vec![];
        while !body.is_empty() {
            param_types.push(body.parse()?);
        }
        Ok(Input { param_types })
    }
}

struct InputParamType {
    simple_ident: syn::Ident,
    ty: Option<ParameterType>,
}
impl syn::parse::Parse for InputParamType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: syn::Token![#] = input.parse()?;
        let meta;
        syn::bracketed!(meta in input);
        let meta_name: syn::Ident = meta.parse()?;
        if meta_name != "param_type" {
            return Err(syn::Error::new_spanned(meta_name, "expected `param_type`"));
        }

        let _: syn::Token![struct] = input.parse()?;
        let simple_ident: syn::Ident = input.parse()?;
        let ty = if input.peek(syn::Token![;]) {
            let _: syn::Token![;] = input.parse()?;
            None
        } else {
            let struct_body;
            syn::parenthesized!(struct_body in input);
            let _: syn::Token![;] = input.parse()?;
            Some(struct_body.parse()?)
        };

        Ok(InputParamType { simple_ident, ty })
    }
}

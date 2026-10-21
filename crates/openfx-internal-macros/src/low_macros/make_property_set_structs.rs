use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};

use crate::low_macros::utils;

mod inputs;
use inputs::*;

pub fn make_property_set_structs(attr: TokenStream, item: TokenStream) -> TokenStream {
    if let Err(error) = utils::err_if_attribute_is_not_empty(attr) {
        return error;
    }

    let mut output_inner = proc_macro2::TokenStream::new();

    let input = syn::parse_macro_input!(item as Input);

    let propsetdefs: HashMap<&syn::Ident, &InputPropsetdef> =
        input.defs.iter().map(|def| (&def.name, def)).collect();

    for set in &input.sets {
        match make_property_set_struct(&mut output_inner, &propsetdefs, set) {
            Ok(()) => {}
            Err(error) => return error.to_compile_error().into(),
        }
    }

    output_inner.into()
}

fn make_property_set_struct(
    output: &mut proc_macro2::TokenStream,
    propsetdefs: &HashMap<&syn::Ident, &InputPropsetdef>,
    set: &InputPropertySet,
) -> syn::Result<()> {
    let mut output_inner = proc_macro2::TokenStream::new();

    for (prop, write) in set.resolved_props(propsetdefs)? {
        make_property_accessors(&mut output_inner, prop, write);
    }

    let struct_name = &set.name;

    output.extend(quote! {
        pub struct #struct_name(crate::sys::generic::core::OfxPropertySetHandle);

        impl #struct_name {
            pub fn sys_handle(&self) -> crate::sys::generic::core::OfxPropertySetHandle {
                self.0
            }
        }

        impl From<crate::sys::generic::core::OfxPropertySetHandle> for #struct_name {
            fn from(handle: crate::sys::generic::core::OfxPropertySetHandle) -> Self {
                Self(handle)
            }
        }

        impl #struct_name {
            #output_inner
        }
    });

    Ok(())
}

fn make_property_accessors(
    output: &mut proc_macro2::TokenStream,
    prop: &InputPropertyItem,
    write: &InputWriteSide,
) {
    let tys: Vec<_> = prop.ty.possible_types().iter().collect();
    let is_ambiguous = tys.len() > 1;

    let cname = syn::Ident::new(
        &format!("k{}", prop.canonical_name),
        prop.canonical_name.span(),
    );

    for ty in prop.ty.possible_types().iter() {
        let simple_ty = ty.simple();
        let fn_name_suffix = if is_ambiguous {
            format!(
                "{}_{}",
                prop.simple_name.to_string().trim_start_matches("r#"),
                simple_ty.to_string().to_lowercase()
            )
        } else {
            prop.simple_name
                .to_string()
                .trim_start_matches("r#")
                .to_owned()
        };

        if write.is_plugin() {
            make_property_setter(output, prop, &cname, &ty, &fn_name_suffix);
        }
        if write.is_host() {
            make_property_getter(output, prop, &cname, &ty, &fn_name_suffix);
        }
    }

    if write.is_plugin() {
        make_property_resetter(output, prop, &cname);
    }
    if write.is_host() && matches!(prop.ty, InputContainerType::Slice(_)) {
        make_property_dimensions_getter(output, prop, &cname);
    }
}

fn make_property_setter(
    output: &mut proc_macro2::TokenStream,
    prop: &InputPropertyItem,
    cname: &syn::Ident,
    ty: &OpenFXTypeExpressive,
    fn_name_suffix: &str,
) {
    for preset in ty.make_impl_setter_preset() {
        let fn_name = syn::Ident::new(
            &format!(
                "set_{}{}",
                fn_name_suffix,
                preset
                    .fn_name_suffix()
                    .map_or(String::new(), |s| format!("_{s}"))
            ),
            prop.simple_name.span(),
        );

        let fn_name_on_suite = syn::Ident::new(
            &format!(
                "prop_set_{}{}{}",
                ty.simple().name_snake_case().to_owned(),
                preset
                    .simple_type_suffix()
                    .map_or(String::new(), |s| format!("_{s}")),
                prop.ty.as_name_suffix(),
            ),
            proc_macro2::Span::call_site(),
        );

        let expressive_ty = preset.expressive_type(ty.ident());
        let expressive_container_ty = match &prop.ty {
            InputContainerType::Scalar(_) => expressive_ty.clone(),
            InputContainerType::Array(_, size) => {
                quote! { [#expressive_ty; #size] }
            }
            InputContainerType::Slice(_) => quote! { &[#expressive_ty] },
        };

        let expressive_value_to_simple_quote = match preset
            .expressive_type_to_simple_type(quote! { value })
        {
            Some(expressive_to_simple_quote) => match &prop.ty {
                InputContainerType::Scalar(_) => expressive_to_simple_quote,
                InputContainerType::Array(_, _) => {
                    quote! { value.map(|value| #expressive_to_simple_quote) }
                }
                InputContainerType::Slice(_) => {
                    quote! { &value.iter().map(|value| #expressive_to_simple_quote).collect::<Vec<_>>() }
                }
            },
            _ => quote! { value },
        };

        let mut doc = quote! {
                /// ## Safety
                ///
                /// - `self.0` must be a valid handle of
                ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
                /// - `suite` must be valid.
        };

        if let Some(additional_safety_doc) = preset.additional_safety_doc() {
            doc.extend(quote! {
                ///
            });
            doc.extend(additional_safety_doc);
        }

        output.extend(
            quote! {
                #doc
                pub unsafe fn #fn_name(&self, suite: &crate::low_plugin::suites::PropertySuiteV1, value: #expressive_container_ty) -> crate::low::Result<()> {
                    let value_simple = #expressive_value_to_simple_quote;
                    suite.#fn_name_on_suite(self.sys_handle(), crate::sys_umbrella::#cname, value_simple)
                }
            }
        );
    }
}

fn make_property_getter(
    output: &mut proc_macro2::TokenStream,
    prop: &InputPropertyItem,
    cname: &syn::Ident,
    ty: &OpenFXTypeExpressive,
    fn_name_suffix: &str,
) {
    for preset in ty.make_impl_getter_preset() {
        let fn_name = syn::Ident::new(
            &format!(
                "get_{}{}",
                fn_name_suffix,
                preset
                    .fn_name_suffix()
                    .map_or(String::new(), |s| format!("_{s}"))
            ),
            prop.simple_name.span(),
        );

        let fn_name_on_suite = syn::Ident::new(
            &format!(
                "prop_get_{}{}{}",
                ty.simple().name_snake_case().to_owned(),
                preset
                    .simple_type_suffix()
                    .map_or(String::new(), |s| format!("_{s}")),
                prop.ty.as_name_suffix(),
            ),
            proc_macro2::Span::call_site(),
        );

        let expressive_ty = preset.expressive_type(ty.ident());
        let simple_to_expressive_quote =
            preset.simple_type_to_expressive_type(quote! { value_simple });
        let is_conversion_unnecessary = simple_to_expressive_quote.is_none();
        let simple_value_to_expressive_quote = match simple_to_expressive_quote {
            Some(simple_to_expressive_quote) => match &prop.ty {
                InputContainerType::Scalar(_) => quote! { #simple_to_expressive_quote },
                InputContainerType::Array(_, _) => {
                    quote! { value_simple.into_iter().map(|value_simple| #simple_to_expressive_quote) }
                }
                InputContainerType::Slice(_) => {
                    quote! { value_simple.into_iter().map(|value_simple| #simple_to_expressive_quote).collect() }
                }
            },
            _ => quote! { value_simple },
        };

        let mut doc = quote! {
            /// ## Safety
            ///
            /// - `self.0` must be a valid handle of
            ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
            /// - `suite` must be valid.
        };

        if let Some(additional_safety_doc) = preset.additional_safety_doc() {
            doc.extend(quote! {
                ///
            });
            doc.extend(additional_safety_doc);
        }

        match &prop.ty {
            InputContainerType::Scalar(_) => output.extend(
                quote! {
                    #doc
                    pub unsafe fn #fn_name(&self, suite: &crate::low_plugin::suites::PropertySuiteV1) -> crate::low::Result<#expressive_ty> {
                        let value_simple = suite.#fn_name_on_suite(self.sys_handle(), crate::sys_umbrella::#cname)?;
                        let value = #simple_value_to_expressive_quote;
                        Ok(value)
                    }
                },
            ),
            InputContainerType::Array(_, size) if is_conversion_unnecessary => output.extend(
                quote! {
                    #doc
                    pub unsafe fn #fn_name(&self, suite: &crate::low_plugin::suites::PropertySuiteV1) -> crate::low::Result<[#expressive_ty; #size]> {
                        let value = suite.#fn_name_on_suite::<#size>(self.sys_handle(), crate::sys_umbrella::#cname)?;
                        Ok(value)
                    }
                }
            ),
            InputContainerType::Array(_, size) => output.extend(
                quote! {
                    #doc
                    pub unsafe fn #fn_name(&self, suite: &crate::low_plugin::suites::PropertySuiteV1) -> crate::low::Result<[#expressive_ty; #size]> {
                        let value_simple = suite.#fn_name_on_suite::<#size>(self.sys_handle(), crate::sys_umbrella::#cname)?;
                        let value = #simple_value_to_expressive_quote;
                        Ok(value)
                    }
                },
            ),
            InputContainerType::Slice(_) => output.extend(
                quote! {
                    #doc
                    pub unsafe fn #fn_name(&self, suite: &crate::low_plugin::suites::PropertySuiteV1) -> crate::low::Result<::std::vec::Vec<#expressive_ty>> {
                        let dimensions = suite.prop_get_dimension(self.sys_handle(), crate::sys_umbrella::#cname)? as usize;
                        let mut value_simple = ::std::vec::Vec::with_capacity(dimensions);
                        unsafe { value_simple.set_len(dimensions); }
                        suite.#fn_name_on_suite(self.sys_handle(), crate::sys_umbrella::#cname, &mut value_simple)?;
                        let value = #simple_value_to_expressive_quote;
                        Ok(value)
                    }
                },
            ),
        }
    }
}

fn make_property_resetter(
    output: &mut proc_macro2::TokenStream,
    prop: &InputPropertyItem,
    cname: &syn::Ident,
) {
    let fn_name = syn::Ident::new(
        &format!(
            "reset_{}",
            prop.simple_name.to_string().trim_start_matches("r#")
        ),
        prop.simple_name.span(),
    );

    output.extend(
        quote! {
            /// ## Safety
            ///
            /// - `self.0` must be a valid handle of
            ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
            /// - `suite` must be valid.
            pub unsafe fn #fn_name(&self, suite: &crate::low_plugin::suites::PropertySuiteV1) -> crate::low::Result<()> {
                unsafe {
                    suite.prop_reset(self.sys_handle(), crate::sys_umbrella::#cname)
                }
            }
        }
    );
}

fn make_property_dimensions_getter(
    output: &mut proc_macro2::TokenStream,
    prop: &InputPropertyItem,
    cname: &syn::Ident,
) {
    let fn_name = syn::Ident::new(
        &format!(
            "len_{}",
            prop.simple_name.to_string().trim_start_matches("r#")
        ),
        prop.simple_name.span(),
    );

    output.extend(
        quote! {
            /// ## Safety
            ///
            /// - `self.0` must be a valid handle of
            ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
            /// - `suite` must be valid.
            pub unsafe fn #fn_name(&self, suite: &crate::low_plugin::suites::PropertySuiteV1) -> crate::low::Result<::std::os::raw::c_int> {
                unsafe {
                    suite.prop_get_dimension(self.sys_handle(), crate::sys_umbrella::#cname)
                }
            }
        }
    );
}

impl OpenFXTypeExpressive {
    fn make_impl_setter_preset(&self) -> Vec<Box<dyn ImplSetterPreset>> {
        match self {
            OpenFXTypeExpressive::Int(_) => vec![Box::new(ImplAccessorsPresetInt)],
            OpenFXTypeExpressive::Double(_) => vec![Box::new(ImplAccessorsPresetDouble)],
            OpenFXTypeExpressive::Enum(_, name) => {
                vec![Box::new(ImplAccessorsPresetEnum { name: name.clone() })]
            }
            OpenFXTypeExpressive::Bool(_) => vec![Box::new(ImplAccessorsPresetBool)],
            OpenFXTypeExpressive::String(_) => vec![Box::new(ImplAccessorsPresetString)],
            OpenFXTypeExpressive::Pointer(_) => vec![Box::new(ImplAccessorsPresetPointer)],
        }
    }
    fn make_impl_getter_preset(&self) -> Vec<Box<dyn ImplGetterPreset>> {
        match self {
            OpenFXTypeExpressive::Int(_) => vec![Box::new(ImplAccessorsPresetInt)],
            OpenFXTypeExpressive::Double(_) => vec![Box::new(ImplAccessorsPresetDouble)],
            OpenFXTypeExpressive::Enum(_, name) => {
                vec![
                    Box::new(ImplAccessorsPresetEnum { name: name.clone() }),
                    Box::new(ImplGetterPresetEnumKnown { name: name.clone() }),
                    Box::new(ImplGetterPresetEnumOwned { name: name.clone() }),
                ]
            }
            OpenFXTypeExpressive::Bool(_) => vec![Box::new(ImplAccessorsPresetBool)],
            OpenFXTypeExpressive::String(_) => vec![
                Box::new(ImplAccessorsPresetString),
                Box::new(ImplGetterPresetStringOwned),
            ],
            OpenFXTypeExpressive::Pointer(_) => vec![Box::new(ImplAccessorsPresetPointer)],
        }
    }
}

trait ImplSetterPreset {
    /// Determines which setter functions on
    /// [`openfx::low_plugin::suites::PropertySuiteV1`] to use, by adding a
    /// suffix to the simple type's name if the returned value is `Some`.
    ///
    /// For example, if the simple type is
    /// [`crate::common::OpenFXTypeSimple::String`], the setter functions would
    /// normally be `prop_set_string{,_at,_array,_n}`, but if this function
    /// returns `Some("ptr")`, the setter functions become
    /// `prop_set_string_ptr{,_at,_array,_n}`.
    fn simple_type_suffix(&self) -> Option<&'static str> {
        None
    }
    /// Determines what the setter function added by this preset should be
    /// called. If the returned value is `Some`, a suffix is appended to the
    /// setter function's name.
    fn fn_name_suffix(&self) -> Option<&'static str> {
        None
    }
    fn additional_safety_doc(&self) -> Option<proc_macro2::TokenStream> {
        None
    }
    /// The expressive type used in the `low` layer, i.e., the type of the value
    /// passed to the setter function that this preset adds.
    fn expressive_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream;
    /// A quote of the expression that converts the expressive type into the
    /// simple type.
    ///
    /// Returning `None` means that the two types are identical, therefore no
    /// conversion is needed.
    fn expressive_type_to_simple_type(
        &self,
        _expressive_val: proc_macro2::TokenStream,
    ) -> Option<proc_macro2::TokenStream> {
        None
    }
}

trait ImplGetterPreset {
    /// See [`ImplSetterPreset::simple_type_suffix`].
    fn simple_type_suffix(&self) -> Option<&'static str> {
        None
    }
    /// See [`ImplSetterPreset::fn_name_suffix`].
    fn fn_name_suffix(&self) -> Option<&'static str> {
        None
    }
    fn additional_safety_doc(&self) -> Option<proc_macro2::TokenStream> {
        None
    }
    /// The expressive type used in the `low` layer, i.e., the type of the value
    /// retrieved from the getter function that this preset adds.
    fn expressive_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream;
    /// A quote of the expression that converts the simple type into the
    /// expressive type.
    ///
    /// Returning `None` means that the two types are identical, therefore no
    /// conversion is needed.
    fn simple_type_to_expressive_type(
        &self,
        _simple_val: proc_macro2::TokenStream,
    ) -> Option<proc_macro2::TokenStream> {
        None
    }
}

struct ImplAccessorsPresetInt;
impl ImplAccessorsPresetInt {
    fn expressive_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        let c_int = quote_spanned! { ident.span() => c_int };
        quote! { ::std::os::raw::#c_int }
    }
}
impl ImplSetterPreset for ImplAccessorsPresetInt {
    fn expressive_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        self.expressive_type(ident)
    }
}
impl ImplGetterPreset for ImplAccessorsPresetInt {
    fn expressive_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        self.expressive_type(ident)
    }
}

struct ImplAccessorsPresetDouble;
impl ImplAccessorsPresetDouble {
    fn expressive_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        let f64 = quote_spanned! { ident.span() => f64 };
        quote! { ::core::primitive::#f64 }
    }
}
impl ImplSetterPreset for ImplAccessorsPresetDouble {
    fn expressive_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        self.expressive_type(ident)
    }
}
impl ImplGetterPreset for ImplAccessorsPresetDouble {
    fn expressive_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        self.expressive_type(ident)
    }
}

struct ImplAccessorsPresetBool;
impl ImplAccessorsPresetBool {
    fn expressive_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        let bool = quote_spanned! { ident.span() => bool };
        quote! { ::core::primitive::#bool }
    }
}
impl ImplSetterPreset for ImplAccessorsPresetBool {
    fn expressive_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        self.expressive_type(ident)
    }
    fn expressive_type_to_simple_type(
        &self,
        expressive_val: proc_macro2::TokenStream,
    ) -> Option<proc_macro2::TokenStream> {
        Some(quote! { #expressive_val as ::std::os::raw::c_int })
    }
}
impl ImplGetterPreset for ImplAccessorsPresetBool {
    fn expressive_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        self.expressive_type(ident)
    }
    fn simple_type_to_expressive_type(
        &self,
        simple_val: proc_macro2::TokenStream,
    ) -> Option<proc_macro2::TokenStream> {
        Some(quote! { #simple_val != 0 })
    }
}

struct ImplAccessorsPresetPointer;
impl ImplAccessorsPresetPointer {
    fn expressive_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        let c_void = quote_spanned! { ident.span() => c_void };
        quote! { ::std::option::Option<::std::ptr::NonNull<::std::ffi::#c_void>> }
    }
}
impl ImplSetterPreset for ImplAccessorsPresetPointer {
    fn expressive_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        self.expressive_type(ident)
    }
}
impl ImplGetterPreset for ImplAccessorsPresetPointer {
    fn additional_safety_doc(&self) -> Option<proc_macro2::TokenStream> {
        Some(quote! {
            /// The lifetime of the retrieved pointer(s) is controlled by the
            /// host.
        })
    }
    fn expressive_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        self.expressive_type(ident)
    }
}

struct ImplAccessorsPresetString;
impl ImplAccessorsPresetString {
    const SIMPLE_TYPE_SUFFIX: Option<&'static str> = Some("ptr");
}
impl ImplSetterPreset for ImplAccessorsPresetString {
    fn simple_type_suffix(&self) -> Option<&'static str> {
        Self::SIMPLE_TYPE_SUFFIX
    }
    fn expressive_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        let cstr = quote_spanned! { ident.span() => CStr };
        quote! { ::std::option::Option<&::std::ffi::#cstr> }
    }
    fn expressive_type_to_simple_type(
        &self,
        expressive_val: proc_macro2::TokenStream,
    ) -> Option<proc_macro2::TokenStream> {
        Some(
            quote! { #expressive_val.and_then(|s| ::std::ptr::NonNull::new(s.as_ptr() as *mut _)) },
        )
    }
}
impl ImplGetterPreset for ImplAccessorsPresetString {
    fn simple_type_suffix(&self) -> Option<&'static str> {
        Self::SIMPLE_TYPE_SUFFIX
    }
    fn additional_safety_doc(&self) -> Option<proc_macro2::TokenStream> {
        Some(quote! {
            /// The lifetime of the retrieved string(s) is controlled by the
            /// host.
        })
    }
    fn expressive_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        let c_char = quote_spanned! { ident.span() => c_char };
        quote! { ::std::option::Option<::std::ptr::NonNull<::std::ffi::#c_char>> }
    }
}

struct ImplGetterPresetStringOwned;
impl ImplGetterPreset for ImplGetterPresetStringOwned {
    fn simple_type_suffix(&self) -> Option<&'static str> {
        Some("owned")
    }
    fn fn_name_suffix(&self) -> Option<&'static str> {
        Some("owned")
    }
    fn expressive_type(&self, _ident: &syn::Ident) -> proc_macro2::TokenStream {
        quote! { ::std::option::Option<::std::ffi::CString> }
    }
}

struct ImplAccessorsPresetEnum {
    name: syn::Ident,
}
impl ImplAccessorsPresetEnum {
    const SIMPLE_TYPE_SUFFIX: Option<&'static str> = ImplAccessorsPresetString::SIMPLE_TYPE_SUFFIX;
}
impl ImplSetterPreset for ImplAccessorsPresetEnum {
    fn simple_type_suffix(&self) -> Option<&'static str> {
        Self::SIMPLE_TYPE_SUFFIX
    }
    fn expressive_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        let name = &self.name;
        let known = quote_spanned! { ident.span() => crate::low::enums::#name };
        quote! { crate::low_plugin::property_sets::EnumPropertyPtr<#known> }
    }
    fn expressive_type_to_simple_type(
        &self,
        expressive_val: proc_macro2::TokenStream,
    ) -> Option<proc_macro2::TokenStream> {
        Some(quote! { #expressive_val.as_ptr() })
    }
}
impl ImplGetterPreset for ImplAccessorsPresetEnum {
    fn simple_type_suffix(&self) -> Option<&'static str> {
        Self::SIMPLE_TYPE_SUFFIX
    }
    fn expressive_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        let name = &self.name;
        let known = quote_spanned! { ident.span() => crate::low::enums::#name };
        quote! { crate::low_plugin::property_sets::LosslessEnumProperty<#known> }
    }
    fn simple_type_to_expressive_type(
        &self,
        simple_val: proc_macro2::TokenStream,
    ) -> Option<proc_macro2::TokenStream> {
        let name = &self.name;
        let known = quote_spanned! { name.span() => crate::low::enums::#name };
        Some(
            quote! { unsafe { crate::low_plugin::property_sets::LosslessEnumProperty::<#known>::from_ptr(#simple_val) } },
        )
    }
}

struct ImplGetterPresetEnumKnown {
    name: syn::Ident,
}
impl ImplGetterPreset for ImplGetterPresetEnumKnown {
    fn simple_type_suffix(&self) -> Option<&'static str> {
        ImplAccessorsPresetString::SIMPLE_TYPE_SUFFIX
    }
    fn fn_name_suffix(&self) -> Option<&'static str> {
        Some("known")
    }
    fn expressive_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        let name = &self.name;
        let known = quote_spanned! { ident.span() => crate::low::enums::#name };
        quote! { ::std::option::Option<#known> }
    }
    fn simple_type_to_expressive_type(
        &self,
        simple_val: proc_macro2::TokenStream,
    ) -> Option<proc_macro2::TokenStream> {
        let name = &self.name;
        let known = quote_spanned! { name.span() => crate::low::enums::#name };
        Some(
            quote! { unsafe { #simple_val.and_then(|simple_val| <#known as crate::low_plugin::property_sets::KnownEnumProperty>::try_from_ptr(simple_val)) } },
        )
    }
}

struct ImplGetterPresetEnumOwned {
    name: syn::Ident,
}
impl ImplGetterPreset for ImplGetterPresetEnumOwned {
    fn simple_type_suffix(&self) -> Option<&'static str> {
        ImplAccessorsPresetString::SIMPLE_TYPE_SUFFIX
    }
    fn fn_name_suffix(&self) -> Option<&'static str> {
        Some("owned")
    }
    fn expressive_type(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        let name = &self.name;
        let known = quote_spanned! { ident.span() => crate::low::enums::#name };
        quote! { crate::low_plugin::property_sets::LosslessEnumPropertyOwned<#known> }
    }
    fn simple_type_to_expressive_type(
        &self,
        simple_val: proc_macro2::TokenStream,
    ) -> Option<proc_macro2::TokenStream> {
        let name = &self.name;
        let known = quote_spanned! { name.span() => crate::low::enums::#name };
        Some(
            quote! { unsafe { crate::low_plugin::property_sets::LosslessEnumPropertyOwned::<#known>::from_ptr(#simple_val) } },
        )
    }
}

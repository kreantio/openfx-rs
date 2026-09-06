use proc_macro::TokenStream;
use quote::{quote, quote_spanned};

use crate::common::type_sys::OpenFXTypeSys;

pub fn make_property_set_structs(tokens: TokenStream) -> TokenStream {
    let mut output_inner = proc_macro2::TokenStream::new();

    let input = syn::parse_macro_input!(tokens as Input);

    for set in input.items {
        make_property_set_struct(&mut output_inner, &set);
    }

    output_inner.into()
}

fn make_property_set_struct(output: &mut proc_macro2::TokenStream, set: &InputPropertySet) {
    let mut output_inner = proc_macro2::TokenStream::new();

    for prop in &set.items {
        make_property_accessors(&mut output_inner, prop);
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
}

fn make_property_accessors(output: &mut proc_macro2::TokenStream, prop: &InputPropertyItem) {
    let tys: Vec<_> = prop.ty.possible_types().iter().collect();
    let is_ambiguous = tys.len() > 1;

    for ty in prop.ty.possible_types().iter() {
        let sys_ty = ty.sys();
        let fn_name_suffix = if is_ambiguous {
            format!(
                "{}_{}",
                prop.simple_name.to_string().trim_start_matches("r#"),
                sys_ty.to_string().to_lowercase()
            )
        } else {
            prop.simple_name
                .to_string()
                .trim_start_matches("r#")
                .to_owned()
        };
        let fn_name_suffix_sys = if is_ambiguous {
            // format!("{}_{}", prop.canonical_name, sys_ty)
            syn::Ident::new(
                &format!("{}_{}", prop.canonical_name, sys_ty),
                prop.canonical_name.span(),
            )
        } else {
            prop.canonical_name.clone()
        };

        if let Some(write_ident) = &prop.accessors.write {
            make_property_setter(
                output,
                prop,
                &ty,
                &fn_name_suffix,
                &fn_name_suffix_sys,
                write_ident,
            );
        }
        if let Some(read_ident) = &prop.accessors.read {
            make_property_getter(
                output,
                prop,
                &ty,
                &fn_name_suffix,
                &fn_name_suffix_sys,
                read_ident,
            );
        }
    }

    if let Some(write_ident) = &prop.accessors.write {
        make_property_resetter(output, prop, write_ident);
    }
    if let Some(read_ident) = &prop.accessors.read
        && matches!(prop.ty, InputContainerType::Array(_))
    {
        make_property_dimensions_getter(output, prop, read_ident);
    }
}

fn make_property_setter(
    output: &mut proc_macro2::TokenStream,
    prop: &InputPropertyItem,
    ty: &OpenFXTypeLow,
    fn_name_suffix: &str,
    fn_name_suffix_sys: &syn::Ident,
    write_ident: &syn::Ident,
) {
    let fn_name = syn::Ident::new(&format!("set_{}", fn_name_suffix), write_ident.span());
    let fn_name_sys = syn::Ident::new(
        &format!("set_{}", fn_name_suffix_sys),
        fn_name_suffix_sys.span(),
    );

    let rust_ty = ty.rust_type_quote_for_setter();
    let container_ty = match &prop.ty {
        InputContainerType::Single(_) => rust_ty.clone(),
        InputContainerType::FixedArray(_, size) => {
            quote! { [#rust_ty; #size] }
        }
        InputContainerType::Array(_) => quote! { &[#rust_ty] },
    };

    let as_sys_quote = ty.as_sys_quote(quote! { value });
    let (value_as_sys_quote, setter_value_quote) = match &prop.ty {
        InputContainerType::Single(_) => (as_sys_quote, quote! { value_sys }),
        InputContainerType::FixedArray(_, _) if ty.is_as_sys_identity() => {
            (quote! { value }, quote! { value_sys })
        }
        InputContainerType::FixedArray(_, _) => (
            quote! { value.map(|value| #as_sys_quote) },
            quote! { value_sys },
        ),
        InputContainerType::Array(_) if ty.is_as_sys_identity() => {
            (quote! { value }, quote! { value_sys })
        }
        InputContainerType::Array(_) => (
            quote! { value.iter().map(|value| #as_sys_quote).collect::<Vec<_>>() },
            quote! { &value_sys },
        ),
    };

    output.extend(
        quote! {
            /// ## SAFETY
            ///
            /// - `self.0` must be a valid handle of
            ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
            /// - `suite` must be a valid pointer to
            ///   [`crate::sys_umbrella::OfxPropertySuiteV1`].
            pub unsafe fn #fn_name(&self, suite: impl Into<*const crate::sys_umbrella::OfxPropertySuiteV1>, value: #container_ty) -> crate::low::Result<()> {
                let value_sys = #value_as_sys_quote;
                let status = crate::sys_helpers_properties_umbrella::#fn_name_sys(suite.into(), self.sys_handle(), #setter_value_quote);
                status.map_err(crate::low::Status::from)
            }
        }
    );
}

fn make_property_getter(
    output: &mut proc_macro2::TokenStream,
    prop: &InputPropertyItem,
    ty: &OpenFXTypeLow,
    fn_name_suffix: &str,
    fn_name_suffix_sys: &syn::Ident,
    read_ident: &syn::Ident,
) {
    let fn_name = syn::Ident::new(&format!("get_{}", fn_name_suffix), read_ident.span());
    let fn_name_sys = syn::Ident::new(
        &format!("get_{}", fn_name_suffix_sys),
        fn_name_suffix_sys.span(),
    );
    let rust_ty = ty.rust_type_quote_for_getter();
    let from_sys_quote = ty.from_sys_quote(quote! { value_sys });
    let value_from_sys_quote = match &prop.ty {
        InputContainerType::Single(_) => quote! { #from_sys_quote },
        InputContainerType::FixedArray(_, _) | InputContainerType::Array(_)
            if ty.is_from_sys_identity() =>
        {
            quote! { value_sys }
        }
        InputContainerType::FixedArray(_, _) => {
            quote! { value_sys.map(|value_sys| #from_sys_quote) }
        }
        InputContainerType::Array(_) => {
            quote! { value_sys.into_iter().map(|value_sys| #from_sys_quote).collect() }
        }
    };

    let docs = quote! {
        /// ## SAFETY
        ///
        /// - `self.0` must be a valid handle of
        ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
        /// - `suite` must be a valid pointer to
        ///   [`crate::sys_umbrella::OfxPropertySuiteV1`].
    };

    match &prop.ty {
        InputContainerType::Single(_) => output.extend(
            quote! {
                #docs
                pub unsafe fn #fn_name(&self, suite: impl Into<*const crate::sys_umbrella::OfxPropertySuiteV1>) -> crate::low::Result<#rust_ty> {
                    let value_sys = crate::sys_helpers_properties_umbrella::#fn_name_sys(suite.into(), self.sys_handle())
                        .map_err(crate::low::Status::from)?;
                    Ok(#value_from_sys_quote)
                }
            },
        ),
        InputContainerType::FixedArray(_, size) => output.extend(
            quote! {
                #docs
                pub unsafe fn #fn_name(&self, suite: impl Into<*const crate::sys_umbrella::OfxPropertySuiteV1>) -> crate::low::Result<[#rust_ty; #size]> {
                    let value_sys = crate::sys_helpers_properties_umbrella::#fn_name_sys(suite.into(), self.sys_handle())
                        .map_err(crate::low::Status::from)?;
                    Ok(#value_from_sys_quote)
                }
            },
        ),
        InputContainerType::Array(_) => {
            let fn_name_dimensions = syn::Ident::new(
                &format!("len_{}", prop.simple_name),
                proc_macro2::Span::call_site(),
            );
            let sys_ty = ty.sys().rust_type_quote_for_getter();

            output.extend(
                quote! {
                    #docs
                    pub unsafe fn #fn_name(&self, suite: impl Into<*const crate::sys_umbrella::OfxPropertySuiteV1>) -> crate::low::Result<::std::vec::Vec<#rust_ty>> {
                        let suite = suite.into();
                        let dimensions = self.#fn_name_dimensions(suite)?;
                        let mut value_sys = ::std::vec![unsafe { ::std::mem::zeroed::<#sys_ty>() }; dimensions as usize];
                        crate::sys_helpers_properties_umbrella::#fn_name_sys(suite, self.sys_handle(), &mut value_sys)
                            .map_err(crate::low::Status::from)?;
                        Ok(#value_from_sys_quote)
                    }
                },
            );
        }
    }
}

fn make_property_resetter(
    output: &mut proc_macro2::TokenStream,
    prop: &InputPropertyItem,
    write_ident: &syn::Ident,
) {
    let fn_name = syn::Ident::new(
        &format!(
            "reset_{}",
            prop.simple_name.to_string().trim_start_matches("r#")
        ),
        write_ident.span(),
    );
    let property_name = syn::Ident::new(
        &format!("k{}", prop.canonical_name),
        prop.canonical_name.span(),
    );

    output.extend(
        quote! {
            /// ## SAFETY
            ///
            /// - `self.0` must be a valid handle of
            ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
            /// - `suite` must be a valid pointer to
            ///   [`crate::sys_umbrella::OfxPropertySuiteV1`].
            pub unsafe fn #fn_name(&self, suite: impl Into<*const crate::sys_umbrella::OfxPropertySuiteV1>) -> crate::low::Result<()> {
                let status = unsafe {
                    crate::sys_helpers::generic::properties::reset_property(
                        suite.into(),
                        self.sys_handle(),
                        crate::sys_umbrella::#property_name.as_ptr(),
                    )
                };
                status.map_err(crate::low::Status::from)
            }
        }
    );
}

fn make_property_dimensions_getter(
    output: &mut proc_macro2::TokenStream,
    prop: &InputPropertyItem,
    read_ident: &syn::Ident,
) {
    let fn_name = syn::Ident::new(
        &format!(
            "len_{}",
            prop.simple_name.to_string().trim_start_matches("r#")
        ),
        read_ident.span(),
    );
    let property_name = syn::Ident::new(
        &format!("k{}", prop.canonical_name),
        prop.canonical_name.span(),
    );

    output.extend(
        quote! {
            /// ## SAFETY
            ///
            /// - `self.0` must be a valid handle of
            ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
            /// - `suite` must be a valid pointer to
            ///   [`crate::sys_umbrella::OfxPropertySuiteV1`].
            pub unsafe fn #fn_name(&self, suite: impl Into<*const crate::sys_umbrella::OfxPropertySuiteV1>) -> crate::low::Result<::std::os::raw::c_int> {
                let status = unsafe {
                    crate::sys_helpers::generic::properties::get_property_dimension(
                        suite.into(),
                        self.sys_handle(),
                        crate::sys_umbrella::#property_name.as_ptr(),
                    )
                };
                status.map_err(crate::low::Status::from)
            }
        }
    );
}

/// ## Examples
///
/// ```rust,ignore
/// openfx_internal_macros::low_make_property_set_structs! {
///     CustomParamInterpFuncIn {
///         r/_ custom_value: [String; 2] @OfxParamPropCustomValue;
///         r/_ interpolation_amount: Double @OfxParamPropInterpolationAmount;
///         r/w interpolation_time: [Double; 2] @OfxParamPropInterpolationTime;
///     }
/// }
/// ```
///
/// ```rust,ignore
/// openfx_internal_macros::low_make_property_set_structs! {
///     ClipInstance {
///         r/_ r#type: String @OfxPropType;
///         // …
///         r/_ supported_components: [Enum(ImageEffectPropSupportedComponents)] @OfxImageEffectPropSupportedComponents;
///         r/_ temporal_clip_access: Bool @OfxImageEffectPropTemporalClipAccess;
///     }
///     // …
///     EffectInstance {
///         r/_ r#type: String @OfxPropType;
///         r/_ context: Enum(ImageEffectPropContext) @OfxImageEffectPropContext;
///         r/_ instance_data: Pointer @OfxPropInstanceData;
///         // …
///     }
///     // …
///     ParamDouble1D {
///         // …
///         r/_ default: [(Int | Double | String | Pointer)] @OfxParamPropDefault;
///         // …
///     }
/// }
/// ```
struct Input {
    items: Vec<InputPropertySet>,
}

impl syn::parse::Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut items = Vec::new();
        while !input.is_empty() {
            let item: InputPropertySet = input.parse()?;
            items.push(item);
        }
        Ok(Input { items })
    }
}

struct InputPropertySet {
    name: syn::Ident,
    items: syn::punctuated::Punctuated<InputPropertyItem, syn::Token![;]>,
}

impl syn::parse::Parse for InputPropertySet {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: syn::Ident = input.parse()?;
        let content;
        syn::braced!(content in input);
        let items = content.parse_terminated(InputPropertyItem::parse, syn::Token![;])?;
        Ok(InputPropertySet { name, items })
    }
}

struct InputPropertyItem {
    accessors: InputAccessorsFunctions,
    simple_name: syn::Ident,
    ty: InputContainerType,
    canonical_name: syn::Ident,
}

impl syn::parse::Parse for InputPropertyItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let accessors: InputAccessorsFunctions = input.parse()?;
        let simple_name: syn::Ident = input.parse()?;
        input.parse::<syn::Token![:]>()?;
        let ty: InputContainerType = input.parse()?;
        input.parse::<syn::Token![@]>()?;
        let canonical_name: syn::Ident = input.parse()?;
        Ok(InputPropertyItem {
            accessors,
            simple_name,
            ty,
            canonical_name,
        })
    }
}

enum InputContainerType {
    Single(InputPossibleTypes),
    FixedArray(InputPossibleTypes, usize),
    Array(InputPossibleTypes),
}

impl syn::parse::Parse for InputContainerType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::token::Bracket) {
            let content;
            syn::bracketed!(content in input);
            let ty = content.parse()?;

            if content.peek(syn::Token![;]) {
                content.parse::<syn::Token![;]>()?;
                let length: syn::LitInt = content.parse()?;
                let length = length.base10_parse()?;
                Ok(Self::FixedArray(ty, length))
            } else {
                Ok(Self::Array(ty))
            }
        } else {
            Ok(Self::Single(input.parse()?))
        }
    }
}

impl InputContainerType {
    fn possible_types(&self) -> &InputPossibleTypes {
        match self {
            InputContainerType::Single(ty) => ty,
            InputContainerType::FixedArray(ty, _) => ty,
            InputContainerType::Array(ty) => ty,
        }
    }
}

struct InputPossibleTypes {
    int: Option<syn::Ident>,
    double: Option<syn::Ident>,
    r#enum: Option<(syn::Ident, syn::Ident)>,
    bool: Option<syn::Ident>,
    string: Option<syn::Ident>,
    pointer: Option<syn::Ident>,
}

impl syn::parse::Parse for InputPossibleTypes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut int = None;
        let mut double = None;
        let mut r#enum = None;
        let mut bool = None;
        let mut string = None;
        let mut pointer = None;

        fn try_set(slot: &mut Option<syn::Ident>, value: syn::Ident) -> syn::Result<()> {
            if slot.is_some() {
                return Err(syn::Error::new_spanned(
                    value,
                    "duplicate type in property type set",
                ));
            }
            *slot = Some(value);
            Ok(())
        }
        fn try_set_enum(
            slot: &mut Option<(syn::Ident, syn::Ident)>,
            value: (syn::Ident, syn::Ident),
        ) -> syn::Result<()> {
            if slot.is_some() {
                return Err(syn::Error::new_spanned(
                    value.0.clone(),
                    "duplicate type in property type set",
                ));
            }
            *slot = Some(value);
            Ok(())
        }

        if input.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);

            while !content.is_empty() {
                let ty: syn::Ident = content.parse()?;
                match ty.to_string().as_str() {
                    "Int" => try_set(&mut int, ty)?,
                    "Double" => try_set(&mut double, ty)?,
                    "Bool" => try_set(&mut bool, ty)?,
                    "String" => try_set(&mut string, ty)?,
                    "Pointer" => try_set(&mut pointer, ty)?,
                    "Enum" => {
                        let enum_content;
                        syn::parenthesized!(enum_content in content);
                        let enum_type = enum_content.parse()?;
                        if !enum_content.is_empty() {
                            return Err(enum_content.error("expected a single enum type"));
                        }
                        try_set_enum(&mut r#enum, (ty, enum_type))?;
                    }
                    _ => return Err(syn::Error::new_spanned(ty, "expected a property type")),
                }

                if content.peek(syn::Token![|]) {
                    content.parse::<syn::Token![|]>()?;
                }
            }
        } else {
            let ty: syn::Ident = input.parse()?;
            match ty.to_string().as_str() {
                "Int" => int = Some(ty),
                "Double" => double = Some(ty),
                "Bool" => bool = Some(ty),
                "String" => string = Some(ty),
                "Pointer" => pointer = Some(ty),
                "Enum" => {
                    let content;
                    syn::parenthesized!(content in input);
                    let enum_type = content.parse()?;
                    if !content.is_empty() {
                        return Err(content.error("expected a single enum type"));
                    }
                    r#enum = Some((ty, enum_type));
                }
                _ => return Err(syn::Error::new_spanned(ty, "expected a property type")),
            }
        }

        if let Some(bool) = &bool
            && int.is_some()
        {
            return Err(syn::Error::new_spanned(
                bool,
                "Int and Bool types are mutually exclusive.",
            ));
        }
        if let Some(r#enum) = &r#enum
            && string.is_some()
        {
            return Err(syn::Error::new_spanned(
                r#enum.0.clone(),
                "Enum and String types are mutually exclusive.",
            ));
        }

        Ok(Self {
            int,
            double,
            r#enum,
            bool,
            string,
            pointer,
        })
    }
}

impl InputPossibleTypes {
    fn iter(&self) -> OpenFXTypeLowIterator {
        let mut types = Vec::new();
        if let Some(int) = &self.int {
            types.push(OpenFXTypeLow::Int(int.clone()));
        }
        if let Some(double) = &self.double {
            types.push(OpenFXTypeLow::Double(double.clone()));
        }
        if let Some((r#enum, name)) = &self.r#enum {
            types.push(OpenFXTypeLow::Enum(r#enum.clone(), name.clone()));
        }
        if let Some(bool) = &self.bool {
            types.push(OpenFXTypeLow::Bool(bool.clone()));
        }
        if let Some(string) = &self.string {
            types.push(OpenFXTypeLow::String(string.clone()));
        }
        if let Some(pointer) = &self.pointer {
            types.push(OpenFXTypeLow::Pointer(pointer.clone()));
        }
        OpenFXTypeLowIterator(types)
    }
}

struct OpenFXTypeLowIterator(Vec<OpenFXTypeLow>);

impl Iterator for OpenFXTypeLowIterator {
    type Item = OpenFXTypeLow;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

struct InputAccessorsFunctions {
    read: Option<syn::Ident>,
    write: Option<syn::Ident>,
}

impl syn::parse::Parse for InputAccessorsFunctions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let read = if input.peek(syn::Token![_]) {
            input.parse::<syn::Token![_]>()?;
            None
        } else {
            let marker: syn::Ident = input.parse()?;
            match marker.to_string().as_str() {
                "r" => Some(marker),
                _ => return Err(syn::Error::new(marker.span(), "expected `r` or `_`")),
            }
        };

        input.parse::<syn::Token![/]>()?;

        let write = if input.peek(syn::Token![_]) {
            input.parse::<syn::Token![_]>()?;
            None
        } else {
            let marker: syn::Ident = input.parse()?;
            match marker.to_string().as_str() {
                "w" => Some(marker),
                _ => return Err(syn::Error::new(marker.span(), "expected `w` or `_`")),
            }
        };

        Ok(InputAccessorsFunctions { read, write })
    }
}

pub enum OpenFXTypeLow {
    Int(syn::Ident),
    Double(syn::Ident),
    Enum(syn::Ident, syn::Ident),
    Bool(syn::Ident),
    String(syn::Ident),
    Pointer(syn::Ident),
}

impl OpenFXTypeLow {
    fn sys(&self) -> OpenFXTypeSys {
        match self {
            OpenFXTypeLow::Int(ident) | OpenFXTypeLow::Bool(ident) => {
                OpenFXTypeSys::Int(ident.clone())
            }
            OpenFXTypeLow::Double(ident) => OpenFXTypeSys::Double(ident.clone()),
            OpenFXTypeLow::String(ident) | OpenFXTypeLow::Enum(ident, _) => {
                OpenFXTypeSys::String(ident.clone())
            }
            OpenFXTypeLow::Pointer(ident) => OpenFXTypeSys::Pointer(ident.clone()),
        }
    }

    fn rust_type_quote_for_setter(&self) -> proc_macro2::TokenStream {
        match self {
            OpenFXTypeLow::Int(ident) => {
                let c_int = quote_spanned! { ident.span() => c_int };
                quote! { ::std::os::raw::#c_int }
            }
            OpenFXTypeLow::Double(ident) => {
                let f64 = quote_spanned! { ident.span() => f64 };
                quote! { ::core::primitive::#f64 }
            }
            OpenFXTypeLow::Enum(ident, name) => {
                quote_spanned! { ident.span() => crate::low::enums::#name }
            }
            OpenFXTypeLow::Bool(ident) => {
                let bool = quote_spanned! { ident.span() => bool };
                quote! { ::core::primitive::#bool }
            }
            OpenFXTypeLow::String(ident) => {
                let cstr = quote_spanned! { ident.span() => CStr };
                quote! { ::std::option::Option<&::std::ffi::#cstr> }
            }
            OpenFXTypeLow::Pointer(ident) => {
                let c_void = quote_spanned! { ident.span() => c_void };
                quote! { ::std::option::Option<::std::ptr::NonNull<::std::ffi::#c_void>> }
            }
        }
    }

    fn rust_type_quote_for_getter(&self) -> proc_macro2::TokenStream {
        match self {
            OpenFXTypeLow::Int(ident) => {
                let c_int = quote_spanned! { ident.span() => c_int };
                quote! { ::std::os::raw::#c_int }
            }
            OpenFXTypeLow::Double(ident) => {
                let f64 = quote_spanned! { ident.span() => f64 };
                quote! { ::core::primitive::#f64 }
            }
            OpenFXTypeLow::Enum(ident, name) => {
                quote_spanned! { ident.span() => crate::low::enums::#name }
            }
            OpenFXTypeLow::Bool(ident) => {
                let bool = quote_spanned! { ident.span() => bool };
                quote! { ::core::primitive::#bool }
            }
            OpenFXTypeLow::String(ident) => {
                let cstr = quote_spanned! { ident.span() => CStr };
                quote! { ::std::option::Option<&::std::ffi::#cstr> }
            }
            OpenFXTypeLow::Pointer(ident) => {
                let c_void = quote_spanned! { ident.span() => c_void };
                quote! { ::std::option::Option<::std::ptr::NonNull<::std::ffi::#c_void>> }
            }
        }
    }

    fn as_sys_quote(&self, low_val: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        match self {
            OpenFXTypeLow::Int(_) | OpenFXTypeLow::Double(_) => {
                quote! { #low_val }
            }
            OpenFXTypeLow::Pointer(_) => {
                quote! { #low_val.map_or(::std::ptr::null_mut(), ::std::ptr::NonNull::as_ptr) }
            }
            OpenFXTypeLow::Bool(_) => {
                quote! { #low_val as ::std::os::raw::c_int }
            }
            OpenFXTypeLow::Enum(_, _) => {
                quote! { #low_val.as_ptr() }
            }
            OpenFXTypeLow::String(_) => {
                quote! { #low_val.map_or(::std::ptr::null(), ::std::ffi::CStr::as_ptr) }
            }
        }
    }

    fn is_as_sys_identity(&self) -> bool {
        match self {
            OpenFXTypeLow::Int(_) | OpenFXTypeLow::Double(_) => true,
            OpenFXTypeLow::Bool(_)
            | OpenFXTypeLow::Enum(_, _)
            | OpenFXTypeLow::String(_)
            | OpenFXTypeLow::Pointer(_) => false,
        }
    }

    #[expect(clippy::wrong_self_convention)]
    fn from_sys_quote(&self, sys_val: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        match self {
            OpenFXTypeLow::Int(_) | OpenFXTypeLow::Double(_) => {
                quote! { #sys_val }
            }
            OpenFXTypeLow::Enum(_, name) => {
                quote! { unsafe { crate::low::enums::#name::from_ptr_null_checked(#sys_val) } }
            }
            OpenFXTypeLow::Bool(_) => {
                quote! { #sys_val != 0 }
            }
            OpenFXTypeLow::String(_) => {
                quote! {
                    if #sys_val.is_null() {
                        None
                    } else {
                        Some(unsafe { ::std::ffi::CStr::from_ptr(#sys_val) })
                    }
                }
            }
            OpenFXTypeLow::Pointer(_) => {
                quote! {
                    ::std::ptr::NonNull::new(#sys_val)
                }
            }
        }
    }

    fn is_from_sys_identity(&self) -> bool {
        match self {
            OpenFXTypeLow::Int(_) | OpenFXTypeLow::Double(_) => true,
            OpenFXTypeLow::Enum(_, _)
            | OpenFXTypeLow::Bool(_)
            | OpenFXTypeLow::String(_)
            | OpenFXTypeLow::Pointer(_) => false,
        }
    }
}

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

    let cname = syn::Ident::new(
        &format!("k{}", prop.canonical_name),
        prop.canonical_name.span(),
    );

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

        if let Some(write_ident) = &prop.accessors.write {
            make_property_setter(output, prop, &cname, &ty, &fn_name_suffix, write_ident);
        }
        if let Some(read_ident) = &prop.accessors.read {
            make_property_getter(output, prop, &cname, &ty, &fn_name_suffix, read_ident);
        }
    }

    if let Some(write_ident) = &prop.accessors.write {
        make_property_resetter(output, prop, &cname, write_ident);
    }
    if let Some(read_ident) = &prop.accessors.read
        && matches!(prop.ty, InputContainerType::Slice(_))
    {
        make_property_dimensions_getter(output, prop, &cname, read_ident);
    }
}

fn make_property_setter(
    output: &mut proc_macro2::TokenStream,
    prop: &InputPropertyItem,
    cname: &syn::Ident,
    ty: &OpenFXTypeLow,
    fn_name_suffix: &str,
    write_ident: &syn::Ident,
) {
    let fn_name = syn::Ident::new(&format!("set_{}", fn_name_suffix), write_ident.span());
    let fn_name_on_suite = syn::Ident::new(
        &format!(
            "prop_set_{}{}",
            ty.sys().name_snake_case(),
            prop.ty.as_name_suffix()
        ),
        proc_macro2::Span::call_site(),
    );

    let rust_ty = ty.rust_type_quote_for_setter();
    let container_ty = match &prop.ty {
        InputContainerType::Scalar(_) => rust_ty.clone(),
        InputContainerType::Array(_, size) => {
            quote! { [#rust_ty; #size] }
        }
        InputContainerType::Slice(_) => quote! { &[#rust_ty] },
    };

    let as_simple_quote = ty.as_simple_quote(quote! { value });
    let value_as_simple_quote = match &prop.ty {
        InputContainerType::Scalar(_) => as_simple_quote,
        InputContainerType::Array(_, _) if ty.is_as_simple_identity() => {
            quote! { value }
        }
        InputContainerType::Array(_, _) => quote! { value.map(|value| #as_simple_quote) },
        InputContainerType::Slice(_) if ty.is_as_simple_identity() => {
            quote! { value }
        }
        InputContainerType::Slice(_) => {
            quote! { &value.iter().map(|value| #as_simple_quote).collect::<Vec<_>>() }
        }
    };

    output.extend(
        quote! {
            /// ## SAFETY
            ///
            /// - `self.0` must be a valid handle of
            ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
            /// - `suite` must be valid.
            pub unsafe fn #fn_name(&self, suite: &crate::low_plugin::suites::PropertySuiteV1, value: #container_ty) -> crate::low::Result<()> {
                let value_simple = #value_as_simple_quote;
                suite.#fn_name_on_suite(self.sys_handle(), crate::sys_umbrella::#cname, value_simple)
            }
        }
    );
}

fn make_property_getter(
    output: &mut proc_macro2::TokenStream,
    prop: &InputPropertyItem,
    cname: &syn::Ident,
    ty: &OpenFXTypeLow,
    fn_name_suffix: &str,
    read_ident: &syn::Ident,
) {
    let fn_name = syn::Ident::new(&format!("get_{}", fn_name_suffix), read_ident.span());
    let fn_name_on_suite = syn::Ident::new(
        &format!(
            "prop_get_{}{}",
            ty.sys().name_snake_case(),
            prop.ty.as_name_suffix()
        ),
        proc_macro2::Span::call_site(),
    );

    let rust_ty = match &prop.ty {
        InputContainerType::Scalar(_) => ty.rust_type_quote_for_getter(Some(quote! {'a})),
        InputContainerType::Array(_, size) if ty.is_from_simple_identity() => {
            ty.rust_type_quote_for_getter(Some(quote! {'a}))
        }
        _ => ty.rust_type_quote_for_getter(None),
    };
    let from_simple_quote = ty.from_simple_quote(quote! { value_simple });
    let value_from_simple_quote = match &prop.ty {
        InputContainerType::Scalar(_) => quote! { #from_simple_quote },
        InputContainerType::Array(_, _) | InputContainerType::Slice(_)
            if ty.is_from_simple_identity() =>
        {
            quote! { value_simple }
        }
        InputContainerType::Array(_, _) => {
            quote! { value_simple.into_iter().map(|value_simple| #from_simple_quote) }
        }
        InputContainerType::Slice(_) => {
            quote! { value_simple.into_iter().map(|value_simple| #from_simple_quote).collect() }
        }
    };

    let docs = quote! {
        /// ## SAFETY
        ///
        /// - `self.0` must be a valid handle of
        ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
        /// - `suite` must be valid.
    };

    match &prop.ty {
        InputContainerType::Scalar(_) => output.extend(
            quote! {
                #docs
                pub unsafe fn #fn_name<'a, 'b>(&'a self, suite: &'b crate::low_plugin::suites::PropertySuiteV1) -> crate::low::Result<#rust_ty>
                    where 'b: 'a
                {
                    let value_simple = suite.#fn_name_on_suite(self.sys_handle(), crate::sys_umbrella::#cname)?;
                    let value = #value_from_simple_quote;
                    Ok(value)
                }
            },
        ),
        InputContainerType::Array(_, size) if ty.is_from_simple_identity() => output.extend(
            quote! {
                #docs
                pub unsafe fn #fn_name<'a, 'b>(&'a self, suite: &'b crate::low_plugin::suites::PropertySuiteV1) -> crate::low::Result<[#rust_ty; #size]>
                    where 'b: 'a
                {
                    let value = suite.#fn_name_on_suite::<#size>(self.sys_handle(), crate::sys_umbrella::#cname)?;
                    Ok(value)
                }
            }
        ),
        InputContainerType::Array(_, size) => output.extend(
            quote! {
                #docs
                pub unsafe fn #fn_name(&self, suite: &crate::low_plugin::suites::PropertySuiteV1) -> crate::low::Result<[#rust_ty; #size]> {
                    let value_simple = suite.#fn_name_on_suite::<#size>(self.sys_handle(), crate::sys_umbrella::#cname)?;
                    let value = #value_from_simple_quote;
                    Ok(value)
                }
            },
        ),
        InputContainerType::Slice(_) => output.extend(
            quote! {
                #docs
                pub unsafe fn #fn_name(&self, suite: &crate::low_plugin::suites::PropertySuiteV1) -> crate::low::Result<::std::vec::Vec<#rust_ty>> {
                    let dimensions = suite.prop_get_dimension(self.sys_handle(), crate::sys_umbrella::#cname)? as usize;
                    let mut value_simple = ::std::vec::Vec::with_capacity(dimensions);
                    unsafe { value_simple.set_len(dimensions); }
                    suite.#fn_name_on_suite(self.sys_handle(), crate::sys_umbrella::#cname, &mut value_simple)?;
                    let value = #value_from_simple_quote;
                    Ok(value)
                }
            },
        ),
    }
}

fn make_property_resetter(
    output: &mut proc_macro2::TokenStream,
    prop: &InputPropertyItem,
    cname: &syn::Ident,
    write_ident: &syn::Ident,
) {
    let fn_name = syn::Ident::new(
        &format!(
            "reset_{}",
            prop.simple_name.to_string().trim_start_matches("r#")
        ),
        write_ident.span(),
    );

    output.extend(
        quote! {
            /// ## SAFETY
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
    read_ident: &syn::Ident,
) {
    let fn_name = syn::Ident::new(
        &format!(
            "len_{}",
            prop.simple_name.to_string().trim_start_matches("r#")
        ),
        read_ident.span(),
    );

    output.extend(
        quote! {
            /// ## SAFETY
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
    Scalar(InputPossibleTypes),
    Array(InputPossibleTypes, usize),
    Slice(InputPossibleTypes),
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
                Ok(Self::Array(ty, length))
            } else {
                Ok(Self::Slice(ty))
            }
        } else {
            Ok(Self::Scalar(input.parse()?))
        }
    }
}

impl InputContainerType {
    fn possible_types(&self) -> &InputPossibleTypes {
        match self {
            InputContainerType::Scalar(ty) => ty,
            InputContainerType::Array(ty, _) => ty,
            InputContainerType::Slice(ty) => ty,
        }
    }

    fn as_name_suffix(&self) -> &str {
        match self {
            InputContainerType::Scalar(_) => "",
            InputContainerType::Array(_, _) => "_array",
            InputContainerType::Slice(_) => "_n",
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

    fn rust_type_quote_for_getter(
        &self,
        life_time: Option<proc_macro2::TokenStream>,
    ) -> proc_macro2::TokenStream {
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
                quote! { ::std::option::Option<& #life_time ::std::ffi::#cstr> }
            }
            OpenFXTypeLow::Pointer(ident) => {
                let c_void = quote_spanned! { ident.span() => c_void };
                quote! { ::std::option::Option<::std::ptr::NonNull<::std::ffi::#c_void>> }
            }
        }
    }

    fn as_simple_quote(&self, low_val: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        match self {
            OpenFXTypeLow::Int(_)
            | OpenFXTypeLow::Double(_)
            | OpenFXTypeLow::String(_)
            | OpenFXTypeLow::Pointer(_) => {
                quote! { #low_val }
            }
            OpenFXTypeLow::Bool(_) => {
                quote! { #low_val as ::std::os::raw::c_int }
            }
            OpenFXTypeLow::Enum(_, _) => {
                quote! { #low_val.as_cstr() }
            }
        }
    }

    fn is_as_simple_identity(&self) -> bool {
        match self {
            OpenFXTypeLow::Int(_)
            | OpenFXTypeLow::Double(_)
            | OpenFXTypeLow::String(_)
            | OpenFXTypeLow::Pointer(_) => true,
            OpenFXTypeLow::Bool(_) | OpenFXTypeLow::Enum(_, _) => false,
        }
    }

    #[expect(clippy::wrong_self_convention)]
    fn from_simple_quote(&self, simple_val: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        match self {
            OpenFXTypeLow::Int(_)
            | OpenFXTypeLow::Double(_)
            | OpenFXTypeLow::String(_)
            | OpenFXTypeLow::Pointer(_) => {
                quote! { #simple_val }
            }
            OpenFXTypeLow::Enum(_, name) => {
                quote! { unsafe { crate::low::enums::#name::from_ptr_null_checked(#simple_val.map_or(std::ptr::null(), |s| s.as_ptr())) } }
            }
            OpenFXTypeLow::Bool(_) => {
                quote! { #simple_val != 0 }
            }
        }
    }

    fn is_from_simple_identity(&self) -> bool {
        match self {
            OpenFXTypeLow::Int(_)
            | OpenFXTypeLow::Double(_)
            | OpenFXTypeLow::String(_)
            | OpenFXTypeLow::Pointer(_) => true,
            OpenFXTypeLow::Bool(_) | OpenFXTypeLow::Enum(_, _) => false,
        }
    }
}

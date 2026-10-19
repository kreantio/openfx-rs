//! ## Agentic activities
//!
//! - The syntax of `make_property_set_structs!` was changed from a highly
//!   customized DSL to a pseudo-Rust syntax. The parsing logic for it was
//!   adapted by AI:
//!   - Author: OpenCode 2 / GLM 5.3 (provider: Z.AI, thinking: Max)
//!   - Reviewed-by: Umaĵo (reviewing and refining)
//! - The syntax of `make_property_set_structs!` was changed to support
//!   `@propsetdef` directly. The parsing logic for it was adapted by AI:
//!   - Author: OpenCode 2 / GLM 5.3 (provider: Z.AI, thinking: Max)
//!   - Author: OpenCode 2 / GLM 5.3 (provider: OpenCode Go, thinking: Max)
//!   - Reviewed-by: Umaĵo (reviewing and refining)

use std::collections::HashMap;

use quote::{quote, quote_spanned};

use crate::{common::type_sys::OpenFXTypeSys, low_macros::utils};

/// ## Examples
///
/// ```rust,ignore
/// make_property_set_structs_v2! {
///     #[propset(write = host)]
///     struct ImageClipInstancePropertySet {
///         // …
///         #[prop(@OfxImageEffectPropPreMultiplication)]
///         image_effect_pre_multiplication: ty_enum!(ImageEffectPropPreMultiplication),
///         #[prop(@OfxImageEffectPropSupportedComponents)]
///         image_effect_supported_components: [ty_enum!(ImageEffectPropSupportedComponents)],
///         #[prop(@OfxImageEffectPropSupportsTiles)]
///         image_effect_supports_tiles: Bool,
///         // …
///         #[prop(@OfxPropType)]
///         r#type: String,
///     }
///     #[propset(write = plugin)]
///     struct ImageEffectDescriptorPropertySet {
///         // …
///         #[prop(@OfxImageEffectPluginPropOverlayInteractV1)]
///         image_effect_plugin_overlay_interact_v_1: Pointer,
///         // …
///         #[prop(write = host, @OfxPluginPropFilePath)]
///         plugin_file_path: String,
///         // …
///         #[prop(@OfxPropType)]
///         r#type: String,
///         // …
///     }
///     #[propset(
///         write = plugin,
///         refs = ParamsAllButGroupPage+ParamsCommon+ParamsDouble+ParamsNumeric+ParamsValue
///     )]
///     struct ParamDouble1DPropertySet {
///         // …
///     }
///     // …
///     #[propsetdefs]
///     mod propsetdefs {
///         #[propsetdef]
///         struct ParamsValue {
///             // …
///             #[prop(@OfxParamPropDefault)]
///             param_default: [ty_one_of!(Int | Double | String | Pointer)],
///             // …
///             #[prop(write = host, @OfxParamPropIsAnimating)]
///             param_is_animating: Bool,
///             // …
///         }
///     }
/// }
/// ```
pub struct Input {
    pub sets: Vec<InputPropertySet>,
    pub defs: Vec<InputPropsetdef>,
}

impl syn::parse::Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content = utils::extract_dummy_mod_content(input)?;

        let mut sets = vec![];
        let mut defs = vec![];

        while !content.is_empty() {
            if utils::peek_attribute(&content, "propsetdefs") {
                let propsetdefs_mod: InputPropsetdefs = content.parse()?;
                defs.extend(propsetdefs_mod.defs);
            } else {
                let set: InputPropertySet = content.parse()?;
                sets.push(set);
            }
        }

        Ok(Input { sets, defs })
    }
}

/// A `#[propset(write = …[, refs = …])] struct … { … }` item.
pub struct InputPropertySet {
    pub name: syn::Ident,
    /// The default `write` side for properties which do not specify one.
    pub write: InputWriteSide,
    /// The `refs = …` propsetdef names, in order.
    pub refs: syn::punctuated::Punctuated<syn::Ident, syn::Token![+]>,
    pub items: syn::punctuated::Punctuated<InputPropertyItem, syn::Token![,]>,
}

impl syn::parse::Parse for InputPropertySet {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // `#[propset(write = …[, refs = A+B+…])]`
        input.parse::<syn::Token![#]>()?;
        let content;
        syn::bracketed!(content in input);
        let attr_name = content.parse::<syn::Ident>()?;
        if attr_name != "propset" {
            return Err(syn::Error::new_spanned(attr_name, "expected `propset`"));
        }
        let inner;
        syn::parenthesized!(inner in content);
        if !content.is_empty() {
            return Err(content.error("unexpected tokens in attribute"));
        }

        let key = inner.parse::<syn::Ident>()?;
        if key != "write" {
            return Err(syn::Error::new_spanned(key, "expected `write`"));
        }
        inner.parse::<syn::Token![=]>()?;
        let write: InputWriteSide = inner.parse()?;

        let refs = if inner.is_empty() {
            syn::punctuated::Punctuated::new()
        } else {
            inner.parse::<syn::Token![,]>()?;
            let key = inner.parse::<syn::Ident>()?;
            if key != "refs" {
                return Err(syn::Error::new_spanned(key, "expected `refs`"));
            }
            inner.parse::<syn::Token![=]>()?;
            syn::punctuated::Punctuated::parse_separated_nonempty(&inner)?
        };
        if !inner.is_empty() {
            return Err(inner.error("unexpected tokens in `propset` attribute"));
        }

        input.parse::<syn::Token![struct]>()?;
        let name: syn::Ident = input.parse()?;
        let body;
        syn::braced!(body in input);
        let items = body.parse_terminated(InputPropertyItem::parse, syn::Token![,])?;

        Ok(InputPropertySet {
            name,
            write,
            refs,
            items,
        })
    }
}

impl InputPropertySet {
    /// Expands the `refs = …` propsetdefs into the set and resolves the
    /// `write` side of every property:
    ///
    /// - a specified `write` decides over unspecified ones;
    /// - multiple specified `write`s must agree, else it errors;
    /// - if none is specified, the set-level `write` applies.
    pub fn resolved_props<'a>(
        &'a self,
        propsetdefs: &HashMap<&'a syn::Ident, &'a InputPropsetdef>,
    ) -> syn::Result<Vec<(&'a InputPropertyItem, &'a InputWriteSide)>> {
        /// Adds an occurrence of a property; later occurrences of the same
        /// canonical name only contribute their specified `write`.
        fn add<'a>(
            item: &'a InputPropertyItem,
            merged: &mut Vec<&'a InputPropertyItem>,
            specified_writes: &mut Vec<Vec<&'a InputWriteSide>>,
            seen: &mut HashMap<&'a syn::Ident, usize>,
        ) {
            match seen.entry(&item.canonical_name) {
                std::collections::hash_map::Entry::Occupied(occupied) => {
                    if let Some(write) = &item.write {
                        specified_writes[*occupied.get()].push(write);
                    }
                }
                std::collections::hash_map::Entry::Vacant(vacant) => {
                    vacant.insert(merged.len());
                    merged.push(item);
                    specified_writes.push(item.write.iter().collect());
                }
            }
        }

        let mut merged: Vec<&InputPropertyItem> = vec![];
        let mut specified_writes: Vec<Vec<&InputWriteSide>> = vec![];
        let mut seen: HashMap<&syn::Ident, usize> = HashMap::new();

        for item in &self.items {
            add(item, &mut merged, &mut specified_writes, &mut seen);
        }
        for ref_name in &self.refs {
            let Some(def) = propsetdefs.get(ref_name) else {
                return Err(syn::Error::new_spanned(
                    ref_name,
                    format!("unknown propsetdef `{ref_name}`"),
                ));
            };
            for item in &def.items {
                add(item, &mut merged, &mut specified_writes, &mut seen);
            }
        }

        merged
            .into_iter()
            .zip(specified_writes)
            .map(|(item, specified_writes)| {
                let write = if specified_writes.is_empty() {
                    &self.write
                } else {
                    let first = specified_writes[0];
                    for write in &specified_writes[1..] {
                        if *write != first {
                            return Err(syn::Error::new_spanned(
                                write.ident(),
                                format!(
                                    "conflicting `write` specifications for property `{}`: `{}` and `{}`",
                                    item.canonical_name,
                                    first.as_str(),
                                    write.as_str(),
                                ),
                            ));
                        }
                    }
                    first
                };

                Ok((item, write))
            })
            .collect()
    }
}

/// A `#[propsetdefs] mod … { … }` item collecting named [`InputPropsetdef`]s.
pub struct InputPropsetdefs {
    pub defs: Vec<InputPropsetdef>,
}

impl syn::parse::Parse for InputPropsetdefs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // `#[propsetdefs]`
        input.parse::<syn::Token![#]>()?;
        let content;
        syn::bracketed!(content in input);
        let attr_name = content.parse::<syn::Ident>()?;
        if attr_name != "propsetdefs" {
            return Err(syn::Error::new_spanned(attr_name, "expected `propsetdefs`"));
        }
        if !content.is_empty() {
            return Err(content.error("unexpected tokens in attribute"));
        }

        let body = utils::extract_dummy_mod_content(input)?;

        let mut defs = vec![];
        while !body.is_empty() {
            defs.push(body.parse()?);
        }

        Ok(InputPropsetdefs { defs })
    }
}

/// A `#[propsetdef] struct … { … }` definition inside an [`InputPropsetdefs`].
pub struct InputPropsetdef {
    pub name: syn::Ident,
    pub items: syn::punctuated::Punctuated<InputPropertyItem, syn::Token![,]>,
}

impl syn::parse::Parse for InputPropsetdef {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // `#[propsetdef]`
        input.parse::<syn::Token![#]>()?;
        let content;
        syn::bracketed!(content in input);
        let attr_name = content.parse::<syn::Ident>()?;
        if attr_name != "propsetdef" {
            return Err(syn::Error::new_spanned(attr_name, "expected `propsetdef`"));
        }
        if !content.is_empty() {
            return Err(content.error("unexpected tokens in attribute"));
        }

        input.parse::<syn::Token![struct]>()?;
        let name: syn::Ident = input.parse()?;
        let body;
        syn::braced!(body in input);
        let items = body.parse_terminated(InputPropertyItem::parse, syn::Token![,])?;

        Ok(InputPropsetdef { name, items })
    }
}

/// A single `#[prop(…)] name: Type,` item of a set or a propsetdef.
pub struct InputPropertyItem {
    pub simple_name: syn::Ident,
    pub ty: InputContainerType,
    pub canonical_name: syn::Ident,
    /// The property-level `write` side, if specified.
    pub write: Option<InputWriteSide>,
}

impl syn::parse::Parse for InputPropertyItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attr = input.parse::<InputPropAttribute>()?;
        let simple_name: syn::Ident = input.parse()?;
        input.parse::<syn::Token![:]>()?;
        let ty: InputContainerType = input.parse()?;
        Ok(InputPropertyItem {
            simple_name,
            ty,
            canonical_name: attr.canonical_name,
            write: attr.write,
        })
    }
}

/// The `#[prop([write = …,] @CanonicalName)]` attribute on an
/// [`InputPropertyItem`].
pub struct InputPropAttribute {
    pub write: Option<InputWriteSide>,
    pub canonical_name: syn::Ident,
}

impl syn::parse::Parse for InputPropAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![#]>()?;
        let content;
        syn::bracketed!(content in input);
        let attr_name = content.parse::<syn::Ident>()?;
        if attr_name != "prop" {
            return Err(syn::Error::new_spanned(attr_name, "expected `prop`"));
        }
        let inner;
        syn::parenthesized!(inner in content);
        if !content.is_empty() {
            return Err(content.error("unexpected tokens in attribute"));
        }

        let write = if inner.peek(syn::Token![@]) {
            None
        } else {
            let key = inner.parse::<syn::Ident>()?;
            if key != "write" {
                return Err(syn::Error::new_spanned(key, "expected `write`"));
            }
            inner.parse::<syn::Token![=]>()?;
            let write: InputWriteSide = inner.parse()?;
            inner.parse::<syn::Token![,]>()?;
            Some(write)
        };

        inner.parse::<syn::Token![@]>()?;
        let canonical_name = inner.parse::<syn::Ident>()?;
        if !inner.is_empty() {
            return Err(inner.error("unexpected tokens in `prop` attribute"));
        }

        Ok(InputPropAttribute {
            write,
            canonical_name,
        })
    }
}

pub enum InputContainerType {
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
    pub fn possible_types(&self) -> &InputPossibleTypes {
        match self {
            InputContainerType::Scalar(ty) => ty,
            InputContainerType::Array(ty, _) => ty,
            InputContainerType::Slice(ty) => ty,
        }
    }

    pub fn as_name_suffix(&self) -> &str {
        match self {
            InputContainerType::Scalar(_) => "",
            InputContainerType::Array(_, _) => "_array",
            InputContainerType::Slice(_) => "_n",
        }
    }
}

pub struct InputPossibleTypes {
    pub int: Option<syn::Ident>,
    pub double: Option<syn::Ident>,
    pub r#enum: Option<(syn::Ident, syn::Ident)>,
    pub bool: Option<syn::Ident>,
    pub string: Option<syn::Ident>,
    pub pointer: Option<syn::Ident>,
}

impl syn::parse::Parse for InputPossibleTypes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut this = Self {
            int: None,
            double: None,
            r#enum: None,
            bool: None,
            string: None,
            pointer: None,
        };

        let ty: syn::Ident = input.parse()?;
        match ty.to_string().as_str() {
            // `ty_one_of!(A | B | …)`
            "ty_one_of" => {
                input.parse::<syn::Token![!]>()?;
                let content;
                syn::parenthesized!(content in input);

                while !content.is_empty() {
                    let ty: syn::Ident = content.parse()?;
                    this.set_simple(ty)?;
                    if !content.is_empty() {
                        content.parse::<syn::Token![|]>()?;
                    }
                }
            }
            // `ty_enum!(EnumType)`
            "ty_enum" => {
                input.parse::<syn::Token![!]>()?;
                let content;
                syn::parenthesized!(content in input);
                let enum_type = content.parse()?;
                if !content.is_empty() {
                    return Err(content.error("expected a single enum type"));
                }
                let enum_marker = syn::Ident::new("Enum", ty.span());
                this.r#enum = Some((enum_marker, enum_type));
            }
            // A single simple type, e.g. `Int`.
            _ => this.set_simple(ty)?,
        }

        if let Some(bool) = &this.bool
            && this.int.is_some()
        {
            return Err(syn::Error::new_spanned(
                bool,
                "Int and Bool types are mutually exclusive.",
            ));
        }

        Ok(this)
    }
}

impl InputPossibleTypes {
    /// Fills the slot of a simple property type (`Int`, `Double`, `Bool`,
    /// `String` or `Pointer`), rejecting unknown or duplicated types.
    pub fn set_simple(&mut self, ty: syn::Ident) -> syn::Result<()> {
        let slot = match ty.to_string().as_str() {
            "Int" => &mut self.int,
            "Double" => &mut self.double,
            "Bool" => &mut self.bool,
            "String" => &mut self.string,
            "Pointer" => &mut self.pointer,
            _ => {
                return Err(syn::Error::new_spanned(
                    ty,
                    "expected a simple property type",
                ));
            }
        };
        if slot.is_some() {
            return Err(syn::Error::new_spanned(
                ty,
                "duplicate type in property type set",
            ));
        }
        *slot = Some(ty);
        Ok(())
    }

    pub fn iter(&self) -> OpenFXTypeLowIterator {
        let mut types = vec![];
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

pub struct OpenFXTypeLowIterator(Vec<OpenFXTypeLow>);

impl Iterator for OpenFXTypeLowIterator {
    type Item = OpenFXTypeLow;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

/// The side which writes a property: `host` or `plugin`.
///
/// - `Host`: the host writes the property, thus the plugin may only read it.
/// - `Plugin`: the plugin writes the property.
#[derive(PartialEq)]
pub enum InputWriteSide {
    Host(syn::Ident),
    Plugin(syn::Ident),
}

impl syn::parse::Parse for InputWriteSide {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let side = input.parse::<syn::Ident>()?;
        match side.to_string().as_str() {
            "host" => Ok(Self::Host(side)),
            "plugin" => Ok(Self::Plugin(side)),
            _ => Err(syn::Error::new_spanned(side, "expected `host` or `plugin`")),
        }
    }
}

impl InputWriteSide {
    pub fn ident(&self) -> &syn::Ident {
        match self {
            Self::Host(ident) | Self::Plugin(ident) => ident,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Host(_) => "host",
            Self::Plugin(_) => "plugin",
        }
    }

    pub fn is_host(&self) -> bool {
        matches!(self, Self::Host(_))
    }

    pub fn is_plugin(&self) -> bool {
        matches!(self, Self::Plugin(_))
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
    pub fn sys(&self) -> OpenFXTypeSys {
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

    pub fn rust_type_quote_for_setter(&self) -> proc_macro2::TokenStream {
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
                let known = quote_spanned! { ident.span() => crate::low::enums::#name };
                quote_spanned! { ident.span() => crate::low_plugin::property_sets::EnumPropertyPtr<#known> }
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

    pub fn rust_type_quote_for_getter(&self) -> proc_macro2::TokenStream {
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
                let known = quote_spanned! { ident.span() => crate::low::enums::#name };
                quote! { crate::low_plugin::property_sets::LosslessEnumProperty<#known> }
            }
            OpenFXTypeLow::Bool(ident) => {
                let bool = quote_spanned! { ident.span() => bool };
                quote! { ::core::primitive::#bool }
            }
            OpenFXTypeLow::String(ident) => {
                let c_char = quote_spanned! { ident.span() => c_char };
                quote! { ::std::option::Option<::std::ptr::NonNull<::std::ffi::#c_char>> }
            }
            OpenFXTypeLow::Pointer(ident) => {
                let c_void = quote_spanned! { ident.span() => c_void };
                quote! { ::std::option::Option<::std::ptr::NonNull<::std::ffi::#c_void>> }
            }
        }
    }

    pub fn as_simple_quote(&self, low_val: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        match self {
            OpenFXTypeLow::Int(_) | OpenFXTypeLow::Double(_) | OpenFXTypeLow::Pointer(_) => {
                quote! { #low_val }
            }
            OpenFXTypeLow::String(_) => {
                quote! { #low_val.and_then(|s| ::std::ptr::NonNull::new(s.as_ptr() as *mut _)) }
            }
            OpenFXTypeLow::Bool(_) => {
                quote! { #low_val as ::std::os::raw::c_int }
            }
            OpenFXTypeLow::Enum(_, _) => {
                quote! { #low_val.as_ptr() }
            }
        }
    }

    pub fn is_as_simple_identity(&self) -> bool {
        match self {
            OpenFXTypeLow::Int(_) | OpenFXTypeLow::Double(_) | OpenFXTypeLow::Pointer(_) => true,
            OpenFXTypeLow::Bool(_) | OpenFXTypeLow::String(_) | OpenFXTypeLow::Enum(_, _) => false,
        }
    }

    #[expect(clippy::wrong_self_convention)]
    pub fn from_simple_quote(
        &self,
        simple_val: proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        match self {
            OpenFXTypeLow::Int(_)
            | OpenFXTypeLow::Double(_)
            | OpenFXTypeLow::String(_)
            | OpenFXTypeLow::Pointer(_) => {
                quote! { #simple_val }
            }
            OpenFXTypeLow::Enum(_, name) => {
                let known = quote_spanned! { name.span() => crate::low::enums::#name };
                quote! { unsafe { crate::low_plugin::property_sets::LosslessEnumProperty::<#known>::from_ptr(#simple_val) } }
            }
            OpenFXTypeLow::Bool(_) => {
                quote! { #simple_val != 0 }
            }
        }
    }

    pub fn is_from_simple_identity(&self) -> bool {
        match self {
            OpenFXTypeLow::Int(_)
            | OpenFXTypeLow::Double(_)
            | OpenFXTypeLow::String(_)
            | OpenFXTypeLow::Pointer(_) => true,
            OpenFXTypeLow::Bool(_) | OpenFXTypeLow::Enum(_, _) => false,
        }
    }
}

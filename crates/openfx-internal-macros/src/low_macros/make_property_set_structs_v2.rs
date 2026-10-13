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

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};

use crate::common::type_sys::OpenFXTypeSys;

pub fn make_property_set_structs(tokens: TokenStream) -> TokenStream {
    let mut output_inner = proc_macro2::TokenStream::new();

    let input = syn::parse_macro_input!(tokens as Input);

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
    ty: &OpenFXTypeLow,
    fn_name_suffix: &str,
) {
    let fn_name = syn::Ident::new(&format!("set_{}", fn_name_suffix), prop.simple_name.span());
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
) {
    let fn_name = syn::Ident::new(&format!("get_{}", fn_name_suffix), prop.simple_name.span());
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
struct Input {
    sets: Vec<InputPropertySet>,
    defs: Vec<InputPropsetdef>,
}

impl syn::parse::Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut sets = vec![];
        let mut defs = vec![];

        while !input.is_empty() {
            if peek_attribute(input, "propsetdefs") {
                let propsetdefs_mod: InputPropsetdefs = input.parse()?;
                defs.extend(propsetdefs_mod.defs);
            } else {
                let set: InputPropertySet = input.parse()?;
                sets.push(set);
            }
        }

        Ok(Input { sets, defs })
    }
}

/// Returns whether the next tokens in `input` are a `#[<name> …]` attribute,
/// without consuming anything.
fn peek_attribute(input: syn::parse::ParseStream, name: &str) -> bool {
    fn test(input: syn::parse::ParseStream, name: &str) -> syn::Result<bool> {
        input.parse::<syn::Token![#]>()?;
        let content;
        syn::bracketed!(content in input);
        let attr_name = content.parse::<syn::Ident>()?;
        Ok(attr_name == name)
    }

    let fork = input.fork();
    test(&fork, name).unwrap_or(false)
}

/// A `#[propset(write = …[, refs = …])] struct … { … }` item.
struct InputPropertySet {
    name: syn::Ident,
    /// The default `write` side for properties which do not specify one.
    write: InputWriteSide,
    /// The `refs = …` propsetdef names, in order.
    refs: syn::punctuated::Punctuated<syn::Ident, syn::Token![+]>,
    items: syn::punctuated::Punctuated<InputPropertyItem, syn::Token![,]>,
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
    fn resolved_props<'a>(
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
struct InputPropsetdefs {
    defs: Vec<InputPropsetdef>,
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

        input.parse::<syn::Token![mod]>()?;
        let _name: syn::Ident = input.parse()?;
        let body;
        syn::braced!(body in input);

        let mut defs = vec![];
        while !body.is_empty() {
            defs.push(body.parse()?);
        }

        Ok(InputPropsetdefs { defs })
    }
}

/// A `#[propsetdef] struct … { … }` definition inside an [`InputPropsetdefs`].
struct InputPropsetdef {
    name: syn::Ident,
    items: syn::punctuated::Punctuated<InputPropertyItem, syn::Token![,]>,
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
struct InputPropertyItem {
    simple_name: syn::Ident,
    ty: InputContainerType,
    canonical_name: syn::Ident,
    /// The property-level `write` side, if specified.
    write: Option<InputWriteSide>,
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
struct InputPropAttribute {
    write: Option<InputWriteSide>,
    canonical_name: syn::Ident,
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
    fn set_simple(&mut self, ty: syn::Ident) -> syn::Result<()> {
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

    fn iter(&self) -> OpenFXTypeLowIterator {
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

struct OpenFXTypeLowIterator(Vec<OpenFXTypeLow>);

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
enum InputWriteSide {
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
    fn ident(&self) -> &syn::Ident {
        match self {
            Self::Host(ident) | Self::Plugin(ident) => ident,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            Self::Host(_) => "host",
            Self::Plugin(_) => "plugin",
        }
    }

    fn is_host(&self) -> bool {
        matches!(self, Self::Host(_))
    }

    fn is_plugin(&self) -> bool {
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

use proc_macro::TokenStream;
use quote::quote;

use crate::common::type_sys::OpenFXTypeSys;

pub fn make_property_accessors(tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(tokens as Input);

    let mut output = proc_macro2::TokenStream::new();

    for item in input.items {
        let tys: Vec<_> = item.ty.possible_types().iter().collect();
        let is_ambiguous = tys.len() > 1;

        let k_name = syn::Ident::new(
            &format!("k{}", item.canonical_name),
            item.canonical_name.span(),
        );
        let k_path = quote! { crate::sys_umbrella::#k_name };

        for ty in tys {
            let fn_name_suffix = if is_ambiguous {
                format!("{}_{}", item.canonical_name, ty)
            } else {
                item.canonical_name.to_string()
            };

            if let Some(set_ident) = &item.functions.set {
                make_property_setter(&mut output, &item, &ty, &k_path, &fn_name_suffix, set_ident);
            }
            if let Some(get_ident) = &item.functions.get {
                make_property_getter(&mut output, &item, &ty, &k_path, &fn_name_suffix, get_ident);
            }
        }

        if let Some(reset_ident) = &item.functions.reset {
            make_property_resetter(&mut output, &k_path, &item.canonical_name, reset_ident);
        }
        if let Some(get_dimensions_ident) = &item.functions.get_dimensions {
            make_property_dimensions_getter(
                &mut output,
                &k_path,
                &item.canonical_name,
                get_dimensions_ident,
            );
        }
    }

    output.into()
}

fn make_property_setter(
    output: &mut proc_macro2::TokenStream,
    item: &InputPropertyItem,
    ty: &OpenFXTypeSys,
    k_path: &proc_macro2::TokenStream,
    fn_name_suffix: &str,
    set_ident: &syn::Ident,
) {
    let fn_name = syn::Ident::new(&format!("set_{}", fn_name_suffix), set_ident.span());
    let setter_for_ty_ident = syn::Ident::new(
        &format!(
            "set_{}{}",
            ty.to_string().to_lowercase(),
            item.ty.dimension_suffix()
        ),
        proc_macro2::Span::call_site(),
    );
    let setter_for_ty_path =
        quote! { crate::sys_helpers::generic::properties::#setter_for_ty_ident };

    let rust_ty = ty.rust_type_quote_for_setter();
    let container_ty = match &item.ty {
        InputContainerType::Single(_) => rust_ty.clone(),
        InputContainerType::FixedArray(_, size) => {
            quote! { [#rust_ty; #size] }
        }
        InputContainerType::Array(_) => {
            quote! { &[#rust_ty] }
        }
    };

    output.extend(quote! {
        /// ## SAFETY
        ///
        /// - `suite` must be a valid pointer to
        ///   [`crate::sys_umbrella::OfxPropertySuiteV1`].
        /// - `handle` must be a valid handle of
        ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
        #[inline(always)]
        #[allow(non_snake_case)]
        pub unsafe fn #fn_name(
            suite: *const crate::sys_umbrella::OfxPropertySuiteV1,
            handle: crate::sys_umbrella::OfxPropertySetHandle,
            value: #container_ty,
        ) -> Result<(), crate::sys_umbrella::OfxStatus> {
            // SAFETY: Type safety is guaranteed by the standard this function
            // is derived from. The caller guarantees the remaining safety
            // requirements.
            unsafe { #setter_for_ty_path(suite, handle, #k_path.as_ptr(), value) }
        }
    });
}

fn make_property_getter(
    output: &mut proc_macro2::TokenStream,
    item: &InputPropertyItem,
    ty: &OpenFXTypeSys,
    k_path: &proc_macro2::TokenStream,
    fn_name_suffix: &str,
    get_ident: &syn::Ident,
) {
    let fn_name = syn::Ident::new(&format!("get_{}", fn_name_suffix), get_ident.span());
    let getter_for_ty_ident = syn::Ident::new(
        &format!(
            "get_{}{}",
            ty.to_string().to_lowercase(),
            item.ty.dimension_suffix()
        ),
        proc_macro2::Span::call_site(),
    );
    let getter_for_ty_path =
        quote! { crate::sys_helpers::generic::properties::#getter_for_ty_ident };

    let rust_ty = ty.rust_type_quote_for_getter();
    let container_ty = match &item.ty {
        InputContainerType::Single(_) => rust_ty.clone(),
        InputContainerType::FixedArray(_, size) => {
            quote! { [#rust_ty; #size] }
        }
        InputContainerType::Array(_) => {
            quote! { [#rust_ty] }
        }
    };

    if matches!(item.ty, InputContainerType::Array(_)) {
        output.extend(quote! {
            /// ## SAFETY
            ///
            /// - `suite` must be a valid pointer to
            ///   [`crate::sys_umbrella::OfxPropertySuiteV1`].
            /// - `handle` must be a valid handle of
            ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
            #[inline(always)]
            #[allow(non_snake_case)]
            pub unsafe fn #fn_name(
                suite: *const crate::sys_umbrella::OfxPropertySuiteV1,
                handle: crate::sys_umbrella::OfxPropertySetHandle,
                values: &mut #container_ty,
            ) -> Result<(), crate::sys_umbrella::OfxStatus> {
                // SAFETY: Type safety is guaranteed by the standard this
                // function is derived from. The caller guarantees the remaining
                // safety requirements.
                unsafe { #getter_for_ty_path(suite, handle, #k_path.as_ptr(), values) }
            }
        });
    } else {
        output.extend(quote! {
            /// ## SAFETY
            ///
            /// - `suite` must be a valid pointer to
            ///   [`crate::sys_umbrella::OfxPropertySuiteV1`].
            /// - `handle` must be a valid handle of
            ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
            #[inline(always)]
            #[allow(non_snake_case)]
            pub unsafe fn #fn_name(
                suite: *const crate::sys_umbrella::OfxPropertySuiteV1,
                handle: crate::sys_umbrella::OfxPropertySetHandle,
            ) -> Result<#container_ty, crate::sys_umbrella::OfxStatus> {
                // SAFETY: Type safety is guaranteed by the standard this
                // function is derived from. The caller guarantees the remaining
                // safety requirements.
                unsafe { #getter_for_ty_path(suite, handle, #k_path.as_ptr()) }
            }
        });
    }
}

fn make_property_resetter(
    output: &mut proc_macro2::TokenStream,
    k_path: &proc_macro2::TokenStream,
    canonical_name: &syn::Ident,
    reset_ident: &syn::Ident,
) {
    let fn_name = syn::Ident::new(&format!("reset_{}", canonical_name), reset_ident.span());
    let reseter_path = quote! { crate::sys_helpers::generic::properties::reset_property };

    output.extend(quote! {
        /// ## SAFETY
        ///
        /// - `suite` must be a valid pointer to
        ///   [`crate::sys_umbrella::OfxPropertySuiteV1`].
        /// - `handle` must be a valid handle of
        ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
        #[inline(always)]
        #[allow(non_snake_case)]
        pub unsafe fn #fn_name(
            suite: *const crate::sys_umbrella::OfxPropertySuiteV1,
            handle: crate::sys_umbrella::OfxPropertySetHandle,
        ) -> Result<(), crate::sys_umbrella::OfxStatus> {
            // SAFETY: Type safety is guaranteed by the standard this function
            // is derived from. The caller guarantees the remaining safety
            // requirements.
            unsafe { #reseter_path(suite, handle, #k_path.as_ptr()) }
        }
    });
}

fn make_property_dimensions_getter(
    output: &mut proc_macro2::TokenStream,
    k_path: &proc_macro2::TokenStream,
    canonical_name: &syn::Ident,
    get_dimensions_ident: &syn::Ident,
) {
    let fn_name = syn::Ident::new(
        &format!("get_dimensions_{}", canonical_name),
        get_dimensions_ident.span(),
    );
    let get_dimensions_path =
        quote! { crate::sys_helpers::generic::properties::get_property_dimension };

    output.extend(quote! {
        /// ## SAFETY
        ///
        /// - `suite` must be a valid pointer to
        ///   [`crate::sys_umbrella::OfxPropertySuiteV1`].
        /// - `handle` must be a valid handle of
        ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
        #[inline(always)]
        #[allow(non_snake_case)]
        pub unsafe fn #fn_name(
            suite: *const crate::sys_umbrella::OfxPropertySuiteV1,
            handle: crate::sys_umbrella::OfxPropertySetHandle,
        ) -> Result<std::os::raw::c_int, crate::sys_umbrella::OfxStatus> {
            // SAFETY: Type safety is guaranteed by the standard this function
            // is derived from. The caller guarantees the remaining safety
            // requirements.
            unsafe { #get_dimensions_path(suite, handle, #k_path.as_ptr()) }
        }
    });
}

/// ## Examples
///
/// ```rust,ignore
/// sys_helpers_make_property_accessors! {
///     OfxImageClipPropConnected: Int { set get reset };
///     OfxImageEffectPropFrameRange: [Double; 2] { set get reset };
///     OfxImageEffectPropSupportedPixelDepths: [String] { set get reset get_dimensions };
///     OfxParamPropDefault: [(Int | Double | String | Pointer)] { set get reset get_dimensions };
/// }
/// ```
struct Input {
    items: syn::punctuated::Punctuated<InputPropertyItem, syn::Token![;]>,
}

impl syn::parse::Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let items =
            syn::punctuated::Punctuated::<InputPropertyItem, syn::Token![;]>::parse_terminated(
                input,
            )?;

        Ok(Input { items })
    }
}

struct InputPropertyItem {
    canonical_name: syn::Ident,
    ty: InputContainerType,
    functions: InputAccessorFunctions,
}

impl syn::parse::Parse for InputPropertyItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let canonical_name: syn::Ident = input.parse()?;
        input.parse::<syn::Token![:]>()?;
        let ty = input.parse::<InputContainerType>()?;
        let functions = input.parse::<InputAccessorFunctions>()?;
        Ok(InputPropertyItem {
            canonical_name,
            ty,
            functions,
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
            let ty: InputPossibleTypes = content.parse()?;
            if content.peek(syn::Token![;]) {
                content.parse::<syn::Token![;]>()?;
                let size: syn::LitInt = content.parse()?;
                Ok(InputContainerType::FixedArray(
                    ty,
                    size.base10_parse::<usize>()?,
                ))
            } else {
                Ok(InputContainerType::Array(ty))
            }
        } else {
            Ok(InputContainerType::Single(input.parse()?))
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

    fn dimension_suffix(&self) -> String {
        match self {
            InputContainerType::Single(_) => "".to_string(),
            InputContainerType::FixedArray(_, size) => {
                format!("s_{}", size)
            }
            InputContainerType::Array(_) => "s".to_string(),
        }
    }
}

struct InputPossibleTypes {
    int: Option<syn::Ident>,
    double: Option<syn::Ident>,
    string: Option<syn::Ident>,
    pointer: Option<syn::Ident>,
}

impl syn::parse::Parse for InputPossibleTypes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut int = None;
        let mut double = None;
        let mut string = None;
        let mut pointer = None;

        fn try_set(slot: &mut Option<syn::Ident>, value: syn::Ident) -> syn::Result<()> {
            if slot.is_some() {
                return Err(syn::Error::new_spanned(
                    value,
                    "Duplicate type in possible types",
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
                    "Int" => try_set(&mut int, ty.clone())?,
                    "Double" => try_set(&mut double, ty.clone())?,
                    "String" => try_set(&mut string, ty.clone())?,
                    "Pointer" => try_set(&mut pointer, ty.clone())?,
                    _ => return Err(syn::Error::new_spanned(ty, "Unknown type")),
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
                "String" => string = Some(ty),
                "Pointer" => pointer = Some(ty),
                _ => return Err(syn::Error::new_spanned(ty, "Unknown type")),
            }
        }

        Ok(InputPossibleTypes {
            int,
            double,
            string,
            pointer,
        })
    }
}

impl InputPossibleTypes {
    fn iter(&self) -> OpenFXTypeSysIterator {
        let mut types = Vec::new();
        if let Some(ident) = &self.int {
            types.push(OpenFXTypeSys::Int(ident.clone()));
        }
        if let Some(ident) = &self.double {
            types.push(OpenFXTypeSys::Double(ident.clone()));
        }
        if let Some(ident) = &self.string {
            types.push(OpenFXTypeSys::String(ident.clone()));
        }
        if let Some(ident) = &self.pointer {
            types.push(OpenFXTypeSys::Pointer(ident.clone()));
        }
        OpenFXTypeSysIterator(types)
    }
}

struct OpenFXTypeSysIterator(Vec<OpenFXTypeSys>);

impl Iterator for OpenFXTypeSysIterator {
    type Item = OpenFXTypeSys;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

struct InputAccessorFunctions {
    set: Option<syn::Ident>,
    get: Option<syn::Ident>,
    reset: Option<syn::Ident>,
    get_dimensions: Option<syn::Ident>,
}

impl syn::parse::Parse for InputAccessorFunctions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        syn::braced!(content in input);

        let mut set = None;
        let mut get = None;
        let mut reset = None;
        let mut get_dimensions = None;

        while !content.is_empty() {
            let func: syn::Ident = content.parse()?;
            match func.to_string().as_str() {
                "set" => set = Some(func.clone()),
                "get" => get = Some(func.clone()),
                "reset" => reset = Some(func.clone()),
                "get_dimensions" => get_dimensions = Some(func.clone()),
                _ => return Err(syn::Error::new_spanned(func, "Unknown function")),
            }
            if content.peek(syn::Token![,]) {
                content.parse::<syn::Token![,]>()?;
            }
        }

        Ok(InputAccessorFunctions {
            set,
            get,
            reset,
            get_dimensions,
        })
    }
}

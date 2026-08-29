use proc_macro::TokenStream;
use quote::quote;

use crate::sys_helpers_macros::common::OpenFXTypeIdent;

pub fn make_property_accessors(tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(tokens as MakePropertyAccessorsInput);

    let mut output: Vec<proc_macro2::TokenStream> = Vec::new();

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
            make_property_get_dimensions(
                &mut output,
                &k_path,
                &item.canonical_name,
                get_dimensions_ident,
            );
        }
    }

    quote! {
        #(#output)*
    }
    .into()
}

fn make_property_setter(
    output: &mut Vec<proc_macro2::TokenStream>,
    item: &MakePropertyAccessorsInputItem,
    ty: &OpenFXTypeIdent,
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
        quote! { crate::generic::sys_helpers::properties::#setter_for_ty_ident };

    let rust_ty = ty.rust_type_quote_for_setter();
    let container_ty = match &item.ty {
        MakePropertyAccessorsInputItemContainerType::Single(_) => rust_ty.clone(),
        MakePropertyAccessorsInputItemContainerType::FixedArray(_, size) => {
            quote! { [#rust_ty; #size] }
        }
        MakePropertyAccessorsInputItemContainerType::Array(_) => {
            quote! { &[#rust_ty] }
        }
    };

    output.push(quote! {
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
    output: &mut Vec<proc_macro2::TokenStream>,
    item: &MakePropertyAccessorsInputItem,
    ty: &OpenFXTypeIdent,
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
        quote! { crate::generic::sys_helpers::properties::#getter_for_ty_ident };

    let rust_ty = ty.rust_type_quote_for_getter();
    let container_ty = match &item.ty {
        MakePropertyAccessorsInputItemContainerType::Single(_) => rust_ty.clone(),
        MakePropertyAccessorsInputItemContainerType::FixedArray(_, size) => {
            quote! { [#rust_ty; #size] }
        }
        MakePropertyAccessorsInputItemContainerType::Array(_) => {
            quote! { [#rust_ty] }
        }
    };

    if matches!(
        item.ty,
        MakePropertyAccessorsInputItemContainerType::Array(_)
    ) {
        output.push(quote! {
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
        output.push(quote! {
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
    output: &mut Vec<proc_macro2::TokenStream>,
    k_path: &proc_macro2::TokenStream,
    canonical_name: &syn::Ident,
    reset_ident: &syn::Ident,
) {
    let fn_name = syn::Ident::new(&format!("reset_{}", canonical_name), reset_ident.span());
    let reseter_path = quote! { crate::generic::sys_helpers::properties::reset_property };

    output.push(quote! {
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

fn make_property_get_dimensions(
    output: &mut Vec<proc_macro2::TokenStream>,
    k_path: &proc_macro2::TokenStream,
    canonical_name: &syn::Ident,
    get_dimensions_ident: &syn::Ident,
) {
    let fn_name = syn::Ident::new(
        &format!("get_dimensions_{}", canonical_name),
        get_dimensions_ident.span(),
    );
    let get_dimensions_path =
        quote! { crate::generic::sys_helpers::properties::get_property_dimension };

    output.push(quote! {
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
struct MakePropertyAccessorsInput {
    items: syn::punctuated::Punctuated<MakePropertyAccessorsInputItem, syn::Token![;]>,
}

impl syn::parse::Parse for MakePropertyAccessorsInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let items = syn::punctuated::Punctuated::<MakePropertyAccessorsInputItem, syn::Token![;]>::parse_terminated(input)?;

        Ok(MakePropertyAccessorsInput { items })
    }
}

struct MakePropertyAccessorsInputItem {
    canonical_name: syn::Ident,
    ty: MakePropertyAccessorsInputItemContainerType,
    functions: MakePropertyAccessorsInputItemFunctions,
}

impl syn::parse::Parse for MakePropertyAccessorsInputItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let canonical_name: syn::Ident = input.parse()?;
        input.parse::<syn::Token![:]>()?;
        let ty = input.parse::<MakePropertyAccessorsInputItemContainerType>()?;
        let functions = input.parse::<MakePropertyAccessorsInputItemFunctions>()?;
        Ok(MakePropertyAccessorsInputItem {
            canonical_name,
            ty,
            functions,
        })
    }
}

enum MakePropertyAccessorsInputItemContainerType {
    Single(MakePropertyAccessorsInputItemElementPossibleTypes),
    FixedArray(MakePropertyAccessorsInputItemElementPossibleTypes, usize),
    Array(MakePropertyAccessorsInputItemElementPossibleTypes),
}

impl syn::parse::Parse for MakePropertyAccessorsInputItemContainerType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::token::Bracket) {
            let content;
            syn::bracketed!(content in input);
            let ty: MakePropertyAccessorsInputItemElementPossibleTypes = content.parse()?;
            if content.peek(syn::Token![;]) {
                content.parse::<syn::Token![;]>()?;
                let size: syn::LitInt = content.parse()?;
                Ok(MakePropertyAccessorsInputItemContainerType::FixedArray(
                    ty,
                    size.base10_parse::<usize>()?,
                ))
            } else {
                Ok(MakePropertyAccessorsInputItemContainerType::Array(ty))
            }
        } else {
            Ok(MakePropertyAccessorsInputItemContainerType::Single(
                input.parse()?,
            ))
        }
    }
}

impl MakePropertyAccessorsInputItemContainerType {
    fn possible_types(&self) -> &MakePropertyAccessorsInputItemElementPossibleTypes {
        match self {
            MakePropertyAccessorsInputItemContainerType::Single(ty) => ty,
            MakePropertyAccessorsInputItemContainerType::FixedArray(ty, _) => ty,
            MakePropertyAccessorsInputItemContainerType::Array(ty) => ty,
        }
    }

    fn dimension_suffix(&self) -> String {
        match self {
            MakePropertyAccessorsInputItemContainerType::Single(_) => "".to_string(),
            MakePropertyAccessorsInputItemContainerType::FixedArray(_, size) => {
                format!("s_{}", size)
            }
            MakePropertyAccessorsInputItemContainerType::Array(_) => "s".to_string(),
        }
    }
}

struct MakePropertyAccessorsInputItemElementPossibleTypes {
    int: Option<syn::Ident>,
    double: Option<syn::Ident>,
    string: Option<syn::Ident>,
    pointer: Option<syn::Ident>,
}

impl syn::parse::Parse for MakePropertyAccessorsInputItemElementPossibleTypes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);
            let mut int = None;
            let mut double = None;
            let mut string = None;
            let mut pointer = None;

            while !content.is_empty() {
                let ty: syn::Ident = content.parse()?;
                match ty.to_string().as_str() {
                    "Int" => int = Some(ty.clone()),
                    "Double" => double = Some(ty.clone()),
                    "String" => string = Some(ty.clone()),
                    "Pointer" => pointer = Some(ty.clone()),
                    _ => return Err(syn::Error::new(ty.span(), "Unknown type")),
                }
                if content.peek(syn::Token![|]) {
                    content.parse::<syn::Token![|]>()?;
                }
            }

            Ok(MakePropertyAccessorsInputItemElementPossibleTypes {
                int,
                double,
                string,
                pointer,
            })
        } else {
            let ty: syn::Ident = input.parse()?;
            match ty.to_string().as_str() {
                "Int" => Ok(MakePropertyAccessorsInputItemElementPossibleTypes {
                    int: Some(ty.clone()),
                    double: None,
                    string: None,
                    pointer: None,
                }),
                "Double" => Ok(MakePropertyAccessorsInputItemElementPossibleTypes {
                    int: None,
                    double: Some(ty.clone()),
                    string: None,
                    pointer: None,
                }),
                "String" => Ok(MakePropertyAccessorsInputItemElementPossibleTypes {
                    int: None,
                    double: None,
                    string: Some(ty.clone()),
                    pointer: None,
                }),
                "Pointer" => Ok(MakePropertyAccessorsInputItemElementPossibleTypes {
                    int: None,
                    double: None,
                    string: None,
                    pointer: Some(ty.clone()),
                }),
                _ => Err(syn::Error::new(ty.span(), "Unknown type")),
            }
        }
    }
}

impl MakePropertyAccessorsInputItemElementPossibleTypes {
    fn iter(&self) -> MakePropertyAccessorsInputItemElementPossibleTypesIterator {
        let mut types = Vec::new();
        if let Some(ident) = &self.int {
            types.push(OpenFXTypeIdent::Int(ident.clone()));
        }
        if let Some(ident) = &self.double {
            types.push(OpenFXTypeIdent::Double(ident.clone()));
        }
        if let Some(ident) = &self.string {
            types.push(OpenFXTypeIdent::String(ident.clone()));
        }
        if let Some(ident) = &self.pointer {
            types.push(OpenFXTypeIdent::Pointer(ident.clone()));
        }
        MakePropertyAccessorsInputItemElementPossibleTypesIterator(types)
    }
}

struct MakePropertyAccessorsInputItemElementPossibleTypesIterator(Vec<OpenFXTypeIdent>);

impl Iterator for MakePropertyAccessorsInputItemElementPossibleTypesIterator {
    type Item = OpenFXTypeIdent;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.remove(0))
        }
    }
}

struct MakePropertyAccessorsInputItemFunctions {
    set: Option<syn::Ident>,
    get: Option<syn::Ident>,
    reset: Option<syn::Ident>,
    get_dimensions: Option<syn::Ident>,
}

impl syn::parse::Parse for MakePropertyAccessorsInputItemFunctions {
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
                _ => return Err(syn::Error::new(func.span(), "Unknown function")),
            }
            if content.peek(syn::Token![,]) {
                content.parse::<syn::Token![,]>()?;
            }
        }

        Ok(MakePropertyAccessorsInputItemFunctions {
            set,
            get,
            reset,
            get_dimensions,
        })
    }
}

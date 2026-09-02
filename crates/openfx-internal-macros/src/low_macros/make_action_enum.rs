use proc_macro::TokenStream;
use quote::{quote, quote_spanned};

pub fn make_action_enum(tokens: TokenStream) -> TokenStream {
    let mut output_inner = proc_macro2::TokenStream::new();

    let input = syn::parse_macro_input!(tokens as Input);
    let enum_name = &input.name;

    for var in &input.items {
        make_variant(&mut output_inner, var);
    }

    let enum_name_str = enum_name.to_string();

    let from_sys_fn = make_from_sys_fn(&enum_name_str, &input.items);
    let sys_fns = make_sys_fns(&enum_name_str, &input.items);

    quote! {
        #[non_exhaustive]
        pub enum #enum_name {
            #output_inner
            /// Use [`Self::sys_match`] instead. Even if matching this variant
            /// for unrecognized actions works now, it will break when this
            /// crate adds dedicated variants for those actions in a future
            /// release.
            UnknownDontUseThisDirectly {
                action: ::std::option::Option<std::ptr::NonNull<::std::os::raw::c_char>>,
                handle: *const ::std::ffi::c_void,
                in_args: crate::sys::generic::core::OfxPropertySetHandle,
                out_args: crate::sys::generic::core::OfxPropertySetHandle,
            }
        }
        impl #enum_name {
            #from_sys_fn
            #sys_fns

            pub fn sys_match(&self, action: &::std::ffi::CStr) -> Option<(*const ::std::ffi::c_void, crate::sys::generic::core::OfxPropertySetHandle, crate::sys::generic::core::OfxPropertySetHandle)> {
                if let Some(self_action) = self.sys_action() && self_action == action {
                    Some((self.sys_handle(), self.sys_in_args(), self.sys_out_args()))
                } else {
                    None
                }
            }
        }
    }
    .into()
}

fn make_variant(output: &mut proc_macro2::TokenStream, var: &InputVariant) {
    let name = &var.name;
    let handle_field = if let Some(handle_type) = &var.handle_type {
        quote! { handle: #handle_type }
    } else {
        // TODO: use `NonNull<std::ffi::c_void>` instead.
        quote! { sys_handle: *const ::std::ffi::c_void }
    };
    let in_args_field = if let Some(in_args) = &var.in_args {
        let name = syn::Ident::new(&format!("Action{}In", name), in_args.span());
        let path = if let Some(from_core) = &var.from_core {
            let core = quote_spanned! { from_core.span() => core };
            quote! { super::#core:: }
        } else {
            quote! {}
        };
        quote! { #in_args: #path #name }
    } else {
        quote! { sys_in_args: crate::sys::generic::core::OfxPropertySetHandle }
    };
    let out_args_field = if let Some(out_args) = &var.out_args {
        let name = syn::Ident::new(&format!("Action{}Out", name), out_args.span());
        let path = if let Some(from_core) = &var.from_core {
            let core = quote_spanned! { from_core.span() => core };
            quote! { super::#core:: }
        } else {
            quote! {}
        };
        quote! { #out_args: #path #name }
    } else {
        quote! { sys_out_args: crate::sys::generic::core::OfxPropertySetHandle }
    };

    output.extend(quote! {
        #name {
            #handle_field,
            #in_args_field,
            #out_args_field,
        },
    });
}

fn make_from_sys_fn(enum_name: &str, items: &[InputVariant]) -> proc_macro2::TokenStream {
    let mut branches: Vec<proc_macro2::TokenStream> = vec![];

    for var in items {
        let var_name = &var.name;
        let k_name = get_k_name(var, enum_name);
        let k_name = syn::Ident::new(&k_name, var.name.span());
        let (handle_ident, handle_from_sys) = if let Some(handle_type) = &var.handle_type {
            (quote! { handle }, quote! { #handle_type::from_sys(handle) })
        } else {
            (quote! { sys_handle }, quote! { handle })
        };
        let (in_args_ident, in_args_from_sys) = if var.in_args.is_some() {
            (quote! { in_args }, quote! { in_args.into() })
        } else {
            (quote! { sys_in_args }, quote! { in_args })
        };
        let (out_args_ident, out_args_from_sys) = if var.out_args.is_some() {
            (quote! { out_args }, quote! { out_args.into() })
        } else {
            (quote! { sys_out_args }, quote! { out_args })
        };
        branches.push(quote! {
            _ if action == crate::sys_umbrella::#k_name => {
                Self::#var_name {
                    #handle_ident: #handle_from_sys,
                    #in_args_ident: #in_args_from_sys,
                    #out_args_ident: #out_args_from_sys,
                }
            }
        });
    }

    quote! {
        /// ## Safety
        ///
        /// - `action` should be a valid C string representing the action.
        pub unsafe fn from_sys(
            action: *const ::std::os::raw::c_char,
            handle: *const ::std::ffi::c_void,
            in_args: crate::sys::generic::core::OfxPropertySetHandle,
            out_args: crate::sys::generic::core::OfxPropertySetHandle,
        ) -> Self {
            let action = if action.is_null() {
                return Self::UnknownDontUseThisDirectly {
                    action: ::std::option::Option::None,
                    handle,
                    in_args,
                    out_args,
                };
            } else {
                unsafe { ::std::ffi::CStr::from_ptr(action) }
            };

            match true {
                #(#branches)*
                _ => Self::UnknownDontUseThisDirectly {
                    action: ::std::option::Option::Some(unsafe { ::std::ptr::NonNull::new_unchecked(action.as_ptr() as *mut ::std::os::raw::c_char) }),
                    handle,
                    in_args,
                    out_args,
                },
            }
        }
    }
}

fn make_sys_fns(enum_name: &str, items: &[InputVariant]) -> proc_macro2::TokenStream {
    let var_names: Vec<_> = items.iter().map(|v| &v.name).collect();
    let k_names: Vec<_> = items
        .iter()
        .map(|v| syn::Ident::new(&get_k_name(v, enum_name), v.name.span()))
        .collect();

    let mut vars_with_handle = Vec::new();
    let mut vars_with_sys_handle = Vec::new();
    let mut vars_with_in_args = Vec::new();
    let mut vars_with_sys_in_args = Vec::new();
    let mut vars_with_out_args = Vec::new();
    let mut vars_with_sys_out_args = Vec::new();

    for item in items {
        let name = item.name.clone();

        if item.handle_type.is_some() {
            vars_with_handle.push(name.clone());
        } else {
            vars_with_sys_handle.push(name.clone());
        }

        if item.in_args.is_some() {
            vars_with_in_args.push(name.clone());
        } else {
            vars_with_sys_in_args.push(name.clone());
        }

        if item.out_args.is_some() {
            vars_with_out_args.push(name);
        } else {
            vars_with_sys_out_args.push(name);
        }
    }

    quote! {
        pub fn sys_action(&self) -> Option<&::std::ffi::CStr> {
            match self {
                #(Self::#var_names { .. } => Some(crate::sys_umbrella::#k_names),)*
                Self::UnknownDontUseThisDirectly { action, .. } => action.map(|a| unsafe { ::std::ffi::CStr::from_ptr(a.as_ptr()) }),
            }
        }

        pub fn sys_handle(&self) -> *const ::std::ffi::c_void {
            match self {
                #(Self::#vars_with_handle { handle, .. } => handle.sys_handle(),)*
                #(Self::#vars_with_sys_handle { sys_handle, .. } => *sys_handle,)*
                Self::UnknownDontUseThisDirectly { handle, .. } => *handle,
            }
        }

        pub fn sys_in_args(&self) -> crate::sys::generic::core::OfxPropertySetHandle {
            match self {
                #(Self::#vars_with_in_args { in_args, .. } => in_args.sys_handle(),)*
                #(Self::#vars_with_sys_in_args { sys_in_args, .. } => *sys_in_args,)*
                Self::UnknownDontUseThisDirectly { in_args, .. } => *in_args,
            }
        }

        pub fn sys_out_args(&self) -> crate::sys::generic::core::OfxPropertySetHandle {
            match self {
                #(Self::#vars_with_out_args { out_args, .. } => out_args.sys_handle(),)*
                #(Self::#vars_with_sys_out_args { sys_out_args, .. } => *sys_out_args,)*
                Self::UnknownDontUseThisDirectly { out_args, .. } => *out_args,
            }
        }
    }
}

fn get_k_name(var: &InputVariant, enum_name: &str) -> String {
    format!(
        "kOfx{}{}",
        if var.from_core.is_some() {
            "Action"
        } else {
            enum_name
        },
        var.name
    )
}

/// ## Examples
///
/// ```rust,ignore
/// make_action_enum! {
///     pub ImageEffectAction {
///         _/_ core::Load,
///         _/_ core::Unload,
///         _/_ core::Describe,
///         _/_ core::CreateInstance,
///         _/_ core::DestroyInstance,
///         i/_ core::BeginInstanceChanged,
///         i/_ core::EndInstanceChanged,
///         i/_ core::InstanceChanged,
///         _/_ core::PurgeCaches,
///         _/_ core::SyncPrivateData,
///         _/_ core::BeginInstanceEdit,
///         _/_ core::EndInstanceEdit,
///         i/_ BeginSequenceRender,
///         i/_ DescribeInContext,
///         i/_ EndSequenceRender,
///         _/o GetClipPreferences,
///         i/o GetFramesNeeded,
///         i/o GetOutputColourspace,
///         i/o GetRegionOfDefinition,
///         i/_ GetRegionsOfInterest,
///         _/o GetTimeDomain,
///         i/_ IsIdentity,
///         i/_ Render,
///     }
/// }
/// ```
struct Input {
    name: syn::Ident,
    items: Vec<InputVariant>,
}

impl syn::parse::Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let content;
        syn::braced!(content in input);
        let items = content
            .parse_terminated(InputVariant::parse, syn::Token![,])?
            .into_iter()
            .collect();
        Ok(Input { name, items })
    }
}

struct InputVariant {
    in_args: Option<syn::Ident>,
    out_args: Option<syn::Ident>,
    from_core: Option<syn::Ident>,
    name: syn::Ident,
    handle_type: Option<syn::Type>,
}

impl syn::parse::Parse for InputVariant {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let in_args = if input.peek(syn::Token![_]) {
            input.parse::<syn::Token![_]>()?;
            None
        } else {
            let marker: syn::Ident = input.parse()?;
            match marker.to_string().as_str() {
                "i" => Some(syn::Ident::new("in_args", marker.span())),
                _ => return Err(syn::Error::new(marker.span(), "expected `i` or `_`")),
            }
        };

        input.parse::<syn::Token![/]>()?;

        let out_args = if input.peek(syn::Token![_]) {
            input.parse::<syn::Token![_]>()?;
            None
        } else {
            let marker: syn::Ident = input.parse()?;
            match marker.to_string().as_str() {
                "o" => Some(syn::Ident::new("out_args", marker.span())),
                _ => return Err(syn::Error::new(marker.span(), "expected `o` or `_`")),
            }
        };

        let from_core: Option<syn::Ident>;
        let name: syn::Ident;
        if input.peek(syn::Ident) && input.peek2(syn::Token![::]) {
            from_core = Some(input.parse()?);
            input.parse::<syn::Token![::]>()?;
            name = input.parse()?;
        } else {
            from_core = None;
            name = input.parse()?;
        }

        let handle_type = if input.peek(syn::Token![:]) {
            input.parse::<syn::Token![:]>()?;
            Some(input.parse()?)
        } else {
            None
        };

        Ok(InputVariant {
            in_args,
            out_args,
            from_core,
            name,
            handle_type,
        })
    }
}

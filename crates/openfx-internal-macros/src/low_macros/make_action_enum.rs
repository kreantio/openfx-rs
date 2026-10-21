use proc_macro::TokenStream;
use quote::{quote, quote_spanned};

use crate::low_macros::utils;

pub fn make_action_enum(attr: TokenStream, item: TokenStream) -> TokenStream {
    if let Err(error) = utils::err_if_attribute_is_not_empty(attr) {
        return error;
    }

    let mut output_inner = proc_macro2::TokenStream::new();

    let input = syn::parse_macro_input!(item as Input);
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

            /// ## Safety
            /// 
            /// This is safe because action names are, by definition, [label
            /// strings][^1], which have static lifetimes. (TODO: remove the
            /// footnote once the documentation is updated.)
            /// 
            /// [label strings]: https://openfx.readthedocs.io/en/latest/Reference/ofxCoreAPI.html#strings
            /// [^1]: <https://github.com/AcademySoftwareFoundation/openfx/issues/273#issuecomment-6233931086>
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

fn make_variant(output: &mut proc_macro2::TokenStream, var: &InputItem) {
    let name = &var.name;
    let handle_field = if let Some(handle_type) = &var.meta.handle {
        quote! { handle: crate::low_plugin::objects::#handle_type }
    } else {
        // TODO: use `NonNull<std::ffi::c_void>` instead.
        quote! { sys_handle: *const ::std::ffi::c_void }
    };
    let in_args_field = if let Some(in_args) = &var.meta.in_args {
        let name = syn::Ident::new(&format!("Action{}In", name), in_args.span());
        let path = if let Some(core) = &var.meta.core {
            let core = quote_spanned! { core.span() => core };
            quote! { super::#core:: }
        } else {
            quote! {}
        };
        quote! { #in_args: #path #name }
    } else {
        quote! { sys_in_args: crate::sys::generic::core::OfxPropertySetHandle }
    };
    let out_args_field = if let Some(out_args) = &var.meta.out_args {
        let name = syn::Ident::new(&format!("Action{}Out", name), out_args.span());
        let path = if let Some(core) = &var.meta.core {
            let core = quote_spanned! { core.span() => core };
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

fn make_from_sys_fn(enum_name: &str, items: &[InputItem]) -> proc_macro2::TokenStream {
    let mut branches: Vec<proc_macro2::TokenStream> = vec![];

    for var in items {
        let var_name = &var.name;
        let cname = get_cname(var, enum_name);
        let cname = syn::Ident::new(&cname, var.name.span());
        let (handle_ident, handle_from_sys) = if let Some(handle_type) = &var.meta.handle {
            (
                quote! { handle },
                quote! { crate::low_plugin::objects::#handle_type::from_sys_ptr(handle) },
            )
        } else {
            (quote! { sys_handle }, quote! { handle })
        };
        let (in_args_ident, in_args_from_sys) = if var.meta.in_args.is_some() {
            (quote! { in_args }, quote! { in_args.into() })
        } else {
            (quote! { sys_in_args }, quote! { in_args })
        };
        let (out_args_ident, out_args_from_sys) = if var.meta.out_args.is_some() {
            (quote! { out_args }, quote! { out_args.into() })
        } else {
            (quote! { sys_out_args }, quote! { out_args })
        };
        branches.push(quote! {
            _ if action == crate::sys_umbrella::#cname => {
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

fn make_sys_fns(enum_name: &str, items: &[InputItem]) -> proc_macro2::TokenStream {
    let var_names: Vec<_> = items.iter().map(|v| &v.name).collect();
    let cnames: Vec<_> = items
        .iter()
        .map(|v| syn::Ident::new(&get_cname(v, enum_name), v.name.span()))
        .collect();

    let mut vars_with_handle = Vec::new();
    let mut vars_with_sys_handle = Vec::new();
    let mut vars_with_in_args = Vec::new();
    let mut vars_with_sys_in_args = Vec::new();
    let mut vars_with_out_args = Vec::new();
    let mut vars_with_sys_out_args = Vec::new();

    for item in items {
        let name = item.name.clone();

        if item.meta.handle.is_some() {
            vars_with_handle.push(name.clone());
        } else {
            vars_with_sys_handle.push(name.clone());
        }

        if item.meta.in_args.is_some() {
            vars_with_in_args.push(name.clone());
        } else {
            vars_with_sys_in_args.push(name.clone());
        }

        if item.meta.out_args.is_some() {
            vars_with_out_args.push(name);
        } else {
            vars_with_sys_out_args.push(name);
        }
    }

    quote! {
        pub fn sys_action(&self) -> Option<&::std::ffi::CStr> {
            match self {
                #(Self::#var_names { .. } => Some(crate::sys_umbrella::#cnames),)*
                Self::UnknownDontUseThisDirectly { action, .. } => action.map(|a| unsafe { ::std::ffi::CStr::from_ptr(a.as_ptr()) }),
            }
        }

        pub fn sys_handle(&self) -> *const ::std::ffi::c_void {
            match self {
                #(Self::#vars_with_handle { handle, .. } => handle.sys_handle() as *const std::ffi::c_void,)*
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

fn get_cname(var: &InputItem, enum_name: &str) -> String {
    format!(
        "kOfx{}{}",
        if var.meta.core.is_some() {
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
///         // …
///         _/_ core::Describe: crate::low_plugin::objects::ImageEffectDescriptor,
///         // …
///         i/_ EndSequenceRender: crate::low_plugin::objects::ImageEffectInstance,
///         _/o GetClipPreferences: crate::low_plugin::objects::ImageEffectInstance,
///         i/o GetFramesNeeded: crate::low_plugin::objects::ImageEffectInstance,
///         // …
///     }
/// }
/// ```
struct Input {
    name: syn::Ident,
    items: Vec<InputItem>,
}

impl syn::parse::Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![enum]>()?;
        let name = input.parse()?;
        let content;
        syn::braced!(content in input);
        let items = content
            .parse_terminated(InputItem::parse, syn::Token![,])?
            .into_iter()
            .collect();
        Ok(Input { name, items })
    }
}

struct InputItem {
    meta: InputItemMeta,
    name: syn::Ident,
}

impl syn::parse::Parse for InputItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if !utils::peek_attribute(input, "action") {
            return Err(input.error("expected attribute `#[action]`"));
        }

        let meta: InputItemMeta = input.parse()?;
        let name: syn::Ident = input.parse()?;

        Ok(InputItem { meta, name })
    }
}

struct InputItemMeta {
    core: Option<syn::Ident>,
    handle: Option<syn::Type>,
    in_args: Option<syn::Ident>,
    out_args: Option<syn::Ident>,
}

impl syn::parse::Parse for InputItemMeta {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![#]>()?;
        let content;
        syn::bracketed!(content in input);

        let attr_name = content.parse::<syn::Ident>()?;
        if attr_name != "action" {
            return Err(syn::Error::new_spanned(attr_name, "expected `action`"));
        }

        let mut core = None;
        let mut handle = None;
        let mut in_args = None;
        let mut out_args = None;

        let inner;
        syn::parenthesized!(inner in content);

        while !inner.is_empty() {
            let key = inner.parse::<syn::Ident>()?;
            match key.to_string().as_str() {
                "core" => {
                    if core.replace(key.clone()).is_some() {
                        return Err(syn::Error::new_spanned(key, "duplicate `core`"));
                    }
                }
                "handle" => {
                    inner.parse::<syn::Token![=]>()?;
                    if handle.replace(inner.parse()?).is_some() {
                        return Err(syn::Error::new_spanned(key, "duplicate `handle`"));
                    }
                }
                "in_args" => {
                    if in_args.replace(key.clone()).is_some() {
                        return Err(syn::Error::new_spanned(key, "duplicate `in_args`"));
                    }
                }
                "out_args" => {
                    if out_args.replace(key.clone()).is_some() {
                        return Err(syn::Error::new_spanned(key, "duplicate `out_args`"));
                    }
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        key,
                        "expected `core`, `handle`, `in_args` or `out_args`",
                    ));
                }
            }
            if !inner.is_empty() {
                inner.parse::<syn::Token![,]>()?;
            }
        }

        Ok(InputItemMeta {
            core,
            in_args,
            out_args,
            handle,
        })
    }
}

use std::{collections::BTreeMap, ffi::CString, path::Path};

use convert_case::Casing;
use quote::quote;

use openfx_datagen::metadata_extracting::{
    PropdefDimension, PropdefType, PropdefTypeSimple, StringEnumVariant, WriteSide,
};

use crate::{config::CodegenConfig, input_metadata::InputMetadata, utils::strip_common_prefix};

pub fn gen_low_enums_from_metadata(
    output_file: &Path,
    metadata: &InputMetadata,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut output_inner = proc_macro2::TokenStream::new();

    let mut enums: Vec<(String, &Vec<StringEnumVariant>)> = vec![];
    for (cname, entry) in &metadata.raw.propdef_map {
        let PropdefType::StringEnum { one_of } = &entry.r#type else {
            continue;
        };

        enums.push((cname.clone(), one_of));
    }
    enums.sort_by(|a, b| a.0.cmp(&b.0));

    for (cname, vars) in enums {
        let simple_enum_name = &cname
            .strip_prefix("kOfx")
            .ok_or("Enum property name should start with \"kOfx\".")?;

        let simple_enum_name = syn::Ident::new(simple_enum_name, proc_macro2::Span::call_site());

        let mut defined_vars: Vec<String> = vec![];
        let mut literal_vars: Vec<String> = vec![];

        for var in vars {
            match var {
                StringEnumVariant::Defined { cname } => defined_vars.push(cname.clone()),
                StringEnumVariant::Literal { value } => literal_vars.push(value.clone()),
            }
        }

        defined_vars.sort();
        literal_vars.sort();

        let simple_defined_names = strip_common_prefix(&defined_vars);
        let pascal_literal_names: Vec<_> = literal_vars
            .iter()
            .map(|n| n.to_case(convert_case::Case::Pascal))
            .collect();

        let defined_vars: Vec<_> = defined_vars
            .iter()
            .map(|v| syn::Ident::new(v, proc_macro2::Span::call_site()))
            .collect();
        let literal_vars: Vec<_> = literal_vars
            .iter()
            .map(|v| CString::new(v.to_owned()).unwrap())
            .collect();
        let simple_defined_names: Vec<_> = simple_defined_names
            .iter()
            .map(|v| syn::Ident::new(v, proc_macro2::Span::call_site()))
            .collect();
        let pascal_literal_names: Vec<_> = pascal_literal_names
            .iter()
            .map(|v| syn::Ident::new(v, proc_macro2::Span::call_site()))
            .collect();

        output_inner.extend(quote! {
            enum #simple_enum_name {
                #(
                    #[sys(#defined_vars)]
                    #simple_defined_names,
                )*
                #(
                    #[sys_literal(#literal_vars)]
                    #pascal_literal_names,
                )*
            }
        });
    }

    std::fs::write(
        output_file,
        prettyplease::unparse(&syn::parse2(quote! {
            #[openfx_internal_macros::low_make_property_enums]
            mod __ {
                #output_inner
            }
        })?),
    )?;

    Ok(())
}

pub fn gen_low_plugin_actions(
    config: &CodegenConfig,
    output_file: &Path,
    metadata: &InputMetadata,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut output_inner_pseudo = proc_macro2::TokenStream::new();
    let mut output_inner_core = proc_macro2::TokenStream::new();
    let mut output_inners: BTreeMap<String, proc_macro2::TokenStream> = BTreeMap::new();

    for (name, action) in &metadata.raw.actiondef_map {
        let (output_inner, action_simple_name) = if let Some(simple_name) =
            config.actions_v2.pseudo.members.get(name)
        {
            (&mut output_inner_pseudo, simple_name.to_owned())
        } else if let Some(simple_name) = name.strip_prefix(&config.actions_v2.core.members_prefix)
        {
            (&mut output_inner_core, format!("Action{simple_name}"))
        } else {
            'x: {
                for (concrete_name, concrete) in &config.actions_v2.concretes {
                    if let Some(simple_name) = name.strip_prefix(&concrete.members_prefix) {
                        break 'x (
                            output_inners.entry(concrete_name.to_owned()).or_default(),
                            format!("Action{simple_name}"),
                        );
                    }
                }

                return Err(format!("Action missing in configuration: {name}").into());
            }
        };

        let in_arg_set_name = format!("{action_simple_name}In");
        let in_arg_set_ident = syn::Ident::new(&in_arg_set_name, proc_macro2::Span::call_site());
        let out_arg_set_name = format!("{action_simple_name}Out");
        let out_arg_set_ident = syn::Ident::new(&out_arg_set_name, proc_macro2::Span::call_site());

        let mut in_arg_items: BTreeMap<&str, proc_macro2::TokenStream> = BTreeMap::new();
        for in_arg_cname in &action.in_args {
            add_property_set_item(
                &mut in_arg_items,
                metadata,
                in_arg_cname,
                None,
                &in_arg_set_name,
            )?;
        }

        let mut out_arg_items: BTreeMap<&str, proc_macro2::TokenStream> = BTreeMap::new();
        for out_arg_cname in &action.out_args {
            add_property_set_item(
                &mut out_arg_items,
                metadata,
                out_arg_cname,
                None,
                &out_arg_set_name,
            )?;
        }

        if !in_arg_items.is_empty() || !out_arg_items.is_empty() {
            let mut output_inner_inner = proc_macro2::TokenStream::new();

            if !in_arg_items.is_empty() {
                let items: Vec<_> = in_arg_items.values().cloned().collect();
                output_inner_inner.extend(quote! {
                    #[propset(write = host)]
                    struct #in_arg_set_ident { #(#items)* }
                });
            }
            if !out_arg_items.is_empty() {
                let items: Vec<_> = out_arg_items.values().cloned().collect();
                output_inner_inner.extend(quote! {
                    #[propset(write = plugin)]
                    struct #out_arg_set_ident { #(#items)* }
                });
            }

            output_inner.extend(quote! {
                #[openfx_internal_macros::low_make_property_set_structs_v2]
                mod __ { #output_inner_inner }
            });
        }
    }

    let mut output = quote! {
        pub mod pseudo {
            #output_inner_pseudo
        }
        pub mod core {
            #output_inner_core
        }
    };

    for (mod_name, output_inner) in &output_inners {
        let mod_name_ident = syn::Ident::new(mod_name, proc_macro2::Span::call_site());
        output.extend(quote! {
            pub mod #mod_name_ident {
                #output_inner
            }
        });
    }

    std::fs::write(output_file, prettyplease::unparse(&syn::parse2(output)?))?;

    Ok(())
}

pub fn gen_low_plugin_property_sets(
    config: &CodegenConfig,
    output_file: &Path,
    metadata: &InputMetadata,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut output_inner = proc_macro2::TokenStream::new();

    for (set_name, set) in &metadata.raw.propset_map {
        let set_name = config
            .property_sets
            .renaming
            .get(set_name)
            .unwrap_or(set_name);

        let mut items: BTreeMap<&str, proc_macro2::TokenStream> = BTreeMap::new();

        for (prop_cname, prop) in &set.props {
            let write = prop.write.as_ref();
            add_property_set_item(&mut items, metadata, prop_cname, write, set_name)?;
        }

        let mut meta_items: Vec<proc_macro2::TokenStream> = vec![];

        meta_items.push(match set.write {
            WriteSide::Host => quote! { write = host },
            WriteSide::Plugin => quote! { write = plugin },
        });

        if !set.props_refs.is_empty() {
            let refs = set
                .props_refs
                .iter()
                .map(|str| syn::Ident::new(str, proc_macro2::Span::call_site()));
            meta_items.push(quote! { refs = #(#refs)+* });
        }

        let parenthesized_meta_items = if meta_items.is_empty() {
            quote! {}
        } else {
            quote! { (#(#meta_items),*) }
        };

        let struct_name_ident = syn::Ident::new(
            &format!("{set_name}PropertySet"),
            proc_macro2::Span::call_site(),
        );

        let items: Vec<_> = items.values().cloned().collect();

        output_inner.extend(quote! {
            #[propset #parenthesized_meta_items]
            struct #struct_name_ident { #(#items)* }
        });
    }

    {
        let mut output_def_inner = proc_macro2::TokenStream::new();

        for (def_name, def) in &metadata.raw.propsetdef_map {
            let mut items: BTreeMap<&str, proc_macro2::TokenStream> = BTreeMap::new();

            for (prop_cname, prop) in &def.props {
                let write = prop.write.as_ref();
                add_property_set_item(&mut items, metadata, prop_cname, write, def_name)?;
            }

            let def_name_ident = syn::Ident::new(def_name, proc_macro2::Span::call_site());

            let items: Vec<_> = items.values().cloned().collect();

            output_def_inner.extend(quote! {
                #[propsetdef]
                struct #def_name_ident { #(#items)* }
            });
        }

        output_inner.extend(quote! {
            #[propsetdefs]
            mod __ { #output_def_inner }
        });
    }

    std::fs::write(
        output_file,
        prettyplease::unparse(&syn::parse2(quote! {
            #[openfx_internal_macros::low_make_property_set_structs_v2]
            mod __ {
                #output_inner
            }
        })?),
    )?;

    Ok(())
}

fn add_property_set_item<'a>(
    items: &mut BTreeMap<&'a str, proc_macro2::TokenStream>,
    metadata: &InputMetadata,
    prop_cname: &'a str,
    write: Option<&WriteSide>,
    set_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let Some(prop_canonical_name) = prop_cname.strip_prefix("k") else {
        return Err(format!("Irregular property cname: {prop_cname}").into());
    };
    let Some(simple_name) = prop_cname.strip_prefix("kOfx") else {
        return Err(format!("Irregular property cname: {prop_cname}").into());
    };
    let super_simple_name = get_property_super_simple_name_from_simple_name(simple_name);
    let prop_snake_name = super_simple_name.to_case(convert_case::Case::Snake);

    let Some(def) = metadata.raw.propdef_map.get(prop_cname) else {
        return Err(format!("Missing propdef: {prop_cname}").into());
    };

    let ty = match &def.r#type {
        PropdefType::Simple { one_of } => {
            let mut tys: Vec<proc_macro2::TokenStream> = vec![];
            for ty in one_of {
                let ty_tokens = match ty {
                    PropdefTypeSimple::Int => quote! { Int },
                    PropdefTypeSimple::Double => quote! { Double },
                    PropdefTypeSimple::Bool => quote! { Bool },
                    PropdefTypeSimple::String => quote! { String },
                    PropdefTypeSimple::Pointer => quote! { Pointer },
                };
                tys.push(ty_tokens);
            }

            if one_of.len() == 1 {
                tys.pop().unwrap()
            } else {
                quote! { ty_one_of!(#(#tys)|*) }
            }
        }
        PropdefType::StringEnum { .. } => {
            let simple_name_ident = syn::Ident::new(simple_name, proc_macro2::Span::call_site());
            quote! { ty_enum!(#simple_name_ident) }
        }
    };

    let ty = match &def.dimension {
        PropdefDimension::Fixed { size } if *size == 1 => ty,
        PropdefDimension::Fixed { size } => {
            let size = syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
            quote! { [#ty; # size] }
        }
        PropdefDimension::Dynamic => {
            quote! { [#ty] }
        }
    };

    let mut meta_items: Vec<proc_macro2::TokenStream> = vec![];

    if let Some(write) = write {
        meta_items.push(match write {
            WriteSide::Host => quote! { write = host },
            WriteSide::Plugin => quote! { write = plugin },
        });
    }

    let prop_canonical_name_ident =
        syn::Ident::new(prop_canonical_name, proc_macro2::Span::call_site());
    let prop_snake_name_ident = match prop_snake_name.as_str() {
        "type" => syn::Ident::new_raw("type", proc_macro2::Span::call_site()),
        prop_snake_name => syn::Ident::new(prop_snake_name, proc_macro2::Span::call_site()),
    };

    meta_items.push(quote! { @#prop_canonical_name_ident });

    let parenthesized_meta_items = if meta_items.is_empty() {
        quote! {}
    } else {
        quote! { (#(#meta_items),*) }
    };

    let item = quote! {
        #[prop #parenthesized_meta_items]
        #prop_snake_name_ident: #ty,
    };
    match items.entry(prop_cname) {
        std::collections::btree_map::Entry::Occupied(occupied_entry) => {
            assert_eq!(occupied_entry.get().to_string(), item.to_string());
            tracing::warn!("Duplicate entry `{prop_cname}` on `{set_name}`.");
        }
        std::collections::btree_map::Entry::Vacant(vacant_entry) => {
            vacant_entry.insert(item);
        }
    };

    Ok(())
}

fn get_property_super_simple_name_from_simple_name(simple_name: &str) -> String {
    static RE: std::sync::LazyLock<regex::Regex> =
        std::sync::LazyLock::new(|| regex::Regex::new(r"Prop([A-Z])").unwrap());

    if !RE.is_match(simple_name) {
        tracing::warn!("Property simple name without the `Prop` part: {simple_name}");
        return simple_name.to_string();
    }

    RE.replace(simple_name, "$1").to_string()
}

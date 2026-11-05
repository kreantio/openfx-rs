use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    ffi::CString,
    path::Path,
};

use convert_case::Casing;
use quote::quote;

use openfx_codegen_config::CodegenConfig;
use openfx_datagen::metadata_extracting::{ActiondefMetadataEntry, PropdefType, StringEnumVariant};

use crate::{
    ex_codegen_metadata::common::{
        add_property_set_item, process_property_set, process_property_set_definitions,
    },
    input_metadata::InputMetadata,
    utils::{new_ident, strip_common_prefix},
};

mod common;

mod gen_low_plugin_parameters;

pub use gen_low_plugin_parameters::gen_low_plugin_parameters;

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

        let simple_enum_name = new_ident(simple_enum_name);

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

        let defined_vars: Vec<_> = defined_vars.iter().map(new_ident).collect();
        let literal_vars: Vec<_> = literal_vars
            .iter()
            .map(|v| CString::new(v.to_owned()).unwrap())
            .collect();
        let simple_defined_names: Vec<_> = simple_defined_names.iter().map(new_ident).collect();
        let pascal_literal_names: Vec<_> = pascal_literal_names.iter().map(new_ident).collect();

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
        quote! {
            #[openfx_internal_macros::low_make_property_enums]
            mod __ {
                #output_inner
            }
        }
        .to_string(),
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
    let mut output_action_inners: BTreeMap<String, proc_macro2::TokenStream> = BTreeMap::new();

    let mut remain_actions: HashSet<&str> = HashSet::new();

    for (name, action) in &metadata.raw.actiondef_map {
        if let Some(action_simple_name) = config.actions.pseudo.members.get(name) {
            let action_simple_name_ident = new_ident(action_simple_name);

            let in_arg_set_name = format!("{action_simple_name}In");
            let out_arg_set_name = format!("{action_simple_name}Out");

            let ProcessActionPropertySetsOutput {
                property_set_structs,
                has_in_args,
                in_arg_set_ident,
                has_out_args,
                out_arg_set_ident,
            } = process_action_property_sets(
                action,
                metadata,
                &in_arg_set_name,
                &out_arg_set_name,
            )?;

            let mut params: Vec<proc_macro2::TokenStream> =
                vec![quote! { *const std::ffi::c_void }];
            if has_in_args {
                params.push(quote! { in_args: #in_arg_set_ident });
            }
            if has_out_args {
                params.push(quote! { out_args: #out_arg_set_ident });
            }

            output_inner_pseudo.extend(quote! {
                pub type #action_simple_name_ident = fn(#(#params),*) -> crate::low::OutboundResult;
            });
            output_inner_pseudo.extend(property_set_structs);
        } else if let Some(action_simple_name) =
            name.strip_prefix(&config.actions.core.members_prefix)
        {
            let Some(_member) = config.actions.core.members.get(action_simple_name) else {
                return Err(format!("Core action member missing in configuration: {name}").into());
            };

            let in_arg_set_name = format!("Action{action_simple_name}In");
            let out_arg_set_name = format!("Action{action_simple_name}Out");

            let ProcessActionPropertySetsOutput {
                property_set_structs,
                ..
            } = process_action_property_sets(
                action,
                metadata,
                &in_arg_set_name,
                &out_arg_set_name,
            )?;

            output_inner_core.extend(property_set_structs);
        } else {
            remain_actions.insert(name);
        };
    }

    let concretes: BTreeMap<_, _> = config.actions.concretes.iter().collect();
    for (concrete_name, concrete) in concretes {
        let output_inner = output_inners.entry(concrete_name.to_owned()).or_default();
        let output_action_inner = output_action_inners
            .entry(concrete_name.to_owned())
            .or_default();

        let members_from_core: BTreeSet<_> = concrete.members_from_core.iter().collect();
        for simple_name in members_from_core {
            let full_name = format!("{}{}", config.actions.core.members_prefix, simple_name);
            let Some(member) = config.actions.core.members.get(simple_name) else {
                return Err(format!(
                    "Core member specified in {} not found in metadata: {}",
                    concrete_name, full_name
                )
                .into());
            };
            let Some(actiondef) = metadata.raw.actiondef_map.get(&full_name) else {
                unreachable!()
            };

            let output_action_enum_item = process_action_enum_item(
                simple_name,
                &concrete.handle_prefix,
                member.handle.as_deref(),
                !actiondef.in_args.is_empty(),
                !actiondef.out_args.is_empty(),
                &[quote! { core }],
            )?;

            output_action_inner.extend(output_action_enum_item);
        }

        let members: BTreeMap<_, _> = concrete.members.iter().collect();
        for (simple_name, member) in members {
            let full_name = format!("{}{}", concrete.members_prefix, simple_name);
            let Some(actiondef) = metadata.raw.actiondef_map.get(&full_name) else {
                return Err(format!("Action not found in metadata: {full_name}").into());
            };
            remain_actions.remove(full_name.as_str());

            let in_arg_set_name = format!("Action{simple_name}In");
            let out_arg_set_name = format!("Action{simple_name}Out");

            let ProcessActionPropertySetsOutput {
                property_set_structs,
                has_in_args,
                has_out_args,
                ..
            } = process_action_property_sets(
                actiondef,
                metadata,
                &in_arg_set_name,
                &out_arg_set_name,
            )?;

            output_inner.extend(property_set_structs);

            let output_action_enum_item = process_action_enum_item(
                simple_name,
                &concrete.handle_prefix,
                member.handle.as_deref(),
                has_in_args,
                has_out_args,
                &[],
            )?;

            output_action_inner.extend(output_action_enum_item);
        }
    }

    if !remain_actions.is_empty() {
        return Err(format!(
            "Some actions are missing in the configuration: {:?}",
            remain_actions
        )
        .into());
    }

    let mut output = quote! {
        pub mod pseudo {
            #output_inner_pseudo
        }
        pub mod core {
            #output_inner_core
        }
    };

    assert!(output_inners.keys().eq(output_action_inners.keys()));

    for (mod_name, output_action_inner) in &output_action_inners {
        let output_inner = output_inners.get(mod_name);

        let action_enum_name = format!("{}Action", mod_name.to_case(convert_case::Case::Pascal));

        let mod_name_ident = new_ident(mod_name);
        let action_enum_name_ident = new_ident(&action_enum_name);
        output.extend(quote! {
            pub mod #mod_name_ident {
                #[openfx_internal_macros::low_make_action_enum]
                enum #action_enum_name_ident { #output_action_inner }

                #output_inner
            }
        });
    }

    std::fs::write(output_file, output.to_string())?;

    Ok(())
}

struct ProcessActionPropertySetsOutput {
    property_set_structs: proc_macro2::TokenStream,
    has_in_args: bool,
    in_arg_set_ident: syn::Ident,
    has_out_args: bool,
    out_arg_set_ident: syn::Ident,
}

fn process_action_property_sets(
    action: &ActiondefMetadataEntry,
    metadata: &InputMetadata,
    in_arg_set_name: &str,
    out_arg_set_name: &str,
) -> Result<ProcessActionPropertySetsOutput, Box<dyn std::error::Error>> {
    let in_arg_set_ident = new_ident(in_arg_set_name);
    let out_arg_set_ident = new_ident(out_arg_set_name);

    let mut in_arg_items: BTreeMap<&str, proc_macro2::TokenStream> = BTreeMap::new();
    for (in_arg_cname, prop) in &action.in_args {
        let write = prop.write.as_ref();
        add_property_set_item(
            &mut in_arg_items,
            metadata,
            in_arg_cname,
            write,
            in_arg_set_name,
        )?;
    }

    let mut out_arg_items: BTreeMap<&str, proc_macro2::TokenStream> = BTreeMap::new();
    for (out_arg_cname, prop) in &action.out_args {
        let write = prop.write.as_ref();
        add_property_set_item(
            &mut out_arg_items,
            metadata,
            out_arg_cname,
            write,
            out_arg_set_name,
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

        Ok(ProcessActionPropertySetsOutput {
            property_set_structs: quote! {
                #[openfx_internal_macros::low_make_property_set_structs]
                mod __ { #output_inner_inner }
            },
            has_in_args: !in_arg_items.is_empty(),
            in_arg_set_ident,
            has_out_args: !out_arg_items.is_empty(),
            out_arg_set_ident,
        })
    } else {
        Ok(ProcessActionPropertySetsOutput {
            property_set_structs: Default::default(),
            has_in_args: false,
            in_arg_set_ident,
            has_out_args: false,
            out_arg_set_ident,
        })
    }
}

fn process_action_enum_item(
    action_simple_name: &str,
    handle_prefix: &str,
    handle_name: Option<&str>,
    has_in_args: bool,
    has_out_args: bool,
    extra_meta_args: &[proc_macro2::TokenStream],
) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let action_name_ident = new_ident(action_simple_name);

    let mut meta_args: Vec<proc_macro2::TokenStream> = extra_meta_args.to_vec();

    if let Some(handle_name) = handle_name {
        let handle_name_ident = new_ident(format!("{handle_prefix}{handle_name}"));
        meta_args.push(quote! { handle = #handle_name_ident });
    }
    if has_in_args {
        meta_args.push(quote! { in_args });
    }
    if has_out_args {
        meta_args.push(quote! { out_args });
    }

    Ok(quote! {
        #[action(#(#meta_args),*)]
        #action_name_ident,
    })
}

pub fn gen_low_plugin_property_sets(
    config: &CodegenConfig,
    output_file: &Path,
    metadata: &InputMetadata,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut output_inner = proc_macro2::TokenStream::new();

    let mut used_refs: HashSet<String> = HashSet::new();

    for (set_name, set) in &metadata.raw.propset_map {
        if config.property_sets.ignore.matches(set_name) {
            continue;
        }

        let set_name = config
            .property_sets
            .renaming
            .get(set_name)
            .unwrap_or(set_name);

        used_refs.extend(set.props_refs.iter().cloned());
        let refs = set.props_refs.iter().map(new_ident).collect::<Vec<_>>();

        output_inner.extend(process_property_set(metadata, set_name, set, &refs, &[])?);
    }

    if let Some(output_def) = process_property_set_definitions(metadata, &used_refs)? {
        output_inner.extend(output_def);
    }

    std::fs::write(
        output_file,
        quote! {
            #[openfx_internal_macros::low_make_property_set_structs]
            mod __ {
                #output_inner
            }
        }
        .to_string(),
    )?;

    Ok(())
}

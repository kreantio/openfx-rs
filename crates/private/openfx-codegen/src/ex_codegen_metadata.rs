use std::{ffi::CString, path::Path};

use convert_case::Casing;
use quote::quote;

use openfx_datagen::metadata_extracting::{
    PropdefType, PropdefTypeSimple, StringEnumVariant, WriteSide,
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

    let output_inner = prettyplease::unparse(&syn::parse2(output_inner)?)
        .lines()
        .map(|l| format!("    {l}"))
        .collect::<Vec<_>>()
        .join("\n");

    std::fs::write(
        output_file,
        format!(
            r#"openfx_internal_macros::low_make_property_enums! {{
{output_inner}
}}"#
        ),
    )?;

    Ok(())
}

pub fn gen_low_plugin_property_sets(
    config: &CodegenConfig,
    output_file: &Path,
    metadata: &InputMetadata,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut output_inner = proc_macro2::TokenStream::new();

    for (set_name, set) in &metadata.raw.propset_map {
        let mut output_set_inner = proc_macro2::TokenStream::new();

        fn add_item(
            output: &mut proc_macro2::TokenStream,
            metadata: &InputMetadata,
            prop_cname: &str,
            write: &WriteSide,
        ) -> Result<(), Box<dyn std::error::Error>> {
            let Some(prop_canonical_name) = prop_cname.strip_prefix("k") else {
                return Err(format!("Irregular property cname: {prop_cname}").into());
            };
            let Some(simple_name) = prop_cname.strip_prefix("kOfx") else {
                return Err(format!("Irregular property cname: {prop_cname}").into());
            };
            let prop_snake_name = simple_name.to_case(convert_case::Case::Snake);

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

                    let ty = if one_of.len() == 1 {
                        tys.pop().unwrap()
                    } else {
                        quote! { (#(#tys)|*) }
                    };

                    let ty = match &def.dimension {
                        openfx_datagen::metadata_extracting::PropdefDimension::Fixed { size }
                            if *size == 1 =>
                        {
                            ty
                        }
                        openfx_datagen::metadata_extracting::PropdefDimension::Fixed { size } => {
                            let size =
                                syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
                            quote! { [#ty; # size] }
                        }
                        openfx_datagen::metadata_extracting::PropdefDimension::Dynamic => {
                            quote! { [#ty] }
                        }
                    };

                    if one_of.len() == 1 {
                        ty
                    } else {
                        quote! { ty_one_of!(#ty) }
                    }
                }
                PropdefType::StringEnum { .. } => {
                    let simple_name_ident =
                        syn::Ident::new(simple_name, proc_macro2::Span::call_site());
                    quote! { ty_enum!(#simple_name_ident) }
                }
            };

            let meta_write = match write {
                openfx_datagen::metadata_extracting::WriteSide::Host => quote! { write = host },
                openfx_datagen::metadata_extracting::WriteSide::Plugin => quote! { write = plugin },
            };

            let prop_canonical_name_ident =
                syn::Ident::new(prop_canonical_name, proc_macro2::Span::call_site());
            let prop_snake_name_ident =
                syn::Ident::new(&prop_snake_name, proc_macro2::Span::call_site());

            output.extend(quote! {
                #[meta(#meta_write, @#prop_canonical_name_ident)]
                #prop_snake_name_ident: #ty,
            });

            Ok(())
        }

        for (prop_cname, prop) in &set.props {
            let write = prop.write.as_ref().unwrap_or(&set.write);

            add_item(&mut output_set_inner, metadata, prop_cname, write)?;
        }

        for ref_name in &set.props_refs {
            let Some(propsetdef) = metadata.raw.propsetdef_map.get(ref_name) else {
                return Err(format!("Missing propsetdef: {ref_name}").into());
            };

            for (prop_cname, prop) in &propsetdef.props {
                let write = prop.write.as_ref().unwrap_or(&set.write);

                add_item(&mut output_set_inner, metadata, prop_cname, write)?;
            }
        }

        let set_name = config
            .property_sets
            .renaming
            .get(set_name)
            .unwrap_or(set_name);
        let struct_name_ident = syn::Ident::new(
            &format!("{set_name}PropertySet"),
            proc_macro2::Span::call_site(),
        );

        output_inner.extend(quote! {
            struct #struct_name_ident { #output_set_inner }
        });
    }

    let output_inner = prettyplease::unparse(&syn::parse2(output_inner)?)
        .lines()
        .map(|l| format!("    {l}"))
        .collect::<Vec<_>>()
        .join("\n");

    std::fs::write(
        output_file,
        format!(
            r#"openfx_internal_macros::low_make_property_set_structs_v2! {{
{output_inner}
}}"#
        ),
    )?;

    Ok(())
}

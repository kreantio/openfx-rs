use std::{collections::HashMap, ffi::CString, path::Path};

use convert_case::Casing;
use quote::quote;

use openfx_datagen::metadata_extracting::{
    PropdefDimension, PropdefType, PropdefTypeSimple, StringEnumVariant,
};

use crate::{
    input_bindings_data::InputBindingsData, input_metadata::InputMetadata,
    utils::strip_common_prefix,
};

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

pub fn gen_sys_helpers_property_accessors(
    output_folder: &Path,
    metadata: &InputMetadata,
    bindings: &InputBindingsData,
) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(output_folder)?;

    // keys: C header file names with extensions.
    let mut output_inners: HashMap<String, proc_macro2::TokenStream> = HashMap::new();

    for (cname, entry) in &metadata.raw.propdef_map {
        let Some(file_name) = bindings.find_item_origin_file_name(cname) else {
            return Err(format!("Failed to find corresponding file name: {cname}").into());
        };

        let Some(name) = cname.strip_prefix("k") else {
            return Err(format!("Irregular cname: {cname}").into());
        };
        let name = syn::Ident::new(name, proc_macro2::Span::call_site());

        let ty = match &entry.r#type {
            PropdefType::Simple { one_of } => {
                let mut tys: Vec<proc_macro2::TokenStream> = vec![];
                for ty in one_of {
                    let ty = match ty {
                        PropdefTypeSimple::Int | PropdefTypeSimple::Bool => quote! { Int },
                        PropdefTypeSimple::Double => quote! { Double },
                        PropdefTypeSimple::String => quote! { String },
                        PropdefTypeSimple::Pointer => quote! { Pointer },
                    };
                    tys.push(quote! { #ty });
                }
                if tys.len() == 1 {
                    tys.pop().unwrap()
                } else {
                    quote! { (#(#tys)|*) }
                }
            }
            PropdefType::StringEnum { .. } => quote! { String },
        };
        let container_ty = match entry.dimension {
            PropdefDimension::Fixed { size: 1 } => ty,
            PropdefDimension::Fixed { size } => {
                let size = syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
                quote! { [#ty; #size] }
            }
            PropdefDimension::Dynamic => quote! { [#ty] },
        };
        let ops = match entry.dimension {
            PropdefDimension::Fixed { .. } => quote! { set get reset },
            PropdefDimension::Dynamic => quote! { set get reset get_dimensions },
        };

        let output_inner = output_inners.entry(file_name.to_owned()).or_default();

        output_inner.extend(quote! {
            #name!(#container_ty: #ops);
        });
    }

    for (file_name, output_inner) in output_inners {
        let mod_name = file_name
            .strip_suffix(".h")
            .unwrap()
            .strip_prefix("ofx")
            .unwrap()
            .to_case(convert_case::Case::Snake);

        let output_file = output_folder.join(format!("{mod_name}.rs"));

        let output_inner = prettyplease::unparse(&syn::parse2(output_inner)?)
            .lines()
            .map(|l| format!("    {l}"))
            .collect::<Vec<_>>()
            .join("\n");

        std::fs::write(
            output_file,
            format!(
                r#"openfx_internal_macros::sys_helpers_make_property_accessors! {{
{output_inner}
}}"#
            ),
        )?;
    }

    Ok(())
}

use std::{ffi::CString, path::Path};

use convert_case::Casing;
use quote::quote;

use openfx_datagen::metadata_extracting::{PropdefType, StringEnumVariant};

use crate::{input_metadata::InputMetadata, utils::strip_common_prefix};

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
        .map(|l| format!("    {}", l))
        .collect::<Vec<_>>()
        .join("\n");

    std::fs::write(
        output_file,
        format!(
            "openfx_internal_macros::low_make_property_enums! {{
{output_inner}
}}"
        ),
    )?;

    Ok(())
}

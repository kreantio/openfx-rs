use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use quote::quote;

use openfx_codegen_config::CodegenConfig;

use crate::utils::{new_ident, strip_common_prefix, to_snake_case};

mod gen_low_plugin_suites;
mod gen_low_statuses;

pub use gen_low_plugin_suites::gen_low_plugin_suites;
pub use gen_low_statuses::gen_low_statuses;

pub fn gen_low_enums_from_c(
    output_file: &Path,
    c_enums: HashMap<String, HashSet<String>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut output_inner = proc_macro2::TokenStream::new();

    let mut c_enums = c_enums.into_iter().collect::<Vec<_>>();
    c_enums.sort_by(|a, b| a.0.cmp(&b.0));

    for (enum_name, vars) in c_enums {
        let simple_enum_name = enum_name
            .strip_prefix("Ofx")
            .ok_or("Enum name should start with \"Ofx\".")?;

        let enum_name = new_ident(&enum_name);
        let simple_enum_name = new_ident(simple_enum_name);

        let mut vars = vars.into_iter().collect::<Vec<_>>();
        vars.sort();

        let simple_vars = strip_common_prefix(&vars);

        let vars: Vec<syn::Ident> = vars.iter().map(new_ident).collect();
        let simple_vars: Vec<syn::Ident> = simple_vars.iter().map(new_ident).collect();

        output_inner.extend(quote! {
            #[sys(#enum_name)]
            enum #simple_enum_name {
                #(
                    #[sys(#vars)]
                    #simple_vars,
                )*
            }
        });
    }

    std::fs::write(
        output_file,
        quote! {
            #[openfx_internal_macros::low_make_property_enums_from_c]
            mod __ {
                #output_inner
            }
        }
        .to_string(),
    )?;

    Ok(())
}

pub fn gen_low_plugin_objects(
    config: &CodegenConfig,
    output_folder: &Path,
    direct_handle_usages_in_suite_functions: HashMap<String, HashSet<(String, String)>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut mapping = config.objects.mapping.iter().collect::<Vec<_>>();
    mapping.sort_by(|a, b| a.0.cmp(b.0));

    // objects.rs
    {
        let mut output = quote! {
            use openfx_internal_macros::low_make_object_struct;
        };

        for (name, entry) in &mapping {
            if entry.omit {
                continue;
            }
            let object_ident = new_ident(name);
            let handle_ident = new_ident(&entry.is);
            output.extend(quote! {
                low_make_object_struct!(#object_ident,#handle_ident);
            });
        }

        std::fs::write(output_folder.join("objects.rs"), output.to_string())?;
    }

    // checks/objects.rs
    {
        let out_folder = output_folder.join("checks");
        std::fs::create_dir_all(&out_folder)?;
        let out_path = out_folder.join("objects.rs");

        let mut output = proc_macro2::TokenStream::new();

        for (name, entry) in &mapping {
            if entry.omit {
                continue;
            }
            let fn_set = direct_handle_usages_in_suite_functions
                .get(&entry.is)
                .cloned()
                .unwrap_or_default();
            let mut fns: HashSet<String> = HashSet::new();
            for (_suite_name, fn_name) in &fn_set {
                if fns.contains(fn_name) {
                    let suite_names = fn_set
                        .iter()
                        .filter(|(_, fn_name_in_set)| fn_name_in_set == fn_name)
                        .map(|(suite_name, _)| suite_name)
                        .collect::<Vec<_>>();
                    return Err(format!(
                        "Duplicate function usage found for function `{}` in suites: {:?}",
                        fn_name, suite_names
                    )
                    .into());
                }
                fns.insert(fn_name.clone());
            }
            if let Some(omit_fns) = &entry.omit_functions {
                fns.retain(|fn_name| !omit_fns.contains(fn_name));
            }

            let object_ident = new_ident(name);

            let mut fns = fns
                .iter()
                .map(|v| new_ident(to_snake_case(v)))
                .collect::<Vec<_>>();
            fns.sort();

            if fns.is_empty() {
                continue;
            }

            output.extend(quote! {
                const _: () = {
                    use crate::low_plugin::objects::#object_ident;
                    #(
                        let _ = #object_ident::#fns;
                    )*
                };
            });
        }

        std::fs::write(out_path, output.to_string())?;
    }

    Ok(())
}

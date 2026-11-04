use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use convert_case::Casing as _;
use quote::quote;

use openfx_codegen_config::CodegenConfig;

use crate::utils::new_ident;

pub fn gen_low_plugin_suites(
    config: &CodegenConfig,
    output_folder: &Path,
    suites: HashMap<String, HashSet<String>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut suite_names: Vec<String> = suites.keys().cloned().collect();
    suite_names.sort();
    let simple_names: Vec<String> = suite_names
        .iter()
        .map(|name| {
            name.strip_prefix("Ofx")
                .ok_or("Suite name should start with \"Ofx\".")
                .map(str::to_owned)
        })
        .collect::<Result<_, _>>()?;

    let simple_idents: Vec<syn::Ident> = simple_names.iter().map(new_ident).collect();

    // suites.rs
    {
        let mut output = quote! {
            use openfx_internal_macros::low_make_suite_struct;
        };

        for (name, simple_ident) in suite_names.iter().zip(simple_idents.iter()) {
            let full_ident = new_ident(name);
            output.extend(quote! {
                low_make_suite_struct!(#simple_ident,#full_ident);
            });
        }

        std::fs::write(output_folder.join("suites.rs"), output.to_string())?;
    }

    // checks/suites.rs
    {
        let out_folder = output_folder.join("checks");
        std::fs::create_dir_all(&out_folder)?;
        let out_path = out_folder.join("suites.rs");

        let mut output = proc_macro2::TokenStream::new();

        for (name, simple_ident) in suite_names.iter().zip(simple_idents.iter()) {
            let special_case = &config.suites.special_cases.get(&simple_ident.to_string());

            let mut fns: Vec<syn::Ident> = vec![];
            for fn_raw_name in &suites[name] {
                let fn_cfg = special_case
                    .and_then(|sc| sc.functions.as_ref())
                    .and_then(|fs| fs.get(fn_raw_name));

                if fn_cfg.is_some_and(|cfg| cfg.omit) {
                    continue;
                }

                let mut fn_name = fn_cfg
                    .and_then(|cfg| cfg.rust_name.as_ref())
                    .cloned()
                    .unwrap_or_else(|| fn_raw_name.to_case(convert_case::Case::Snake));

                if fn_cfg.is_some_and(|cfg| cfg.prefer_checking_sys_name_in_suite) {
                    fn_name = format!("sys_{fn_name}");
                }

                let fn_ident = new_ident(&fn_name);
                fns.push(fn_ident);
            }
            fns.sort();

            if fns.is_empty() {
                continue;
            }

            output.extend(quote! {
                const _: () = {
                    use crate::low_plugin::suites::#simple_ident;
                    #(
                        let _ = #simple_ident::#fns;
                    )*
                };
            });
        }

        std::fs::write(out_path, output.to_string())?;
    }

    // impl_host_for_fetch_suites.rs
    {
        let mut output = quote! {
            use openfx_internal_macros::low_plugin_impl_host_for_fetch_suite;
        };

        for simple_ident in &simple_idents {
            let special_case = &config.suites.special_cases.get(&simple_ident.to_string());

            if let Some(corrected_cname) = special_case.and_then(|sc| sc.key_name.as_deref()) {
                let corrected_cname_ident = new_ident(corrected_cname);
                output.extend(quote! {
                    low_plugin_impl_host_for_fetch_suite!(#simple_ident @ #corrected_cname_ident);
                });
            } else {
                output.extend(quote! {
                    low_plugin_impl_host_for_fetch_suite!(#simple_ident);
                });
            }
        }

        std::fs::write(
            output_folder.join("impl_host_for_fetch_suites.rs"),
            output.to_string(),
        )?;
    }

    Ok(())
}

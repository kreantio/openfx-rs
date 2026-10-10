use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use convert_case::Casing as _;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{config::CodegenConfig, utils::strip_common_prefix};

pub fn gen_low_statuses(
    config: &CodegenConfig,
    output_file: &Path,
    statuses: HashSet<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut checks: TokenStream = TokenStream::new();
    let mut non_main_ones: HashSet<String> = HashSet::new();
    for group in &config.statuses.identical_ones {
        if let Some((main, rest)) = group.split_first() {
            let main_ident = syn::Ident::new(main, proc_macro2::Span::call_site());
            non_main_ones.extend(
                rest.iter()
                    .map(|x| x.strip_prefix("kOfxStat").unwrap().to_owned()),
            );
            for non_main in rest {
                let non_main_ident = syn::Ident::new(non_main, proc_macro2::Span::call_site());
                checks.extend(quote! {
                    const _: () = assert!(crate::sys_umbrella::#main_ident == crate::sys_umbrella::#non_main_ident);
                });
            }
        }
    }

    let mut statuses: Vec<String> = statuses
        .into_iter()
        .filter(|status| !non_main_ones.contains(status))
        .collect();
    statuses.sort();
    let status_sys_ident = statuses
        .iter()
        .map(|status| {
            syn::Ident::new(
                &format!("kOfxStat{}", status),
                proc_macro2::Span::call_site(),
            )
        })
        .collect::<Vec<_>>();
    let statuses = statuses
        .iter()
        .map(|status| syn::Ident::new(status, proc_macro2::Span::call_site()))
        .collect::<Vec<_>>();

    let code = quote! {
        pub enum Status {
            #(#statuses,)*
            Unknown(crate::sys_umbrella::OfxStatus),
        }
        #checks
        impl From<crate::sys_umbrella::OfxStatus> for Status {
            fn from(status: crate::sys_umbrella::OfxStatus) -> Self {
                match status {
                    #(crate::sys_umbrella::#status_sys_ident => Self::#statuses,)*
                    _ => Self::Unknown(status),
                }
            }
        }
        impl From<Status> for crate::sys_umbrella::OfxStatus {
            fn from(status: Status) -> Self {
                match status {
                    #(Status::#statuses => crate::sys_umbrella::#status_sys_ident,)*
                    Status::Unknown(status) => status,
                }
            }
        }
    };
    std::fs::write(output_file, prettyplease::unparse(&syn::parse2(code)?))?;

    Ok(())
}

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

        let enum_name = syn::Ident::new(&enum_name, proc_macro2::Span::call_site());
        let simple_enum_name = syn::Ident::new(simple_enum_name, proc_macro2::Span::call_site());

        let mut vars = vars.into_iter().collect::<Vec<_>>();
        vars.sort();

        let simple_vars = strip_common_prefix(&vars);

        let vars: Vec<syn::Ident> = vars
            .iter()
            .map(|v| syn::Ident::new(v, proc_macro2::Span::call_site()))
            .collect();
        let simple_vars: Vec<syn::Ident> = simple_vars
            .iter()
            .map(|v| syn::Ident::new(v, proc_macro2::Span::call_site()))
            .collect();

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

    let output_inner = prettyplease::unparse(&syn::parse2(output_inner)?)
        .lines()
        .map(|l| format!("    {}", l))
        .collect::<Vec<_>>()
        .join("\n");

    std::fs::write(
        output_file,
        format!(
            "openfx_internal_macros::low_make_property_enums_from_c! {{
{output_inner}
}}"
        ),
    )?;

    Ok(())
}

pub fn gen_low_plugin_suites(
    config: &CodegenConfig,
    output_folder_c: &Path,
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

    let simple_idents: Vec<syn::Ident> = simple_names
        .iter()
        .map(|name| syn::Ident::new(name, proc_macro2::Span::call_site()))
        .collect();

    // low_suites_plugin
    {
        let mut output = proc_macro2::TokenStream::new();

        for (name, simple_ident) in suite_names.iter().zip(simple_idents.iter()) {
            let special_case = &config.suites.special_cases.get(&simple_ident.to_string());
            let fns_to_omit = special_case.and_then(|sc| sc.omit_functions.as_ref());
            let fn_rust_names = special_case.and_then(|sc| sc.function_rust_names.as_ref());

            let full_ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            let mut fns: Vec<syn::Ident> = suites[name]
                .iter()
                .filter(|v| fns_to_omit.is_none_or(|o| !o.contains(*v)))
                .map(|v| {
                    syn::Ident::new(
                        &fn_rust_names
                            .and_then(|m| m.get(v))
                            .cloned()
                            .unwrap_or_else(|| v.to_case(convert_case::Case::Snake)),
                        proc_macro2::Span::call_site(),
                    )
                })
                .collect();
            fns.sort();

            output.extend(quote! {
                openfx_internal_macros::low_make_suite_struct!(#simple_ident:#full_ident: #(#fns,)*);
            });
        }

        std::fs::write(
            output_folder_c.join("low_suites_plugin.rs"),
            prettyplease::unparse(&syn::parse2(output)?),
        )?;
    }

    // low_plugin_impl_host_for_fetch_suites
    {
        let mut output = proc_macro2::TokenStream::new();

        for simple_ident in &simple_idents {
            let special_case = &config.suites.special_cases.get(&simple_ident.to_string());

            if let Some(corrected_k_name) = special_case.and_then(|sc| sc.key_name.as_deref()) {
                let corrected_k_ident =
                    syn::Ident::new(corrected_k_name, proc_macro2::Span::call_site());
                output.extend(quote! {
                    openfx_internal_macros::low_plugin_impl_host_for_fetch_suite!(#simple_ident @ #corrected_k_ident);
                });
            } else {
                output.extend(quote! {
                    openfx_internal_macros::low_plugin_impl_host_for_fetch_suite!(#simple_ident);
                });
            }
        }

        std::fs::write(
            output_folder_c.join("low_plugin_impl_host_for_fetch_suites.rs"),
            prettyplease::unparse(&syn::parse2(output)?),
        )?;
    }

    Ok(())
}

pub fn gen_low_plugin_objects(
    config: &CodegenConfig,
    output_file: &Path,
    direct_handle_usages_in_suite_functions: HashMap<String, HashSet<(String, String)>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut output = proc_macro2::TokenStream::new();

    let mut mapping = config.objects.mapping.iter().collect::<Vec<_>>();
    mapping.sort_by(|a, b| a.0.cmp(b.0));

    for (name, entry) in mapping {
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

        let object_ident = syn::Ident::new(name, proc_macro2::Span::call_site());

        let handle_ident = syn::Ident::new(&entry.is, proc_macro2::Span::call_site());
        let mut fns = fns
            .iter()
            .map(|v| {
                syn::Ident::new(
                    &v.to_case(convert_case::Case::Snake),
                    proc_macro2::Span::call_site(),
                )
            })
            .collect::<Vec<_>>();
        fns.sort();

        output.extend(quote! {
            openfx_internal_macros::low_make_object_struct!(#object_ident:#handle_ident: #(#fns,)*);
        });
    }

    std::fs::write(output_file, prettyplease::unparse(&syn::parse2(output)?))?;

    Ok(())
}

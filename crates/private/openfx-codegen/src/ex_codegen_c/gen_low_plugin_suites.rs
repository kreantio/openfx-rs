use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use quote::quote;

use openfx_codegen_config::CodegenConfig;

use crate::utils::{new_ident, to_snake_case};

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

    let mut output = quote! {
        use openfx_internal_macros::low_make_suite_struct;
        use crate::sys_umbrella as su;
    };

    // suites.rs: `low_make_suite_struct!`s
    {
        for (name, simple_ident) in suite_names.iter().zip(simple_idents.iter()) {
            let full_ident = new_ident(name);
            output.extend(quote! {
                low_make_suite_struct!(#simple_ident,#full_ident);
            });
        }
    }
    // suites.rs: impl_host_for_fetch_suites
    {
        let mut output_inner = proc_macro2::TokenStream::new();

        for simple_ident in &simple_idents {
            let special_case = &config.suites.special_cases.get(&simple_ident.to_string());

            if let Some(corrected_cname) = special_case.and_then(|sc| sc.key_name.as_deref()) {
                output_inner.extend(process_impl_host_for_fetch_suite(
                    simple_ident.clone(),
                    Some(new_ident(corrected_cname)),
                )?);
            } else {
                output_inner.extend(process_impl_host_for_fetch_suite(
                    simple_ident.clone(),
                    None,
                )?);
            }
        }

        output.extend(quote! {
            impl crate::low_plugin::Host {
                #output_inner
            }
        });
    }

    std::fs::write(output_folder.join("suites.rs"), output.to_string())?;

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
                    .unwrap_or_else(|| to_snake_case(fn_raw_name));

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

    Ok(())
}

fn process_impl_host_for_fetch_suite(
    simple_ident: syn::Ident,
    corrected_k_ident: Option<syn::Ident>,
) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let simple_ident_str = simple_ident.to_string();
    let Some(v_index) = simple_ident_str.rfind('V') else {
        return Err(format!("expected identifier to contain 'V': {simple_ident_str}").into());
    };
    let Some(version_number) = simple_ident_str[v_index + 1..].parse::<i32>().ok() else {
        return Err(
            format!("expected a valid version number after 'V': {simple_ident_str}").into(),
        );
    };
    let version_number = syn::LitInt::new(&version_number.to_string(), simple_ident.span());
    let fn_ident = syn::Ident::new(
        &format!("fetch_{}", to_snake_case(&simple_ident_str),),
        simple_ident.span(),
    );
    let k_ident = if let Some(corrected_k_ident) = &corrected_k_ident {
        corrected_k_ident.to_owned()
    } else {
        syn::Ident::new(
            &format!("kOfx{}", &simple_ident_str[..v_index]),
            simple_ident.span(),
        )
    };

    const DOC: &str = "## Safety\n\n`self` must be valid.";
    Ok(quote! {
        #[doc = #DOC]
        pub unsafe fn #fn_ident(&self) -> Option<#simple_ident> {
            let sys_suite = unsafe { self.sys_fetch_suite(su::#k_ident, #version_number) };
            #simple_ident::try_from(sys_suite)
        }
    })
}

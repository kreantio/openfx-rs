use std::{collections::HashSet, path::Path};

use quote::quote;

use openfx_codegen_config::CodegenConfig;

use crate::utils::new_ident;

pub fn gen_low_statuses(
    config: &CodegenConfig,
    output_file: &Path,
    statuses: HashSet<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut output = proc_macro2::TokenStream::new();

    let mut non_main_ones: HashSet<String> = HashSet::new();
    for group in &config.statuses.identical_ones {
        if let Some((main, rest)) = group.split_first() {
            let main_ident = new_ident(main);
            non_main_ones.extend(
                rest.iter()
                    .map(|x| {
                        x.strip_prefix("kOfxStat")
                            .map(ToString::to_string)
                            .ok_or_else(|| {
                                format!("Expected the status `{x}` to start with `kOfxStat`!")
                            })
                    })
                    .collect::<Result<Vec<String>, _>>()?,
            );
            for non_main in rest {
                let non_main_ident = new_ident(non_main);
                output.extend(quote! {
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

    let statuses_ok: Vec<_> = statuses.extract_if(.., |status| status == "OK").collect();
    if statuses_ok.is_empty() {
        return Err("Expected the status `kOfxStatOK` to be present.".into());
    }

    {
        let statuses_errors: Vec<_> = statuses
            .extract_if(.., |status| {
                config
                    .statuses
                    .groups
                    .errors
                    .members_matcher
                    .matches(status)
            })
            .collect();
        let enum_name_ident = new_ident("StatusError");
        gen_group(&mut output, &enum_name_ident, statuses_errors)?;
    }

    {
        let statuses_replies: Vec<_> = statuses
            .extract_if(.., |status| {
                config
                    .statuses
                    .groups
                    .replies
                    .members_matcher
                    .matches(status)
            })
            .collect();
        let enum_name_ident = new_ident("StatusReply");
        gen_group(&mut output, &enum_name_ident, statuses_replies)?;
    }

    if !statuses.is_empty() {
        return Err(format!("Unhandled statuses: {:?}", statuses).into());
    }

    std::fs::write(output_file, output.to_string())?;

    Ok(())
}

fn gen_group(
    output: &mut proc_macro2::TokenStream,
    enum_name_ident: &syn::Ident,
    statuses: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    if statuses.is_empty() {
        return Err(format!("No statuses provided to generate `enum {enum_name_ident}`!").into());
    }

    let status_sys_idents = statuses
        .iter()
        .map(|status| new_ident(format!("kOfxStat{}", status)))
        .collect::<Vec<_>>();
    let statuses = statuses.iter().map(new_ident).collect::<Vec<_>>();
    let status_docs = status_sys_idents
        .iter()
        .map(|ident| format!("See: [`crate::sys_umbrella::{ident}`]"))
        .collect::<Vec<_>>();

    output.extend(quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[must_use]
        #[non_exhaustive]
        pub enum #enum_name_ident {
            #(
                #[doc = #status_docs]
                #statuses,
            )*
        }
        impl #enum_name_ident {
            pub fn try_from_sys(sys_status: crate::sys_umbrella::OfxStatus) -> Option<Self> {
                match sys_status {
                    #(crate::sys_umbrella::#status_sys_idents => Some(Self::#statuses),)*
                    _ => None,
                }
            }
        }
        impl From<#enum_name_ident> for crate::sys_umbrella::OfxStatus {
            fn from(status: #enum_name_ident) -> Self {
                match status {
                    #(#enum_name_ident::#statuses => crate::sys_umbrella::#status_sys_idents,)*
                }
            }
        }
    });

    Ok(())
}

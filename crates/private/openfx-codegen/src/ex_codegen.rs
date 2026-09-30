use std::{collections::HashSet, path::Path};

use proc_macro2::TokenStream;
use quote::quote;

use crate::CodegenConfig;

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

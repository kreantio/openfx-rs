use std::{
    collections::{BTreeSet, HashSet},
    path::Path,
};

use quote::quote;

use openfx_codegen_config::CodegenConfig;

use crate::{
    ex_codegen_metadata::common::{
        PropertyNames, process_property_set, process_property_set_definitions,
    },
    input_metadata::InputMetadata,
    utils::{new_ident, to_snake_case},
};

pub fn gen_low_plugin_parameters(
    config: &CodegenConfig,
    output_file: &Path,
    metadata: &InputMetadata,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut output = quote! {
        use std::ffi::CStr;
        use crate::{low::InboundResult, low_plugin::suites::ParameterSuiteV1, sys_umbrella as su};
    };

    {
        let output_property_sets = process_parameter_property_sets(config, metadata)?;
        output.extend(output_property_sets);
    }
    {
        let output_impl = process_impl_param_set_descriptor(config)?;
        output.extend(output_impl);
    }

    std::fs::write(output_file, output.to_string())?;

    Ok(())
}

fn process_parameter_property_sets(
    config: &CodegenConfig,
    metadata: &InputMetadata,
) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let mut output_inner = proc_macro2::TokenStream::new();

    let mut used_refs: HashSet<String> = HashSet::new();

    for (parameter_type_name, entry) in &config.parameters.mapping {
        let Some(set) = metadata.raw.propset_map.get(&entry.property_set) else {
            return Err(format!("Missing property set: {}", entry.property_set).into());
        };

        let set_name = format!("Param{parameter_type_name}");

        let mut extra_meta_args: Vec<proc_macro2::TokenStream> = vec![];

        used_refs.extend(set.props_refs.iter().cloned());
        let refs = set.props_refs.iter().map(new_ident).collect::<Vec<_>>();

        let mut m_allow_list: BTreeSet<String> = BTreeSet::new();
        for prop_name in set.props.keys() {
            if config.parameters.generic_properties.contains(prop_name) {
                m_allow_list.insert(prop_name.clone());
            }
        }
        for ref_name in &set.props_refs {
            let Some(def) = metadata.raw.propsetdef_map.get(ref_name) else {
                return Err(format!("Missing propsetdef: {ref_name}").into());
            };
            for prop_name in def.props.keys() {
                if config.parameters.generic_properties.contains(prop_name) {
                    m_allow_list.insert(prop_name.clone());
                }
            }
        }
        {
            let ty = if let Some(ty) = &entry.r#type {
                let el_ty = match ty.element_type {
                    openfx_codegen_config::CodegenConfigParameterMappingEntryElementType::Int => quote! { Int },
                    openfx_codegen_config::CodegenConfigParameterMappingEntryElementType::Double => quote! { Double },
                    openfx_codegen_config::CodegenConfigParameterMappingEntryElementType::String => quote! { String },
                    openfx_codegen_config::CodegenConfigParameterMappingEntryElementType::Pointer => quote! { Pointer },
                    openfx_codegen_config::CodegenConfigParameterMappingEntryElementType::Bool => quote! { Bool },
                };
                let dim =
                    syn::LitInt::new(&ty.dimensions.to_string(), proc_macro2::Span::call_site());
                quote! { (#el_ty; #dim) }
            } else {
                tracing::warn!(
                    "Valueless parameter type `{}`'s property set has generic properties: {}",
                    parameter_type_name,
                    m_allow_list.iter().cloned().collect::<Vec<_>>().join(", ")
                );
                quote! { None }
            };

            let mut m_allow_list_idents: Vec<syn::Ident> = vec![];
            for cname in &m_allow_list {
                let mut names = PropertyNames::new(cname);
                m_allow_list_idents.push(new_ident(names.snake_name()?.as_str()));
            }
            extra_meta_args.push(
                quote! { monomorphization = { type = #ty, allow_list = [#(#m_allow_list_idents),*] } },
            )
        }

        output_inner.extend(process_property_set(
            metadata,
            &set_name,
            set,
            &refs,
            &extra_meta_args,
        )?);
    }

    if let Some(output_def) = process_property_set_definitions(metadata, &used_refs)? {
        output_inner.extend(output_def);
    }

    Ok(quote! {
        #[openfx_internal_macros::low_make_property_set_structs]
        mod __ {
            #output_inner
        }
    })
}

fn process_impl_param_set_descriptor(
    config: &CodegenConfig,
) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let mut output_inner = proc_macro2::TokenStream::new();

    for parameter_type_name in config.parameters.mapping.keys() {
        let set_name = new_ident(format!("Param{parameter_type_name}PropertySet"));
        let type_key = new_ident(format!("kOfxParamType{parameter_type_name}"));

        let param_name_snake = to_snake_case(parameter_type_name);
        let param_define_fn_name = new_ident(format!("param_define_{param_name_snake}"));
        let param_define_fn_name_no_output =
            new_ident(format!("param_define_{param_name_snake}_no_output"));

        const DOC: &str = "## Safety\n\n`self` and `parameter_suite` must be valid.";

        output_inner.extend(quote! {
            #[doc = #DOC]
            pub unsafe fn #param_define_fn_name(
                &self,
                parameter_suite: &ParameterSuiteV1,
                name: &CStr
            ) -> InboundResult<#set_name> {
                unsafe {
                    self.param_define(parameter_suite, su::#type_key, name)
                        .map_ok(#set_name::from)
                }
            }
            #[doc = #DOC]
            pub unsafe fn #param_define_fn_name_no_output(
                &self,
                parameter_suite: &ParameterSuiteV1,
                name: &CStr
            ) -> InboundResult<()> {
                unsafe {
                    self.param_define_no_output(parameter_suite, su::#type_key, name)
                }
            }
        });
    }

    Ok(quote! {
        impl crate::low_plugin::objects::ParamSetDescriptor{
            #output_inner
        }
    })
}

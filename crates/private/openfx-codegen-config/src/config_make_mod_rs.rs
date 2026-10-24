use crate::common::CodegenConfigStringMatcherAnyOf;

#[derive(serde::Deserialize)]
pub struct CodegenConfigMakeModRs {
    pub ignore: CodegenConfigStringMatcherAnyOf,
}

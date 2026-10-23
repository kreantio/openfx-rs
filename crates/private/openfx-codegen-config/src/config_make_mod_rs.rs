use crate::common::CodegenConfigStringMatcher;

#[derive(serde::Deserialize)]
pub struct CodegenConfigMakeModRs {
    pub ignore: Vec<CodegenConfigStringMatcher>,
}

pub mod bindings_for_c_headers;

mod vibe_zone;

#[derive(serde::Deserialize)]
pub struct CodegenConfig {
    pub suites: CodegenConfigSuites,
}

impl CodegenConfig {
    pub fn from_toml_str(toml_str: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(toml_str)
    }
}

#[derive(serde::Deserialize)]
pub struct CodegenConfigSuites {
    pub key_name_special_cases: std::collections::HashMap<String, String>,
}

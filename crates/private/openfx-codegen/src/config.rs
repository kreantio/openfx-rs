use std::collections::{HashMap, HashSet};

use serde::Deserialize as _;

#[derive(serde::Deserialize)]
pub struct CodegenConfig {
    pub statuses: CodegenConfigStatuses,
    pub suites: CodegenConfigSuites,
    pub objects: CodegenConfigObjects,
}

impl CodegenConfig {
    pub fn from_toml_str(toml_str: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(toml_str)
    }
}

#[derive(serde::Deserialize)]
pub struct CodegenConfigStatuses {
    pub identical_ones: Vec<Vec<String>>,
}

#[derive(serde::Deserialize)]
pub struct CodegenConfigSuites {
    pub special_cases: std::collections::HashMap<String, CodegenConfigSuiteSpecialCase>,
}

#[derive(serde::Deserialize)]
pub struct CodegenConfigSuiteSpecialCase {
    pub key_name: Option<String>,
    pub omit_functions: Option<HashSet<String>>,
    pub function_rust_names: Option<HashMap<String, String>>,
}

#[derive(serde::Deserialize)]
pub struct CodegenConfigObjects {
    #[serde(deserialize_with = "CodegenConfigObjectMappingEntry::deserialize_hashmap")]
    pub mapping: HashMap<String, CodegenConfigObjectMappingEntry>,
}

#[derive(Default, serde::Deserialize)]
pub struct CodegenConfigObjectMappingEntry {
    pub is: String,
    /// Indicates whether this object has a corresponding property set defined
    /// in `ofxPropsBySet.h`'s `prop_sets`. This mapping's key corresponds to
    /// the property set's key in that collection.
    #[serde(default)]
    pub set: CodegenConfigObjectMappingEntrySet,
    #[serde(default)]
    pub omit: bool,
    pub omit_functions: Option<HashSet<String>>,
}

impl CodegenConfigObjectMappingEntry {
    fn deserialize_hashmap<'de, D>(deserializer: D) -> Result<HashMap<String, Self>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // See: https://stackoverflow.com/a/76088737
        #[derive(serde::Deserialize)]
        #[serde(untagged)]
        enum OneOf {
            String(String),
            Zelf(CodegenConfigObjectMappingEntry),
        }

        let hashmap: HashMap<String, OneOf> = HashMap::deserialize(deserializer)?;

        Ok(hashmap
            .into_iter()
            .map(|(key, value)| {
                (
                    key,
                    match value {
                        OneOf::String(is) => Self {
                            is,
                            ..Default::default()
                        },
                        OneOf::Zelf(zelf) => zelf,
                    },
                )
            })
            .collect::<HashMap<_, _>>())
    }
}

#[derive(Default)]
pub enum CodegenConfigObjectMappingEntrySet {
    /// `false`
    #[default]
    Absent,
    /// `true`
    Present,
    /// e.g. `"foo"`
    PresentCustom(String),
}

impl<'de> serde::Deserialize<'de> for CodegenConfigObjectMappingEntrySet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        #[serde(untagged)]
        enum OneOf {
            Bool(bool),
            String(String),
        }

        Ok(match OneOf::deserialize(deserializer)? {
            OneOf::Bool(false) => Self::Absent,
            OneOf::Bool(true) => Self::Present,
            OneOf::String(name) => Self::PresentCustom(name),
        })
    }
}

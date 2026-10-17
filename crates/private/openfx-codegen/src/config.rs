use std::collections::{HashMap, HashSet};

use serde::Deserialize as _;

#[derive(serde::Deserialize)]
pub struct CodegenConfig {
    pub statuses: CodegenConfigStatuses,
    pub suites: CodegenConfigSuites,
    pub objects: CodegenConfigObjects,
    pub property_sets: CodegenConfigPropertySets,
    pub actions: CodegenConfigActions,
}

impl CodegenConfig {
    pub fn from_yaml_str(yaml_str: &str) -> Result<Self, serde_saphyr::Error> {
        serde_saphyr::from_str(yaml_str)
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
    pub functions: Option<HashMap<String, CodegenConfigSuiteSpecialCaseFunction>>,
}

#[derive(serde::Deserialize)]
pub struct CodegenConfigSuiteSpecialCaseFunction {
    #[serde(default)]
    pub omit: bool,
    #[serde(default)]
    pub rust_name: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct CodegenConfigObjects {
    #[serde(deserialize_with = "CodegenConfigObjectMappingEntry::deserialize_hashmap")]
    pub mapping: HashMap<String, CodegenConfigObjectMappingEntry>,
}

#[derive(serde::Deserialize)]
pub struct CodegenConfigPropertySets {
    pub renaming: HashMap<String, String>,
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

#[derive(serde::Deserialize)]
pub struct CodegenConfigActions {
    pub pseudo: CodegenConfigActionsPseudo,
    pub core: CodegenConfigActionsCore,
    pub concretes: HashMap<String, CodegenConfigActionsConcrete>,
}

#[derive(serde::Deserialize)]
pub struct CodegenConfigActionsPseudo {
    pub members: HashSet<String>,
}

#[derive(serde::Deserialize)]
pub struct CodegenConfigActionsCore {
    pub members_prefix: String,
    pub members: HashMap<String, CodegenConfigActionsMember>,
}

#[derive(serde::Deserialize)]
pub struct CodegenConfigActionsConcrete {
    pub handle_prefix: String,
    pub members_from_core: HashSet<String>,
    pub members_prefix: String,
    pub members: HashMap<String, CodegenConfigActionsMember>,
}

#[derive(serde::Deserialize)]
pub struct CodegenConfigActionsMember {
    pub handle: Option<String>,
}

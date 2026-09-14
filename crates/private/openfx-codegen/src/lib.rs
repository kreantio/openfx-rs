use std::collections::{HashMap, HashSet};

use crate::vibe_zone::deserializers::{
    deserialize_CodegenConfigObjectMappingEntry,
    deserialize_CodegenConfigObjectParameterSetMappingEntry,
};

pub mod bindings_for_c_headers;

mod vibe_zone;

#[derive(serde::Deserialize)]
pub struct CodegenConfig {
    pub suites: CodegenConfigSuites,
    pub objects: CodegenConfigObjects,
}

impl CodegenConfig {
    pub fn from_toml_str(toml_str: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(toml_str)
    }
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
    pub unused_property_sets_in_metadata: HashSet<String>,
    pub mapping: HashMap<String, CodegenConfigObjectMappingEntry>,
    pub parameter_set_mapping: HashMap<String, CodegenConfigObjectParameterSetMappingEntry>,
}

pub struct CodegenConfigObjectMappingEntry {
    pub is: String,
    /// Indicates whether this object has a corresponding property set defined
    /// in `ofxPropsBySet.h`'s `prop_sets`. This mapping's key corresponds to
    /// the property set's key in that collection.
    pub set: bool,
    pub omit: bool,
}

impl<'de> serde::Deserialize<'de> for CodegenConfigObjectMappingEntry {
    /// Author: OpenCode / Omen Alpha (Default)
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserialize_CodegenConfigObjectMappingEntry(deserializer)
    }
}

pub struct CodegenConfigObjectParameterSetMappingEntry {
    pub is: String,
    /// The key of the `ofxPropsBySet.h` `prop_sets` entry that corresponds to
    /// this object.
    pub set: String,
    pub ty_d: Option<(String, u32)>,
}

impl<'de> serde::Deserialize<'de> for CodegenConfigObjectParameterSetMappingEntry {
    /// Author: OpenCode / Omen Alpha (Default)
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserialize_CodegenConfigObjectParameterSetMappingEntry(deserializer)
    }
}

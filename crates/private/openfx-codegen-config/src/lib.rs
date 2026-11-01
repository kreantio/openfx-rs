use std::collections::{BTreeMap, BTreeSet};

use serde::Deserialize as _;

use crate::config_make_mod_rs::CodegenConfigMakeModRs;

mod common;
mod config_make_mod_rs;

pub use common::{CodegenConfigStringMatcher, CodegenConfigStringMatcherAnyOf};

#[derive(serde::Deserialize)]
pub struct CodegenConfig {
    pub make_mod_rs: CodegenConfigMakeModRs,

    pub statuses: CodegenConfigStatuses,
    pub suites: CodegenConfigSuites,
    pub objects: CodegenConfigObjects,
    pub parameters: CodegenConfigParameters,
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
    pub groups: CodegenConfigStatusesGroups,
}

#[derive(serde::Deserialize)]
pub struct CodegenConfigStatusesGroups {
    pub errors: CodegenConfigStatusesGroup,
    pub replies: CodegenConfigStatusesGroup,
}

#[derive(serde::Deserialize)]
pub struct CodegenConfigStatusesGroup {
    pub members_matcher: CodegenConfigStringMatcherAnyOf,
}

#[derive(serde::Deserialize)]
pub struct CodegenConfigSuites {
    pub special_cases: BTreeMap<String, CodegenConfigSuiteSpecialCase>,
}

#[derive(serde::Deserialize)]
pub struct CodegenConfigSuiteSpecialCase {
    pub key_name: Option<String>,
    pub functions: Option<BTreeMap<String, CodegenConfigSuiteSpecialCaseFunction>>,
}

#[derive(serde::Deserialize)]
pub struct CodegenConfigSuiteSpecialCaseFunction {
    #[serde(default)]
    pub omit: bool,
    #[serde(default)]
    pub rust_name: Option<String>,
    #[serde(default)]
    pub prefer_checking_sys_name_in_suite: bool,
}

#[derive(serde::Deserialize)]
pub struct CodegenConfigObjects {
    #[serde(deserialize_with = "CodegenConfigObjectMappingEntry::deserialize_mapping")]
    pub mapping: BTreeMap<String, CodegenConfigObjectMappingEntry>,
}

#[derive(serde::Deserialize)]
pub struct CodegenConfigParameters {
    pub mapping: BTreeMap<String, CodegenConfigParameterMappingEntry>,
    pub property_accessor_monomorphization_allow_list: BTreeSet<String>,
}

#[derive(serde::Deserialize)]
pub struct CodegenConfigParameterMappingEntry {
    pub property_set: String,
    #[serde(rename = "ty")]
    pub r#type: Option<CodegenConfigParameterMappingEntryType>,
}

pub struct CodegenConfigParameterMappingEntryType {
    pub element_type: CodegenConfigParameterMappingEntryElementType,
    pub dimensions: usize,
}

impl<'de> serde::Deserialize<'de> for CodegenConfigParameterMappingEntryType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let tuple: (CodegenConfigParameterMappingEntryElementType, usize) =
            serde::Deserialize::deserialize(deserializer)?;
        Ok(Self {
            element_type: tuple.0,
            dimensions: tuple.1,
        })
    }
}

#[derive(serde::Deserialize)]
pub enum CodegenConfigParameterMappingEntryElementType {
    Int,
    Double,
    String,
    Pointer,
    Bool,
}

#[derive(serde::Deserialize)]
pub struct CodegenConfigPropertySets {
    pub ignore: CodegenConfigStringMatcherAnyOf,
    pub renaming: BTreeMap<String, String>,
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
    pub omit_functions: Option<BTreeSet<String>>,
}

impl CodegenConfigObjectMappingEntry {
    fn deserialize_mapping<'de, D>(deserializer: D) -> Result<BTreeMap<String, Self>, D::Error>
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

        let map: BTreeMap<String, OneOf> = BTreeMap::deserialize(deserializer)?;

        Ok(map
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
            .collect::<BTreeMap<_, _>>())
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
    pub concretes: BTreeMap<String, CodegenConfigActionsConcrete>,
}

#[derive(serde::Deserialize)]
pub struct CodegenConfigActionsPseudo {
    pub members: BTreeSet<String>,
}

#[derive(serde::Deserialize)]
pub struct CodegenConfigActionsCore {
    pub members_prefix: String,
    pub members: BTreeMap<String, CodegenConfigActionsMember>,
}

#[derive(serde::Deserialize)]
pub struct CodegenConfigActionsConcrete {
    pub handle_prefix: String,
    pub members_from_core: BTreeSet<String>,
    pub members_prefix: String,
    pub members: BTreeMap<String, CodegenConfigActionsMember>,
}

#[derive(serde::Deserialize)]
pub struct CodegenConfigActionsMember {
    pub handle: Option<String>,
}

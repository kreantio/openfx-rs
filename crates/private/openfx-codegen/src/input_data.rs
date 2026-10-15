use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    sync::LazyLock,
};

use convert_case::Casing;
use openfx_datagen::{
    parsing::{
        RootItem, RootItemWithCommentAbove, TypeStraightforward, TypedefStructField,
        TypedefStructFieldType,
    },
    processing::Bindings,
};
use regex::Regex;

pub struct InputData {
    pub bindings: HashMap<String, Bindings>,

    pub name_to_file_name: HashMap<String, String>,
}

pub fn load_input_data(
    input_data_folder: PathBuf,
) -> Result<InputData, Box<dyn std::error::Error>> {
    let mut bindings = HashMap::new();

    let mut name_to_file_name = HashMap::new();

    for entry in std::fs::read_dir(input_data_folder.join("generated/bindings"))? {
        let entry = entry?;
        let path = entry.path();
        if !entry.file_type()?.is_file()
            || path.extension().and_then(|s| s.to_str()) != Some("json")
        {
            continue;
        }
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .and_then(|s| s.strip_suffix(".json"))
            .ok_or(format!(
                "Failed to get file name with extension `.json` stripped for path: {:?}",
                path
            ))?
            .to_string();
        let single_bindings: Bindings = serde_json::from_str(&std::fs::read_to_string(&path)?)?;

        for item in &single_bindings.items {
            let RootItemWithCommentAbove::Item { item, .. } = item else {
                continue;
            };
            name_to_file_name.insert(item.name().to_owned(), name.clone());
            if let RootItem::TypedefOpaquePointer {
                pointee_struct_name,
                ..
            } = item
            {
                name_to_file_name.insert(pointee_struct_name.to_owned(), name.clone());
            }
        }

        bindings.insert(name, single_bindings);
    }

    Ok(InputData {
        bindings,
        name_to_file_name,
    })
}

#[derive(Default)]
pub struct Info {
    pub statuses: HashSet<String>,
    pub c_enums: HashMap<String, HashSet<String>>,
    pub suites: HashMap<String, HashSet<String>>,
    /// direct = first parameter + bare type (i.e., no `&` and `*`)
    pub direct_handle_usages_in_suite_functions: HashMap<String, HashSet<(String, String)>>,

    pub root_item_idents_per_header: HashMap<String, HashSet<String>>,
}

const COLORSPACE_HEADER_NAME: &str = "ofx-native-v1.5_aces-v1.3_ocio-v2.3.h";

pub fn collect_info(input_data: &InputData) -> Info {
    let mut info = Info::default();

    for (file_name, bindings) in &input_data.bindings {
        if file_name == COLORSPACE_HEADER_NAME {
            continue;
        }

        let mod_name = file_name
            .strip_suffix(".h")
            .unwrap()
            .strip_prefix("ofx")
            .unwrap()
            .to_case(convert_case::Case::Snake);

        for item in &bindings.items {
            if let Some(name) = item.name()
                && let Some(simple_name) = name.strip_prefix("kOfxStat")
            {
                info.statuses.insert(simple_name.to_owned());
            }

            let RootItemWithCommentAbove::Item { item, .. } = item else {
                continue;
            };

            info.root_item_idents_per_header
                .entry(mod_name.clone())
                .or_default()
                .insert(item.name().to_owned());

            match item {
                RootItem::Define { .. } => {}
                RootItem::TypedefPrimitive { .. } => {}
                RootItem::TypedefOpaquePointer {
                    pointee_struct_name,
                    ..
                } => {
                    info.root_item_idents_per_header
                        .entry(mod_name.clone())
                        .or_default()
                        .insert(pointee_struct_name.to_owned());
                }
                RootItem::TypedefFunction { .. } => {}
                RootItem::TypedefStruct { name, fields } => {
                    static RE_SUITE: LazyLock<Regex> =
                        LazyLock::new(|| Regex::new(r#"^.+SuiteV\d+$"#).unwrap());
                    if !RE_SUITE.is_match(name) {
                        continue;
                    }

                    for field in fields {
                        let TypedefStructField::Item { item, .. } = field else {
                            continue;
                        };

                        let TypedefStructFieldType::FunctionPointer { parameters, .. } =
                            &item.r#type
                        else {
                            continue;
                        };

                        info.suites
                            .entry(name.to_owned())
                            .or_default()
                            .insert(item.name.to_owned());

                        let Some(first_param) = parameters.first() else {
                            continue;
                        };

                        if let TypeStraightforward::TypeIdentifier { is } = &first_param.r#type {
                            info.direct_handle_usages_in_suite_functions
                                .entry(is.as_str().to_owned())
                                .or_default()
                                .insert((name.to_owned(), item.name.to_owned()));
                        }
                    }
                }
                RootItem::TypedefEnum { name, variants } => {
                    info.c_enums
                        .entry(name.to_owned())
                        .or_default()
                        .extend(variants.iter().map(|v| v.name.clone()));

                    for variant in variants {
                        info.root_item_idents_per_header
                            .entry(mod_name.clone())
                            .or_default()
                            .insert(variant.name.clone());
                    }
                }
            }
        }
    }

    info
}

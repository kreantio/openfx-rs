use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use openfx_datagen::{
    parsing::{DefineValue, RootItem, RootItemWithCommentAbove},
    processing::Bindings,
};

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

impl InputData {
    pub fn find_item_origin_file_name(&self, name: &str) -> Option<&str> {
        self.name_to_file_name.get(name).map(|x| x.as_str())
    }

    pub fn find_item(&self, name: &str) -> Option<&RootItem> {
        let file_name = self.find_item_origin_file_name(name).unwrap();

        self.bindings.get(file_name)?.items.iter().find_map(|item| {
            if let RootItemWithCommentAbove::Item { item, .. } = item
                && item.name() == name
            {
                Some(item)
            } else {
                None
            }
        })
    }

    pub fn find_define_value(&self, name: &str) -> Option<&DefineValue> {
        self.find_item(name).and_then(|item| {
            if let RootItem::Define { value, .. } = item {
                Some(value)
            } else {
                None
            }
        })
    }
}

#[derive(Default)]
pub struct Info {
    pub statuses: HashSet<String>,
}

pub fn collect_info(input_data: &InputData) -> Info {
    let mut info = Info::default();

    for bindings in input_data.bindings.values() {
        for item in &bindings.items {
            if let Some(name) = item.name()
                && let Some(simple_name) = name.strip_prefix("kOfxStat")
            {
                info.statuses.insert(simple_name.to_string());
            }
        }
    }

    info
}

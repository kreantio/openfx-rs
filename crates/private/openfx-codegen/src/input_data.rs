use std::{collections::HashMap, path::PathBuf};

use openfx_datagen::parsing::Bindings;

pub struct InputData {
    pub bindings: HashMap<String, Bindings>,
}

pub fn load_input_data(
    input_data_folder: PathBuf,
) -> Result<InputData, Box<dyn std::error::Error>> {
    let mut bindings = HashMap::new();

    for entry in std::fs::read_dir(input_data_folder.join("generated/bindings"))? {
        let entry = entry?;
        let path = entry.path();
        if !entry.file_type()?.is_file()
            || path.extension().and_then(|s| s.to_str()) != Some("json")
        {
            continue;
        }
        let name = path
            .file_stem()
            .ok_or(format!("Failed to get file stem for path: {:?}", path))?
            .to_string_lossy()
            .to_string();
        bindings.insert(
            name,
            serde_json::from_str(&std::fs::read_to_string(&path)?)?,
        );
    }

    Ok(InputData { bindings })
}

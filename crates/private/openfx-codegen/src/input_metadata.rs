use std::path::PathBuf;

pub struct InputMetadata {
    pub raw: openfx_datagen::metadata_extracting::Metadata,
}

pub fn load_input_metadata(
    metadata_folder: PathBuf,
) -> Result<InputMetadata, Box<dyn std::error::Error>> {
    let raw: openfx_datagen::metadata_extracting::Metadata =
        serde_json::from_str(&std::fs::read_to_string(metadata_folder.join("raw.json"))?)?;

    Ok(InputMetadata { raw })
}

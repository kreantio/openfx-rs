use std::path::PathBuf;

#[derive(clap::Parser)]
pub struct Jsonschema {
    #[clap(long)]
    schema: PathBuf,
    #[clap(long)]
    input: PathBuf,
}

pub fn run(jsonschema: Jsonschema) -> Result<(), Box<dyn std::error::Error>> {
    let schema_path = std::fs::read_to_string(&jsonschema.schema)?;
    let input_path = std::fs::read_to_string(&jsonschema.input)?;

    let schema = serde_json::from_str(&schema_path)?;
    let input = serde_json::from_str(&input_path)?;

    let validator = jsonschema::validator_for(&schema)?;

    let mut has_errors = false;
    for error in validator.iter_errors(&input) {
        has_errors = true;
        tracing::error!(
            "File: {}; Location: {}; Error: {}",
            jsonschema.input.display(),
            error.instance_path(),
            error
        );
    }

    if has_errors {
        std::process::exit(1);
    }

    Ok(())
}

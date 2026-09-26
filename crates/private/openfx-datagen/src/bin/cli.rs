use std::{collections::HashMap, path::PathBuf};

use clap::Parser;
use rayon::iter::{IntoParallelRefIterator as _, ParallelIterator as _};

use openfx_datagen::parsing::{Bindings, parse};

#[derive(Parser, Debug)]
struct Args {
    /// the path to the input C headers directory
    #[arg(long)]
    input_c_headers: PathBuf,

    /// the path to the output directory for generated data
    #[arg(long)]
    output_data: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = Args::parse();

    let mut input_entries: Vec<(std::fs::DirEntry, String)> = vec![];
    for entry in std::fs::read_dir(&args.input_c_headers)? {
        let entry = entry?;
        let path = entry.path();
        if !entry.file_type()?.is_file() || path.extension().is_none_or(|ext| ext != "h") {
            continue;
        }
        let name = path
            .file_stem()
            .ok_or_else(|| format!("Failed to get file stem for path: {:?}", path))?
            .to_string_lossy()
            .to_string();
        if !name.starts_with("ofx") {
            continue;
        }
        input_entries.push((entry, name));
    }

    let parsed_headers: HashMap<_, _> = input_entries
        .par_iter()
        .map(
            |(entry, name)| -> Result<
                (String, Bindings),
                Box<dyn std::error::Error + Send + Sync>,
            > {
                let code = std::fs::read_to_string(entry.path())?;
                Ok((name.to_owned(), parse(&code)?))
            },
        )
        .collect::<Result<HashMap<_, _>, _>>()?;

    let output_bindings_path = args.output_data.join("bindings");
    std::fs::create_dir_all(&output_bindings_path)?;

    parsed_headers.par_iter().try_for_each(
        |(name, items)| -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            let output_path = output_bindings_path.join(format!("{}.json", name));
            let file = std::fs::File::create(&output_path)?;
            serde_json::to_writer_pretty(file, items)?;

            Ok(())
        },
    )?;

    Ok(())
}

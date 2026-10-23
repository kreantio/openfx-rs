use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
};

use clap::Parser;
use openfx_codegen_config::{CodegenConfig, CodegenConfigStringMatcher};

#[derive(clap::Parser)]
struct Cli {
    #[arg(long)]
    codegen_config: PathBuf,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    MakeModRs {
        input: PathBuf,
        #[clap(short, long)]
        recursive: bool,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    let codegen_config_str = std::fs::read_to_string(&cli.codegen_config)?;
    let codegen_config = CodegenConfig::from_yaml_str(&codegen_config_str)?;

    match cli.command {
        Commands::MakeModRs { input, recursive } => {
            make_mod_rs(&codegen_config, &input, recursive)?;
        }
    }

    Ok(())
}

fn make_mod_rs(
    cfg: &CodegenConfig,
    input: &Path,
    recursive: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if input.join("mod.rs").exists() {
        std::fs::remove_file(input.join("mod.rs"))?;
        tracing::info!("{input:?}: existing mod.rs is removed.");
    }

    let mut mods: BTreeSet<String> = BTreeSet::new();

    for entry in std::fs::read_dir(input)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let path = entry.path();

            let Some(mod_name) = path.file_name() else {
                return Err(format!("Failed to get folder name: {:?}", path).into());
            };
            let mod_name = mod_name.to_string_lossy().to_string();

            if is_ignored_mod_name(&mod_name, &cfg.make_mod_rs.ignore) {
                continue;
            }

            if recursive {
                make_mod_rs(cfg, &path, recursive)?;
            }
            if path.join("mod.rs").exists() {
                mods.insert(mod_name);
            }
        } else if entry.file_type()?.is_file()
            && entry.path().extension().is_some_and(|ext| ext == "rs")
        {
            let path = entry.path();
            let Some(mod_name) = path.file_stem() else {
                return Err(format!("Failed to get file stem: {:?}", path).into());
            };
            let mod_name = mod_name.to_string_lossy().to_string();
            mods.insert(mod_name);
        }
    }

    remove_ignored_mod_names(&mut mods, &cfg.make_mod_rs.ignore);

    if mods.is_empty() {
        tracing::info!("{input:?}: skipped because no submodules were found.");
        return Ok(());
    }

    let bad_mod_names: Vec<String> = mods
        .iter()
        .filter(|m| !is_good_mod_name(m))
        .cloned()
        .collect();
    if !bad_mod_names.is_empty() {
        return Err(format!("Invalid module names found: {:?}", bad_mod_names).into());
    }

    let mut content = String::new();
    for m in mods {
        content.push_str(&format!("pub mod {m};\n",));
    }

    std::fs::write(input.join("mod.rs"), content)?;
    tracing::info!("{input:?}: mod.rs created.");

    Ok(())
}

fn remove_ignored_mod_names(mods: &mut BTreeSet<String>, ignore: &[CodegenConfigStringMatcher]) {
    mods.retain(|m| !is_ignored_mod_name(m, ignore));
}

fn is_ignored_mod_name(name: &str, ignore: &[CodegenConfigStringMatcher]) -> bool {
    ignore.iter().any(|pattern| pattern.matches(name))
}

fn is_good_mod_name(name: &str) -> bool {
    syn::parse_str::<syn::Ident>(name).is_ok()
}

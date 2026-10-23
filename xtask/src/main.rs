use std::path::{Path, PathBuf};

use clap::Parser;

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

#[derive(serde::Deserialize)]
struct CodegenConfig {
    make_mod_rs: CodegenConfigMakeModRs,
}

impl CodegenConfig {
    fn from_yaml_str(yaml_str: &str) -> Result<Self, serde_saphyr::Error> {
        serde_saphyr::from_str(yaml_str)
    }
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

#[derive(serde::Deserialize)]
struct CodegenConfigMakeModRs {
    ignore: Vec<CodegenConfigStringMatcher>,
}

enum CodegenConfigStringMatcher {
    Exact(String),
    Regex(regex::Regex),
}

impl<'de> serde::Deserialize<'de> for CodegenConfigStringMatcher {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        #[serde(untagged)]
        enum OneOf {
            Exact(String),
            Regex { regex: String },
        }
        let one_of = OneOf::deserialize(deserializer)?;
        Ok(match one_of {
            OneOf::Exact(s) => CodegenConfigStringMatcher::Exact(s),
            OneOf::Regex { regex } => CodegenConfigStringMatcher::Regex(
                regex::Regex::new(&regex).map_err(serde::de::Error::custom)?,
            ),
        })
    }
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

    let mut mods: Vec<String> = Vec::new();

    for entry in std::fs::read_dir(input)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let path = entry.path();
            if recursive {
                make_mod_rs(cfg, &path, recursive)?;
            }
            if path.join("mod.rs").exists() {
                mods.push(
                    path.file_name()
                        .expect("Failed to get folder name.")
                        .to_string_lossy()
                        .to_string(),
                );
            }
        } else if entry.file_type()?.is_file()
            && entry.path().extension().is_some_and(|ext| ext == "rs")
        {
            let name = entry
                .path()
                .file_stem()
                .expect("Failed to get file stem.")
                .to_string_lossy()
                .to_string();
            mods.push(name);
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

fn remove_ignored_mod_names(mods: &mut Vec<String>, ignore: &[CodegenConfigStringMatcher]) {
    mods.retain(|m| {
        !ignore.iter().any(|pattern| match pattern {
            CodegenConfigStringMatcher::Exact(s) => s == m,
            CodegenConfigStringMatcher::Regex(re) => re.is_match(m),
        })
    });
}

fn is_good_mod_name(name: &str) -> bool {
    let Some(first) = name.chars().next() else {
        return false;
    };
    if !first.is_ascii_alphabetic() && first != '_' {
        return false;
    }
    name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

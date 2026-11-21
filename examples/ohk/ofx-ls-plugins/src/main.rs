use std::{
    fmt::Display,
    io,
    path::{Path, PathBuf},
    str::FromStr,
};

use clap::Parser as _;
use ohk::bundles::{PluginBundle, PluginBundleConstructionError};

#[derive(Clone, Copy)]
enum Mode {
    FlatFolder,
    Bundle,
    Binary,
}

#[derive(Clone, Copy, Default)]
enum ModeEx {
    #[default]
    Auto,
    Mode(Mode),
}

impl FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "flat-folder" => Ok(Mode::FlatFolder),
            "bundle" => Ok(Mode::Bundle),
            "binary" => Ok(Mode::Binary),
            _ => Err(format!("invalid mode: {s}")),
        }
    }
}
impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Mode::FlatFolder => "flat-folder",
            Mode::Bundle => "bundle",
            Mode::Binary => "binary",
        };
        write!(f, "{s}")
    }
}

impl FromStr for ModeEx {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "auto" {
            Ok(ModeEx::Auto)
        } else {
            Mode::from_str(s).map(ModeEx::Mode)
        }
    }
}
impl Display for ModeEx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ModeEx::Auto => "auto",
            ModeEx::Mode(mode) => &mode.to_string(),
        };
        write!(f, "{s}")
    }
}

#[derive(clap::Parser)]
struct Args {
    #[clap(long, default_value_t)]
    mode: ModeEx,

    path: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .init();

    let args = Args::parse();

    let is_path_to_file = is_file_recursive(&args.path)?;

    let mode = match args.mode {
        ModeEx::Auto => match () {
            _ if is_path_to_file && args.path.extension().is_some_and(|ext| ext == "ofx") => {
                Mode::Binary
            }
            _ if !is_path_to_file
                && args
                    .path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.ends_with(".ofx.bundle")) =>
            {
                Mode::Bundle
            }
            _ if !is_path_to_file => Mode::FlatFolder,
            _ => {
                return Err(
                    format!("Failed to auto-detect mode for: {}", args.path.display()).into(),
                );
            }
        },
        ModeEx::Mode(mode) => mode,
    };

    let mut bundles: Vec<(PathBuf, Result<PluginBundle, PluginBundleConstructionError>)> = vec![];

    match mode {
        Mode::FlatFolder => {
            for entry in std::fs::read_dir(&args.path)? {
                let entry = entry?;
                let path = entry.path();
                let is_path_to_file = is_file_recursive(&path)?;
                if !is_path_to_file
                    && path
                        .file_name()
                        .and_then(|name| name.to_str())
                        .is_some_and(|name| name.ends_with(".ofx.bundle"))
                {
                    bundles.push((path.clone(), unsafe { PluginBundle::load(&path) }));
                }
            }
        }
        Mode::Bundle => {
            bundles.push((args.path.clone(), unsafe { PluginBundle::load(&args.path) }));
        }
        Mode::Binary => {
            bundles.push((
                args.path.clone(),
                unsafe { PluginBundle::load_only_with_binary(&args.path) }
                    .map_err(|err| PluginBundleConstructionError::Libloading { source: err }),
            ));
        }
    }

    let bundle_length = bundles.len();
    for (i, (path, result)) in bundles.into_iter().enumerate() {
        println!(
            "{} {}",
            path.file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("<unnamed>"),
            if result.is_ok() { "Ok" } else { "Error" }
        );
        match result {
            Err(err) => {
                println!("    {err}");
            }
            Ok(bundle) => {
                let result = bundle.initialize();
                if !result.errors.is_empty() {
                    tracing::warn!(
                        "There are plugin bundle manager initialization errors for `{}`: {:?}",
                        path.display(),
                        result.errors
                    );
                }

                if let Some(manager) = result.manager {
                    for plugin in manager.plugins() {
                        println!("    {:?}", plugin.plugin_identifier());
                    }
                }
            }
        }

        if i < bundle_length - 1 {
            println!();
        }
    }

    Ok(())
}

fn is_file_recursive(path: &Path) -> io::Result<bool> {
    if path.is_symlink() {
        return is_file_recursive(&path.read_link()?);
    }
    Ok(path.is_file())
}

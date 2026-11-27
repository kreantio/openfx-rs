use openfx::sys::core::OfxPlugin;
use opk::{Plugins, PluginsSurroundings, export_plugins_ex, host::Host, plugin_struct_ex};

use tracing_subscriber::{Layer, Registry, layer::SubscriberExt as _};

use crate::{definitions::ROOT_IDENTIFIER, plugin_1_basics::PluginExampleBasics};

mod definitions;

mod internal_utils;
mod plugin_1_basics;

struct ExamplePlugins;
export_plugins_ex!(ExamplePlugins);

impl Plugins for ExamplePlugins {
    fn plugins(s: PluginsSurroundings, _host: Option<&Host>) -> Vec<OfxPlugin> {
        set_up_tracing(&s);

        vec![plugin_struct_ex!(PluginExampleBasics)]
    }
}

#[allow(dead_code)]
enum PendingLogLevel {
    Info,
    Error,
}

fn set_up_tracing(s: &PluginsSurroundings) {
    #[allow(unused_mut)]
    let mut layers: Vec<Box<dyn Layer<Registry> + Send + Sync>> = vec![Box::new(
        tracing_subscriber::fmt::layer().with_writer(std::io::stderr),
    )];
    let mut pending_logs: Vec<(PendingLogLevel, String)> = vec![];

    cfg_select! {
        target_os = "macos" => {
            add_tracing_layers(&mut layers, &mut pending_logs);
        }
        _ => {}
    };

    let subscriber = tracing_subscriber::registry().with(layers);

    let dispatch = tracing::Dispatch::new(subscriber);

    if let Err(err) = s.set_tracing_dispatch(dispatch.clone()) {
        pending_logs.push((
            PendingLogLevel::Error,
            format!("Failed to set the tracing dispatch for `{ROOT_IDENTIFIER}`: {err:?}"),
        ));
    }

    tracing::dispatcher::with_default(&dispatch, || {
        for (level, message) in pending_logs {
            match level {
                PendingLogLevel::Info => tracing::info!("{}", message),
                PendingLogLevel::Error => tracing::error!("{}", message),
            }
        }
    });
}

#[cfg(target_os = "macos")]
fn add_tracing_layers(
    layers: &mut Vec<Box<dyn Layer<Registry> + Send + Sync>>,
    pending_logs: &mut Vec<(PendingLogLevel, String)>,
) {
    use std::path::PathBuf;

    let is_sandboxed = !std::env::var("APP_SANDBOX_CONTAINER_ID")
        .unwrap_or_default()
        .trim()
        .is_empty();
    pending_logs.push((
        PendingLogLevel::Info,
        format!("add_tracing_layers: is_sandboxed={is_sandboxed}"),
    ));

    if is_sandboxed {
        let home_path: PathBuf = objc2_foundation::NSHomeDirectory().to_string().into();
        let resolve_logs_path =
            home_path.join("Library/Application Support/Blackmagic Design/DaVinci Resolve/logs");

        if resolve_logs_path.exists() {
            let log_file_path = resolve_logs_path.join(format!("{ROOT_IDENTIFIER}.log"));
            if let Ok(file) = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&log_file_path)
            {
                layers.push(Box::new(tracing_subscriber::fmt::layer().with_writer(file)));
            }
        }
    } else {
        // This doesn't work for DaVinci Resolve from App Store: `Sandbox: logd_helper(…) deny(1) file-read-data /Applications/DaVinci Resolve.app/Contents/Libraries/Fusion/Plugins/openfx.plugin`
        layers.push(Box::new(tracing_oslog::OsLogger::new(
            ROOT_IDENTIFIER,
            "openfx-plugin",
        )));
    }
}

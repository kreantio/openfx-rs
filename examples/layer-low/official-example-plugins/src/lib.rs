use std::sync::OnceLock;

use tracing_subscriber::{Layer, Registry, layer::SubscriberExt as _};

use openfx::{
    export_plugins, plugin_struct,
    sys::core::{OfxHost, OfxPlugin},
    sys_plugin_helpers::Plugins,
};

use crate::{definitions::ROOT_IDENTIFIER, plugin_opengl::PluginExampleOpenGL};

mod definitions;

mod helpers;
mod plugin_opengl;

static TRACING_DISPATCH: OnceLock<tracing::Dispatch> = OnceLock::new();

struct ExamplePlugins;
export_plugins!(ExamplePlugins);

enum PendingLogLevel {
    Info,
    Error,
}

impl Plugins for ExamplePlugins {
    fn plugins(_host: Option<*const OfxHost>) -> Vec<OfxPlugin> {
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

        if let Err(err) = TRACING_DISPATCH.set(dispatch.clone()) {
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

        vec![plugin_struct!(PluginExampleOpenGL)]
    }
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

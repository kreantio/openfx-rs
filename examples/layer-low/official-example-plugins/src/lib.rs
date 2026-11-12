use std::sync::OnceLock;

use openfx::{
    export_plugins, plugin_struct,
    sys::core::{OfxHost, OfxPlugin},
    sys_plugin_helpers::Plugins,
};
use tracing_subscriber::{Layer, Registry, layer::SubscriberExt as _};

use crate::{definitions::ROOT_IDENTIFIER, plugin_opengl::PluginExampleOpenGL};

mod definitions;

mod helpers;
mod plugin_opengl;

static TRACING_DISPATCH: OnceLock<tracing::Dispatch> = OnceLock::new();

struct ExamplePlugins;
export_plugins!(ExamplePlugins);

impl Plugins for ExamplePlugins {
    fn plugins(_host: Option<*const OfxHost>) -> Vec<OfxPlugin> {
        let mut layers: Vec<Box<dyn Layer<Registry> + Send + Sync>> = vec![Box::new(
            tracing_subscriber::fmt::layer().with_writer(std::io::stderr),
        )];

        cfg_select! {
            target_os = "macos" => {
                layers.push(
                    Box::new(tracing_oslog::OsLogger::new(ROOT_IDENTIFIER, "openfx-plugin"))
                );
            }
            _ => {}
        };

        let subscriber = tracing_subscriber::registry().with(layers);

        if let Err(err) = TRACING_DISPATCH.set(tracing::Dispatch::new(subscriber)) {
            eprintln!("Failed to set the tracing dispatch for `{ROOT_IDENTIFIER}`: {err:?}");
        }

        vec![plugin_struct!(PluginExampleOpenGL)]
    }
}

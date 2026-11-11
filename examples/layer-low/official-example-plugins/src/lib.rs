use openfx::{
    export_plugins, plugin_struct,
    sys::core::{OfxHost, OfxPlugin},
    sys_plugin_helpers::Plugins,
};

use crate::plugin_opengl::PluginExampleOpenGL;

mod definitions;

mod helpers;
mod plugin_opengl;

struct ExamplePlugins;
export_plugins!(ExamplePlugins);

impl Plugins for ExamplePlugins {
    fn plugins(_host: Option<*const OfxHost>) -> Vec<OfxPlugin> {
        vec![plugin_struct!(PluginExampleOpenGL)]
    }
}

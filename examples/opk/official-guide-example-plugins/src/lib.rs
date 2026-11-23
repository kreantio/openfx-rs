use openfx::sys::core::OfxPlugin;
use opk::{Plugins, export_plugins_ex, host::Host, plugin_struct_ex};

use crate::plugin_1_basics::PluginExampleBasics;

mod definitions;

mod internal_utils;
mod plugin_1_basics;

struct ExamplePlugins;
export_plugins_ex!(ExamplePlugins);

impl Plugins for ExamplePlugins {
    fn plugins(_host: Option<&Host>) -> Vec<OfxPlugin> {
        vec![plugin_struct_ex!(PluginExampleBasics)]
    }
}

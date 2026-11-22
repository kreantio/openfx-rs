use openfx::{
    export_plugins,
    low_plugin::{Host, Plugins},
    plugin_struct,
    sys::core::OfxPlugin,
};

use crate::{
    plugin_1_basics::PluginExampleBasics, plugin_2_invert::PluginExampleInvert,
    plugin_3_gain::PluginExampleGain, plugin_4_saturation::PluginExampleSaturation,
    plugin_5_circle::PluginExampleCircle,
};

mod definitions;

mod helpers;
mod plugin_1_basics;
mod plugin_2_invert;
mod plugin_3_gain;
mod plugin_4_saturation;
mod plugin_5_circle;

struct ExamplePlugins;
export_plugins!(ExamplePlugins);

impl Plugins for ExamplePlugins {
    fn plugins(_host: Option<Host>) -> Vec<OfxPlugin> {
        vec![
            plugin_struct!(PluginExampleBasics),
            plugin_struct!(PluginExampleInvert),
            plugin_struct!(PluginExampleGain),
            plugin_struct!(PluginExampleSaturation),
            plugin_struct!(PluginExampleCircle),
        ]
    }
}

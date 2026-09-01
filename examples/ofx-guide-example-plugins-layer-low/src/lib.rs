#![feature(decl_macro)]

use openfx::generic::{
    sys::core::{OfxHost, OfxPlugin},
    sys_helpers::{Plugins, export_plugins, plugin_struct},
};

use crate::{
    plugin_1_basics::PluginExampleBasic, plugin_2_invert::PluginExampleInvert,
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
    fn plugins(_host: Option<*const OfxHost>) -> Vec<OfxPlugin> {
        vec![
            plugin_struct!(PluginExampleBasic),
            plugin_struct!(PluginExampleInvert),
            plugin_struct!(PluginExampleGain),
            plugin_struct!(PluginExampleSaturation),
            plugin_struct!(PluginExampleCircle),
        ]
    }
}

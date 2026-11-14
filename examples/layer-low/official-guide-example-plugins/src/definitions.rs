use std::ffi::CStr;

pub const PLUGINS_GROUPING: &CStr = c"openfx-rs: layer low: Port of Official Guide Example";

pub struct PluginInfo {
    pub identifier: &'static CStr,
    pub label: &'static CStr,
}

pub const PLUGIN_1_BASICS_INFO: PluginInfo = PluginInfo {
    identifier:  c"umajho.kreantio.openfx-rs.official-guide-example-plugin-ports.layer-low:BasicsExamplePlugin",
    label: c"OFX Basics Example",
};

pub const PLUGIN_2_INVERT_INFO: PluginInfo = PluginInfo {
    identifier:  c"umajho.kreantio.openfx-rs.official-guide-example-plugin-ports.layer-low:InvertExamplePlugin",
    label: c"OFX Invert Example",
};

pub const PLUGIN_3_GAIN_INFO: PluginInfo = PluginInfo {
    identifier:
        c"umajho.kreantio.openfx-rs.official-guide-example-plugin-ports.layer-low:GainExamplePlugin",
    label: c"OFX Gain Example",
};

pub const PLUGIN_4_SATURATION_INFO: PluginInfo = PluginInfo {
    identifier:  c"umajho.kreantio.openfx-rs.official-guide-example-plugin-ports.layer-low:SaturationExamplePlugin",
    label: c"OFX Saturation Example",
};

pub const PLUGIN_5_CIRCLE_INFO: PluginInfo = PluginInfo {
    identifier:  c"umajho.kreantio.openfx-rs.official-guide-example-plugin-ports.layer-low:CircleExamplePlugin",
    label: c"OFX Circle Example",
};

use std::ffi::CStr;

pub const PLUGINS_GROUPING: &CStr = c"openfx-rs: layer low: Port of Official Guide Example";

pub const PLUGIN_1_BASICS_IDENTIFIER: &CStr =
    c"umajho.kreantio.openfx-rs.official-guide-example-plugin-ports.layer-low:BasicsExamplePlugin";
pub const PLUGIN_1_BASICS_LABEL: &CStr = c"OFX Basics Example";

pub const PLUGIN_2_INVERT_IDENTIFIER: &CStr =
    c"umajho.kreantio.openfx-rs.official-guide-example-plugin-ports.layer-low:InvertExamplePlugin";
pub const PLUGIN_2_INVERT_LABEL: &CStr = c"OFX Invert Example";

pub const PLUGIN_3_GAIN_IDENTIFIER: &CStr =
    c"umajho.kreantio.openfx-rs.official-guide-example-plugin-ports.layer-low:GainExamplePlugin";
pub const PLUGIN_3_GAIN_LABEL: &CStr = c"OFX Gain Example";

pub const PLUGIN_4_SATURATION_IDENTIFIER: &CStr =
    c"umajho.kreantio.openfx-rs.official-guide-example-plugin-ports.layer-low:SaturationExamplePlugin";
pub const PLUGIN_4_SATURATION_LABEL: &CStr = c"OFX Saturation Example";

pub const PLUGIN_5_CIRCLE_IDENTIFIER: &CStr =
    c"umajho.kreantio.openfx-rs.official-guide-example-plugin-ports.layer-low:CircleExamplePlugin";
pub const PLUGIN_5_CIRCLE_LABEL: &CStr = c"OFX Circle Example";

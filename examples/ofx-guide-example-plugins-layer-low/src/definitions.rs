use std::ffi::CStr;

pub const PLUGINS_GROUPING: &CStr = c"openfx-rs: OFX Guide Example";

pub const PLUGIN_1_BASICS_IDENTIFIER: &CStr =
    c"umajho.kreantio.openfx-rs.ofx-guide-example-plugins.layer-sys:BasicExamplePlugin";
pub const PLUGIN_1_BASICS_LABEL: &CStr = c"[layer sys] OFX Basics Example";

pub const PLUGIN_2_INVERT_IDENTIFIER: &CStr =
    c"umajho.kreantio.openfx-rs.ofx-guide-example-plugins.layer-sys:InvertExamplePlugin";
pub const PLUGIN_2_INVERT_LABEL: &CStr = c"[layer sys] OFX Invert Example";

pub const PLUGIN_3_GAIN_IDENTIFIER: &CStr =
    c"umajho.kreantio.openfx-rs.ofx-guide-example-plugins.layer-sys:GainExamplePlugin";
pub const PLUGIN_3_GAIN_LABEL: &CStr = c"[layer sys] OFX Gain Example";

pub const PLUGIN_4_SATURATION_IDENTIFIER: &CStr =
    c"umajho.kreantio.openfx-rs.ofx-guide-example-plugins.layer-sys:SaturationExamplePlugin";
pub const PLUGIN_4_SATURATION_LABEL: &CStr = c"[layer sys] OFX Saturation Example";

pub const PLUGIN_5_CIRCLE_IDENTIFIER: &CStr =
    c"umajho.kreantio.openfx-rs.ofx-guide-example-plugins.layer-sys:CircleExamplePlugin";
pub const PLUGIN_5_CIRCLE_LABEL: &CStr = c"[layer sys] OFX Circle Example";

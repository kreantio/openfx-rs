use std::ffi::CStr;

pub const ROOT_IDENTIFIER: &str =
    "umajho.kreantio.openfx-rs.official-example-plugin-ports.layer-low";

pub const PLUGINS_GROUPING: &CStr = c"openfx-rs: layer low: Port of Official Example";

pub const PLUGIN_OPENGL_IDENTIFIER: &CStr =
    c"umajho.kreantio.openfx-rs.official-example-plugin-ports.layer-low:OpenGLSamplePlugin";
pub const PLUGIN_OPENGL_LABEL: &CStr = c"OFX OpenGL Example";

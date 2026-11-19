use std::ffi::CStr;

pub const ROOT_IDENTIFIER: &str =
    "umajho.kreantio.openfx-rs.official-example-plugin-ports.layer-low";

pub const PLUGINS_GROUPING: &CStr = c"openfx-rs: layer low: Port of Official Example";

pub struct PluginInfo {
    pub identifier: &'static CStr,
    pub label: &'static CStr,
}

pub const PLUGIN_OPENGL_INFO: PluginInfo = PluginInfo {
    identifier:
        c"umajho.kreantio.openfx-rs.official-example-plugin-ports.layer-low:OpenGLSamplePlugin",
    label: c"OFX OpenGL Example",
};

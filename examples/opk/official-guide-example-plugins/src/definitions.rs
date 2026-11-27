use std::ffi::CStr;

pub const ROOT_IDENTIFIER: &str =
    "umajho.kreantio.openfx-rs.official-guide-example-plugin-ports.opk";

pub const PLUGINS_GROUPING: &CStr = c"openfx-rs: opk: Port of Official Guide Example";

pub struct PluginInfo {
    pub identifier: &'static CStr,
    pub label: &'static CStr,
}

pub const PLUGIN_1_BASICS_INFO: PluginInfo = PluginInfo {
    identifier:
        c"umajho.kreantio.openfx-rs.official-guide-example-plugin-ports.opk:BasicsExamplePlugin",
    label: c"OFX Basics Example",
};

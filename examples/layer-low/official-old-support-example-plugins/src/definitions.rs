use std::ffi::CStr;

pub const ROOT_IDENTIFIER: &str =
    "umajho.kreantio.openfx-rs.official-old-support-example-plugin-ports.layer-low";

pub const PLUGINS_GROUPING: &CStr = c"openfx-rs: layer low: Port of Official Old Support Example";

pub struct PluginInfo {
    pub identifier: &'static CStr,
    pub label: &'static CStr,
    pub description: &'static CStr,
}

pub const PLUGIN_GPU_GAIN_INFO: PluginInfo = PluginInfo {
    identifier:  c"umajho.kreantio.openfx-rs.official-old-support-example-plugin-ports.layer-low:GPUGain",
    label:  c"GPU Gain",
    description: c"Apply separate RGB gain adjustments to each channels; CUDA/OpenCL Buffers/OpenCL Images/Metal Buffers",
};

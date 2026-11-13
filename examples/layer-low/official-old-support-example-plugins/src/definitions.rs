use std::ffi::CStr;

pub const ROOT_IDENTIFIER: &str =
    "umajho.kreantio.openfx-rs.official-old-support-example-plugin-ports.layer-low";

pub const PLUGINS_GROUPING: &CStr = c"openfx-rs: layer low: Port of Official Old Support Example";

pub const PLUGIN_GPU_GAIN_IDENTIFIER: &CStr =
    c"umajho.kreantio.openfx-rs.official-old-support-example-plugin-ports.layer-low:GPUGain";
pub const PLUGIN_GPU_GAIN_LABEL: &CStr = c"GPU Gain";
pub const PLUGIN_GPU_GAIN_DESCRIPTION: &CStr = c"Apply separate RGB gain adjustments to each channels; CUDA/OpenCL Buffers/OpenCL Images/Metal";

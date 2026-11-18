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

pub const PLUGIN_GPU_GAIN_FORCE_OPENCL_BUFFERS_INFO: PluginInfo = PluginInfo {
    identifier:  c"umajho.kreantio.openfx-rs.official-old-support-example-plugin-ports.layer-low:GPUGain.opencl-buffers",
    label:  c"GPU Gain (Force OpenCL Buffers)",
    description: c"Apply separate RGB gain adjustments to each channels; Force OpenCL Buffers",
};

pub const PLUGIN_GPU_GAIN_FORCE_OPENCL_IMAGES_INFO: PluginInfo = PluginInfo {
    identifier:  c"umajho.kreantio.openfx-rs.official-old-support-example-plugin-ports.layer-low:GPUGain.opencl-images",
    label:  c"GPU Gain (Force OpenCL Images)",
    description: c"Apply separate RGB gain adjustments to each channels; Force OpenCL Images",
};

pub const PLUGIN_GPU_GAIN_FORCE_METAL_BUFFERS_INFO: PluginInfo = PluginInfo {
    identifier:  c"umajho.kreantio.openfx-rs.official-old-support-example-plugin-ports.layer-low:GPUGain.metal-buffers",
    label:  c"GPU Gain (Force Metal Buffers)",
    description: c"Apply separate RGB gain adjustments to each channels; Force Metal Buffers",
};

pub const PLUGIN_GPU_GAIN_FORCE_CUDA_INFO: PluginInfo = PluginInfo {
    identifier:  c"umajho.kreantio.openfx-rs.official-old-support-example-plugin-ports.layer-low:GPUGain.cuda",
    label:  c"GPU Gain (Force CUDA)",
    description: c"Apply separate RGB gain adjustments to each channels; Force CUDA",
};

pub const PLUGIN_GPU_GAIN_FORCE_CPU_INFO: PluginInfo = PluginInfo {
    identifier:
        c"umajho.kreantio.openfx-rs.official-old-support-example-plugin-ports.layer-low:GPUGain.cpu",
    label: c"GPU Gain (Force CPU)",
    description: c"Apply separate RGB gain adjustments to each channels; Force CPU",
};

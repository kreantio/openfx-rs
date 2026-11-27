/// See: https://openfx.readthedocs.io/en/latest/Reference/ofxPackaging.html#installation-directory-hierarchy
///
/// Deprecated architectures (`MacOS-x86-64`, `IRIX` and `IRIX64 `) are not
/// included.
pub const CURRENT_ARCHITECTURE: &str = cfg_select! {
    all(target_os = "macos", target_arch = "aarch64") => "MacOS",
    all(target_os = "windows", target_arch = "x86") => "Win32",
    all(target_os = "windows", target_arch = "x86_64") => "Win64",
    all(target_os = "windows", target_arch = "aarch64") => "Win-arm64ec",
    all(target_os = "linux", target_arch = "x86") => "Linux-x86",
    all(target_os = "linux", target_arch = "x86_64") => "Linux-x86-64",
};

#[allow(unused_imports)]
use std::{
    ffi::{CString, c_void},
    ptr,
    sync::OnceLock,
};

#[cfg(target_os = "macos")]
use libloading::Library;

use crate::plugin_opengl::gl;

#[cfg(target_os = "macos")]
const OPENGL_LIBRARY_PATH: &str = "/System/Library/Frameworks/OpenGL.framework/OpenGL";

#[cfg(target_os = "macos")]
static OPENGL_LIBRARY: OnceLock<Result<Library, String>> = OnceLock::new();

/// ## Agentic activities
///
/// - The first version of the implementation came up during a troubleshooting
///   chat session. The first version included library paths for Linux and
///   Windows, but the reviewer can't verify whether they work, therefore they
///   have been removed. The first version also included `is_loaded` checks
///   that were based solely on the current usage in `plugin_opengl.rs`. The
///   reviewer reimplemented the checks in `gl.rs` in a more robust manner.
///   - Author: OpenCode 2 / Kimi K3.1 Pro (provider: Moonshot (CN), thinking:
///     Default)
///   - Reviewed-by: Umaĵo
#[cfg(target_os = "macos")]
pub fn load() -> Result<(), String> {
    let library = OPENGL_LIBRARY
        .get_or_init(|| {
            unsafe { Library::new(OPENGL_LIBRARY_PATH) }.map_err(|error| error.to_string())
        })
        .as_ref()
        .map_err(|error| error.clone())?;

    gl::inner::load_with(|name| {
        let name = CString::new(name).expect("OpenGL symbol contained a NUL byte");

        unsafe {
            library
                .get::<*const c_void>(name.as_bytes_with_nul())
                .map(|symbol| *symbol)
                .unwrap_or(ptr::null())
        }
    });

    Ok(())
}

/// TODO: Implement `load` for other platforms once the maintainer(s) have
/// access to the respective environments.
#[cfg(not(target_os = "macos"))]
pub fn load() -> Result<(), String> {
    Ok(())
}

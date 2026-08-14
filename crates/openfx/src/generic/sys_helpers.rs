use std::ffi::{CStr, c_char, c_int, c_uint, c_void};

use crate::generic::sys::core::{OfxHost, OfxPlugin, OfxPropertySetHandle, OfxStatus};

pub trait Plugin {
    const PLUGIN_API: &'static CStr;
    const API_VERSION: c_int;
    const PLUGIN_IDENTIFIER: &'static CStr;
    const PLUGIN_VERSION_MAJOR: c_uint;
    const PLUGIN_VERSION_MINOR: c_uint;
    extern "C" fn set_host(host: *mut OfxHost);
    extern "C" fn main_entry(
        action: *const c_char,
        handle: *const c_void,
        in_args: OfxPropertySetHandle,
        out_args: OfxPropertySetHandle,
    ) -> OfxStatus;
}

pub trait Plugins {
    /// ## Safety
    ///
    /// The `host` parameter should not be kept.
    fn plugins(host: Option<*const OfxHost>) -> Vec<OfxPlugin>;
}

pub macro plugin_struct($plugin_type:ty) {
    OfxPlugin {
        pluginApi: <$plugin_type as Plugin>::PLUGIN_API.as_ptr(),
        apiVersion: <$plugin_type as Plugin>::API_VERSION,
        pluginIdentifier: <$plugin_type as Plugin>::PLUGIN_IDENTIFIER.as_ptr(),
        pluginVersionMajor: <$plugin_type as Plugin>::PLUGIN_VERSION_MAJOR,
        pluginVersionMinor: <$plugin_type as Plugin>::PLUGIN_VERSION_MINOR,
        setHost: Some(<$plugin_type as Plugin>::set_host),
        mainEntry: Some(<$plugin_type as Plugin>::main_entry),
    }
}

pub macro export_plugins($plugins_type:ty) {
    #[expect(non_upper_case_globals)]
    static mut __OPENFX_SYS_HELPERS__HOST: *const OfxHost = std::ptr::null();
    #[expect(non_upper_case_globals)]
    static mut __OPENFX_SYS_HELPERS__PLUGINS: *const OfxPlugin = std::ptr::null();
    #[expect(non_upper_case_globals)]
    static mut __OPENFX_SYS_HELPERS__PLUGIN_COUNT: usize = 0;

    #[expect(non_snake_case)]
    fn __OPENFX_SYS_HELPERS__initialize_plugins() {
        unsafe {
            if __OPENFX_SYS_HELPERS__PLUGINS.is_null() {
                let host = if __OPENFX_SYS_HELPERS__HOST.is_null() {
                    None
                } else {
                    Some(__OPENFX_SYS_HELPERS__HOST)
                };
                let plugins = <$plugins_type as Plugins>::plugins(host);
                __OPENFX_SYS_HELPERS__PLUGIN_COUNT = plugins.len();
                __OPENFX_SYS_HELPERS__PLUGINS = Box::leak(plugins.into_boxed_slice()).as_ptr();
            }
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn OfxSetHost(host: *const OfxHost) {
        unsafe {
            __OPENFX_SYS_HELPERS__HOST = host;
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn OfxGetNumberOfPlugins() -> c_int {
        __OPENFX_SYS_HELPERS__initialize_plugins();
        unsafe { __OPENFX_SYS_HELPERS__PLUGIN_COUNT as c_int }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn OfxGetPlugin(nth: c_int) -> *const OfxPlugin {
        __OPENFX_SYS_HELPERS__initialize_plugins();
        unsafe {
            if nth < 0 || nth as usize >= __OPENFX_SYS_HELPERS__PLUGIN_COUNT {
                std::ptr::null()
            } else {
                __OPENFX_SYS_HELPERS__PLUGINS.add(nth as usize)
            }
        }
    }
}

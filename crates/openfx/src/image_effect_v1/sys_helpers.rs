use std::ffi::{CStr, c_char, c_int, c_uint, c_void};

use crate::{
    generic::{
        self,
        sys::core::{OfxHost, OfxPropertySetHandle, OfxStatus},
    },
    image_effect_v1::sys::image_effect::kOfxImageEffectPluginApi,
};

pub trait Plugin {
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

impl<T: Plugin> generic::sys_helpers::Plugin for T {
    const PLUGIN_API: &'static CStr = kOfxImageEffectPluginApi;
    const API_VERSION: c_int = 1;
    const PLUGIN_IDENTIFIER: &'static CStr = T::PLUGIN_IDENTIFIER;
    const PLUGIN_VERSION_MAJOR: c_uint = T::PLUGIN_VERSION_MAJOR;
    const PLUGIN_VERSION_MINOR: c_uint = T::PLUGIN_VERSION_MINOR;
    #[inline(always)]
    extern "C" fn set_host(host: *mut OfxHost) {
        T::set_host(host)
    }
    #[inline(always)]
    extern "C" fn main_entry(
        action: *const c_char,
        handle: *const c_void,
        in_args: OfxPropertySetHandle,
        out_args: OfxPropertySetHandle,
    ) -> OfxStatus {
        T::main_entry(action, handle, in_args, out_args)
    }
}

pub mod properties {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/generated/code_from_cpp/sys_helpers_property_accessors.rs",
    ));
}

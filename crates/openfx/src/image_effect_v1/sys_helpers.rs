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
    //! The list of headers for properties included in this module is currently
    //! maintained manually.

    use crate::generic::sys_helpers::properties::include_accessors;
    use crate::internal::sys_helpers_macros::{
        make_property_dimension_getter, make_property_getter, make_property_resetter,
        make_property_setter,
    };

    include_accessors!(colour);
    include_accessors!(draw_suite);
    include_accessors!(gpu_render);
    include_accessors!(image_effect);
    include_accessors!(interact);
    include_accessors!(key_syms);
    include_accessors!(old);
    include_accessors!(param);
    include_accessors!(parametric_param);
}

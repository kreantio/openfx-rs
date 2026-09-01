use std::ffi::{CStr, c_uint};

use crate::sys_umbrella::OfxHost;

pub mod actions {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/generated/code_from_cpp/low_actions_plugin.rs",
    ));
}

pub trait Plugin {
    const PLUGIN_IDENTIFIER: &'static CStr;
    const PLUGIN_VERSION_MAJOR: c_uint;
    const PLUGIN_VERSION_MINOR: c_uint;
    fn set_host(host: *mut OfxHost);
    fn main_entry(
        action: actions::image_effect::ImageEffectAction,
    ) -> crate::generic::low::Result<()>;
}

impl<T: Plugin> crate::image_effect_v1::sys_helpers::Plugin for T {
    const PLUGIN_IDENTIFIER: &'static CStr = T::PLUGIN_IDENTIFIER;
    const PLUGIN_VERSION_MAJOR: c_uint = T::PLUGIN_VERSION_MAJOR;
    const PLUGIN_VERSION_MINOR: c_uint = T::PLUGIN_VERSION_MINOR;

    #[inline(always)]
    extern "C" fn set_host(host: *mut OfxHost) {
        T::set_host(host);
    }

    #[inline(always)]
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    extern "C" fn main_entry(
        action: *const std::ffi::c_char,
        handle: *const std::ffi::c_void,
        in_args: crate::sys_umbrella::OfxPropertySetHandle,
        out_args: crate::sys_umbrella::OfxPropertySetHandle,
    ) -> crate::sys_umbrella::OfxStatus {
        let action = unsafe {
            actions::image_effect::ImageEffectAction::from_sys(action, handle, in_args, out_args)
        };

        match T::main_entry(action) {
            Ok(()) => crate::sys_umbrella::kOfxStatOK,
            Err(status) => status.into(),
        }
    }
}

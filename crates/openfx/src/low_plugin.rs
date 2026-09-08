use std::ffi::{CStr, c_int, c_uint};

use crate::{low_plugin::property_sets::ImageEffectHostPropertySet, sys_umbrella::OfxHost};

pub mod actions {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/generated/code_from_cpp/low_actions_plugin.rs",
    ));
}

pub mod suites {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/generated/code_from_c/low_suites_plugin.rs",
    ));
}

pub mod objects {
    //! This module contains structs that wrap object handles, excluding
    //! property sets and parameters.

    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/generated/code_from_c/low_objects_plugin.rs",
    ));
}

pub mod property_sets {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/generated/code_from_cpp/low_property_sets_plugin.rs",
    ));
}

pub trait Plugin {
    const PLUGIN_IDENTIFIER: &'static CStr;
    const PLUGIN_VERSION_MAJOR: c_uint;
    const PLUGIN_VERSION_MINOR: c_uint;
    fn set_host(host: Host);
    fn main_entry(action: actions::image_effect::ImageEffectAction) -> crate::low::Result<()>;
}

impl<T: Plugin> crate::sys_helpers::image_effect_v1::Plugin for T {
    const PLUGIN_IDENTIFIER: &'static CStr = T::PLUGIN_IDENTIFIER;
    const PLUGIN_VERSION_MAJOR: c_uint = T::PLUGIN_VERSION_MAJOR;
    const PLUGIN_VERSION_MINOR: c_uint = T::PLUGIN_VERSION_MINOR;

    #[inline(always)]
    extern "C" fn set_host(host: *mut OfxHost) {
        let host = unsafe { Host::from_sys(host as *const OfxHost) };
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

pub trait HostOwned {}

/// ## Safety
///
/// This type is not `Send`: it contains host-owned pointers whose lifetime is
/// governed by the host.
///
/// If users can guarantee that no threads outlive their plugin's lifetime
/// expected by the host, they may create a wrapper type that implements `Send`:
///
/// ```rs,ignore
/// struct GuaranteeSend<T: HostOwned>(T);
/// unsafe impl<T: HostOwned> Send for GuaranteeSend<T> {}
/// ```
///
/// ### On `Sync`
///
/// Standards-compliant hosts are responsible for ensuring that calls to their
/// functions are thread-safe:
///
/// - [ofxThreadSafety.rst] states that “the host may need to perform locking
///   on the various function calls over the API.”
/// - Official examples using `kOfxImageEffectRenderFullySafe`, such as
///   [invert.cpp], do not use locking mechanisms.
///
/// [ofxThreadSafety.rst]: https://github.com/AcademySoftwareFoundation/openfx/blob/3de640d6f645fe6e346acd57e568d8b0a5ae4574/Documentation/sources/Reference/ofxThreadSafety.rst?plain=1#L41
/// [invert.cpp]: https://github.com/AcademySoftwareFoundation/openfx/blob/3de640d6f645fe6e346acd57e568d8b0a5ae4574/Documentation/sources/Guide/Code/Example2/invert.cpp
pub struct Host {
    sys: *const crate::sys::generic::core::OfxHost,
    host: ImageEffectHostPropertySet,
}

impl HostOwned for Host {}

unsafe impl Sync for Host {}

impl Host {
    /// ## Safety
    ///
    /// `host` must be a valid pointer to [`OfxHost`].
    pub unsafe fn from_sys(host: *const OfxHost) -> Self {
        let sys = host;
        let host = unsafe { host.as_ref_unchecked() };

        Self {
            sys,
            host: ImageEffectHostPropertySet::from(host.host),
        }
    }

    pub fn sys(&self) -> *const OfxHost {
        self.sys
    }

    pub fn host(&self) -> &ImageEffectHostPropertySet {
        &self.host
    }

    /// ## Safety
    ///
    /// The `fetchSuite` function pointer in `self.sys` must be valid.
    pub unsafe fn sys_fetch_suite(
        &self,
        suite_name: &CStr,
        suite_version: c_int,
    ) -> *const std::ffi::c_void {
        unsafe {
            let fetch_suite = self.sys().as_ref_unchecked().fetchSuite.unwrap_unchecked();
            fetch_suite(self.host().sys_handle(), suite_name.as_ptr(), suite_version)
        }
    }
}

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/generated/code_from_c/low_plugin_impl_host_for_fetch_suites.rs",
));

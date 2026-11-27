use std::{
    ffi::{CStr, CString, c_char, c_int, c_uint, c_void},
    ptr::NonNull,
    sync::{OnceLock, RwLock},
};

use openfx::sys::{
    core::{OfxHost, OfxPlugin, OfxPropertySetHandle, OfxStatus},
    image_effect::kOfxImageEffectPluginApi,
};
use snafu::Snafu;

/// A type representing a dynamically loaded OpenFX plugin.
#[derive(Debug)]
pub struct Plugin {
    #[expect(dead_code, reason = "TODO")]
    sys: NonNull<OfxPlugin>,

    /// The lifetime of this pointer is tied to [Self::sys].
    plugin_identifier: *const CStr,
    plugin_version: (c_uint, c_uint),

    #[expect(dead_code, reason = "TODO")]
    set_host: unsafe extern "C" fn(*mut OfxHost),
    #[expect(dead_code, reason = "TODO")]
    main_entry: unsafe extern "C" fn(
        action: *const c_char,
        handle: *const c_void,
        in_args: OfxPropertySetHandle,
        out_args: OfxPropertySetHandle,
    ) -> OfxStatus,

    #[expect(dead_code, reason = "TODO")]
    description: OnceLock<RwLock<PluginDescription>>,
}

#[derive(Debug)]
struct PluginDescription {}

#[derive(Debug, Snafu)]
pub enum PluginConstructionError {
    ApiMismatch { actual_api: Option<CString> },
    ApiVersionMismatch { actual_version: c_int },
    MissingFields { fields: Vec<&'static str> },
}

impl Plugin {
    pub fn try_new(sys: NonNull<OfxPlugin>) -> Result<Self, PluginConstructionError> {
        let sys_ref = unsafe { sys.as_ref() };

        let plugin_api = sys_ref.pluginApi;
        let plugin_api = if plugin_api.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(plugin_api) })
        };
        if plugin_api.is_none_or(|s| s != kOfxImageEffectPluginApi) {
            return Err(PluginConstructionError::ApiMismatch {
                actual_api: plugin_api.map(|s| s.to_owned()),
            });
        }

        let api_version = sys_ref.apiVersion;
        if api_version != 1 {
            return Err(PluginConstructionError::ApiVersionMismatch {
                actual_version: api_version,
            });
        }

        let mut missing_fields = vec![];

        let plugin_identifier = if sys_ref.pluginIdentifier.is_null() {
            missing_fields.push("pluginIdentifier");
            None
        } else {
            Some(unsafe { CStr::from_ptr(sys_ref.pluginIdentifier) })
        };

        let set_host = if let Some(set_host) = sys_ref.setHost {
            Some(set_host)
        } else {
            missing_fields.push("setHost");
            None
        };

        let main_entry = if let Some(main_entry) = sys_ref.mainEntry {
            Some(main_entry)
        } else {
            missing_fields.push("mainEntry");
            None
        };

        if let Some(plugin_identifier) = plugin_identifier
            && let Some(set_host) = set_host
            && let Some(main_entry) = main_entry
        {
            Ok(Self {
                sys,
                plugin_identifier,
                plugin_version: (sys_ref.pluginVersionMajor, sys_ref.pluginVersionMinor),
                set_host,
                main_entry,
                description: OnceLock::new(),
            })
        } else {
            Err(PluginConstructionError::MissingFields {
                fields: missing_fields,
            })
        }
    }

    #[expect(clippy::needless_lifetimes, reason = "Make it explicit for clarity.")]
    pub fn plugin_identifier<'a>(&'a self) -> &'a CStr {
        unsafe { &*self.plugin_identifier }
    }

    /// `(<major-version>, <minor-version>)` of the plugin.
    pub fn plugin_version(&self) -> (c_uint, c_uint) {
        self.plugin_version
    }
}

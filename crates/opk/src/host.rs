use std::{
    ffi::{CStr, c_int},
    ops::Deref,
    sync::{Arc, OnceLock},
};

use openfx::sys::{image_effect::kOfxImageEffectSuite, property::kOfxPropertySuite};
use snafu::Snafu;

use crate::utils::GuaranteeSend;

pub struct HostInner {
    low: openfx::low_plugin::Host,

    low_image_effect_suite_v1: OnceLock<
        Result<GuaranteeSend<openfx::low_plugin::suites::ImageEffectSuiteV1>, FetchSuiteError>,
    >,
    low_property_suite_v1: OnceLock<
        Result<GuaranteeSend<openfx::low_plugin::suites::PropertySuiteV1>, FetchSuiteError>,
    >,
}

/// ## Safety
///
/// If the host is provided via the `OfxSetHost` function exported by the plugin
/// binary:
/// - This is safe until `OfxGetPlugin` is called.
///
/// If the host is provided via [`openfx::sys::core::OfxPlugin::setHost`]:
/// - This is safe for the lifetime of the loaded plugin binary.
unsafe impl Send for HostInner {}

impl HostInner {
    pub fn from_low(low: openfx::low_plugin::Host) -> Self {
        Self {
            low,
            low_image_effect_suite_v1: OnceLock::new(),
            low_property_suite_v1: OnceLock::new(),
        }
    }

    pub fn low(&self) -> &openfx::low_plugin::Host {
        &self.low
    }
}

#[derive(Debug, Clone, Snafu)]
pub enum FetchSuiteError {
    NotSupportedByHost { name: &'static CStr, version: c_int },
}

impl HostInner {
    pub fn low_image_effect_suite_v1(
        &self,
    ) -> Result<&openfx::low_plugin::suites::ImageEffectSuiteV1, &FetchSuiteError> {
        self.low_image_effect_suite_v1
            .get_or_init(|| {
                unsafe { self.low.fetch_image_effect_suite_v1() }
                    .ok_or(FetchSuiteError::NotSupportedByHost {
                        name: kOfxImageEffectSuite,
                        version: 1,
                    })
                    .map(GuaranteeSend)
            })
            .as_ref()
            .map(|s| &s.0)
    }

    pub fn low_property_suite_v1(
        &self,
    ) -> Result<&openfx::low_plugin::suites::PropertySuiteV1, &FetchSuiteError> {
        self.low_property_suite_v1
            .get_or_init(|| {
                unsafe { self.low.fetch_property_suite_v1() }
                    .ok_or(FetchSuiteError::NotSupportedByHost {
                        name: kOfxPropertySuite,
                        version: 1,
                    })
                    .map(GuaranteeSend)
            })
            .as_ref()
            .map(|s| &s.0)
    }
}

/// A wrapper ultimately around a pointer of [openfx::sys::core::OfxHost].
///
/// ## Note
///
/// Because the host's lifetime is tied to the loaded plugin binary, code
/// outside this crate can only hold references to it to guarantee safety.
pub struct Host(Arc<HostInner>);

impl Host {
    pub fn from_low(low: openfx::low_plugin::Host) -> Self {
        Self(Arc::new(HostInner::from_low(low)))
    }

    /// ## Note
    ///
    /// [`std::clone::Clone`] is deliberately not implemented, so code outside
    /// this crate cannot obtain a plain [`Host`] instance. See the [`Host`]
    /// documentation for more details.
    #[expect(dead_code, reason = "Its documentation is important.")]
    pub(crate) fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl Deref for Host {
    type Target = HostInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

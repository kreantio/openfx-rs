mod internal_utils;
pub mod shared_data_helper;

use std::sync::Arc;

use openfx::{
    low_plugin::{Host, HostOwned},
    sys::{
        generic::{
            core::{OfxStatus, kOfxStatErrMissingHostFeature},
            property::{OfxPropertySuiteV1, kOfxPropertySuite},
        },
        image_effect_v1::{
            image_effect::{OfxImageEffectSuiteV1, kOfxImageEffectSuite},
            param::{OfxParameterSuiteV1, kOfxParameterSuite},
        },
    },
};

pub struct GuaranteeSend<T: HostOwned>(pub T);
/// ## Safety
///
/// This plugin does not spawn threads, so the host exclusively controls its
/// lifecycle.
unsafe impl<T: HostOwned> Send for GuaranteeSend<T> {}

#[derive(Clone)]
pub struct SharedData<'a> {
    pub host: Arc<GuaranteeSend<Host>>,
    pub property_suite: &'a OfxPropertySuiteV1,
    pub image_effect_suite: &'a OfxImageEffectSuiteV1,
    pub parameter_suite: &'a OfxParameterSuiteV1,
}

impl<'a> SharedData<'a> {
    pub fn try_new(host: GuaranteeSend<Host>) -> Result<Self, OfxStatus> {
        let property_suite =
            unsafe { host.0.fetch_suite(kOfxPropertySuite, 1) } as *const OfxPropertySuiteV1;
        let property_suite = unsafe {
            property_suite
                .as_ref()
                .ok_or(kOfxStatErrMissingHostFeature)?
        };

        let image_effect_suite =
            unsafe { host.0.fetch_suite(kOfxImageEffectSuite, 1) } as *const OfxImageEffectSuiteV1;
        let image_effect_suite = unsafe {
            image_effect_suite
                .as_ref()
                .ok_or(kOfxStatErrMissingHostFeature)?
        };

        let parameter_suite =
            unsafe { host.0.fetch_suite(kOfxParameterSuite, 1) } as *const OfxParameterSuiteV1;
        let parameter_suite = unsafe {
            parameter_suite
                .as_ref()
                .ok_or(kOfxStatErrMissingHostFeature)?
        };

        Ok(SharedData {
            host: Arc::new(host),
            property_suite,
            image_effect_suite,
            parameter_suite,
        })
    }
}

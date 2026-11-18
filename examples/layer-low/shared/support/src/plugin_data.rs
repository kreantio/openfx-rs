use std::{ops::Deref, sync::Arc};

use openfx::{
    low::StatusError,
    low_plugin::{
        Host,
        suites::{ImageEffectSuiteV1, MultiThreadSuiteV1, ParameterSuiteV1, PropertySuiteV1},
    },
};

use crate::guarantee_send::GuaranteeSend;

#[derive(Clone)]
pub struct PluginData(pub Arc<PluginDataInner>);

#[derive(Default, Clone)]
pub struct PluginDataaFeatures {
    pub multi_thread_suite: bool,
}

pub struct PluginDataInner {
    // set of pointers provided by the host
    pub host: GuaranteeSend<Host>,
    pub property_suite: GuaranteeSend<PropertySuiteV1>,
    pub image_effect_suite: GuaranteeSend<ImageEffectSuiteV1>,
    pub parameter_suite: GuaranteeSend<ParameterSuiteV1>,
    pub multi_thread_suite: Option<GuaranteeSend<MultiThreadSuiteV1>>,
}

impl PluginData {
    pub fn try_new(
        host: GuaranteeSend<Host>,
        features: PluginDataaFeatures,
    ) -> Result<Self, StatusError> {
        let property_suite =
            unsafe { host.fetch_property_suite_v1() }.ok_or(StatusError::ErrMissingHostFeature)?;
        let image_effect_suite = unsafe { host.fetch_image_effect_suite_v1() }
            .ok_or(StatusError::ErrMissingHostFeature)?;
        let parameter_suite =
            unsafe { host.fetch_parameter_suite_v1() }.ok_or(StatusError::ErrMissingHostFeature)?;
        let multi_thread_suite = if features.multi_thread_suite {
            unsafe { host.fetch_multi_thread_suite_v1() }
        } else {
            None
        };

        Ok(PluginData(Arc::new(PluginDataInner {
            host,
            property_suite: GuaranteeSend(property_suite),
            image_effect_suite: GuaranteeSend(image_effect_suite),
            parameter_suite: GuaranteeSend(parameter_suite),
            multi_thread_suite: multi_thread_suite.map(GuaranteeSend),
        })))
    }
}

impl Deref for PluginData {
    type Target = PluginDataInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

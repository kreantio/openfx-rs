use std::{ops::Deref, sync::Arc};

use openfx::{
    low::StatusError,
    low_plugin::{
        Host, LifetimePlugin,
        objects::ImageClipInstance,
        suites::{ImageEffectSuiteV1, ParameterSuiteV1, PropertySuiteV1},
    },
};

#[derive(Clone, Copy)]
pub struct GuaranteeSend<T: LifetimePlugin>(pub T);
/// ## Safety
///
/// This plugin does not spawn threads, so the host exclusively controls its
/// lifecycle.
unsafe impl<T: LifetimePlugin> Send for GuaranteeSend<T> {}
impl<T: LifetimePlugin> Deref for GuaranteeSend<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Copy)]
pub struct GuaranteeSendClipInstance(pub ImageClipInstance);
/// ## Safety
///
/// This is safe only for the lifetime of the plugin instance from which the
/// clip instance was retrieved.
unsafe impl Send for GuaranteeSendClipInstance {}

#[derive(Clone)]
pub struct SharedData(pub Arc<SharedDataInner>);

pub struct SharedDataInner {
    // set of pointers provided by the host
    #[expect(unused)]
    pub host: GuaranteeSend<Host>,
    pub property_suite: GuaranteeSend<PropertySuiteV1>,
    pub image_effect_suite: GuaranteeSend<ImageEffectSuiteV1>,
    pub parameter_suite: GuaranteeSend<ParameterSuiteV1>,
}

impl SharedData {
    pub fn try_new(host: GuaranteeSend<Host>) -> Result<Self, StatusError> {
        let property_suite =
            unsafe { host.fetch_property_suite_v1() }.ok_or(StatusError::ErrMissingHostFeature)?;
        let image_effect_suite = unsafe { host.fetch_image_effect_suite_v1() }
            .ok_or(StatusError::ErrMissingHostFeature)?;
        let parameter_suite =
            unsafe { host.fetch_parameter_suite_v1() }.ok_or(StatusError::ErrMissingHostFeature)?;

        Ok(SharedData(Arc::new(SharedDataInner {
            host,
            property_suite: GuaranteeSend(property_suite),
            image_effect_suite: GuaranteeSend(image_effect_suite),
            parameter_suite: GuaranteeSend(parameter_suite),
        })))
    }
}

impl Deref for SharedData {
    type Target = SharedDataInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

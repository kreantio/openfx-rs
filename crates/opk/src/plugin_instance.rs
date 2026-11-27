use std::{marker::PhantomData, ptr::NonNull};

use openfx::low::Status;
use snafu::{Snafu, whatever};

use crate::{
    actions::{self, CommonActionOutput},
    plugin::PluginInstanceTracker,
    utils::ok_or_status,
};

pub trait PluginInstance: Send + Sync + Sized {
    fn handle_destroy_instance(
        self,
        #[allow(unused_variables)] s: &actions::image_effect::DestroyInstanceActionSurroundings,
    ) -> actions::image_effect::DestroyInstanceActionResult {
        Ok(CommonActionOutput::Ok)
    }

    fn handle_is_identity(
        &self,
        #[allow(unused_variables)] s: &actions::image_effect::IsIdentityActionSurroundings,
    ) -> actions::image_effect::IsIdentityActionResult {
        Ok(CommonActionOutput::ReplyDefault)
    }
}

#[derive(Debug, Snafu)]
pub enum PluginInstanceHelperError {
    HostStatusNonOk {
        status: Status,
    },
    #[snafu(whatever)]
    Whatever {
        message: String,
        #[snafu(source(from(Box<dyn std::error::Error>, Some)))]
        source: Option<Box<dyn std::error::Error>>,
    },
}

/// A helper passed to [`crate::plugin::Plugin::handle_create_instance`] for
/// creating plugin instances.
///
/// The plugin instance created through it is later retrieved through
/// [`PluginInstanceRetriever`] and eventually destroyed through
/// [`PluginInstanceDestroyer`].
pub struct PluginInstanceCreator<'a, Instance: PluginInstance> {
    s_prop: &'a openfx::low_plugin::suites::PropertySuiteV1,
    s_ifx: &'a openfx::low_plugin::suites::ImageEffectSuiteV1,
    low_image_effect_instance: &'a openfx::low_plugin::objects::ImageEffectInstance,
    tracker: &'a PluginInstanceTracker,

    _plugin_instance: PhantomData<Instance>,
}

impl<'a, Instance: PluginInstance> PluginInstanceCreator<'a, Instance> {
    pub(crate) fn new(
        s_prop: &'a openfx::low_plugin::suites::PropertySuiteV1,
        s_ifx: &'a openfx::low_plugin::suites::ImageEffectSuiteV1,
        low_image_effect_instance: &'a openfx::low_plugin::objects::ImageEffectInstance,
        tracker: &'a PluginInstanceTracker,
    ) -> Self {
        Self {
            s_prop,
            s_ifx,
            low_image_effect_instance,
            tracker,
            _plugin_instance: PhantomData,
        }
    }

    /// Creates a plugin instance associated with the image effect instance
    /// provided by the host.
    ///
    /// Calling this method multiple times is safe, but if it is called more
    /// than once, previously created instances are leaked.
    ///
    /// If it is not called, no plugin instance will be associated with the
    /// image effect instance, and the corresponding [`PluginInstanceRetriever`]
    /// will return [`None`].
    ///
    /// This method sets the property with the key `kOfxPropInstanceData` on
    /// the image effect instance's property set. It is also fine (i.e., not
    /// unsafe) to set that property yourself, since [`PluginInstanceRetriever`]
    /// and [`PluginInstanceDestroyer`] check whether they recognize the
    /// property value. However, if you both set the property manually and call
    /// this function, all values except the last one are lost, unless you
    /// retrieved them beforehand for some reason.
    pub fn create(&self, instance: Instance) -> Result<(), PluginInstanceHelperError> {
        let s_prop = self.s_prop;
        let s_ifx = self.s_ifx;

        let props = ok_or_status(unsafe { self.low_image_effect_instance.get_property_set(s_ifx) })
            .map_err(|s| PluginInstanceHelperError::HostStatusNonOk { status: s })?;

        let instance = Box::new(instance);
        let instance_raw = Box::into_raw(instance);
        let Some(instance_raw) = NonNull::new(instance_raw as *mut _) else {
            whatever!("How can `Box::into_raw` return null?");
        };

        ok_or_status(unsafe { props.set_instance_data(s_prop, Some(instance_raw)) })
            .map_err(|s| PluginInstanceHelperError::HostStatusNonOk { status: s })?;

        self.tracker.insert(instance_raw);

        Ok(())
    }
}

pub struct PluginInstanceDestroyer<'a, Instance: PluginInstance> {
    s_prop: &'a openfx::low_plugin::suites::PropertySuiteV1,
    s_ifx: &'a openfx::low_plugin::suites::ImageEffectSuiteV1,
    low_image_effect_instance: &'a openfx::low_plugin::objects::ImageEffectInstance,
    tracker: &'a PluginInstanceTracker,

    _plugin_instance: PhantomData<Instance>,
}

impl<'a, Instance: PluginInstance> PluginInstanceDestroyer<'a, Instance> {
    pub(crate) fn new(
        s_prop: &'a openfx::low_plugin::suites::PropertySuiteV1,
        s_ifx: &'a openfx::low_plugin::suites::ImageEffectSuiteV1,
        low_image_effect_instance: &'a openfx::low_plugin::objects::ImageEffectInstance,
        tracker: &'a PluginInstanceTracker,
    ) -> Self {
        Self {
            s_prop,
            s_ifx,
            low_image_effect_instance,
            tracker,
            _plugin_instance: PhantomData,
        }
    }

    pub fn destroy(&self) -> Result<Option<Instance>, PluginInstanceHelperError> {
        let s_prop = self.s_prop;
        let s_ifx = self.s_ifx;

        let props = ok_or_status(unsafe { self.low_image_effect_instance.get_property_set(s_ifx) })
            .map_err(|s| PluginInstanceHelperError::HostStatusNonOk { status: s })?;

        let instance_raw = ok_or_status(unsafe { props.get_instance_data(s_prop) })
            .map_err(|s| PluginInstanceHelperError::HostStatusNonOk { status: s })?;
        let Some(instance_raw) = instance_raw else {
            return Ok(None);
        };
        if !self.tracker.remove(instance_raw) {
            // Not set by us.
            return Ok(None);
        }
        let instance = unsafe { Box::from_raw(instance_raw.as_ptr() as *mut Instance) };

        Ok(Some(*instance))
    }
}

pub struct PluginInstanceRetriever<'a, Instance: PluginInstance> {
    s_prop: &'a openfx::low_plugin::suites::PropertySuiteV1,
    s_ifx: &'a openfx::low_plugin::suites::ImageEffectSuiteV1,
    low_image_effect_instance: &'a openfx::low_plugin::objects::ImageEffectInstance,
    tracker: &'a PluginInstanceTracker,

    _plugin_instance: PhantomData<Instance>,
}

impl<'a, Instance: PluginInstance> PluginInstanceRetriever<'a, Instance> {
    pub(crate) fn new(
        s_prop: &'a openfx::low_plugin::suites::PropertySuiteV1,
        s_ifx: &'a openfx::low_plugin::suites::ImageEffectSuiteV1,
        low_image_effect_instance: &'a openfx::low_plugin::objects::ImageEffectInstance,
        tracker: &'a PluginInstanceTracker,
    ) -> Self {
        Self {
            s_prop,
            s_ifx,
            low_image_effect_instance,
            tracker,
            _plugin_instance: PhantomData,
        }
    }

    pub fn retrieve(&self) -> Result<Option<&Instance>, PluginInstanceHelperError> {
        let s_prop = self.s_prop;
        let s_ifx = self.s_ifx;

        let props = ok_or_status(unsafe { self.low_image_effect_instance.get_property_set(s_ifx) })
            .map_err(|s| PluginInstanceHelperError::HostStatusNonOk { status: s })?;

        let instance_raw = ok_or_status(unsafe { props.get_instance_data(s_prop) })
            .map_err(|s| PluginInstanceHelperError::HostStatusNonOk { status: s })?;
        let Some(instance_raw) = instance_raw else {
            return Ok(None);
        };
        if !self.tracker.contains(instance_raw) {
            // Not set by us.
            return Ok(None);
        }
        let instance = unsafe { &*(instance_raw.as_ptr() as *const Instance) };

        Ok(Some(instance))
    }
}

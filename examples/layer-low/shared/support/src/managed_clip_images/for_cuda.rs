use std::{ffi::c_void, ptr::NonNull};

use openfx::{low::StatusError, low_plugin::objects::ImageClipInstance, sys::core::OfxTime};

use crate::{
    managed_clip_images::{ManagedClipImageBase, impl_base_functions},
    plugin_data::PluginData,
};

pub struct ManagedClipImageForCuda {
    base: ManagedClipImageBase,

    data_ptr: NonNull<c_void>,
}

impl_base_functions!(ManagedClipImageForCuda);

impl ManagedClipImageForCuda {
    pub fn try_new_from_clip_if_available(
        plugin_data: &PluginData,
        clip: &ImageClipInstance,
        frame: OfxTime,
    ) -> Result<Option<Self>, StatusError> {
        let base = ManagedClipImageBase::try_new_from_clip_if_available(plugin_data, clip, frame)?;
        if let Some(b) = base {
            Ok(Some(Self::try_new_from_base(plugin_data, b)?))
        } else {
            Ok(None)
        }
    }

    /// Construct from a clip by fetching an image at the given frame.
    pub fn try_new_from_clip(
        plugin_data: &PluginData,
        clip: &ImageClipInstance,
        frame: OfxTime,
    ) -> Result<Self, StatusError> {
        let base = ManagedClipImageBase::try_new_from_clip(plugin_data, clip, frame)?;
        Self::try_new_from_base(plugin_data, base)
    }

    fn try_new_from_base(
        plugin_data: &PluginData,
        base: ManagedClipImageBase,
    ) -> Result<Self, StatusError> {
        let s_prop = &plugin_data.property_suite.0;

        let props = &base.props;

        let Some(data_ptr) =
            (unsafe { props.get_image_data(s_prop) }).ok_or(StatusError::Failed)?
        else {
            return Err(StatusError::Failed);
        };

        Ok(Self { base, data_ptr })
    }

    pub fn data_ptr(&self) -> NonNull<c_void> {
        self.data_ptr
    }
}

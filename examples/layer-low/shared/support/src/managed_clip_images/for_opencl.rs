use std::{ffi::c_void, ptr::NonNull};

use openfx::{
    low::StatusError,
    low_plugin::objects::ImageClipInstance,
    sys::{core::OfxTime, gpu_render::kOfxImageEffectPropOpenCLImage},
};

use crate::{
    managed_clip_images::{ManagedClipImageBase, impl_base_functions},
    plugin_data::PluginData,
};

pub struct ManagedClipImageForOpencl {
    base: ManagedClipImageBase,

    data: ManagedClipImageForOpenclData,
}

pub enum ManagedClipImageForOpenclData {
    Image(NonNull<c_void>),
    Buffer(NonNull<c_void>),
}

impl_base_functions!(ManagedClipImageForOpencl);

impl ManagedClipImageForOpencl {
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

        let data = if let Some(ptr) =
            // TODO: The upstream appears to mistakenly associate
            // `kOfxImageEffectPropOpenCLImage` with the Render action's
            // `inArgs` property set rather than with the image's property set.
            // Report this upstream.
            unsafe {
                s_prop.prop_get_pointer(props.sys_handle(), kOfxImageEffectPropOpenCLImage)
            }
            // TODO: Only allow some of the errors to be ignored?
            .unwrap_or(None)
        {
            ManagedClipImageForOpenclData::Image(ptr)
        } else {
            let Some(data_ptr) =
                (unsafe { props.get_image_data(s_prop) }).ok_or(StatusError::Failed)?
            else {
                return Err(StatusError::Failed);
            };
            ManagedClipImageForOpenclData::Buffer(data_ptr)
        };

        Ok(Self { base, data })
    }

    pub fn data(&self) -> &ManagedClipImageForOpenclData {
        &self.data
    }
}

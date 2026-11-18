use std::{
    ffi::{c_int, c_void},
    ptr::NonNull,
};

use openfx::{low::StatusError, low_plugin::objects::ImageClipInstance, sys::core::OfxTime};

use crate::{
    managed_clip_images::{ManagedClipImageBase, impl_base_functions},
    plugin_data::PluginData,
};

pub struct ManagedClipImageForCpu {
    base: ManagedClipImageBase,

    row_bytes: c_int,
    data_ptr: NonNull<c_void>,
}

unsafe impl Sync for ManagedClipImageForCpu {}

impl_base_functions!(ManagedClipImageForCpu);

impl ManagedClipImageForCpu {
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

        let row_bytes = unsafe { props.get_image_row_bytes(s_prop) }.ok_or(StatusError::Failed)?;

        Ok(Self {
            base,
            row_bytes,
            data_ptr,
        })
    }

    pub fn row_bytes(&self) -> c_int {
        self.row_bytes
    }
    pub fn data_ptr(&self) -> NonNull<c_void> {
        self.data_ptr
    }

    /// Look up a pixel in the image. Returns `None` if the pixel was not in the
    /// bounds of the image.
    pub fn raw_address(&self, x: c_int, y: c_int) -> Option<*mut c_void> {
        // Inside the bounds of this image?
        if x < self.base.bounds.x1
            || x >= self.base.bounds.x2
            || y < self.base.bounds.y1
            || y >= self.base.bounds.y2
        {
            return None;
        }

        // turn image plane coordinates into offsets from the bottom left
        let x_offset = x - self.base.bounds.x1;
        let y_offset = y - self.base.bounds.y1;

        // Find the start of our row, using byte arithmetic
        let row_start = unsafe {
            (self.data_ptr.as_ptr() as *mut u8).offset(y_offset as isize * self.row_bytes as isize)
        };

        // finally find the position of the first component of the column
        Some(unsafe {
            row_start.offset(x_offset as isize * self.bytes_per_pixel() as isize) as *mut c_void
        })
    }
}

use std::{
    ffi::{c_int, c_void},
    ptr::NonNull,
};

use openfx::{
    low::{
        StatusError,
        enums::{ImageEffectPropComponents, ImageEffectPropPixelDepth},
    },
    low_plugin::{
        property_sets::{
            ImageEffectDescriptorPropertySet, ImageInstancePropertySet, LosslessEnumPropertyOwned,
        },
        suites::{ImageEffectSuiteV1, PropertySuiteV1},
    },
    sys::core::{OfxRectD, OfxRectI},
};

use crate::helpers::shared_data::{GuaranteeSend, SharedData};

pub fn rect_i_from_array(arr: &[c_int; 4]) -> OfxRectI {
    OfxRectI {
        x1: arr[0],
        y1: arr[1],
        x2: arr[2],
        y2: arr[3],
    }
}

pub fn rect_d_to_array(rect: &OfxRectD) -> [f64; 4] {
    [rect.x1, rect.y1, rect.x2, rect.y2]
}

pub unsafe fn set_labels(
    s_prop: &PropertySuiteV1,
    props: &ImageEffectDescriptorPropertySet,
    label: &std::ffi::CStr,
) -> Result<(), StatusError> {
    (unsafe {
        props
            .set_label(s_prop, Some(label))
            .ok_or(StatusError::Failed)
    })?;
    (unsafe {
        props
            .set_short_label(s_prop, Some(label))
            .ok_or(StatusError::Failed)
    })?;
    // If unset, DaVinci Resolve will display an empty label on the side panel.
    (unsafe {
        props
            .set_long_label(s_prop, Some(label))
            .ok_or(StatusError::Failed)
    })?;

    Ok(())
}

pub struct ClipImageManaged {
    image_effect_suite: GuaranteeSend<ImageEffectSuiteV1>,
    props: ImageInstancePropertySet,

    n_comps: c_int,
    pixel_depth: BitDepth,
    row_bytes: c_int,
    bounds: OfxRectI,
    pixel_aspect_ratio: f64,
    data_ptr: NonNull<c_void>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum BitDepth {
    Byte,
    Short,
    Float,
}

impl ClipImageManaged {
    /// ## Safety
    ///
    /// The caller must ensure that the input `image_props` is valid.
    pub unsafe fn try_new(
        shared_data: &SharedData,
        props: ImageInstancePropertySet,
    ) -> Result<Self, StatusError> {
        let s_prop = &shared_data.property_suite.0;

        let Some(data_ptr) =
            (unsafe { props.get_image_data(s_prop) }).ok_or(StatusError::Failed)?
        else {
            return Err(StatusError::Failed);
        };

        let n_comps = match unsafe { props.get_image_effect_components_owned(s_prop) }
            .ok_or(StatusError::Failed)?
        {
            LosslessEnumPropertyOwned::Known(ImageEffectPropComponents::Alpha) => 1,
            LosslessEnumPropertyOwned::Known(ImageEffectPropComponents::RGB) => 3,
            LosslessEnumPropertyOwned::Known(ImageEffectPropComponents::RGBA) => 4,
            x => {
                tracing::error!("Unsupported image effect components: {:?}", x.as_c_str());
                return Err(StatusError::ErrUnsupported);
            }
        };
        let pixel_depth = match unsafe { props.get_image_effect_pixel_depth_owned(s_prop) }
            .ok_or(StatusError::Failed)?
        {
            LosslessEnumPropertyOwned::Known(ImageEffectPropPixelDepth::Byte) => BitDepth::Byte,
            LosslessEnumPropertyOwned::Known(ImageEffectPropPixelDepth::Short) => BitDepth::Short,
            LosslessEnumPropertyOwned::Known(ImageEffectPropPixelDepth::Float) => BitDepth::Float,
            x => {
                tracing::error!("Unsupported image effect pixel depth: {:?}", x.as_c_str());
                return Err(StatusError::ErrUnsupported);
            }
        };
        let row_bytes = unsafe { props.get_image_row_bytes(s_prop) }.ok_or(StatusError::Failed)?;
        let bounds = unsafe { props.get_image_bounds(s_prop) }.ok_or(StatusError::Failed)?;
        let bounds = rect_i_from_array(&bounds);
        let pixel_aspect_ratio =
            unsafe { props.get_image_pixel_aspect_ratio(s_prop) }.ok_or(StatusError::Failed)?;

        Ok(Self {
            image_effect_suite: shared_data.image_effect_suite,
            props,

            n_comps,
            pixel_depth,
            row_bytes,
            bounds,
            pixel_aspect_ratio,
            data_ptr,
        })
    }

    pub fn n_comps(&self) -> c_int {
        self.n_comps
    }
    pub fn pixel_depth(&self) -> BitDepth {
        self.pixel_depth
    }
    pub fn bytes_per_component(&self) -> c_int {
        match self.pixel_depth {
            BitDepth::Byte => 1,
            BitDepth::Short => 2,
            BitDepth::Float => 4,
        }
    }
    pub fn bytes_per_pixel(&self) -> c_int {
        self.bytes_per_component() * self.n_comps
    }
    pub fn row_bytes(&self) -> c_int {
        self.row_bytes
    }
    pub fn bounds(&self) -> OfxRectI {
        self.bounds
    }
    pub fn pixel_aspect_ratio(&self) -> f64 {
        self.pixel_aspect_ratio
    }
    pub fn data_ptr(&self) -> NonNull<c_void> {
        self.data_ptr
    }

    pub fn raw_address(&self, x: c_int, y: c_int) -> Option<*mut c_void> {
        if x < self.bounds.x1 || x >= self.bounds.x2 || y < self.bounds.y1 || y >= self.bounds.y2 {
            return None;
        }

        let x_offset = x - self.bounds.x1;
        let y_offset = y - self.bounds.y1;

        let row_start = unsafe {
            (self.data_ptr.as_ptr() as *mut u8).offset(y_offset as isize * self.row_bytes as isize)
        };

        Some(unsafe {
            row_start.offset(x_offset as isize * self.bytes_per_pixel() as isize) as *mut c_void
        })
    }
}

impl Drop for ClipImageManaged {
    fn drop(&mut self) {
        let _ = unsafe { self.image_effect_suite.0.clip_release_image(&self.props) };
    }
}

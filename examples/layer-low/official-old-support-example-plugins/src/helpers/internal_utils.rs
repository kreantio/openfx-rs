#![allow(dead_code)]

use std::{
    ffi::{c_int, c_void},
    ptr::NonNull,
};

use openfx::{
    low::{
        InboundResult, StatusError,
        enums::{ImageEffectPropComponents, ImageEffectPropPixelDepth},
    },
    low_plugin::{
        objects::ImageClipInstance,
        property_sets::{ImageInstancePropertySet, LosslessEnumPropertyOwned},
        suites::ImageEffectSuiteV1,
    },
    sys::{
        core::{OfxRectI, OfxTime},
        gpu_render::kOfxImageEffectPropOpenCLImage,
    },
};

use crate::helpers::shared_data::{GuaranteeSend, SharedData};

pub mod renderer_info;

pub fn rect_i_from_array(arr: &[c_int; 4]) -> OfxRectI {
    OfxRectI {
        x1: arr[0],
        y1: arr[1],
        x2: arr[2],
        y2: arr[3],
    }
}

macro_rules! set_labels {
    ($s_prop:expr, $props:expr, $label:expr) => {{
        {
            $props
                .set_label($s_prop, Some($label))
                .ok_or(StatusError::Failed)
        }?;
        {
            $props
                .set_short_label($s_prop, Some($label))
                .ok_or(StatusError::Failed)
        }?;
        // If unset, DaVinci Resolve will display an empty label on the side panel.
        {
            $props
                .set_long_label($s_prop, Some($label))
                .ok_or(StatusError::Failed)
        }?;
    }};
}
pub(crate) use set_labels;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum BitDepth {
    Byte,
    Short,
    Float,
}

/// Struct to manage OFX images.
///
/// Originally copied from: `$REPO_HOME/examples/layer-low/official-guide-example-plugins/src/helpers/internal_utils.rs``
struct ManagedClipImageBase {
    image_effect_suite: GuaranteeSend<ImageEffectSuiteV1>,
    props: ImageInstancePropertySet,

    n_comps: c_int,
    pixel_depth: BitDepth,
    bounds: OfxRectI,
    pixel_aspect_ratio: f64,
}

unsafe impl Sync for ManagedClipImageBase {}

impl ManagedClipImageBase {
    unsafe fn try_new_from_clip_if_available(
        shared_data: &SharedData,
        clip: &ImageClipInstance,
        frame: OfxTime,
    ) -> Result<Option<Self>, StatusError> {
        let prop_set =
            match unsafe { clip.clip_get_image(&shared_data.image_effect_suite, frame, None) } {
                InboundResult::Ok(img) => img,
                InboundResult::KnownError(StatusError::Failed) => return Ok(None), // (perhaps) unconnected
                _ => return Err(StatusError::Failed),
            };

        let ret = unsafe { Self::try_new_from_property_set(shared_data, prop_set) };
        if !ret.is_ok() {
            unsafe { Self::release_image(&shared_data.image_effect_suite, &prop_set) };
        }
        ret.map(Some)
    }

    /// Construct from a clip by fetching an image at the given frame.
    unsafe fn try_new_from_clip(
        shared_data: &SharedData,
        clip: &ImageClipInstance,
        frame: OfxTime,
    ) -> Result<Self, StatusError> {
        let prop_set = unsafe { clip.clip_get_image(&shared_data.image_effect_suite, frame, None) }
            .ok_or(StatusError::Failed)?;

        let ret = unsafe { Self::try_new_from_property_set(shared_data, prop_set) };
        if !ret.is_ok() {
            unsafe { Self::release_image(&shared_data.image_effect_suite, &prop_set) };
        }
        ret
    }

    /// Construct from a property set that represents the image.
    ///
    /// ## Safety
    ///
    /// The caller must ensure that the input `image_props` is valid.
    unsafe fn try_new_from_property_set(
        shared_data: &SharedData,
        props: ImageInstancePropertySet,
    ) -> Result<Self, StatusError> {
        let s_prop = &shared_data.property_suite.0;

        // figure out the data types

        // how many components per pixel?
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

        // what is the data type
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

        let bounds = unsafe { props.get_image_bounds(s_prop) }.ok_or(StatusError::Failed)?;
        let bounds = rect_i_from_array(&bounds);

        let pixel_aspect_ratio =
            unsafe { props.get_image_pixel_aspect_ratio(s_prop) }.ok_or(StatusError::Failed)?;

        Ok(Self {
            image_effect_suite: shared_data.image_effect_suite,
            props,

            n_comps,
            pixel_depth,
            bounds,
            pixel_aspect_ratio,
        })
    }

    /// number of components.
    pub fn n_comps(&self) -> c_int {
        self.n_comps
    }
    pub fn pixel_depth(&self) -> BitDepth {
        self.pixel_depth
    }
    /// bytes per component, 1, 2 or 4 for byte, short and float images.
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
    pub fn bounds(&self) -> OfxRectI {
        self.bounds
    }
    pub fn pixel_aspect_ratio(&self) -> f64 {
        self.pixel_aspect_ratio
    }

    /// ## TODO
    ///
    /// Figure out how to properly handle non-OK statuses.
    unsafe fn release_image(
        image_effect_suite: &ImageEffectSuiteV1,
        props: &ImageInstancePropertySet,
    ) {
        let _ = unsafe { image_effect_suite.clip_release_image(props) };
    }
}

/// Destructor.
impl Drop for ManagedClipImageBase {
    fn drop(&mut self) {
        unsafe { Self::release_image(&self.image_effect_suite.0, &self.props) };
    }
}

macro_rules! impl_base_functions {
    ($struct:ident) => {
        impl $struct {
            pub fn n_comps(&self) -> c_int {
                self.base.n_comps()
            }
            pub fn pixel_depth(&self) -> BitDepth {
                self.base.pixel_depth()
            }
            pub fn bytes_per_component(&self) -> c_int {
                self.base.bytes_per_component()
            }
            pub fn bytes_per_pixel(&self) -> c_int {
                self.base.bytes_per_pixel()
            }
            pub fn bounds(&self) -> OfxRectI {
                self.base.bounds()
            }
            pub fn pixel_aspect_ratio(&self) -> f64 {
                self.base.pixel_aspect_ratio()
            }
        }
    };
}

pub struct ManagedClipImageForCpu {
    base: ManagedClipImageBase,

    row_bytes: c_int,
    data_ptr: NonNull<c_void>,
}

unsafe impl Sync for ManagedClipImageForCpu {}

impl_base_functions!(ManagedClipImageForCpu);

impl ManagedClipImageForCpu {
    pub unsafe fn try_new_from_clip_if_available(
        shared_data: &SharedData,
        clip: &ImageClipInstance,
        frame: OfxTime,
    ) -> Result<Option<Self>, StatusError> {
        let base = unsafe {
            ManagedClipImageBase::try_new_from_clip_if_available(shared_data, clip, frame)
        }?;
        if let Some(b) = base {
            Ok(Some(unsafe { Self::try_new_from_base(shared_data, b) }?))
        } else {
            Ok(None)
        }
    }

    /// Construct from a clip by fetching an image at the given frame.
    pub unsafe fn try_new_from_clip(
        shared_data: &SharedData,
        clip: &ImageClipInstance,
        frame: OfxTime,
    ) -> Result<Self, StatusError> {
        let base = unsafe { ManagedClipImageBase::try_new_from_clip(shared_data, clip, frame) }?;
        unsafe { Self::try_new_from_base(shared_data, base) }
    }

    unsafe fn try_new_from_base(
        shared_data: &SharedData,
        base: ManagedClipImageBase,
    ) -> Result<Self, StatusError> {
        let s_prop = &shared_data.property_suite.0;

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
    pub unsafe fn try_new_from_clip_if_available(
        shared_data: &SharedData,
        clip: &ImageClipInstance,
        frame: OfxTime,
    ) -> Result<Option<Self>, StatusError> {
        let base = unsafe {
            ManagedClipImageBase::try_new_from_clip_if_available(shared_data, clip, frame)
        }?;
        if let Some(b) = base {
            Ok(Some(unsafe { Self::try_new_from_base(shared_data, b) }?))
        } else {
            Ok(None)
        }
    }

    /// Construct from a clip by fetching an image at the given frame.
    pub unsafe fn try_new_from_clip(
        shared_data: &SharedData,
        clip: &ImageClipInstance,
        frame: OfxTime,
    ) -> Result<Self, StatusError> {
        let base = unsafe { ManagedClipImageBase::try_new_from_clip(shared_data, clip, frame) }?;
        unsafe { Self::try_new_from_base(shared_data, base) }
    }

    unsafe fn try_new_from_base(
        shared_data: &SharedData,
        base: ManagedClipImageBase,
    ) -> Result<Self, StatusError> {
        let s_prop = &shared_data.property_suite.0;

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

pub struct ManagedClipImageForMetal {
    base: ManagedClipImageBase,

    data_ptr: NonNull<c_void>,
}

impl_base_functions!(ManagedClipImageForMetal);

impl ManagedClipImageForMetal {
    pub unsafe fn try_new_from_clip_if_available(
        shared_data: &SharedData,
        clip: &ImageClipInstance,
        frame: OfxTime,
    ) -> Result<Option<Self>, StatusError> {
        let base = unsafe {
            ManagedClipImageBase::try_new_from_clip_if_available(shared_data, clip, frame)
        }?;
        if let Some(b) = base {
            Ok(Some(unsafe { Self::try_new_from_base(shared_data, b) }?))
        } else {
            Ok(None)
        }
    }

    /// Construct from a clip by fetching an image at the given frame.
    pub unsafe fn try_new_from_clip(
        shared_data: &SharedData,
        clip: &ImageClipInstance,
        frame: OfxTime,
    ) -> Result<Self, StatusError> {
        let base = unsafe { ManagedClipImageBase::try_new_from_clip(shared_data, clip, frame) }?;
        unsafe { Self::try_new_from_base(shared_data, base) }
    }

    unsafe fn try_new_from_base(
        shared_data: &SharedData,
        base: ManagedClipImageBase,
    ) -> Result<Self, StatusError> {
        let s_prop = &shared_data.property_suite.0;

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

pub struct ManagedClipImageForCuda {
    base: ManagedClipImageBase,

    data_ptr: NonNull<c_void>,
}

impl_base_functions!(ManagedClipImageForCuda);

impl ManagedClipImageForCuda {
    pub unsafe fn try_new_from_clip_if_available(
        shared_data: &SharedData,
        clip: &ImageClipInstance,
        frame: OfxTime,
    ) -> Result<Option<Self>, StatusError> {
        let base = unsafe {
            ManagedClipImageBase::try_new_from_clip_if_available(shared_data, clip, frame)
        }?;
        if let Some(b) = base {
            Ok(Some(unsafe { Self::try_new_from_base(shared_data, b) }?))
        } else {
            Ok(None)
        }
    }

    /// Construct from a clip by fetching an image at the given frame.
    pub unsafe fn try_new_from_clip(
        shared_data: &SharedData,
        clip: &ImageClipInstance,
        frame: OfxTime,
    ) -> Result<Self, StatusError> {
        let base = unsafe { ManagedClipImageBase::try_new_from_clip(shared_data, clip, frame) }?;
        unsafe { Self::try_new_from_base(shared_data, base) }
    }

    unsafe fn try_new_from_base(
        shared_data: &SharedData,
        base: ManagedClipImageBase,
    ) -> Result<Self, StatusError> {
        let s_prop = &shared_data.property_suite.0;

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

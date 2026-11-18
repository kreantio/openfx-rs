use std::ffi::c_int;

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
    sys::core::{OfxRectI, OfxTime},
};

use crate::{
    guarantee_send::GuaranteeSend, plugin_data::PluginData, sys_type_utils::rect_i_from_array,
    types::BitDepth,
};

pub mod for_cpu;
#[cfg(feature = "cuda")]
pub mod for_cuda;
#[cfg(feature = "metal")]
pub mod for_metal;
#[cfg(feature = "opencl")]
pub mod for_opencl;
#[cfg(feature = "opengl")]
pub mod for_opengl;

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
    fn try_new_from_clip_if_available(
        plugin_data: &PluginData,
        clip: &ImageClipInstance,
        frame: OfxTime,
    ) -> Result<Option<Self>, StatusError> {
        let prop_set =
            match unsafe { clip.clip_get_image(&plugin_data.image_effect_suite, frame, None) } {
                InboundResult::Ok(img) => img,
                InboundResult::KnownError(StatusError::Failed) => return Ok(None), // (perhaps) unconnected
                _ => return Err(StatusError::Failed),
            };

        let ret = unsafe { Self::try_new_from_property_set(plugin_data, prop_set) };
        if !ret.is_ok() {
            unsafe { Self::release_image(&plugin_data.image_effect_suite, &prop_set) };
        }
        ret.map(Some)
    }

    /// Construct from a clip by fetching an image at the given frame.
    fn try_new_from_clip(
        plugin_data: &PluginData,
        clip: &ImageClipInstance,
        frame: OfxTime,
    ) -> Result<Self, StatusError> {
        let prop_set = unsafe { clip.clip_get_image(&plugin_data.image_effect_suite, frame, None) }
            .ok_or(StatusError::Failed)?;

        let ret = unsafe { Self::try_new_from_property_set(plugin_data, prop_set) };
        if !ret.is_ok() {
            unsafe { Self::release_image(&plugin_data.image_effect_suite, &prop_set) };
        }
        ret
    }

    /// Construct from a property set that represents the image.
    ///
    /// ## Safety
    ///
    /// The caller must ensure that the input `image_props` is valid.
    unsafe fn try_new_from_property_set(
        plugin_data: &PluginData,
        props: ImageInstancePropertySet,
    ) -> Result<Self, StatusError> {
        let s_prop = &plugin_data.property_suite.0;

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
            image_effect_suite: plugin_data.image_effect_suite,
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
            pub fn n_comps(&self) -> ::std::ffi::c_int {
                self.base.n_comps()
            }
            pub fn pixel_depth(&self) -> crate::types::BitDepth {
                self.base.pixel_depth()
            }
            pub fn bytes_per_component(&self) -> ::std::ffi::c_int {
                self.base.bytes_per_component()
            }
            pub fn bytes_per_pixel(&self) -> ::std::ffi::c_int {
                self.base.bytes_per_pixel()
            }
            pub fn bounds(&self) -> ::openfx::sys::core::OfxRectI {
                self.base.bounds()
            }
            pub fn pixel_aspect_ratio(&self) -> ::std::primitive::f64 {
                self.base.pixel_aspect_ratio()
            }
        }
    };
}
use impl_base_functions;

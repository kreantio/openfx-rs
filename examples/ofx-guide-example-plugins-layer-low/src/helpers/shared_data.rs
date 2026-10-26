use std::{
    ffi::{c_int, c_void},
    ops::Deref,
    ptr::NonNull,
    sync::Arc,
};

use openfx::{
    low::{
        StatusError,
        enums::{ImageEffectPropComponents, ImageEffectPropPixelDepth},
    },
    low_plugin::{
        Host, LifetimePlugin,
        objects::ImageClipInstance,
        property_sets::{ImageInstancePropertySet, LosslessEnumPropertyOwned},
        suites::{ImageEffectSuiteV1, ParameterSuiteV1, PropertySuiteV1},
    },
    sys::core::OfxRectI,
};

use super::internal_utils::rect_i_from_array;

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
pub struct GuaranteeSendClipInInstance(pub ImageClipInstance);
/// ## Safety
///
/// This is safe only for the lifetime of the plugin instance from which the
/// clip instance was retrieved.
unsafe impl Send for GuaranteeSendClipInInstance {}

#[derive(Clone)]
pub struct SharedData(pub Arc<SharedDataInner>);

pub struct SharedDataInner {
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

macro_rules! param_get_value_at_time{
    (
        $parameter_suite:expr,
        $param_handle:expr,
        $time:expr,
        $(&mut $value:ident),+ $(,)?
    ) => {
        {
            let param_get_value_at_time = unsafe {
                $parameter_suite
                    .sys_ref()
                    .paramGetValueAtTime
                    .unwrap_unchecked()
            };
            let time = $time;
            #[allow(clippy::macro_metavars_in_unsafe)]
            ::openfx::low::InboundResult::from_sys(unsafe {
                param_get_value_at_time($param_handle, time, $(&mut $value),+)
            })
        }
    }
}
pub(crate) use param_get_value_at_time;

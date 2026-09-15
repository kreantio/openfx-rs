use std::{
    ffi::{CStr, c_int, c_void},
    ops::Deref,
    ptr::NonNull,
    sync::Arc,
};

use openfx::{
    low::{self, Status},
    low_plugin::{
        Host, LifetimePlugin,
        objects::{ImageClipInstance, ImageEffectDescriptor, ImageEffectInstance},
        property_sets::ImageInstancePropertySet,
        suites::{ImageEffectSuiteV1, ParameterSuiteV1, PropertySuiteV1},
    },
    sys::{
        generic::core::{
            OfxPropertySetHandle, OfxRectI, OfxTime, kOfxStatErrMissingHostFeature, kOfxStatOK,
        },
        image_effect_v1::param::{OfxParamHandle, OfxParamSetHandle},
    },
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
    pub fn try_new(host: GuaranteeSend<Host>) -> openfx::low::Result<Self> {
        let property_suite =
            unsafe { host.fetch_property_suite_v1() }.ok_or(Status::ErrMissingHostFeature)?;
        let image_effect_suite =
            unsafe { host.fetch_image_effect_suite_v1() }.ok_or(Status::ErrMissingHostFeature)?;
        let parameter_suite =
            unsafe { host.fetch_parameter_suite_v1() }.ok_or(Status::ErrMissingHostFeature)?;

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

impl SharedData {
    pub fn parameter_suite_helper(&self) -> ParameterSuiteHelper {
        ParameterSuiteHelper {
            parameter_suite: self.parameter_suite,
        }
    }

    /// ## Safety
    ///
    /// The caller must ensure that the input `effect` is valid, and will remain
    /// valid for the lifetime of the returned value.
    pub unsafe fn make_param_set_helper_for_image_effect_descriptor(
        &self,
        handle: &ImageEffectDescriptor,
    ) -> low::Result<ParamSetHelper> {
        let param_set = unsafe { handle.get_param_set(&self.image_effect_suite.0) }?;

        Ok(unsafe {
            self.parameter_suite_helper()
                .make_param_set_helper(param_set.sys_handle())
        })
    }

    /// ## Safety
    ///
    /// The caller must ensure that the input `effect` is valid, and will remain
    /// valid for the lifetime of the returned value.
    pub unsafe fn make_param_set_helper_for_image_effect_instance(
        &self,
        handle: &ImageEffectInstance,
    ) -> low::Result<ParamSetHelper> {
        let param_set = unsafe { handle.get_param_set(&self.image_effect_suite.0) }?;

        Ok(unsafe {
            self.parameter_suite_helper()
                .make_param_set_helper(param_set.sys_handle())
        })
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
    ) -> low::Result<Self> {
        let s_prop = &shared_data.property_suite.0;

        let Some(data_ptr) = (unsafe { props.get_image_data(s_prop)? }) else {
            return Err(Status::Failed);
        };

        let n_comps = match unsafe { props.get_image_effect_components(s_prop)? } {
            low::enums::ImageEffectPropComponents::Alpha => 1,
            low::enums::ImageEffectPropComponents::RGB => 3,
            low::enums::ImageEffectPropComponents::RGBA => 4,
            _ => 0,
        };
        let pixel_depth = match unsafe { props.get_image_effect_pixel_depth(s_prop)? } {
            low::enums::ImageEffectPropPixelDepth::Byte => BitDepth::Byte,
            low::enums::ImageEffectPropPixelDepth::Short => BitDepth::Short,
            low::enums::ImageEffectPropPixelDepth::Float => BitDepth::Float,
            _ => return Err(Status::ErrUnsupported),
        };
        let row_bytes = unsafe { props.get_image_row_bytes(s_prop)? };
        let bounds = unsafe { props.get_image_bounds(s_prop)? };
        let bounds = rect_i_from_array(&bounds);
        let pixel_aspect_ratio = unsafe { props.get_image_pixel_aspect_ratio(s_prop)? };

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

pub struct ParameterSuiteHelper {
    parameter_suite: GuaranteeSend<ParameterSuiteV1>,
}

impl ParameterSuiteHelper {
    pub fn inner(&self) -> &GuaranteeSend<ParameterSuiteV1> {
        &self.parameter_suite
    }

    /// ## Safety
    ///
    /// The caller must ensure that the input `handle` is valid.
    pub unsafe fn make_param_set_helper(&self, handle: OfxParamSetHandle) -> ParamSetHelper {
        ParamSetHelper {
            parameter_suite_helper: ParameterSuiteHelper {
                parameter_suite: self.parameter_suite,
            },
            param_set: handle,
        }
    }

    /// ## Safety
    ///
    /// The caller must ensure that the input `param_set` is valid.
    pub unsafe fn param_define(
        &self,
        param_set: OfxParamSetHandle,
        param_type: &CStr,
        name: &CStr,
    ) -> low::Result<OfxPropertySetHandle> {
        let param_define = unsafe {
            self.parameter_suite
                .sys_ref()
                .paramDefine
                .ok_or(kOfxStatErrMissingHostFeature)?
        };

        let mut props: OfxPropertySetHandle = std::ptr::null_mut();
        if let stat =
            (unsafe { param_define(param_set, param_type.as_ptr(), name.as_ptr(), &mut props) })
            && stat != kOfxStatOK
        {
            Err(Status::from(stat))
        } else {
            Ok(props)
        }
    }

    /// ## Safety
    ///
    /// The caller must ensure that the input `param_set` is valid.
    pub unsafe fn param_get_handle(
        &self,
        param_set: OfxParamSetHandle,
        name: &CStr,
    ) -> low::Result<OfxParamHandle> {
        let param_get_handle = unsafe {
            self.parameter_suite
                .sys_ref()
                .paramGetHandle
                .ok_or(kOfxStatErrMissingHostFeature)?
        };

        let mut props: OfxParamHandle = std::ptr::null_mut();
        if let stat = (unsafe {
            param_get_handle(param_set, name.as_ptr(), &mut props, std::ptr::null_mut())
        }) && stat != kOfxStatOK
        {
            Err(Status::from(stat))
        } else {
            Ok(props)
        }
    }

    /// ## Safety
    ///
    /// The caller must ensure that the input `param_handle` is valid.
    unsafe fn param_get_value_at_time<T>(
        &self,
        param_handle: OfxParamHandle,
        time: OfxTime,
    ) -> openfx::low::Result<T> {
        let mut value: T = unsafe { std::mem::zeroed() };
        param_get_value_at_time!(self, param_handle, time, &mut value);
        Ok(value)
    }

    /// ## Safety
    ///
    /// The caller must ensure that the input `param_handle` is valid.
    pub unsafe fn param_get_value_at_time_double(
        &self,
        param_handle: OfxParamHandle,
        time: OfxTime,
    ) -> openfx::low::Result<f64> {
        unsafe { self.param_get_value_at_time(param_handle, time) }
    }

    /// ## Safety
    ///
    /// The caller must ensure that the input `param_handle` is valid.
    pub unsafe fn param_get_value_at_time_int(
        &self,
        param_handle: OfxParamHandle,
        time: OfxTime,
    ) -> openfx::low::Result<c_int> {
        unsafe { self.param_get_value_at_time(param_handle, time) }
    }
}

/// FIXME: should give back `OfxResult` instead of using `?` internally. This
/// might require a proc macro for defining a variadic inner function.
pub macro param_get_value_at_time(
    $parameter_suite_helper:expr,
    $param_handle:expr,
    $time:expr,
    $(&mut $value:ident),+ $(,)?
) {
    {
        let param_get_value_at_time = unsafe {
            $parameter_suite_helper
                .inner()
                .sys_ref()
                .paramGetValueAtTime
                .ok_or(openfx::low::Status::ErrMissingHostFeature)?
        };
        let time = $time;
        #[allow(clippy::macro_metavars_in_unsafe)]
        if let stat = unsafe { param_get_value_at_time($param_handle, time, $(&mut $value),+) } && stat != ::openfx::sys::generic::core::kOfxStatOK {
            return Err(openfx::low::Status::from(stat));
        }
    }
}

pub struct ParamSetHelper {
    parameter_suite_helper: ParameterSuiteHelper,
    param_set: OfxParamSetHandle,
}

impl ParamSetHelper {
    #[expect(unused)]
    pub fn param_set(&self) -> OfxParamSetHandle {
        self.param_set
    }

    pub fn param_define(
        &self,
        param_type: &CStr,
        name: &CStr,
    ) -> low::Result<OfxPropertySetHandle> {
        unsafe {
            self.parameter_suite_helper
                .param_define(self.param_set, param_type, name)
        }
    }

    pub fn param_get_handle(&self, name: &CStr) -> low::Result<OfxParamHandle> {
        unsafe {
            self.parameter_suite_helper
                .param_get_handle(self.param_set, name)
        }
    }
}

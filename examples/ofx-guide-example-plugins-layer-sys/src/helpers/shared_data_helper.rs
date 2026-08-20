use std::ffi::{CStr, c_int, c_void};

use openfx::generic::sys::core::{
    OfxPropertySetHandle, OfxPropertySetStruct, OfxRectD, OfxRectI, OfxStatus, OfxTime,
    kOfxBitDepthByte, kOfxBitDepthFloat, kOfxBitDepthShort, kOfxStatErrMissingHostFeature,
    kOfxStatErrUnsupported, kOfxStatFailed, kOfxStatOK,
};
use openfx::generic::sys_helpers::properties::get_OfxPropInstanceData;
use openfx::image_effect_v1::sys::image_effect::{
    OfxImageClipHandle, OfxImageEffectHandle, OfxImageEffectSuiteV1, kOfxImageComponentAlpha,
    kOfxImageComponentRGB, kOfxImageComponentRGBA,
};
use openfx::image_effect_v1::sys::param::{OfxParamHandle, OfxParamSetHandle, OfxParameterSuiteV1};
use openfx::image_effect_v1::sys_helpers::properties::{
    get_OfxImageEffectPropComponents, get_OfxImageEffectPropPixelDepth, get_OfxImagePropBounds,
    get_OfxImagePropData, get_OfxImagePropPixelAspectRatio, get_OfxImagePropRowBytes,
};

use super::internal_utils::rect_i_from_array;

use super::SharedData;

pub struct SharedDataHelper<'data> {
    shared_data: &'data SharedData<'static>,
}

impl<'data> SharedDataHelper<'data> {
    /// ## Safety
    ///
    /// The caller must ensure that the input `shared_data` is valid and the
    /// data it holds will remain valid for the lifetime of the returned value.
    pub unsafe fn try_new(shared_data: &'data SharedData<'static>) -> Result<Self, OfxStatus> {
        Ok(Self { shared_data })
    }

    pub fn inner(&self) -> &SharedData<'static> {
        self.shared_data
    }

    pub fn image_effect_suite_helper(&self) -> ImageEffectSuiteHelper<'data> {
        ImageEffectSuiteHelper {
            image_effect_suite: self.shared_data.image_effect_suite,
        }
    }
    pub fn parameter_suite_helper(&self) -> ParameterSuiteHelper<'data> {
        ParameterSuiteHelper {
            parameter_suite: self.shared_data.parameter_suite,
        }
    }

    /// ## Safety
    ///
    /// The caller must ensure that the input `effect` is valid.
    ///
    /// The caller must ensure that the type `T` is correct.
    pub unsafe fn get_instance_data<T>(
        &self,
        effect: OfxImageEffectHandle,
    ) -> Result<&T, OfxStatus> {
        // let instance_props = unsafe { self.make_property_set_helper_for_image_effect(effect) }?;
        // let instance_data_ptr = instance_props.prop_get_pointer(kOfxPropInstanceData, 0)?;
        let props = unsafe { self.image_effect_suite_helper().get_property_set(effect) }?;
        let instance_data_ptr =
            unsafe { get_OfxPropInstanceData(self.shared_data.property_suite, props) }?;
        if instance_data_ptr.is_null() {
            return Err(kOfxStatFailed);
        }
        Ok(unsafe { &*(instance_data_ptr as *const T) })
    }

    /// ## Safety
    ///
    /// The caller must ensure that the input `effect` is valid, and will remain
    /// valid for the lifetime of the returned value.
    pub unsafe fn make_param_set_helper_for_image_effect(
        &self,
        handle: OfxImageEffectHandle,
    ) -> Result<ParamSetHelper<'data>, OfxStatus> {
        let param_set = unsafe { self.image_effect_suite_helper().get_param_set(handle) }?;

        Ok(unsafe {
            self.parameter_suite_helper()
                .make_param_set_helper(param_set)
        })
    }

    pub unsafe fn get_property_set_from_image_effect(
        &self,
        handle: OfxImageEffectHandle,
    ) -> Result<OfxPropertySetHandle, OfxStatus> {
        let mut props: *mut OfxPropertySetStruct = std::ptr::null_mut();
        let stat = unsafe {
            self.shared_data
                .image_effect_suite
                .getPropertySet
                .ok_or(kOfxStatErrMissingHostFeature)?(handle, &mut props)
        };
        if stat != kOfxStatOK {
            return Err(stat);
        }

        if props.is_null() {
            Err(kOfxStatFailed)
        } else {
            Ok(props as OfxPropertySetHandle)
        }
    }

    /// ## Safety
    ///
    /// The caller must ensure that the input `clip` is valid, and will remain
    /// valid for the lifetime of the returned reference.
    pub unsafe fn make_clip_image_managed(
        &self,
        clip: OfxImageClipHandle,
        time: OfxTime,
        region: Option<&OfxRectD>,
    ) -> Result<Option<ClipImageManaged<'data>>, OfxStatus> {
        let image_handle = unsafe {
            self.image_effect_suite_helper()
                .clip_get_image(clip, time, region)
        }?;

        unsafe { ClipImageManaged::try_new(self, image_handle) }
    }
}

pub struct ImageEffectSuiteHelper<'data> {
    image_effect_suite: &'data OfxImageEffectSuiteV1,
}

impl<'data> ImageEffectSuiteHelper<'data> {
    #[expect(unused)]
    pub fn inner(&self) -> &OfxImageEffectSuiteV1 {
        self.image_effect_suite
    }

    /// ## Safety
    ///
    /// The caller must ensure that the input `handle` is valid.
    unsafe fn get_property_set(
        &self,
        handle: OfxImageEffectHandle,
    ) -> Result<OfxPropertySetHandle, OfxStatus> {
        let get_property_set = self
            .image_effect_suite
            .getPropertySet
            .ok_or(kOfxStatErrMissingHostFeature)?;

        let mut props: *mut OfxPropertySetStruct = std::ptr::null_mut();
        if let stat = (unsafe { get_property_set(handle, &mut props) })
            && stat != kOfxStatOK
        {
            Err(stat)
        } else {
            Ok(props as OfxPropertySetHandle)
        }
    }

    /// ## Safety
    ///
    /// The caller must ensure that the input `handle` is valid.
    pub unsafe fn clip_define(
        &self,
        image_effect: OfxImageEffectHandle,
        name: &CStr,
    ) -> Result<OfxPropertySetHandle, OfxStatus> {
        let clip_define = self
            .image_effect_suite
            .clipDefine
            .ok_or(kOfxStatErrMissingHostFeature)?;

        let mut props: OfxPropertySetHandle = std::ptr::null_mut();
        if let stat = (unsafe { clip_define(image_effect, name.as_ptr(), &mut props) })
            && stat != kOfxStatOK
        {
            Err(stat)
        } else {
            Ok(props)
        }
    }

    /// ## Safety
    ///
    /// The caller must ensure that the input `image_effect` is valid.
    pub unsafe fn clip_get_handle(
        &self,
        image_effect: OfxImageEffectHandle,
        name: &CStr,
    ) -> Result<OfxImageClipHandle, OfxStatus> {
        let clip_get_handle = self
            .image_effect_suite
            .clipGetHandle
            .ok_or(kOfxStatErrMissingHostFeature)?;

        let mut clip: OfxImageClipHandle = std::ptr::null_mut();
        if let stat = (unsafe {
            clip_get_handle(image_effect, name.as_ptr(), &mut clip, std::ptr::null_mut())
        }) && stat != kOfxStatOK
        {
            Err(stat)
        } else {
            Ok(clip)
        }
    }

    /// Use [`SharedDataHelper::make_clip_image_managed`].
    ///
    /// ## Safety
    ///
    /// The caller must ensure that the input `clip` is valid.
    pub unsafe fn clip_get_image(
        &self,
        clip: OfxImageClipHandle,
        time: OfxTime,
        region: Option<&OfxRectD>,
    ) -> Result<OfxPropertySetHandle, OfxStatus> {
        let clip_get_image = self
            .image_effect_suite
            .clipGetImage
            .ok_or(kOfxStatErrMissingHostFeature)?;

        let mut image: OfxPropertySetHandle = std::ptr::null_mut();
        if let stat = (unsafe {
            clip_get_image(
                clip,
                time,
                region.map_or(std::ptr::null(), |r| r as *const OfxRectD),
                &mut image,
            )
        }) && stat != kOfxStatOK
        {
            Err(stat)
        } else {
            Ok(image)
        }
    }

    /// Use [`SharedDataHelper::make_clip_image_managed`] to get a managed image
    /// that does not require calling this function manually to release it.
    ///
    /// ## Safety
    ///
    /// The caller must ensure that `image_handle` is a valid image handle that
    /// has not been released yet, and that the image is not used after this
    /// call.
    pub unsafe fn clip_release_image(
        &self,
        image_handle: OfxPropertySetHandle,
    ) -> Result<(), OfxStatus> {
        let clip_release_image = self
            .image_effect_suite
            .clipReleaseImage
            .ok_or(kOfxStatErrMissingHostFeature)?;

        if let stat = (unsafe { clip_release_image(image_handle) })
            && stat != kOfxStatOK
        {
            Err(stat)
        } else {
            Ok(())
        }
    }

    /// ## Safety
    ///
    /// The caller must ensure that the input `image_effect` is valid.
    pub unsafe fn get_param_set(
        &self,
        image_effect: OfxImageEffectHandle,
    ) -> Result<OfxParamSetHandle, OfxStatus> {
        let get_param_set = self
            .image_effect_suite
            .getParamSet
            .ok_or(kOfxStatErrMissingHostFeature)?;

        let mut param_set: OfxParamSetHandle = std::ptr::null_mut();
        if let stat = (unsafe { get_param_set(image_effect, &mut param_set) })
            && stat != kOfxStatOK
        {
            Err(stat)
        } else {
            Ok(param_set)
        }
    }

    /// ## Safety
    ///
    /// The caller must ensure that the input `clip` is valid.
    pub unsafe fn clip_get_region_of_definition(
        &self,
        clip: OfxImageClipHandle,
        time: OfxTime,
    ) -> Result<OfxRectD, OfxStatus> {
        let clip_get_region_of_definition = self
            .image_effect_suite
            .clipGetRegionOfDefinition
            .ok_or(kOfxStatErrMissingHostFeature)?;

        let mut bounds: OfxRectD = OfxRectD {
            x1: 0.0,
            y1: 0.0,
            x2: 0.0,
            y2: 0.0,
        };
        if let stat = (unsafe { clip_get_region_of_definition(clip, time, &mut bounds) })
            && stat != kOfxStatOK
        {
            Err(stat)
        } else {
            Ok(bounds)
        }
    }
}

pub struct ClipImageManaged<'data> {
    image_effect_suite_helper: ImageEffectSuiteHelper<'data>,
    image_handle: OfxPropertySetHandle,

    n_comps: c_int,
    pixel_depth: BitDepth,
    row_bytes: c_int,
    bounds: OfxRectI,
    pixel_aspect_ratio: f64,
    data_ptr: *mut c_void,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum BitDepth {
    Byte,
    Short,
    Float,
}

impl<'data> ClipImageManaged<'data> {
    /// ## Safety
    ///
    /// The caller must ensure that the input `image_handle` is valid.
    unsafe fn try_new(
        shared_data_helper: &SharedDataHelper<'data>,
        image_handle: OfxPropertySetHandle,
    ) -> Result<Option<Self>, OfxStatus> {
        let s_prop = shared_data_helper.inner().property_suite;

        let data_ptr = unsafe { get_OfxImagePropData(s_prop, image_handle) }?;
        if data_ptr.is_null() {
            return Ok(None);
        }

        let n_comps = {
            let components = unsafe { get_OfxImageEffectPropComponents(s_prop, image_handle) }?;
            if components.is_null() {
                0
            } else {
                match unsafe { CStr::from_ptr(components) } {
                    c if c == kOfxImageComponentRGBA => 4,
                    c if c == kOfxImageComponentRGB => 3,
                    c if c == kOfxImageComponentAlpha => 1,
                    _ => 0,
                }
            }
        };
        let pixel_depth = {
            let pixel_depth = unsafe { get_OfxImageEffectPropPixelDepth(s_prop, image_handle) }?;
            if pixel_depth.is_null() {
                return Err(kOfxStatErrUnsupported);
            }
            match unsafe { CStr::from_ptr(pixel_depth) } {
                c if c == kOfxBitDepthByte => BitDepth::Byte,
                c if c == kOfxBitDepthShort => BitDepth::Short,
                c if c == kOfxBitDepthFloat => BitDepth::Float,
                _ => return Err(kOfxStatErrUnsupported),
            }
        };
        let row_bytes = unsafe { get_OfxImagePropRowBytes(s_prop, image_handle) }?;
        let bounds = unsafe { get_OfxImagePropBounds(s_prop, image_handle) }?;
        let bounds = rect_i_from_array(&bounds);
        let pixel_aspect_ratio = unsafe { get_OfxImagePropPixelAspectRatio(s_prop, image_handle) }?;

        Ok(Some(Self {
            image_effect_suite_helper: shared_data_helper.image_effect_suite_helper(),
            image_handle,

            n_comps,
            pixel_depth,
            row_bytes,
            bounds,
            pixel_aspect_ratio,
            data_ptr,
        }))
    }

    /// ## Safety
    ///
    /// The caller must ensure that the returned `OfxPropertySetHandle` will not
    /// be used after the `ClipImageManaged` instance is dropped.
    pub fn image_handle(&self) -> OfxPropertySetHandle {
        self.image_handle
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
    #[expect(unused)]
    pub fn row_bytes(&self) -> c_int {
        self.row_bytes
    }
    #[expect(unused)]
    pub fn bounds(&self) -> OfxRectI {
        self.bounds
    }
    pub fn pixel_aspect_ratio(&self) -> f64 {
        self.pixel_aspect_ratio
    }
    #[expect(unused)]
    pub fn data_ptr(&self) -> *mut c_void {
        self.data_ptr
    }

    pub fn raw_address(&self, x: c_int, y: c_int) -> Option<*mut c_void> {
        if x < self.bounds.x1 || x >= self.bounds.x2 || y < self.bounds.y1 || y >= self.bounds.y2 {
            return None;
        }

        let x_offset = x - self.bounds.x1;
        let y_offset = y - self.bounds.y1;

        let row_start = unsafe {
            (self.data_ptr as *mut u8).offset(y_offset as isize * self.row_bytes as isize)
        };

        Some(unsafe {
            row_start.offset(x_offset as isize * self.bytes_per_pixel() as isize) as *mut c_void
        })
    }
}

impl<'data> Drop for ClipImageManaged<'data> {
    fn drop(&mut self) {
        let _ = unsafe {
            self.image_effect_suite_helper
                .clip_release_image(self.image_handle)
        };
    }
}

pub struct ParameterSuiteHelper<'data> {
    parameter_suite: &'data OfxParameterSuiteV1,
}

impl<'data> ParameterSuiteHelper<'data> {
    pub fn inner(&self) -> &OfxParameterSuiteV1 {
        self.parameter_suite
    }

    /// ## Safety
    ///
    /// The caller must ensure that the input `handle` is valid.
    pub unsafe fn make_param_set_helper(&self, handle: OfxParamSetHandle) -> ParamSetHelper<'data> {
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
    ) -> Result<OfxPropertySetHandle, OfxStatus> {
        let param_define = self
            .parameter_suite
            .paramDefine
            .ok_or(kOfxStatErrMissingHostFeature)?;

        let mut props: OfxPropertySetHandle = std::ptr::null_mut();
        if let stat =
            (unsafe { param_define(param_set, param_type.as_ptr(), name.as_ptr(), &mut props) })
            && stat != kOfxStatOK
        {
            Err(stat)
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
    ) -> Result<OfxParamHandle, OfxStatus> {
        let param_get_handle = self
            .parameter_suite
            .paramGetHandle
            .ok_or(kOfxStatErrMissingHostFeature)?;

        let mut props: OfxParamHandle = std::ptr::null_mut();
        if let stat = (unsafe {
            param_get_handle(param_set, name.as_ptr(), &mut props, std::ptr::null_mut())
        }) && stat != kOfxStatOK
        {
            Err(stat)
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
    ) -> Result<T, OfxStatus> {
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
    ) -> Result<f64, OfxStatus> {
        unsafe { self.param_get_value_at_time(param_handle, time) }
    }

    /// ## Safety
    ///
    /// The caller must ensure that the input `param_handle` is valid.
    pub unsafe fn param_get_value_at_time_int(
        &self,
        param_handle: OfxParamHandle,
        time: OfxTime,
    ) -> Result<c_int, OfxStatus> {
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
        let param_get_value_at_time = $parameter_suite_helper
            .inner()
            .paramGetValueAtTime
            .ok_or(::openfx::generic::sys::core::kOfxStatErrMissingHostFeature)?;
        let time = $time;
        #[allow(clippy::macro_metavars_in_unsafe)]
        if let stat = unsafe { param_get_value_at_time($param_handle, time, $(&mut $value),+) } && stat != ::openfx::generic::sys::core::kOfxStatOK {
            return Err(stat);
        }
    }
}

pub struct ParamSetHelper<'data> {
    parameter_suite_helper: ParameterSuiteHelper<'data>,
    param_set: OfxParamSetHandle,
}

impl<'data> ParamSetHelper<'data> {
    #[expect(unused)]
    pub fn param_set(&self) -> OfxParamSetHandle {
        self.param_set
    }

    pub fn param_define(
        &self,
        param_type: &CStr,
        name: &CStr,
    ) -> Result<OfxPropertySetHandle, OfxStatus> {
        unsafe {
            self.parameter_suite_helper
                .param_define(self.param_set, param_type, name)
        }
    }

    pub fn param_get_handle(&self, name: &CStr) -> Result<OfxParamHandle, OfxStatus> {
        unsafe {
            self.parameter_suite_helper
                .param_get_handle(self.param_set, name)
        }
    }
}

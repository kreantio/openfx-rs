use std::ffi::{CStr, c_int, c_uint};

use crate::{
    low::InboundResult,
    low_plugin::{
        objects::{ParamSetDescriptor, ParamSetInstance},
        property_sets::ParamSetPropertySet,
        suites::ParameterSuiteV1,
    },
    sys_umbrella::{OfxParamHandle, OfxParamSetHandle, OfxPropertySetHandle, OfxRangeD, OfxTime},
};

/// Implementations of methods directly operating on parameter suites.
#[openfx_internal_macros::low_impl_suite]
impl ParameterSuiteV1 {
    /// See: [`crate::sys_umbrella::OfxParameterSuiteV1::paramDefine`]
    ///
    /// ## Safety
    ///
    /// `self` and `param_set` must be valid.
    ///
    /// ## TODO
    ///
    /// Enum type `kOfxParamType*`?
    pub(crate) unsafe fn param_define(
        &self,
        param_set: &ParamSetDescriptor,
        param_type: &CStr,
        name: &CStr,
    ) -> InboundResult<OfxPropertySetHandle> {
        let sys_fn = unsafe { sys_fn!() };

        let mut property_set = std::mem::MaybeUninit::<OfxPropertySetHandle>::uninit();

        InboundResult::from_sys(unsafe {
            sys_fn(
                param_set.sys_handle(),
                param_type.as_ptr(),
                name.as_ptr(),
                property_set.as_mut_ptr(),
            )
        })
        .map_ok(|()| unsafe { property_set.assume_init() })
    }

    /// See: [`crate::sys_umbrella::OfxParameterSuiteV1::paramDefine`]
    ///
    /// ## Safety
    ///
    /// `self` and `param_set` must be valid.
    ///
    /// ## TODO
    ///
    /// See [`Self::param_define`].
    pub(crate) unsafe fn param_define_no_output(
        &self,
        param_set: &ParamSetDescriptor,
        param_type: &CStr,
        name: &CStr,
    ) -> InboundResult<()> {
        let sys_fn = unsafe { sys_fn!("paramDefine") };

        InboundResult::from_sys(unsafe {
            sys_fn(
                param_set.sys_handle(),
                param_type.as_ptr(),
                name.as_ptr(),
                std::ptr::null_mut(),
            )
        })
    }

    /// See: [`crate::sys_umbrella::OfxParameterSuiteV1::paramEditBegin`]
    ///
    /// ## Safety
    ///
    /// `self` and `param_set` must be valid.
    pub(crate) unsafe fn param_edit_begin(
        &self,
        param_set: &ParamSetInstance,
        name: &CStr,
    ) -> InboundResult<()> {
        let sys_fn = unsafe { sys_fn!() };

        InboundResult::from_sys(unsafe { sys_fn(param_set.sys_handle(), name.as_ptr()) })
    }

    /// See: [`crate::sys_umbrella::OfxParameterSuiteV1::paramEditEnd`]
    ///
    /// ## Safety
    ///
    /// `self` and `param_set` must be valid.
    pub(crate) unsafe fn param_edit_end(&self, param_set: &ParamSetInstance) -> InboundResult<()> {
        let sys_fn = unsafe { sys_fn!() };

        InboundResult::from_sys(unsafe { sys_fn(param_set.sys_handle()) })
    }

    /// See: [`crate::sys_umbrella::OfxParameterSuiteV1::paramGetHandle`]
    ///
    /// ## Safety
    ///
    /// `self` and `param_set` must be valid.
    pub(crate) unsafe fn param_get_handle(
        &self,
        param_set: &ParamSetInstance,
        name: &CStr,
    ) -> InboundResult<(OfxParamHandle, OfxPropertySetHandle)> {
        let sys_fn = unsafe { sys_fn!() };

        let mut param = std::mem::MaybeUninit::<OfxParamHandle>::uninit();
        let mut property_set = std::mem::MaybeUninit::<OfxPropertySetHandle>::uninit();

        InboundResult::from_sys(unsafe {
            sys_fn(
                param_set.sys_handle(),
                name.as_ptr(),
                param.as_mut_ptr(),
                property_set.as_mut_ptr(),
            )
        })
        .map_ok(|()| unsafe { (param.assume_init(), property_set.assume_init()) })
    }

    /// See: [`crate::sys_umbrella::OfxParameterSuiteV1::paramGetHandle`]
    ///
    /// ## Safety
    ///
    /// `self` and `param_set` must be valid.
    pub(crate) unsafe fn param_get_param_handle(
        &self,
        param_set: &ParamSetInstance,
        name: &CStr,
    ) -> InboundResult<OfxParamHandle> {
        let sys_fn = unsafe { sys_fn!("paramGetHandle") };

        let mut param = std::mem::MaybeUninit::<OfxParamHandle>::uninit();

        InboundResult::from_sys(unsafe {
            sys_fn(
                param_set.sys_handle(),
                name.as_ptr(),
                param.as_mut_ptr(),
                std::ptr::null_mut(),
            )
        })
        .map_ok(|()| unsafe { param.assume_init() })
    }

    /// See: [`crate::sys_umbrella::OfxParameterSuiteV1::paramSetGetPropertySet`]
    ///
    /// ## Note
    ///
    /// `pub(super)` to satisfy the check enforced by
    /// [`openfx_internal_macros::low_make_suite_struct`].
    ///
    /// ## Safety
    ///
    /// `self` and `sys_param_set` must be valid.
    pub(super) unsafe fn param_set_get_property_set(
        &self,
        sys_param_set: OfxParamSetHandle,
    ) -> InboundResult<OfxPropertySetHandle> {
        let sys_fn = unsafe { sys_fn!() };

        let mut prop_handle = std::mem::MaybeUninit::<OfxPropertySetHandle>::uninit();

        InboundResult::from_sys(unsafe { sys_fn(sys_param_set, prop_handle.as_mut_ptr()) })
            .map_ok(|()| unsafe { prop_handle.assume_init() })
    }

    /// See: [`crate::sys_umbrella::OfxParameterSuiteV1::paramSetGetPropertySet`]
    ///
    /// ## Safety
    ///
    /// `self` and `param_set` must be valid.
    #[no_sys_fn]
    pub(crate) unsafe fn param_set_get_descriptor_property_set(
        &self,
        param_set: &ParamSetDescriptor,
    ) -> InboundResult<ParamSetPropertySet> {
        unsafe {
            self.param_set_get_property_set(param_set.sys_handle())
                .map_ok(ParamSetPropertySet::from)
        }
    }

    /// See: [`crate::sys_umbrella::OfxParameterSuiteV1::paramSetGetPropertySet`]
    ///
    /// ## Safety
    ///
    /// `self` and `param_set` must be valid.
    #[no_sys_fn]
    pub(crate) unsafe fn param_set_get_instance_property_set(
        &self,
        param_set: &ParamSetInstance,
    ) -> InboundResult<ParamSetPropertySet> {
        unsafe {
            self.param_set_get_property_set(param_set.sys_handle())
                .map_ok(ParamSetPropertySet::from)
        }
    }
}

/// ## Note
///
/// Unlike most of the suites in [`crate::low_plugin::suites`], methods that
/// directly operate on parameters (not parameter sets) in this suite:
///
/// - Operate directly on [`OfxParamHandle`]s from [`crate::sys`] rather than
///   wrapped objects from [`crate::low_plugin::objects`], since the wrappers
///   for parameters WILL live in [`crate::low_plugin::parameters`], and it
///   doesn't make sense to create these methods for each property set wrapper
///   individually.
/// - Are `pub` instead of `pub(crate)`, allowing library users to call them
///   freely when the parameter type that they want to interact with is not
///   available in [`crate::low_plugin::parameters`].
///
/// TO REVIEWERS: remind me if the statements above need updating.
#[openfx_internal_macros::low_impl_suite]
impl ParameterSuiteV1 {
    /// See: [`crate::sys_umbrella::OfxParameterSuiteV1::paramCopy`]
    ///
    /// ## Safety
    ///
    /// `self`, `sys_param_to` and `sys_param_from` must be valid.
    pub unsafe fn param_copy(
        &self,
        sys_param_to: OfxParamHandle,
        sys_param_from: OfxParamHandle,
        dst_offset: OfxTime,
        frame_range: Option<&OfxRangeD>,
    ) -> InboundResult<()> {
        let sys_fn = unsafe { sys_fn!() };

        InboundResult::from_sys(unsafe {
            sys_fn(
                sys_param_to,
                sys_param_from,
                dst_offset,
                frame_range.map_or(std::ptr::null(), |r| r),
            )
        })
    }

    /// See: [`crate::sys_umbrella::OfxParameterSuiteV1::paramDeleteAllKeys`]
    ///
    /// ## Safety
    ///
    /// `self` and `sys_param_handle` must be valid.
    pub unsafe fn param_delete_all_keys(
        &self,
        sys_param_handle: OfxParamHandle,
    ) -> InboundResult<()> {
        let sys_fn = unsafe { sys_fn!() };

        InboundResult::from_sys(unsafe { sys_fn(sys_param_handle) })
    }

    /// See: [`crate::sys_umbrella::OfxParameterSuiteV1::paramDeleteKey`]
    ///
    /// ## Safety
    ///
    /// `self` and `sys_param_handle` must be valid.
    pub unsafe fn param_delete_key(
        &self,
        sys_param_handle: OfxParamHandle,
        time: OfxTime,
    ) -> InboundResult<()> {
        let sys_fn = unsafe { sys_fn!() };

        InboundResult::from_sys(unsafe { sys_fn(sys_param_handle, time) })
    }

    /// See: [`crate::sys_umbrella::OfxParameterSuiteV1::paramGetKeyIndex`]
    ///
    /// ## Safety
    ///
    /// `self` and `sys_param_handle` must be valid.
    pub unsafe fn param_get_key_index(
        &self,
        sys_param_handle: OfxParamHandle,
        time: OfxTime,
        direction: c_int,
    ) -> InboundResult<c_int> {
        let sys_fn = unsafe { sys_fn!() };

        let mut index = std::mem::MaybeUninit::<c_int>::uninit();

        InboundResult::from_sys(unsafe {
            sys_fn(sys_param_handle, time, direction, index.as_mut_ptr())
        })
        .map_ok(|()| unsafe { index.assume_init() })
    }

    /// See: [`crate::sys_umbrella::OfxParameterSuiteV1::paramGetKeyTime`]
    ///
    /// ## Safety
    ///
    /// `self` and `sys_param_handle` must be valid.
    pub unsafe fn param_get_key_time(
        &self,
        sys_param_handle: OfxParamHandle,
        nth_key: c_uint,
    ) -> InboundResult<OfxTime> {
        let sys_fn = unsafe { sys_fn!() };

        let mut time = std::mem::MaybeUninit::<OfxTime>::uninit();

        InboundResult::from_sys(unsafe { sys_fn(sys_param_handle, nth_key, time.as_mut_ptr()) })
            .map_ok(|()| unsafe { time.assume_init() })
    }

    /// See: [`crate::sys_umbrella::OfxParameterSuiteV1::paramGetNumKeys`]
    ///
    /// ## Safety
    ///
    /// `self` and `sys_param_handle` must be valid.
    pub unsafe fn param_get_num_keys(
        &self,
        sys_param_handle: OfxParamHandle,
    ) -> InboundResult<c_uint> {
        let sys_fn = unsafe { sys_fn!() };

        let mut num_keys = std::mem::MaybeUninit::<c_uint>::uninit();

        InboundResult::from_sys(unsafe { sys_fn(sys_param_handle, num_keys.as_mut_ptr()) })
            .map_ok(|()| unsafe { num_keys.assume_init() })
    }

    /// See: [`crate::sys_umbrella::OfxParameterSuiteV1::paramGetPropertySet`]
    ///
    /// ## Safety
    ///
    /// `self` and `sys_param` must be valid.
    pub unsafe fn param_get_property_set(
        &self,
        sys_param: OfxParamHandle,
    ) -> InboundResult<OfxPropertySetHandle> {
        let sys_fn = unsafe { sys_fn!() };

        let mut prop_handle = std::mem::MaybeUninit::<OfxPropertySetHandle>::uninit();

        InboundResult::from_sys(unsafe { sys_fn(sys_param, prop_handle.as_mut_ptr()) })
            .map_ok(|()| unsafe { prop_handle.assume_init() })
    }
}

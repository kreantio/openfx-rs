use std::ffi::{CStr, c_int, c_uint};

use crate::{
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
    ) -> crate::low::Result<OfxPropertySetHandle> {
        let sys_fn = unsafe { sys_fn!() };

        let mut property_set = std::mem::MaybeUninit::<OfxPropertySetHandle>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(
                param_set.sys_handle(),
                param_type.as_ptr(),
                name.as_ptr(),
                property_set.as_mut_ptr(),
            )
        })?;

        Ok(unsafe { property_set.assume_init() })
    }

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
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!("paramDefine") };

        crate::low::Status::result_from(unsafe {
            sys_fn(
                param_set.sys_handle(),
                param_type.as_ptr(),
                name.as_ptr(),
                std::ptr::null_mut(),
            )
        })
    }

    /// ## Safety
    ///
    /// `self` and `param_set` must be valid.
    pub(crate) unsafe fn param_edit_begin(
        &self,
        param_set: &ParamSetInstance,
        name: &CStr,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(param_set.sys_handle(), name.as_ptr()) })
    }

    /// ## Safety
    ///
    /// `self` and `param_set` must be valid.
    pub(crate) unsafe fn param_edit_end(
        &self,
        param_set: &ParamSetInstance,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(param_set.sys_handle()) })
    }

    /// ## Safety
    ///
    /// `self` and `param_set` must be valid.
    pub(crate) unsafe fn param_get_handle(
        &self,
        param_set: &ParamSetInstance,
        name: &CStr,
    ) -> crate::low::Result<(OfxParamHandle, OfxPropertySetHandle)> {
        let sys_fn = unsafe { sys_fn!() };

        let mut param = std::mem::MaybeUninit::<OfxParamHandle>::uninit();
        let mut property_set = std::mem::MaybeUninit::<OfxPropertySetHandle>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(
                param_set.sys_handle(),
                name.as_ptr(),
                param.as_mut_ptr(),
                property_set.as_mut_ptr(),
            )
        })?;

        Ok(unsafe { (param.assume_init(), property_set.assume_init()) })
    }

    /// ## Safety
    ///
    /// `self` and `param_set` must be valid.
    #[expect(unused)]
    pub(crate) unsafe fn param_get_param_handle(
        &self,
        param_set: &ParamSetInstance,
        name: &CStr,
    ) -> crate::low::Result<OfxParamHandle> {
        let sys_fn = unsafe { sys_fn!("paramGetHandle") };

        let mut param = std::mem::MaybeUninit::<OfxParamHandle>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(
                param_set.sys_handle(),
                name.as_ptr(),
                param.as_mut_ptr(),
                std::ptr::null_mut(),
            )
        })?;

        Ok(unsafe { param.assume_init() })
    }

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
    ) -> crate::low::Result<OfxPropertySetHandle> {
        let sys_fn = unsafe { sys_fn!() };

        let mut prop_handle = std::mem::MaybeUninit::<OfxPropertySetHandle>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(sys_param_set, prop_handle.as_mut_ptr())
        })?;

        Ok(unsafe { prop_handle.assume_init() })
    }

    /// ## Safety
    ///
    /// `self` and `param_set` must be valid.
    #[no_sys_fn]
    pub(crate) unsafe fn param_set_get_descriptor_property_set(
        &self,
        param_set: &ParamSetDescriptor,
    ) -> crate::low::Result<ParamSetPropertySet> {
        Ok(ParamSetPropertySet::from(unsafe {
            self.param_set_get_property_set(param_set.sys_handle())?
        }))
    }

    /// ## Safety
    ///
    /// `self` and `param_set` must be valid.
    #[no_sys_fn]
    pub(crate) unsafe fn param_set_get_instance_property_set(
        &self,
        param_set: &ParamSetInstance,
    ) -> crate::low::Result<ParamSetPropertySet> {
        Ok(ParamSetPropertySet::from(unsafe {
            self.param_set_get_property_set(param_set.sys_handle())?
        }))
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
    /// ## Safety
    ///
    /// `self`, `sys_param_to` and `sys_param_from` must be valid.
    pub unsafe fn param_copy(
        &self,
        sys_param_to: OfxParamHandle,
        sys_param_from: OfxParamHandle,
        dst_offset: OfxTime,
        frame_range: Option<&OfxRangeD>,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe {
            sys_fn(
                sys_param_to,
                sys_param_from,
                dst_offset,
                frame_range.map_or(std::ptr::null(), |r| r),
            )
        })
    }

    /// ## Safety
    ///
    /// `self` and `sys_param_handle` must be valid.
    pub unsafe fn param_delete_all_keys(
        &self,
        sys_param_handle: OfxParamHandle,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(sys_param_handle) })
    }

    /// ## Safety
    ///
    /// `self` and `sys_param_handle` must be valid.
    pub unsafe fn param_delete_key(
        &self,
        sys_param_handle: OfxParamHandle,
        time: OfxTime,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(sys_param_handle, time) })
    }

    /// ## Safety
    ///
    /// `self` and `sys_param_handle` must be valid.
    pub unsafe fn param_get_key_index(
        &self,
        sys_param_handle: OfxParamHandle,
        time: OfxTime,
        direction: c_int,
    ) -> crate::low::Result<c_int> {
        let sys_fn = unsafe { sys_fn!() };

        let mut index = std::mem::MaybeUninit::<c_int>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(sys_param_handle, time, direction, index.as_mut_ptr())
        })?;

        Ok(unsafe { index.assume_init() })
    }

    /// ## Safety
    ///
    /// `self` and `sys_param_handle` must be valid.
    pub unsafe fn param_get_key_time(
        &self,
        sys_param_handle: OfxParamHandle,
        nth_key: c_uint,
    ) -> crate::low::Result<OfxTime> {
        let sys_fn = unsafe { sys_fn!() };

        let mut time = std::mem::MaybeUninit::<OfxTime>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(sys_param_handle, nth_key, time.as_mut_ptr())
        })?;

        Ok(unsafe { time.assume_init() })
    }

    /// ## Safety
    ///
    /// `self` and `sys_param_handle` must be valid.
    pub unsafe fn param_get_num_keys(
        &self,
        sys_param_handle: OfxParamHandle,
    ) -> crate::low::Result<c_uint> {
        let sys_fn = unsafe { sys_fn!() };

        let mut num_keys = std::mem::MaybeUninit::<c_uint>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(sys_param_handle, num_keys.as_mut_ptr())
        })?;

        Ok(unsafe { num_keys.assume_init() })
    }

    /// ## Safety
    ///
    /// `self` and `sys_param` must be valid.
    pub unsafe fn param_get_property_set(
        &self,
        sys_param: OfxParamHandle,
    ) -> crate::low::Result<OfxPropertySetHandle> {
        let sys_fn = unsafe { sys_fn!() };

        let mut prop_handle = std::mem::MaybeUninit::<OfxPropertySetHandle>::uninit();

        crate::low::Status::result_from(unsafe { sys_fn(sys_param, prop_handle.as_mut_ptr()) })?;

        Ok(unsafe { prop_handle.assume_init() })
    }
}

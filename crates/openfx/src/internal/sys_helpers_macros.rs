pub(crate) macro type_ident_to_type {
    (Int, $_:tt) => { std::os::raw::c_int },
    (Double, $_:tt) => { f64 },
    (String, set) => { *const std::os::raw::c_char },
    (String, get) => { *mut std::os::raw::c_char },
    (Pointer, $_:tt) => { *mut std::ffi::c_void },
}
pub(crate) macro get_suite_fn_by_type_ident {
    ($suite:ident, Int, set, 1) => { (&*$suite).propSetInt.unwrap_unchecked() },
    ($suite:ident, Int, set, ...) => { (&*$suite).propSetIntN.unwrap_unchecked() },
    ($suite:ident, Int, get, 1) => { (&*$suite).propGetInt.unwrap_unchecked() },
    ($suite:ident, Int, get, ...) => { (&*$suite).propGetIntN.unwrap_unchecked() },
    ($suite:ident, Double, set, 1) => { (&*$suite).propSetDouble.unwrap_unchecked() },
    ($suite:ident, Double, set, ...) => { (&*$suite).propSetDoubleN.unwrap_unchecked() },
    ($suite:ident, Double, get, 1) => { (&*$suite).propGetDouble.unwrap_unchecked() },
    ($suite:ident, Double, get, ...) => { (&*$suite).propGetDoubleN.unwrap_unchecked() },
    ($suite:ident, String, set, 1) => { (&*$suite).propSetString.unwrap_unchecked() },
    ($suite:ident, String, set, ...) => { (&*$suite).propSetStringN.unwrap_unchecked() },
    ($suite:ident, String, get, 1) => { (&*$suite).propGetString.unwrap_unchecked() },
    ($suite:ident, String, get, ...) => { (&*$suite).propGetStringN.unwrap_unchecked() },
    ($suite:ident, Pointer, set, 1) => { (&*$suite).propSetPointer.unwrap_unchecked() },
    ($suite:ident, Pointer, set, ...) => { (&*$suite).propSetPointerN.unwrap_unchecked() },
    ($suite:ident, Pointer, get, 1) => { (&*$suite).propGetPointer.unwrap_unchecked() },
    ($suite:ident, Pointer, get, ...) => { (&*$suite).propGetPointerN.unwrap_unchecked() },
}

pub macro make_property_setter_for_type {
    (pub $name:ident, 1, $type:ident) => {
        /// ## SAFETY
        ///
        /// - `suite` must be a valid pointer to
        ///   [`crate::sys_umbrella::OfxPropertySuiteV1`].
        /// - `handle` must be a valid handle of
        ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
        /// - The type of `property`'s value must match the type this function
        ///   is specialized for.
        #[inline(always)]
        pub unsafe fn $name(
            suite: *const crate::sys_umbrella::OfxPropertySuiteV1,
            handle: crate::sys_umbrella::OfxPropertySetHandle,
            property: *const std::os::raw::c_char,
            value: type_ident_to_type!($type, set),
        ) -> Result<(), crate::sys_umbrella::OfxStatus> {
            // SAFETY: granted by the standard
            let suite_fn = unsafe { get_suite_fn_by_type_ident!(suite, $type, set, 1) };
            if let s = unsafe { suite_fn(handle, property, 0, value) }
                && s != crate::sys_umbrella::kOfxStatOK {
                Err(s)
            } else {
                Ok(())
            }
        }
    },
    (pub(crate) $name:ident, $n:literal, $type:ident) => {
        /// ## SAFETY
        ///
        /// - `suite` must be a valid pointer to
        ///   [`crate::sys_umbrella::OfxPropertySuiteV1`].
        /// - `handle` must be a valid handle of
        ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
        /// - The type of `property`'s value must match the type this function
        ///   is specialized for.
        #[inline(always)]
        pub(crate) unsafe fn $name(
            suite: *const crate::sys_umbrella::OfxPropertySuiteV1,
            handle: crate::sys_umbrella::OfxPropertySetHandle,
            property: *const std::os::raw::c_char,
            values: [type_ident_to_type!($type, set); $n],
        ) -> Result<(), crate::sys_umbrella::OfxStatus> {
            // SAFETY: granted by the standard
            let suite_fn = unsafe { get_suite_fn_by_type_ident!(suite, $type, set, ...) };
            if let s = unsafe { suite_fn(handle, property, $n, values.as_ptr()) }
                && s != crate::sys_umbrella::kOfxStatOK {
                Err(s)
            } else {
                Ok(())
            }
        }
    },
    (pub $name:ident, ..., $type:ident) => {
        /// ## SAFETY
        ///
        /// - `suite` must be a valid pointer to
        ///   [`crate::sys_umbrella::OfxPropertySuiteV1`].
        /// - `handle` must be a valid handle of
        ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
        /// - The type of `property`'s value must match the type this function
        ///   is specialized for.
        #[inline(always)]
        pub unsafe fn $name(
            suite: *const crate::sys_umbrella::OfxPropertySuiteV1,
            handle: crate::sys_umbrella::OfxPropertySetHandle,
            property: *const std::os::raw::c_char,
            values: &[type_ident_to_type!($type, set)],
        ) -> Result<(), crate::sys_umbrella::OfxStatus> {
            // SAFETY: granted by the standard
            let suite_fn = unsafe { get_suite_fn_by_type_ident!(suite, $type, set, ...) };
            let count = values.len() as std::os::raw::c_int;
            if let s = unsafe { suite_fn(handle, property, count, values.as_ptr()) }
                && s != crate::sys_umbrella::kOfxStatOK {
                Err(s)
            } else {
                Ok(())
            }
        }
    },
}

pub macro make_property_getter_for_type {
    (pub $name:ident, 1, $type:ident) => {
        /// ## SAFETY
        ///
        /// - `suite` must be a valid pointer to
        ///   [`crate::sys_umbrella::OfxPropertySuiteV1`].
        /// - `handle` must be a valid handle of
        ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
        /// - The type of `property`'s value must match the type this function
        ///   is specialized for.
        #[inline(always)]
        pub unsafe fn $name(
            suite: *const crate::sys_umbrella::OfxPropertySuiteV1,
            handle: crate::sys_umbrella::OfxPropertySetHandle,
            property: *const std::os::raw::c_char,
        ) -> Result<type_ident_to_type!($type, get), crate::sys_umbrella::OfxStatus> {
            // SAFETY: granted by the standard
            let suite_fn = unsafe { get_suite_fn_by_type_ident!(suite, $type, get, 1) };
            let mut value: type_ident_to_type!($type, get) = <type_ident_to_type!($type, get)>::default();
            if let s = unsafe { suite_fn(handle, property, 0, &mut value) }
                && s != crate::sys_umbrella::kOfxStatOK {
                Err(s)
            } else {
                Ok(value)
            }
        }
    },
    (pub(crate) $name:ident, $n:literal, $type:ident) => {
        /// ## SAFETY
        ///
        /// - `suite` must be a valid pointer to
        ///   [`crate::sys_umbrella::OfxPropertySuiteV1`].
        /// - `handle` must be a valid handle of
        ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
        /// - The type of `property`'s value must match the type this function
        ///   is specialized for.
        #[inline(always)]
        pub(crate) unsafe fn $name(
            suite: *const crate::sys_umbrella::OfxPropertySuiteV1,
            handle: crate::sys_umbrella::OfxPropertySetHandle,
            property: *const std::os::raw::c_char,
        ) -> Result<[type_ident_to_type!($type, get); $n], crate::sys_umbrella::OfxStatus> {
            // SAFETY: granted by the standard
            let suite_fn = unsafe { get_suite_fn_by_type_ident!(suite, $type, get, ...) };
            let mut value: [type_ident_to_type!($type, get); $n] = [<type_ident_to_type!($type, get)>::default(); $n];
            if let s = unsafe { suite_fn(handle, property, $n, value.as_mut_ptr()) }
                && s != crate::sys_umbrella::kOfxStatOK {
                Err(s)
            } else {
                Ok(value)
            }
        }
    },
    (pub $name:ident, ..., $type:ident) => {
        /// ## SAFETY
        ///
        /// - `suite` must be a valid pointer to
        ///   [`crate::sys_umbrella::OfxPropertySuiteV1`].
        /// - `handle` must be a valid handle of
        ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
        /// - The type of `property`'s value must match the type this function
        ///   is specialized for.
        #[inline(always)]
        pub unsafe fn $name(
            suite: *const crate::sys_umbrella::OfxPropertySuiteV1,
            handle: crate::sys_umbrella::OfxPropertySetHandle,
            property: *const std::os::raw::c_char,
            values: &mut [type_ident_to_type!($type, get)],
        ) -> Result<(), crate::sys_umbrella::OfxStatus> {
            // SAFETY: granted by the standard
            let suite_fn = unsafe { get_suite_fn_by_type_ident!(suite, $type, get, ...) };
            let count = values.len() as std::os::raw::c_int;
            if let s = unsafe { suite_fn(handle, property, count, values.as_mut_ptr()) }
                && s != crate::sys_umbrella::kOfxStatOK {
                Err(s)
            } else {
                Ok(())
            }
        }
    },
}

pub macro make_property_setter {
    (@, $name:ident, $key_name:ident, $fn_name:path, $value_type:ty, $type:ident) => {
        /// ## SAFETY
        ///
        /// - `suite` must be a valid pointer to
        ///   [`crate::sys_umbrella::OfxPropertySuiteV1`].
        /// - `handle` must be a valid handle of
        ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
        #[inline(always)]
        #[allow(non_snake_case)]
        pub unsafe fn $name(
            suite: *const crate::sys_umbrella::OfxPropertySuiteV1,
            handle: crate::sys_umbrella::OfxPropertySetHandle,
            value: $value_type,
        ) -> Result<(), crate::sys_umbrella::OfxStatus> {
            // SAFETY: Type safety is guaranteed by the standard this function is
            // derived from. The caller guarantees the remaining safety
            // requirements.
            unsafe {
                $fn_name(
                    suite,
                    handle,
                    crate::sys_umbrella::$key_name.as_ptr(),
                    value,
                )
            }
        }
    },
    ($name:ident, $key_name:ident, $fn_name:path, 1, $type:ident) => {
        make_property_setter!(@, $name, $key_name, $fn_name, type_ident_to_type!($type, set), $type);
    },
    ($name:ident, $key_name:ident, $fn_name:path, $n:literal, $type:ident) => {
        make_property_setter!(@, $name, $key_name, $fn_name, [type_ident_to_type!($type, set); $n], $type);
    },
    ($name:ident, $key_name:ident, $fn_name:path, ..., $type:ident) => {
        make_property_setter!(@, $name, $key_name, $fn_name, &[type_ident_to_type!($type, set)], $type);
    },
}

pub macro make_property_getter {
    (@fixed, $name:ident, $key_name:ident, $fn_name:path, $value_type:ty, $type:ident) => {
        /// ## SAFETY
        ///
        /// - `suite` must be a valid pointer to
        ///   [`crate::sys_umbrella::OfxPropertySuiteV1`].
        /// - `handle` must be a valid handle of
        ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
        #[inline(always)]
        #[allow(non_snake_case)]
        pub unsafe fn $name(
            suite: *const crate::sys_umbrella::OfxPropertySuiteV1,
            handle: crate::sys_umbrella::OfxPropertySetHandle,
        ) -> Result<$value_type, crate::sys_umbrella::OfxStatus> {
            // SAFETY: Type safety is guaranteed by the standard this function
            // is derived from. The caller guarantees the remaining safety
            // requirements.
            unsafe {
                $fn_name(
                    suite,
                    handle,
                    crate::sys_umbrella::$key_name.as_ptr(),
                )
            }
        }
    },
    (@..., $name:ident, $key_name:ident, $fn_name:path, $value_type:ty, $type:ident) => {
        /// ## SAFETY
        ///
        /// - `suite` must be a valid pointer to
        ///   [`crate::sys_umbrella::OfxPropertySuiteV1`].
        /// - `handle` must be a valid handle of
        ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
        #[inline(always)]
        #[allow(non_snake_case)]
        pub unsafe fn $name(
            suite: *const crate::sys_umbrella::OfxPropertySuiteV1,
            handle: crate::sys_umbrella::OfxPropertySetHandle,
            values: &mut $value_type,
        ) -> Result<(), crate::sys_umbrella::OfxStatus> {
            // SAFETY: Type safety is guaranteed by the standard this function
            // is derived from. The caller guarantees the remaining safety
            // requirements.
            unsafe {
                $fn_name(
                    suite,
                    handle,
                    crate::sys_umbrella::$key_name.as_ptr(),
                    values,
                )
            }
        }
    },
    ($name:ident, $key_name:ident, $fn_name:path, 1, $type:ident) => {
        make_property_getter!(@fixed, $name, $key_name, $fn_name, type_ident_to_type!($type, get), $type);
    },
    ($name:ident, $key_name:ident, $fn_name:path, $n:literal, $type:ident) => {
        make_property_getter!(@fixed, $name, $key_name, $fn_name, [type_ident_to_type!($type, get); $n], $type);
    },
    ($name:ident, $key_name:ident, $fn_name:path, ..., $type:ident) => {
        make_property_getter!(@..., $name, $key_name, $fn_name, [type_ident_to_type!($type, get)], $type);
    },
}

pub macro make_property_resetter {
    ($name:ident, $key_name:ident) => {
        /// ## SAFETY
        ///
        /// - `suite` must be a valid pointer to
        ///   [`crate::sys_umbrella::OfxPropertySuiteV1`].
        /// - `handle` must be a valid handle of
        ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
        #[inline(always)]
        #[allow(non_snake_case)]
        pub unsafe fn $name(
            suite: *const crate::sys_umbrella::OfxPropertySuiteV1,
            handle: crate::sys_umbrella::OfxPropertySetHandle,
        ) -> Result<(), crate::sys_umbrella::OfxStatus> {
            unsafe {
                crate::generic::sys_helpers::properties::reset_property(
                    suite,
                    handle,
                    crate::sys_umbrella::$key_name.as_ptr(),
                )
            }
        }
    },
}

pub macro make_property_dimension_getter {
    ($name:ident, $key_name:ident) => {
        /// ## SAFETY
        ///
        /// - `suite` must be a valid pointer to
        ///   [`crate::sys_umbrella::OfxPropertySuiteV1`].
        /// - `handle` must be a valid handle of
        ///   [`crate::sys_umbrella::OfxPropertySetHandle`].
        #[inline(always)]
        #[allow(non_snake_case)]
        pub unsafe fn $name(
            suite: *const crate::sys_umbrella::OfxPropertySuiteV1,
            handle: crate::sys_umbrella::OfxPropertySetHandle,
        ) -> Result<std::os::raw::c_int, crate::sys_umbrella::OfxStatus> {
            unsafe {
                crate::generic::sys_helpers::properties::get_property_dimension(
                    suite,
                    handle,
                    crate::sys_umbrella::$key_name.as_ptr(),
                )
            }
        }
    },
}

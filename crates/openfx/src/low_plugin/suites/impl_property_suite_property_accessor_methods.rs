#![allow(unused_parens, reason = "false positive")]

use std::{
    ffi::{CStr, CString, c_char, c_int, c_void},
    ptr::NonNull,
};

use crate::{low_plugin::suites::PropertySuiteV1, sys_umbrella::OfxPropertySetHandle};

/// ## Note
///
/// Unlike other suites in [`crate::low_plugin::suites`], methods in this suite:
///
/// - Operate directly on `OfxPropertySetHandle`s from [`crate::sys`] rather than
///   wrapped objects from [`crate::low_plugin::objects`], since the wrappers for
///   property sets live in [`crate::low_plugin::property_sets`], and it doesn't
///   make sense to create these methods for each property set wrapper
///   individually.
/// - Are `pub` instead of `pub(crate)`, allowing library users to call them
///   freely rather than going through [`crate::low_plugin::property_sets`].
#[openfx_internal_macros::low_impl_suite]
impl PropertySuiteV1 {
    /// ## Safety
    ///
    /// `self` and `properties` must be valid.
    pub unsafe fn prop_get_dimension(
        &self,
        properties: OfxPropertySetHandle,
        property: &CStr,
    ) -> crate::low::Result<c_int> {
        let sys_fn = unsafe { sys_fn!() };

        let mut count = std::mem::MaybeUninit::<c_int>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(properties, property.as_ptr(), count.as_mut_ptr())
        })?;

        Ok(unsafe { count.assume_init() })
    }

    /// ## Safety
    ///
    /// `self` and `properties` must be valid.
    pub unsafe fn prop_reset(
        &self,
        properties: OfxPropertySetHandle,
        property: &CStr,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(properties, property.as_ptr()) })
    }
}

macro_rules! impl_accessor_methods {
    (@doc) => {
        r#"## Safety

 - `self.0` must be a valid pointer to
   [`crate::sys_umbrella::OfxPropertySuiteV1`].
 - `properties` must be a valid handle of
   [`crate::sys_umbrella::OfxPropertySetHandle`].
 - The type of `property`'s value must match the type this function
   is specialized for."#
    };
    (@convert_fn @sys_get_to_low { sys_get_to_low = { $sys_get_to_low:expr }, low_to_sys_set = { $low_to_sys_set:expr }, }) => {
        $sys_get_to_low
    };
    (@convert_fn @low_to_sys_set { sys_get_to_low = { $sys_get_to_low:expr }, low_to_sys_set = { $low_to_sys_set:expr }, }) => {
        $low_to_sys_set
    };
    (@convert[$($_:tt)*] false $value:ident) => {
        $value
    };
    (@convert[sys_get_to_low,scalar] $c:tt $value:ident) => {
        impl_accessor_methods!(@convert_fn @sys_get_to_low $c)($value)
    };
    (@convert[low_to_sys_set,scalar] $c:tt $value:ident) => {
        impl_accessor_methods!(@convert_fn @low_to_sys_set $c)($value)
    };
    (@convert[sys_get_to_low,array] $c:tt $value:ident) => {
        $value.map(impl_accessor_methods!(@convert_fn @sys_get_to_low $c))
    };
    (@convert[low_to_sys_set,array] $c:tt $value:ident) => {
        $value.map(impl_accessor_methods!(@convert_fn @low_to_sys_set $c))
    };
    (@convert:pre[sys_get_to_low,slice] false values=$values:ident tmp_values=$tmp_values:ident ty_get=$ty_get:ty) => {
        let $tmp_values = $values;
    };
    (@convert:post[sys_get_to_low,slice] false values=$values:ident tmp_values=$tmp_values:ident) => {};
    (@convert:pre[sys_get_to_low,slice] $c:tt values=$values:ident tmp_values=$tmp_values:ident ty_get=$ty_get:ty) => {
        let mut $tmp_values = Vec::<$ty_get>::with_capacity($values.len());
        unsafe { $tmp_values.set_len($values.len()); }
    };
    (@convert:post[sys_get_to_low,slice] $c:tt values=$values:ident tmp_values=$tmp_values:ident) => {
        for (i, tmp) in $tmp_values.iter().enumerate() {
            ($values)[i] = impl_accessor_methods!(@convert_fn @sys_get_to_low $c)(*tmp);
        }
    };
    (@convert[low_to_sys_set,slice] $c:tt $value:ident) => {
        $value.iter().copied().map(impl_accessor_methods!(@convert_fn @low_to_sys_set $c)).collect::<Vec<_>>()
    };
    (
        type = { sys_get = $ty_get:ty, sys_set = $ty_set:ty, low = $ty_low:ty },
        names = {
            get = $name_get:ident, get_at = $name_get_at:ident, get_array = $name_get_array:ident, get_all = $name_get_all:ident,
            set = $name_set:ident, set_at = $name_set_at:ident, set_array = $name_set_array:ident, set_all = $name_set_all:ident,
        },
        sys_fn_names = {
            get = $get_sys_fn:literal, get_n = $get_n_sys_fn:literal,
            set = $set_sys_fn:literal, set_n = $set_n_sys_fn:literal,
        },
        conversions = $conversions:tt,
    ) => {
        impl_accessor_methods!(
            @get
            type = { sys_get = $ty_get, low = $ty_low },
            names = {
                get = $name_get, get_at = $name_get_at, get_array = $name_get_array, get_all = $name_get_all,
            },
            sys_fn_names = {
                get = $get_sys_fn, get_n = $get_n_sys_fn,
            },
            conversions = $conversions,
        );
        impl_accessor_methods!(
            @set
            type = { sys_set = $ty_set, low = $ty_low },
            names = {
                set = $name_set, set_at = $name_set_at, set_array = $name_set_array, set_all = $name_set_all,
            },
            sys_fn_names = {
                set = $set_sys_fn, set_n = $set_n_sys_fn,
            },
            conversions = $conversions,
        );
    };
    (
        @get
        type = { sys_get = $ty_get:ty, low = $ty_low:ty },
        names = {
            get = $name_get:ident, get_at = $name_get_at:ident, get_array = $name_get_array:ident, get_all = $name_get_all:ident,
        },
        sys_fn_names = {
            get = $get_sys_fn:literal, get_n = $get_n_sys_fn:literal,
        },
        conversions = $conversions:tt,
    ) => {
        #[openfx_internal_macros::low_impl_suite]
        impl PropertySuiteV1 {
            #[doc = impl_accessor_methods!(@doc)]
            #[no_sys_fn]
            pub unsafe fn $name_get(
                &self,
                properties: OfxPropertySetHandle,
                property: &CStr,
            ) -> crate::low::Result<$ty_low> {
                unsafe { self.$name_get_at(properties, property, 0) }
            }

            #[doc = impl_accessor_methods!(@doc)]
            pub unsafe fn $name_get_at(
                &self,
                properties: OfxPropertySetHandle,
                property: &CStr,
                index: c_int,
            ) -> crate::low::Result<$ty_low> {
                let sys_fn = unsafe { sys_fn!($get_sys_fn) };

                let mut value = std::mem::MaybeUninit::<$ty_get>::uninit();

                crate::low::Status::result_from(unsafe {
                    sys_fn(properties, property.as_ptr(), index, value.as_mut_ptr())
                })?;

                let value = unsafe { value.assume_init() };
                let value = impl_accessor_methods!(@convert[sys_get_to_low,scalar] $conversions value);

                Ok(value)
            }

            #[doc = impl_accessor_methods!(@doc)]
            pub unsafe fn $name_get_array<const N: usize>(
                &self,
                properties: OfxPropertySetHandle,
                property: &CStr,
            ) -> crate::low::Result<[$ty_low; N]> {
                let sys_fn = unsafe { sys_fn!($get_n_sys_fn) };

                let mut values: [$ty_get; N] =
                    unsafe { std::mem::MaybeUninit::uninit().assume_init() };

                crate::low::Status::result_from(unsafe {
                    sys_fn(
                        properties,
                        property.as_ptr(),
                        N as c_int,
                        values.as_mut_ptr(),
                    )
                })?;

                let values = impl_accessor_methods!(@convert[sys_get_to_low,array] $conversions values);

                Ok(values)
            }

            #[doc = impl_accessor_methods!(@doc)]
            pub unsafe fn $name_get_all(
                &self,
                properties: OfxPropertySetHandle,
                property: &CStr,
                values: &mut [$ty_low],
            ) -> crate::low::Result<()> {
                let sys_fn = unsafe { sys_fn!($get_n_sys_fn) };

                impl_accessor_methods!(@convert:pre[sys_get_to_low,slice] $conversions values=values tmp_values=tmp_values ty_get=$ty_get);

                crate::low::Status::result_from(unsafe {
                    sys_fn(
                        properties,
                        property.as_ptr(),
                        tmp_values.len() as c_int,
                        tmp_values.as_mut_ptr(),
                    )
                })?;

                impl_accessor_methods!(@convert:post[sys_get_to_low,slice] $conversions values=values tmp_values=tmp_values);

                Ok(())
            }
        }
    };
    (
        @set
        type = { sys_set = $ty_set:ty, low = $ty_low:ty },
        names = {
            set = $name_set:ident, set_at = $name_set_at:ident, set_array = $name_set_array:ident, set_all = $name_set_all:ident,
        },
        sys_fn_names = {
            set = $set_sys_fn:literal, set_n = $set_n_sys_fn:literal,
        },
        conversions = $conversions:tt,
    ) => {
        #[openfx_internal_macros::low_impl_suite]
        impl PropertySuiteV1 {
            #[doc = impl_accessor_methods!(@doc)]
            #[no_sys_fn]
            pub unsafe fn $name_set(
                &self,
                properties: OfxPropertySetHandle,
                property: &CStr,
                value: $ty_low,
            ) -> crate::low::Result<()> {
                unsafe { self.$name_set_at(properties, property, 0, value) }
            }

            #[doc = impl_accessor_methods!(@doc)]
            pub unsafe fn $name_set_at(
                &self,
                properties: OfxPropertySetHandle,
                property: &CStr,
                index: c_int,
                value: $ty_low,
            ) -> crate::low::Result<()> {
                let sys_fn = unsafe { sys_fn!($set_sys_fn) };

                let value = impl_accessor_methods!(@convert[low_to_sys_set,scalar] $conversions value);

                crate::low::Status::result_from(unsafe {
                    sys_fn(properties, property.as_ptr(), index, value)
                })
            }

            #[doc = impl_accessor_methods!(@doc)]
            pub unsafe fn $name_set_array<const N: usize>(
                &self,
                properties: OfxPropertySetHandle,
                property: &CStr,
                values: [$ty_low; N],
            ) -> crate::low::Result<()> {
                let sys_fn = unsafe { sys_fn!($set_n_sys_fn) };

                let values = impl_accessor_methods!(@convert[low_to_sys_set,array] $conversions values);

                crate::low::Status::result_from(unsafe {
                    sys_fn(properties, property.as_ptr(), N as c_int, values.as_ptr())
                })
            }

            #[doc = impl_accessor_methods!(@doc)]
            pub unsafe fn $name_set_all(
                &self,
                properties: OfxPropertySetHandle,
                property: &CStr,
                values: &[$ty_low],
            ) -> crate::low::Result<()> {
                let sys_fn = unsafe { sys_fn!($set_n_sys_fn) };

                let values = impl_accessor_methods!(@convert[low_to_sys_set,slice] $conversions values);

                crate::low::Status::result_from(unsafe {
                    sys_fn(
                        properties,
                        property.as_ptr(),
                        values.len() as c_int,
                        values.as_ptr(),
                    )
                })
            }
        }
    };
}

impl_accessor_methods!(
    type = { sys_get = c_int, sys_set = c_int, low = c_int },
    names = {
        get = prop_get_int, get_at = prop_get_int_at, get_array = prop_get_int_array, get_all = prop_get_int_n,
        set = prop_set_int, set_at = prop_set_int_at, set_array = prop_set_int_array, set_all = prop_set_int_n,
    },
    sys_fn_names = {
        get = "propGetInt", get_n = "propGetIntN",
        set = "propSetInt", set_n = "propSetIntN",
    },
    conversions = false,
);

impl_accessor_methods!(
    type = { sys_get = f64, sys_set = f64, low = f64 },
    names = {
        get = prop_get_double, get_at = prop_get_double_at, get_array = prop_get_double_array, get_all = prop_get_double_n,
        set = prop_set_double, set_at = prop_set_double_at, set_array = prop_set_double_array, set_all = prop_set_double_n,
    },
    sys_fn_names = {
        get = "propGetDouble", get_n = "propGetDoubleN",
        set = "propSetDouble", set_n = "propSetDoubleN",
    },
    conversions = false,
);

impl_accessor_methods!(
    @get
    type = { sys_get = *mut c_char, low = Option<NonNull<c_char>> },
    names = {
        get = prop_get_string_ptr, get_at = prop_get_string_ptr_at, get_array = prop_get_string_ptr_array, get_all = prop_get_string_ptr_n,
    },
    sys_fn_names = {
        get = "propGetString", get_n = "propGetStringN",
    },
    conversions = {
        sys_get_to_low = { (|x: *mut c_char| NonNull::new(x) ) },
        low_to_sys_set = { () },
    },
);

impl_accessor_methods!(
    @get
    type = { sys_get = *mut c_char, low = Option<CString> },
    names = {
        get = prop_get_string_owned, get_at = prop_get_string_owned_at, get_array = prop_get_string_owned_array, get_all = prop_get_string_owned_n,
    },
    sys_fn_names = {
        get = "propGetString", get_n = "propGetStringN",
    },
    conversions = {
        sys_get_to_low = { (|x: *mut c_char| if x.is_null() { None } else { Some((unsafe { CStr::from_ptr(x) }).to_owned()) }) },
        low_to_sys_set = { () },
    },
);

impl_accessor_methods!(
    @set
    type = { sys_set = *const c_char, low = Option<&CStr> },
    names = {
        set = prop_set_string, set_at = prop_set_string_at, set_array = prop_set_string_array, set_all = prop_set_string_n,
    },
    sys_fn_names = {
        set = "propSetString", set_n = "propSetStringN",
    },
    conversions = {
        sys_get_to_low = { () },
        low_to_sys_set = { (|s: Option<&CStr>| s.map_or(std::ptr::null(), |s| s.as_ptr())) },
    },
);

impl_accessor_methods!(
    type = { sys_get = *mut c_void, sys_set = *mut c_void, low = Option<std::ptr::NonNull<::std::ffi::c_void>> },
    names = {
        get = prop_get_pointer, get_at = prop_get_pointer_at, get_array = prop_get_pointer_array, get_all = prop_get_pointer_n,
        set = prop_set_pointer, set_at = prop_set_pointer_at, set_array = prop_set_pointer_array, set_all = prop_set_pointer_n,
    },
    sys_fn_names = {
        get = "propGetPointer", get_n = "propGetPointerN",
        set = "propSetPointer", set_n = "propSetPointerN",
    },
    conversions = {
        sys_get_to_low = { (|x: *mut c_void| std::ptr::NonNull::new(x)) },
        low_to_sys_set = { (|x: Option<std::ptr::NonNull<::std::ffi::c_void>>| x.map_or(std::ptr::null_mut(), |s| s.as_ptr())) },
    },
);

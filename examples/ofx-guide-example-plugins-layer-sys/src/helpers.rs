mod internal_utils;
pub mod shared_data_helper;

use std::ffi::{c_char, c_int, c_void};

use openfx::{
    generic::sys::{
        core::{
            OfxPropertySetHandle, OfxPropertySetStruct, OfxStatus, kOfxStatErrMissingHostFeature,
        },
        property::{OfxPropertySuiteV1, kOfxPropertySuite},
    },
    image_effect_v1::sys::{
        image_effect::{OfxImageEffectSuiteV1, kOfxImageEffectSuite},
        param::{OfxParameterSuiteV1, kOfxParameterSuite},
    },
};

#[derive(Clone)]
pub struct SaferHostStruct<'a> {
    pub host: &'a OfxPropertySetStruct,
    pub fetch_suite: unsafe extern "C" fn(
        host: OfxPropertySetHandle,
        suite_name: *const c_char,
        suite_version: c_int,
    ) -> *const c_void,
}

#[derive(Clone)]
pub struct SharedData<'a> {
    pub host_struct: SaferHostStruct<'a>,
    pub property_suite: &'a OfxPropertySuiteV1,
    pub image_effect_suite: &'a OfxImageEffectSuiteV1,
    pub parameter_suite: &'a OfxParameterSuiteV1,
}

impl<'a> SharedData<'a> {
    pub fn try_new(host_struct: SaferHostStruct<'a>) -> Result<Self, OfxStatus> {
        let property_suite = unsafe {
            (host_struct.fetch_suite)(
                host_struct.host as *const _ as OfxPropertySetHandle,
                kOfxPropertySuite.as_ptr(),
                1,
            )
        } as *const OfxPropertySuiteV1;
        let property_suite = unsafe {
            property_suite
                .as_ref()
                .ok_or(kOfxStatErrMissingHostFeature)?
        };

        let image_effect_suite = unsafe {
            (host_struct.fetch_suite)(
                host_struct.host as *const _ as OfxPropertySetHandle,
                kOfxImageEffectSuite.as_ptr(),
                1,
            )
        } as *const OfxImageEffectSuiteV1;
        let image_effect_suite = unsafe {
            image_effect_suite
                .as_ref()
                .ok_or(kOfxStatErrMissingHostFeature)?
        };

        let parameter_suite = unsafe {
            (host_struct.fetch_suite)(
                host_struct.host as *const _ as OfxPropertySetHandle,
                kOfxParameterSuite.as_ptr(),
                1,
            )
        } as *const OfxParameterSuiteV1;
        let parameter_suite = unsafe {
            parameter_suite
                .as_ref()
                .ok_or(kOfxStatErrMissingHostFeature)?
        };

        Ok(SharedData {
            host_struct,
            property_suite,
            image_effect_suite,
            parameter_suite,
        })
    }
}

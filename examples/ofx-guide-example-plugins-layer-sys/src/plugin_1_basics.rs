use std::{
    ffi::{CStr, c_char, c_int, c_void},
    sync::{Mutex, OnceLock},
};

use openfx::{
    generic::sys::{
        core::{
            OfxHost, OfxPlugin, OfxPropertySetHandle, OfxPropertySetStruct, OfxStatus,
            kOfxActionCreateInstance, kOfxActionDescribe, kOfxActionDestroyInstance,
            kOfxActionLoad, kOfxActionUnload, kOfxPropInstanceData, kOfxPropLabel, kOfxPropName,
            kOfxStatErrMissingHostFeature, kOfxStatErrUnsupported, kOfxStatFailed, kOfxStatOK,
            kOfxStatReplyDefault,
        },
        property::{OfxPropertySuiteV1, kOfxPropertySuite},
    },
    image_effect_v1::sys::image_effect::{
        OfxImageEffectHandle, OfxImageEffectSuiteV1, kOfxImageComponentAlpha,
        kOfxImageComponentRGBA, kOfxImageEffectActionDescribeInContext,
        kOfxImageEffectActionIsIdentity, kOfxImageEffectContextFilter, kOfxImageEffectPluginApi,
        kOfxImageEffectPluginPropGrouping, kOfxImageEffectPropContext,
        kOfxImageEffectPropSupportedComponents, kOfxImageEffectPropSupportedContexts,
        kOfxImageEffectSuite,
    },
};

use crate::definitions::{PLUGIN_1_BASICS_IDENTIFIER, PLUGIN_1_BASICS_LABEL, PLUGINS_GROUPING};

pub static EFFECT_PLUGIN_STRUCT_BASICS: OfxPlugin = OfxPlugin {
    pluginApi: kOfxImageEffectPluginApi.as_ptr(),
    apiVersion: 1,
    pluginIdentifier: PLUGIN_1_BASICS_IDENTIFIER.as_ptr(),
    pluginVersionMajor: 1,
    pluginVersionMinor: 0,
    setHost: Some(set_host),
    mainEntry: Some(main_entry),
};

static HOST_STRUCT: OnceLock<SaferHostStruct<'static>> = OnceLock::new();
#[derive(Clone)]
struct SaferHostStruct<'a> {
    host: &'a OfxPropertySetStruct,
    fetch_suite: unsafe extern "C" fn(
        host: OfxPropertySetHandle,
        suite_name: *const c_char,
        suite_version: c_int,
    ) -> *const c_void,
}

static SHARED_DATA: Mutex<Option<SharedData<'static>>> = Mutex::new(None);
struct SharedData<'a> {
    #[expect(unused)]
    host_struct: SaferHostStruct<'a>,
    property_suite: &'a OfxPropertySuiteV1,
    image_effect_suite: &'a OfxImageEffectSuiteV1,
}

unsafe extern "C" fn set_host(host_struct: *mut OfxHost) {
    fn inner(host_struct: *mut OfxHost) -> Result<(), &'static str> {
        let host_struct = unsafe {
            host_struct
                .as_mut()
                .ok_or("`host_struct` should not be null.")?
        };
        let host = unsafe {
            host_struct
                .host
                .as_mut()
                .ok_or("`host_struct.host` should not be null.")?
        };
        let fetch_suite = host_struct
            .fetchSuite
            .ok_or("`host_struct.fetchSuite` should not be null.")?;

        if HOST_STRUCT
            .set(SaferHostStruct { host, fetch_suite })
            .is_err()
        {
            return Err("`HOST_STRUCT` has already been initialized before.");
        }
        Ok(())
    }

    match inner(host_struct) {
        Ok(_) => {}
        Err(err) => {
            tracing::error!("Failed to set host: {}", err);
        }
    }
}

unsafe extern "C" fn main_entry(
    action: *const c_char,
    handle: *const c_void,
    in_args: OfxPropertySetHandle,
    out_args: OfxPropertySetHandle,
) -> OfxStatus {
    let effect = handle as OfxImageEffectHandle;
    let action = if action.is_null() {
        return kOfxStatReplyDefault;
    } else {
        unsafe { CStr::from_ptr(action) }
    };

    let result = match true {
        _ if action == kOfxActionLoad => action_load(),
        _ if action == kOfxActionUnload => action_unload(),
        _ if action == kOfxActionDescribe => action_describe(effect),
        _ if action == kOfxImageEffectActionDescribeInContext => {
            action_describe_in_context(effect, in_args)
        }
        _ if action == kOfxActionCreateInstance => action_create_instance(effect),
        _ if action == kOfxActionDestroyInstance => action_destroy_instance(effect),
        _ if action == kOfxImageEffectActionIsIdentity => {
            action_is_identity(effect, in_args, out_args)
        }
        _ => Err(kOfxStatReplyDefault),
    };

    match result {
        Ok(_) => kOfxStatOK as OfxStatus,
        Err(status) => status,
    }
}

fn action_load() -> Result<(), OfxStatus> {
    let host_struct = HOST_STRUCT.get().ok_or(kOfxStatFailed)?.clone();

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

    let mut shared_data = SHARED_DATA.lock().map_err(|_| kOfxStatFailed)?;
    if shared_data.is_some() {
        Err(kOfxStatFailed)
    } else {
        *shared_data = Some(SharedData {
            host_struct,
            property_suite,
            image_effect_suite,
        });
        Ok(())
    }
}

fn action_unload() -> Result<(), OfxStatus> {
    let mut shared_data = SHARED_DATA.lock().map_err(|_| kOfxStatFailed)?;
    if shared_data.take().is_none() {
        Err(kOfxStatFailed)
    } else {
        Ok(())
    }
}

fn action_describe(descriptor: OfxImageEffectHandle) -> Result<(), OfxStatus> {
    let data = SHARED_DATA.lock().map_err(|_| kOfxStatFailed)?;
    let data = data.as_ref().ok_or(kOfxStatFailed)?;

    let get_property_set = data
        .image_effect_suite
        .getPropertySet
        .ok_or(kOfxStatErrMissingHostFeature)?;

    let mut effect_props = std::ptr::null_mut();
    if let stat = (unsafe { get_property_set(descriptor, &mut effect_props) })
        && stat != kOfxStatOK
    {
        return Err(stat);
    }

    let prop_set_string = data
        .property_suite
        .propSetString
        .ok_or(kOfxStatErrMissingHostFeature)?;

    unsafe {
        if let stat = prop_set_string(
            effect_props,
            kOfxPropLabel.as_ptr(),
            0,
            PLUGIN_1_BASICS_LABEL.as_ptr(),
        ) && stat != kOfxStatOK
        {
            return Err(stat);
        }
        if let stat = prop_set_string(
            effect_props,
            kOfxImageEffectPluginPropGrouping.as_ptr(),
            0,
            PLUGINS_GROUPING.as_ptr(),
        ) && stat != kOfxStatOK
        {
            return Err(stat);
        }
        if let stat = prop_set_string(
            effect_props,
            kOfxImageEffectPropSupportedContexts.as_ptr(),
            0,
            kOfxImageEffectContextFilter.as_ptr(),
        ) && stat != kOfxStatOK
        {
            return Err(stat);
        }
    }

    Ok(())
}

fn action_describe_in_context(
    descriptor: OfxImageEffectHandle,
    in_args: OfxPropertySetHandle,
) -> Result<(), OfxStatus> {
    let data = SHARED_DATA.lock().map_err(|_| kOfxStatFailed)?;
    let data = data.as_ref().ok_or(kOfxStatFailed)?;

    let prop_get_string = data
        .property_suite
        .propGetString
        .ok_or(kOfxStatErrMissingHostFeature)?;
    let prop_set_string = data
        .property_suite
        .propSetString
        .ok_or(kOfxStatErrMissingHostFeature)?;
    let clip_define = data
        .image_effect_suite
        .clipDefine
        .ok_or(kOfxStatErrMissingHostFeature)?;

    let mut context: *mut c_char = std::ptr::null_mut();
    unsafe {
        if let stat = prop_get_string(
            in_args,
            kOfxImageEffectPropContext.as_ptr(),
            0,
            &mut context,
        ) && stat != kOfxStatOK
        {
            return Err(stat);
        }
    }
    let context = unsafe { CStr::from_ptr(context) };
    if context != kOfxImageEffectContextFilter {
        return Err(kOfxStatErrUnsupported);
    }

    let mut props: *mut OfxPropertySetStruct = std::ptr::null_mut();
    unsafe {
        if let stat = clip_define(descriptor, c"Output".as_ptr(), &mut props)
            && stat != kOfxStatOK
        {
            return Err(stat);
        }
        if let stat = prop_set_string(
            props,
            kOfxImageEffectPropSupportedComponents.as_ptr(),
            0,
            kOfxImageComponentRGBA.as_ptr(),
        ) && stat != kOfxStatOK
        {
            return Err(stat);
        }
        if let stat = prop_set_string(
            props,
            kOfxImageEffectPropSupportedComponents.as_ptr(),
            1,
            kOfxImageComponentAlpha.as_ptr(),
        ) && stat != kOfxStatOK
        {
            return Err(stat);
        }
    }

    let mut props: *mut OfxPropertySetStruct = std::ptr::null_mut();
    unsafe {
        if let stat = clip_define(descriptor, c"Source".as_ptr(), &mut props)
            && stat != kOfxStatOK
        {
            return Err(stat);
        }
        if let stat = prop_set_string(
            props,
            kOfxImageEffectPropSupportedComponents.as_ptr(),
            0,
            kOfxImageComponentRGBA.as_ptr(),
        ) && stat != kOfxStatOK
        {
            return Err(stat);
        }
        if let stat = prop_set_string(
            props,
            kOfxImageEffectPropSupportedComponents.as_ptr(),
            1,
            kOfxImageComponentAlpha.as_ptr(),
        ) && stat != kOfxStatOK
        {
            return Err(stat);
        }
    }

    Ok(())
}

fn action_create_instance(instance: OfxImageEffectHandle) -> Result<(), OfxStatus> {
    let data = SHARED_DATA.lock().map_err(|_| kOfxStatFailed)?;
    let data = data.as_ref().ok_or(kOfxStatFailed)?;

    let get_property_set = data
        .image_effect_suite
        .getPropertySet
        .ok_or(kOfxStatErrMissingHostFeature)?;
    let prop_set_pointer = data
        .property_suite
        .propSetPointer
        .ok_or(kOfxStatErrMissingHostFeature)?;

    let mut effect_props: *mut OfxPropertySetStruct = std::ptr::null_mut();
    if let stat = (unsafe { get_property_set(instance, &mut effect_props) })
        && stat != kOfxStatOK
    {
        return Err(stat);
    }

    let my_string = Box::new(String::from(
        "This is random instance data that could be anything you want.",
    ));
    let my_string = Box::into_raw(my_string) as *mut c_void;
    unsafe {
        if let stat = prop_set_pointer(effect_props, kOfxPropInstanceData.as_ptr(), 0, my_string)
            && stat != kOfxStatOK
        {
            drop(Box::from_raw(my_string.cast::<String>()));
            return Err(stat);
        }
    }

    Ok(())
}

fn action_destroy_instance(instance: OfxImageEffectHandle) -> Result<(), OfxStatus> {
    let data = SHARED_DATA.lock().map_err(|_| kOfxStatFailed)?;
    let data = data.as_ref().ok_or(kOfxStatFailed)?;

    let get_property_set = data
        .image_effect_suite
        .getPropertySet
        .ok_or(kOfxStatErrMissingHostFeature)?;
    let prop_get_pointer = data
        .property_suite
        .propGetPointer
        .ok_or(kOfxStatErrMissingHostFeature)?;

    let mut effect_props: *mut OfxPropertySetStruct = std::ptr::null_mut();
    if let stat = (unsafe { get_property_set(instance, &mut effect_props) })
        && stat != kOfxStatOK
    {
        return Err(stat);
    }

    let mut my_string: *mut c_void = std::ptr::null_mut();
    if let stat = (unsafe {
        prop_get_pointer(
            effect_props,
            kOfxPropInstanceData.as_ptr(),
            0,
            &mut my_string,
        )
    }) && stat != kOfxStatOK
    {
        return Err(stat);
    }

    // assert!(!my_string.is_null(), "Instance data should not be null!");

    drop(unsafe { Box::from_raw(my_string.cast::<String>()) });

    Ok(())
}

fn action_is_identity(
    _instance: OfxImageEffectHandle,
    _in_args: OfxPropertySetHandle,
    out_args: OfxPropertySetHandle,
) -> Result<(), OfxStatus> {
    let data = SHARED_DATA.lock().map_err(|_| kOfxStatFailed)?;
    let data = data.as_ref().ok_or(kOfxStatFailed)?;

    let prop_set_string = data
        .property_suite
        .propSetString
        .ok_or(kOfxStatErrMissingHostFeature)?;
    if let stat = unsafe { prop_set_string(out_args, kOfxPropName.as_ptr(), 0, c"Source".as_ptr()) }
        && stat != kOfxStatOK
    {
        return Err(stat);
    }

    Ok(())
}

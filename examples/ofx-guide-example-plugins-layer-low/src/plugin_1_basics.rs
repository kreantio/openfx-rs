use std::{
    ffi::{CStr, c_void},
    sync::Mutex,
};

use openfx::{
    low::{Status, enums::ImageEffectPropContext},
    low_plugin::{
        Host, HostOwned, Plugin,
        actions::image_effect::{ActionDescribeInContextIn, ImageEffectAction},
    },
    sys::{
        generic::{
            core::{
                OfxPropertySetHandle, OfxPropertySetStruct, kOfxStatErrMissingHostFeature,
                kOfxStatFailed, kOfxStatOK,
            },
            property::{OfxPropertySuiteV1, kOfxPropertySuite},
        },
        image_effect_v1::image_effect::{
            OfxImageEffectHandle, OfxImageEffectSuiteV1, kOfxImageComponentAlpha,
            kOfxImageComponentRGBA, kOfxImageEffectContextFilter, kOfxImageEffectSuite,
        },
    },
    sys_helpers::{
        generic::properties::{get_OfxPropInstanceData, set_OfxPropInstanceData, set_OfxPropLabel},
        image_effect_v1::properties::{
            set_OfxImageEffectPluginPropGrouping, set_OfxImageEffectPropSupportedComponents,
            set_OfxImageEffectPropSupportedContexts,
        },
    },
};

use crate::definitions::{PLUGIN_1_BASICS_IDENTIFIER, PLUGIN_1_BASICS_LABEL, PLUGINS_GROUPING};

struct GuaranteeSend<T: HostOwned>(T);
/// ## Safety
///
/// This plugin does not spawn threads, so the host exclusively controls its
/// lifecycle.
unsafe impl<T: HostOwned> Send for GuaranteeSend<T> {}

static HOST_BEFORE_ACTION_LOAD: Mutex<Option<GuaranteeSend<Host>>> = Mutex::new(None);
static SHARED_DATA: Mutex<Option<SharedData<'static>>> = Mutex::new(None);
struct SharedData<'a> {
    #[expect(unused)]
    host: GuaranteeSend<Host>,
    property_suite: &'a OfxPropertySuiteV1,
    image_effect_suite: &'a OfxImageEffectSuiteV1,
}

pub struct PluginExampleBasic;
impl Plugin for PluginExampleBasic {
    const PLUGIN_IDENTIFIER: &'static CStr = PLUGIN_1_BASICS_IDENTIFIER;
    const PLUGIN_VERSION_MAJOR: std::ffi::c_uint = 1;
    const PLUGIN_VERSION_MINOR: std::ffi::c_uint = 0;

    fn set_host(host: Host) {
        let mut lock = HOST_BEFORE_ACTION_LOAD
            .lock()
            .expect("Failed to lock HOST_BEFORE_ACTION_LOAD.");
        if lock.is_some() {
            panic!("HOST_BEFORE_ACTION_LOAD has already been set.");
        }
        lock.replace(GuaranteeSend(host));
    }

    fn main_entry(action: ImageEffectAction) -> openfx::low::Result<()> {
        match action {
            ImageEffectAction::Load { .. } => action_load(),
            ImageEffectAction::Unload { .. } => action_unload(),
            ImageEffectAction::Describe { sys_handle, .. } => {
                action_describe(sys_handle as OfxImageEffectHandle)
            }
            ImageEffectAction::DescribeInContext {
                sys_handle,
                in_args,
                ..
            } => action_describe_in_context(sys_handle as OfxImageEffectHandle, in_args),
            ImageEffectAction::CreateInstance { sys_handle, .. } => {
                action_create_instance(sys_handle as OfxImageEffectHandle)
            }
            ImageEffectAction::DestroyInstance { sys_handle, .. } => {
                action_destroy_instance(sys_handle as OfxImageEffectHandle)
            }
            ImageEffectAction::IsIdentity { sys_out_args, .. } => action_is_identity(sys_out_args),
            _ => Err(Status::ReplyDefault),
        }
    }
}

fn action_load() -> openfx::low::Result<()> {
    let host = HOST_BEFORE_ACTION_LOAD
        .lock()
        .map_err(|_| kOfxStatFailed)?
        .take()
        .ok_or(kOfxStatFailed)?;

    let property_suite =
        unsafe { host.0.fetch_suite(kOfxPropertySuite, 1) } as *const OfxPropertySuiteV1;
    let property_suite = unsafe {
        property_suite
            .as_ref()
            .ok_or(kOfxStatErrMissingHostFeature)?
    };

    let image_effect_suite =
        unsafe { host.0.fetch_suite(kOfxImageEffectSuite, 1) } as *const OfxImageEffectSuiteV1;
    let image_effect_suite = unsafe {
        image_effect_suite
            .as_ref()
            .ok_or(kOfxStatErrMissingHostFeature)?
    };

    let mut shared_data = SHARED_DATA.lock().map_err(|_| kOfxStatFailed)?;
    if shared_data.is_some() {
        Err(Status::Failed)
    } else {
        *shared_data = Some(SharedData {
            host,
            property_suite,
            image_effect_suite,
        });
        Ok(())
    }
}

fn action_unload() -> openfx::low::Result<()> {
    let mut shared_data = SHARED_DATA.lock().map_err(|_| kOfxStatFailed)?;
    if shared_data.take().is_none() {
        Err(Status::Failed)
    } else {
        Ok(())
    }
}

fn action_describe(descriptor: OfxImageEffectHandle) -> openfx::low::Result<()> {
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
        return Err(Status::from(stat));
    }

    let s_prop = data.property_suite;

    unsafe {
        set_OfxPropLabel(s_prop, effect_props, PLUGIN_1_BASICS_LABEL.as_ptr())?;
        set_OfxImageEffectPluginPropGrouping(s_prop, effect_props, PLUGINS_GROUPING.as_ptr())?;
        set_OfxImageEffectPropSupportedContexts(
            s_prop,
            effect_props,
            &[kOfxImageEffectContextFilter.as_ptr()],
        )?;
    }

    Ok(())
}

fn action_describe_in_context(
    descriptor: OfxImageEffectHandle,
    in_args: ActionDescribeInContextIn,
) -> openfx::low::Result<()> {
    let data = SHARED_DATA.lock().map_err(|_| kOfxStatFailed)?;
    let data = data.as_ref().ok_or(kOfxStatFailed)?;

    let s_prop = data.property_suite;
    let clip_define = data
        .image_effect_suite
        .clipDefine
        .ok_or(kOfxStatErrMissingHostFeature)?;

    let context = unsafe { in_args.get_image_effect_context(s_prop) }?;
    if context != ImageEffectPropContext::Filter {
        return Err(Status::ErrUnsupported);
    }

    let mut props: *mut OfxPropertySetStruct = std::ptr::null_mut();
    unsafe {
        if let stat = clip_define(descriptor, c"Output".as_ptr(), &mut props)
            && stat != kOfxStatOK
        {
            return Err(Status::from(stat));
        }
        set_OfxImageEffectPropSupportedComponents(
            s_prop,
            props,
            &[
                kOfxImageComponentRGBA.as_ptr(),
                kOfxImageComponentAlpha.as_ptr(),
            ],
        )?;
    }

    let mut props: *mut OfxPropertySetStruct = std::ptr::null_mut();
    unsafe {
        if let stat = clip_define(descriptor, c"Source".as_ptr(), &mut props)
            && stat != kOfxStatOK
        {
            return Err(Status::from(stat));
        }
        set_OfxImageEffectPropSupportedComponents(
            s_prop,
            props,
            &[
                kOfxImageComponentRGBA.as_ptr(),
                kOfxImageComponentAlpha.as_ptr(),
            ],
        )?;
    }

    Ok(())
}

fn action_create_instance(instance: OfxImageEffectHandle) -> openfx::low::Result<()> {
    let data = SHARED_DATA.lock().map_err(|_| kOfxStatFailed)?;
    let data = data.as_ref().ok_or(kOfxStatFailed)?;

    let get_property_set = data
        .image_effect_suite
        .getPropertySet
        .ok_or(kOfxStatErrMissingHostFeature)?;
    let s_prop = data.property_suite;

    let mut effect_props: *mut OfxPropertySetStruct = std::ptr::null_mut();
    if let stat = (unsafe { get_property_set(instance, &mut effect_props) })
        && stat != kOfxStatOK
    {
        return Err(Status::from(stat));
    }

    let my_string = Box::new(String::from(
        "This is random instance data that could be anything you want.",
    ));
    let my_string = Box::into_raw(my_string) as *mut c_void;
    unsafe { set_OfxPropInstanceData(s_prop, effect_props, my_string) }?;

    Ok(())
}

fn action_destroy_instance(instance: OfxImageEffectHandle) -> openfx::low::Result<()> {
    let data = SHARED_DATA.lock().map_err(|_| kOfxStatFailed)?;
    let data = data.as_ref().ok_or(kOfxStatFailed)?;

    let get_property_set = data
        .image_effect_suite
        .getPropertySet
        .ok_or(kOfxStatErrMissingHostFeature)?;
    let s_prop = data.property_suite;

    let mut effect_props: *mut OfxPropertySetStruct = std::ptr::null_mut();
    if let stat = (unsafe { get_property_set(instance, &mut effect_props) })
        && stat != kOfxStatOK
    {
        return Err(Status::from(stat));
    }

    let my_string = unsafe { get_OfxPropInstanceData(s_prop, effect_props) }?;

    // assert!(!my_string.is_null(), "Instance data should not be null!");

    drop(unsafe { Box::from_raw(my_string.cast::<String>()) });

    Ok(())
}

fn action_is_identity(out_args: OfxPropertySetHandle) -> openfx::low::Result<()> {
    let data = SHARED_DATA.lock().map_err(|_| kOfxStatFailed)?;
    let data = data.as_ref().ok_or(kOfxStatFailed)?;

    let s_prop = data.property_suite;
    unsafe { set_OfxPropLabel(s_prop, out_args, c"Source".as_ptr()) }?;

    Ok(())
}

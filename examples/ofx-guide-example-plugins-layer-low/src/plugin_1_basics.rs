use std::{
    ffi::{CStr, c_void},
    sync::Mutex,
};

use openfx::{
    low::{
        Status,
        enums::{
            ImageEffectPropContext, ImageEffectPropSupportedComponents,
            ImageEffectPropSupportedContexts,
        },
    },
    low_plugin::{
        Host, Plugin,
        actions::image_effect::{ActionDescribeInContextIn, ImageEffectAction},
        objects::{ImageEffectDescriptor, ImageEffectInstance},
    },
    sys::generic::core::{OfxPropertySetHandle, kOfxStatOK},
    sys_helpers::generic::properties::{set_OfxPropInstanceData, set_OfxPropLabel},
};

use crate::{
    definitions::{PLUGIN_1_BASICS_IDENTIFIER, PLUGIN_1_BASICS_LABEL, PLUGINS_GROUPING},
    helpers::shared_data::{GuaranteeSend, SharedData},
};

static HOST_BEFORE_ACTION_LOAD: Mutex<Option<GuaranteeSend<Host>>> = Mutex::new(None);
static SHARED_DATA: Mutex<Option<SharedData>> = Mutex::new(None);

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
            ImageEffectAction::Describe { handle, .. } => action_describe(handle),
            ImageEffectAction::DescribeInContext {
                handle, in_args, ..
            } => action_describe_in_context(handle, in_args),
            ImageEffectAction::CreateInstance { handle, .. } => action_create_instance(handle),
            ImageEffectAction::DestroyInstance { handle, .. } => action_destroy_instance(handle),
            ImageEffectAction::IsIdentity { sys_out_args, .. } => action_is_identity(sys_out_args),
            _ => Err(Status::ReplyDefault),
        }
    }
}

fn action_load() -> openfx::low::Result<()> {
    let host = HOST_BEFORE_ACTION_LOAD
        .lock()
        .map_err(|_| Status::Failed)?
        .take()
        .ok_or(Status::Failed)?;

    let mut data = SHARED_DATA.lock().map_err(|_| Status::Failed)?;
    if data.is_some() {
        Err(Status::Failed)
    } else {
        *data = Some(SharedData::try_new(host)?);
        Ok(())
    }
}

fn action_unload() -> openfx::low::Result<()> {
    let mut shared_data = SHARED_DATA.lock().map_err(|_| Status::Failed)?;
    if shared_data.take().is_none() {
        Err(Status::Failed)
    } else {
        Ok(())
    }
}

fn action_describe(descriptor: ImageEffectDescriptor) -> openfx::low::Result<()> {
    let data = {
        let data = SHARED_DATA.lock().map_err(|_| Status::Failed)?;
        let data = data.as_ref().ok_or(Status::Failed)?;
        data.clone()
    };

    let s_prop = &data.property_suite.0;

    let props = unsafe { data.get_property_set_from_image_effect_descriptor(&descriptor) }?;

    unsafe {
        props.set_label(s_prop, Some(PLUGIN_1_BASICS_LABEL))?;
        props.set_image_effect_plugin_grouping(s_prop, Some(PLUGINS_GROUPING))?;
        props.set_image_effect_supported_contexts(
            s_prop,
            &[ImageEffectPropSupportedContexts::Filter],
        )?;
    }

    Ok(())
}

fn action_describe_in_context(
    descriptor: ImageEffectDescriptor,
    in_args: ActionDescribeInContextIn,
) -> openfx::low::Result<()> {
    let data = {
        let data = SHARED_DATA.lock().map_err(|_| Status::Failed)?;
        let data = data.as_ref().ok_or(Status::Failed)?;
        data.clone()
    };

    let s_prop = &data.property_suite.0;
    let s_ifx = data.image_effect_suite_helper();

    let context = unsafe { in_args.get_image_effect_context(s_prop) }?;
    if context != ImageEffectPropContext::Filter {
        return Err(Status::ErrUnsupported);
    }

    let props = unsafe { s_ifx.clip_define(&descriptor, c"Output") }?;
    (unsafe {
        props.set_image_effect_supported_components(
            s_prop,
            &[
                ImageEffectPropSupportedComponents::RGBA,
                ImageEffectPropSupportedComponents::Alpha,
            ],
        )
    })?;

    let props = unsafe { s_ifx.clip_define(&descriptor, c"Source") }?;
    (unsafe {
        props.set_image_effect_supported_components(
            s_prop,
            &[
                ImageEffectPropSupportedComponents::RGBA,
                ImageEffectPropSupportedComponents::Alpha,
            ],
        )
    })?;

    Ok(())
}

fn action_create_instance(instance: ImageEffectInstance) -> openfx::low::Result<()> {
    let data = {
        let data = SHARED_DATA.lock().map_err(|_| Status::Failed)?;
        let data = data.as_ref().ok_or(Status::Failed)?;
        data.clone()
    };

    let s_prop = &data.property_suite.0;

    let props = unsafe { data.get_property_set_from_image_effect_instance(&instance) }?;

    let my_string = Box::new(String::from(
        "This is random instance data that could be anything you want.",
    ));
    let my_string = Box::into_raw(my_string) as *mut c_void;
    if let Some(stat) =
        unsafe { set_OfxPropInstanceData(s_prop.sys_ptr(), props.sys_handle(), my_string) }.err()
        && stat != kOfxStatOK
    {
        drop(unsafe { Box::from_raw(my_string.cast::<String>()) });
        return Err(Status::from(stat));
    }

    Ok(())
}

fn action_destroy_instance(instance: ImageEffectInstance) -> openfx::low::Result<()> {
    let data = {
        let data = SHARED_DATA.lock().map_err(|_| Status::Failed)?;
        let data = data.as_ref().ok_or(Status::Failed)?;
        data.clone()
    };

    let s_prop = &data.property_suite.0;

    let props = unsafe { data.get_property_set_from_image_effect_instance(&instance) }?;

    let my_string =
        unsafe { props.get_instance_data(s_prop) }?.expect("Instance data should not be null");

    drop(unsafe { Box::from_raw(my_string.as_ptr().cast::<String>()) });

    Ok(())
}

fn action_is_identity(out_args: OfxPropertySetHandle) -> openfx::low::Result<()> {
    let data = {
        let data = SHARED_DATA.lock().map_err(|_| Status::Failed)?;
        let data = data.as_ref().ok_or(Status::Failed)?;
        data.clone()
    };

    let s_prop = &data.property_suite.0;

    unsafe { set_OfxPropLabel(s_prop.sys_ptr(), out_args, c"Source".as_ptr()) }?;

    Ok(())
}

use std::{
    ffi::{CStr, c_void},
    sync::Mutex,
};

use openfx::{
    low::{
        OutboundResult, Status, StatusError, StatusReply,
        enums::{
            ImageEffectPropContext, ImageEffectPropSupportedComponents,
            ImageEffectPropSupportedContexts,
        },
    },
    low_plugin::{
        Host, Plugin,
        actions::image_effect::{
            ActionDescribeInContextIn, ActionIsIdentityOut, ImageEffectAction,
        },
        objects::{ImageEffectDescriptor, ImageEffectInstance},
    },
    sys::core::{kOfxPropInstanceData, kOfxPropLabel},
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

    fn main_entry(action: ImageEffectAction) -> OutboundResult {
        match action {
            ImageEffectAction::Load { .. } => action_load(),
            ImageEffectAction::Unload { .. } => action_unload(),
            ImageEffectAction::Describe { handle, .. } => action_describe(handle),
            ImageEffectAction::DescribeInContext {
                handle, in_args, ..
            } => action_describe_in_context(handle, in_args),
            ImageEffectAction::CreateInstance { handle, .. } => action_create_instance(handle),
            ImageEffectAction::DestroyInstance { handle, .. } => action_destroy_instance(handle),
            ImageEffectAction::IsIdentity { out_args, .. } => action_is_identity(out_args),
            _ => Ok(StatusReply::ReplyDefault.into()),
        }
    }
}

fn action_load() -> OutboundResult {
    let host = HOST_BEFORE_ACTION_LOAD
        .lock()
        .map_err(|_| StatusError::Failed)?
        .take()
        .ok_or(StatusError::Failed)?;

    let mut data = SHARED_DATA.lock().map_err(|_| StatusError::Failed)?;
    if data.is_some() {
        Err(StatusError::Failed)
    } else {
        *data = Some(SharedData::try_new(host)?);
        Ok(Status::OK)
    }
}

fn action_unload() -> OutboundResult {
    let mut shared_data = SHARED_DATA.lock().map_err(|_| StatusError::Failed)?;
    if shared_data.take().is_none() {
        Err(StatusError::Failed)
    } else {
        Ok(Status::OK)
    }
}

fn action_describe(descriptor: ImageEffectDescriptor) -> OutboundResult {
    let data = {
        let data = SHARED_DATA.lock().map_err(|_| StatusError::Failed)?;
        let data = data.as_ref().ok_or(StatusError::Failed)?;
        data.clone()
    };

    let s_prop = &data.property_suite.0;

    let props = unsafe { descriptor.get_property_set(&data.image_effect_suite.0) }
        .ok_or(StatusError::Failed)?;

    unsafe {
        props
            .set_label(s_prop, Some(PLUGIN_1_BASICS_LABEL))
            .ok_or(StatusError::Failed)?;
        props
            .set_image_effect_plugin_grouping(s_prop, Some(PLUGINS_GROUPING))
            .ok_or(StatusError::Failed)?;
        props
            .set_image_effect_supported_contexts(
                s_prop,
                &[ImageEffectPropSupportedContexts::Filter.into()],
            )
            .ok_or(StatusError::Failed)?;
    }

    Ok(Status::OK)
}

fn action_describe_in_context(
    descriptor: ImageEffectDescriptor,
    in_args: ActionDescribeInContextIn,
) -> OutboundResult {
    let data = {
        let data = SHARED_DATA.lock().map_err(|_| StatusError::Failed)?;
        let data = data.as_ref().ok_or(StatusError::Failed)?;
        data.clone()
    };

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;

    let context = unsafe { in_args.get_image_effect_context(s_prop) }.ok_or(StatusError::Failed)?;
    if context != ImageEffectPropContext::Filter {
        return Err(StatusError::ErrUnsupported);
    }

    let props = unsafe { descriptor.clip_define(s_ifx, c"Output") }.ok_or(StatusError::Failed)?;
    (unsafe {
        props.set_image_effect_supported_components(
            s_prop,
            &[
                ImageEffectPropSupportedComponents::RGBA.into(),
                ImageEffectPropSupportedComponents::Alpha.into(),
            ],
        )
    })
    .ok_or(StatusError::Failed)?;

    let props = unsafe { descriptor.clip_define(s_ifx, c"Source") }.ok_or(StatusError::Failed)?;
    (unsafe {
        props.set_image_effect_supported_components(
            s_prop,
            &[
                ImageEffectPropSupportedComponents::RGBA.into(),
                ImageEffectPropSupportedComponents::Alpha.into(),
            ],
        )
    })
    .ok_or(StatusError::Failed)?;

    Ok(Status::OK)
}

fn action_create_instance(instance: ImageEffectInstance) -> OutboundResult {
    let data = {
        let data = SHARED_DATA.lock().map_err(|_| StatusError::Failed)?;
        let data = data.as_ref().ok_or(StatusError::Failed)?;
        data.clone()
    };

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;

    let props = unsafe { instance.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    let my_string = Box::new(String::from(
        "This is random instance data that could be anything you want.",
    ));
    let my_string = Box::into_raw(my_string);

    if let r = unsafe {
        s_prop.prop_set_pointer(
            props.sys_handle(),
            kOfxPropInstanceData,
            std::ptr::NonNull::new(my_string.cast::<c_void>()),
        )
    } && !r.is_ok()
    {
        drop(unsafe { Box::from_raw(my_string.cast::<String>()) });
        return Err(StatusError::Failed);
    }

    Ok(Status::OK)
}

fn action_destroy_instance(instance: ImageEffectInstance) -> OutboundResult {
    let data = {
        let data = SHARED_DATA.lock().map_err(|_| StatusError::Failed)?;
        let data = data.as_ref().ok_or(StatusError::Failed)?;
        data.clone()
    };

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;

    let props = unsafe { instance.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    let my_string = unsafe { props.get_instance_data(s_prop) }.ok_or(StatusError::Failed)?;
    let Some(my_string) = my_string else {
        tracing::error!("Instance data should not be null");
        return Err(StatusError::Failed);
    };

    drop(unsafe { Box::from_raw(my_string.as_ptr().cast::<String>()) });

    Ok(Status::OK)
}

fn action_is_identity(out_args: ActionIsIdentityOut) -> OutboundResult {
    let data = {
        let data = SHARED_DATA.lock().map_err(|_| StatusError::Failed)?;
        let data = data.as_ref().ok_or(StatusError::Failed)?;
        data.clone()
    };

    let s_prop = &data.property_suite.0;

    (unsafe { s_prop.prop_set_string(out_args.sys_handle(), kOfxPropLabel, Some(c"Source")) })
        .ok_or(StatusError::Failed)?;

    Ok(Status::OK)
}

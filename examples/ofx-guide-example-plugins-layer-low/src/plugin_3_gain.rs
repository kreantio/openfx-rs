mod processing;

use std::{
    ffi::{CStr, c_int, c_void},
    sync::Mutex,
};

use openfx::{
    low::{
        OutboundResult, Status, StatusError, StatusReply,
        enums::{
            ImageEffectPluginRenderThreadSafety, ImageEffectPropContext,
            ImageEffectPropSupportedComponents, ImageEffectPropSupportedContexts,
            ImageEffectPropSupportedPixelDepths, ParamPropDoubleType,
        },
    },
    low_plugin::{
        Host, Plugin,
        actions::image_effect::{
            ActionDescribeInContextIn, ActionIsIdentityIn, ActionIsIdentityOut, ActionRenderIn,
            ImageEffectAction,
        },
        objects::{ImageEffectDescriptor, ImageEffectInstance},
        property_sets::{ParamsBytePropertySet, ParamsDouble1DPropertySet},
    },
    sys::{
        generic::core::{kOfxPropInstanceData, kOfxPropLabel},
        image_effect_v1::param::{OfxParamHandle, kOfxParamTypeBoolean, kOfxParamTypeDouble},
    },
};

use processing::{pixel_processing, rect_i_from_array};

use crate::{
    definitions::{PLUGIN_3_GAIN_IDENTIFIER, PLUGIN_3_GAIN_LABEL, PLUGINS_GROUPING},
    helpers::shared_data::{
        BitDepth, ClipImageManaged, GuaranteeSend, GuaranteeSendClipInInstance, SharedData,
        param_get_value_at_time,
    },
};

static HOST_BEFORE_ACTION_LOAD: Mutex<Option<GuaranteeSend<Host>>> = Mutex::new(None);
static SHARED_DATA: Mutex<Option<SharedData>> = Mutex::new(None);

struct MyInstanceData {
    source_clip: GuaranteeSendClipInInstance,
    output_clip: GuaranteeSendClipInInstance,

    gain_param: OfxParamHandle,
    apply_to_alpha_param: OfxParamHandle,
}

fn shared_data_lockless() -> Result<SharedData, StatusError> {
    let data = SHARED_DATA.lock().map_err(|_| StatusError::Failed)?;
    let data = data.as_ref().ok_or(StatusError::Failed)?;
    Ok(data.clone())
}

const GAIN_PARAM_NAME: &CStr = c"gain";
const APPLY_TO_ALPHA_PARAM_NAME: &CStr = c"applyToAlpha";

pub struct PluginExampleGain;
impl Plugin for PluginExampleGain {
    const PLUGIN_IDENTIFIER: &'static CStr = PLUGIN_3_GAIN_IDENTIFIER;
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
            ImageEffectAction::IsIdentity {
                handle,
                in_args,
                out_args,
            } => action_is_identity(handle, in_args, out_args),
            ImageEffectAction::Render {
                handle, in_args, ..
            } => action_render(handle, in_args),
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
    let mut data = SHARED_DATA.lock().map_err(|_| StatusError::Failed)?;
    if data.take().is_none() {
        Err(StatusError::Failed)
    } else {
        Ok(Status::OK)
    }
}

fn action_describe(descriptor: ImageEffectDescriptor) -> OutboundResult {
    let data = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;

    let props = unsafe { descriptor.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    unsafe {
        props
            .set_label(s_prop, Some(PLUGIN_3_GAIN_LABEL))
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
        props
            .set_image_effect_supported_pixel_depths(
                s_prop,
                &[
                    ImageEffectPropSupportedPixelDepths::Float.into(),
                    ImageEffectPropSupportedPixelDepths::Short.into(),
                    ImageEffectPropSupportedPixelDepths::Byte.into(),
                ],
            )
            .ok_or(StatusError::Failed)?;
        props
            .set_image_effect_plugin_render_thread_safety(
                s_prop,
                ImageEffectPluginRenderThreadSafety::FullySafe.into(),
            )
            .ok_or(StatusError::Failed)?;
        props
            .set_image_effect_plugin_host_frame_threading(s_prop, true)
            .ok_or(StatusError::Failed)?;
    }

    Ok(Status::OK)
}

fn action_describe_in_context(
    descriptor: ImageEffectDescriptor,
    in_args: ActionDescribeInContextIn,
) -> OutboundResult {
    let data = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;
    let s_param = &data.parameter_suite.0;

    let context = unsafe { in_args.get_image_effect_context(s_prop) }.ok_or(StatusError::Failed)?;
    if context != ImageEffectPropContext::Filter {
        return Err(StatusError::ErrUnsupported);
    }

    for name in [c"Output", c"Source"] {
        let props = unsafe { descriptor.clip_define(s_ifx, name) }.ok_or(StatusError::Failed)?;

        (unsafe {
            props.set_image_effect_supported_components(
                s_prop,
                &[
                    ImageEffectPropSupportedComponents::RGBA.into(),
                    ImageEffectPropSupportedComponents::Alpha.into(),
                    ImageEffectPropSupportedComponents::RGB.into(),
                ],
            )
        })
        .ok_or(StatusError::Failed)?;
    }

    let param_set = unsafe { descriptor.get_param_set(s_ifx) }.ok_or(StatusError::Failed)?;

    {
        let param_props =
            unsafe { param_set.param_define(s_param, kOfxParamTypeDouble, GAIN_PARAM_NAME) }
                .ok_or(StatusError::Failed)?;
        let param_props = ParamsDouble1DPropertySet::from(param_props);

        unsafe {
            param_props
                .set_param_double_type(s_prop, ParamPropDoubleType::Scale.into())
                .ok_or(StatusError::Failed)?;
            param_props
                .set_param_default_double(s_prop, &[1.0])
                .ok_or(StatusError::Failed)?;
            param_props
                .set_param_min_double(s_prop, &[0.0])
                .ok_or(StatusError::Failed)?;
            param_props
                .set_param_display_min_double(s_prop, &[0.0])
                .ok_or(StatusError::Failed)?;
            param_props
                .set_param_display_max_double(s_prop, &[10.0])
                .ok_or(StatusError::Failed)?;
            param_props
                .set_label(s_prop, Some(c"Gain"))
                .ok_or(StatusError::Failed)?;
            param_props
                .set_param_hint(s_prop, Some(c"How much to multiply the image by."))
                .ok_or(StatusError::Failed)?;
        }
    }

    {
        let param_props = unsafe {
            param_set.param_define(s_param, kOfxParamTypeBoolean, APPLY_TO_ALPHA_PARAM_NAME)
        }
        .ok_or(StatusError::Failed)?;
        let param_props = ParamsBytePropertySet::from(param_props);

        unsafe {
            param_props
                .set_param_default_int(s_prop, &[0])
                .ok_or(StatusError::Failed)?;
            param_props
                .set_label(s_prop, Some(c"Apply To Alpha"))
                .ok_or(StatusError::Failed)?;
            param_props
                .set_param_hint(
                    s_prop,
                    Some(c"Whether to apply the gain value to alpha as well."),
                )
                .ok_or(StatusError::Failed)?;
        }
    }

    Ok(Status::OK)
}

fn action_create_instance(instance: ImageEffectInstance) -> OutboundResult {
    let data = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;
    let s_param = &data.parameter_suite.0;

    let instance_props = unsafe { instance.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    let source_clip =
        unsafe { instance.clip_get_clip_handle(s_ifx, c"Source") }.ok_or(StatusError::Failed)?;
    let output_clip =
        unsafe { instance.clip_get_clip_handle(s_ifx, c"Output") }.ok_or(StatusError::Failed)?;

    let param_set = unsafe { instance.get_param_set(s_ifx) }.ok_or(StatusError::Failed)?;
    let gain_param = unsafe { param_set.param_get_param_handle(s_param, GAIN_PARAM_NAME) }
        .ok_or(StatusError::Failed)?;
    let apply_to_alpha_param =
        unsafe { param_set.param_get_param_handle(s_param, APPLY_TO_ALPHA_PARAM_NAME) }
            .ok_or(StatusError::Failed)?;

    let my_data = MyInstanceData {
        source_clip: GuaranteeSendClipInInstance(source_clip),
        output_clip: GuaranteeSendClipInInstance(output_clip),
        gain_param,
        apply_to_alpha_param,
    };
    let my_data_ptr = Box::into_raw(Box::new(my_data)) as *mut c_void;

    // SAFETY: the pointee is kept alive by `Box::into_raw` until it is
    // reclaimed with `Box::from_raw` in `action_destroy_instance`.
    if let r = unsafe {
        s_prop.prop_set_pointer(
            instance_props.sys_handle(),
            kOfxPropInstanceData,
            std::ptr::NonNull::new(my_data_ptr),
        )
    } && !r.is_ok()
    {
        drop(unsafe { Box::from_raw(my_data_ptr as *mut MyInstanceData) });
        return Err(StatusError::Failed);
    }

    Ok(Status::OK)
}

fn action_destroy_instance(instance: ImageEffectInstance) -> OutboundResult {
    let data = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;

    let props = unsafe { instance.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    let Some(my_data_ptr) =
        (unsafe { props.get_instance_data(s_prop) }).ok_or(StatusError::Failed)?
    else {
        return Err(StatusError::Failed);
    };

    drop(unsafe { Box::from_raw(my_data_ptr.as_ptr() as *mut MyInstanceData) });

    Ok(Status::OK)
}

fn action_is_identity(
    effect: ImageEffectInstance,
    in_args: ActionIsIdentityIn,
    out_args: ActionIsIdentityOut,
) -> OutboundResult {
    let data = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;
    let s_param = &data.parameter_suite.0;

    let instance_props = unsafe { effect.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    let Some(my_data_ptr) =
        (unsafe { instance_props.get_instance_data(s_prop) }).ok_or(StatusError::Failed)?
    else {
        return Err(StatusError::Failed);
    };
    let my_data = unsafe { &*(my_data_ptr.as_ptr() as *const MyInstanceData) };

    let time = unsafe { in_args.get_time(s_prop) }.ok_or(StatusError::Failed)?;
    let mut gain: f64 = Default::default();
    param_get_value_at_time!(s_param, my_data.gain_param, time, &mut gain);

    if (gain - 1.0).abs() < 0.000000001 {
        (unsafe { s_prop.prop_set_string(out_args.sys_handle(), kOfxPropLabel, Some(c"Source")) })
            .ok_or(StatusError::Failed)?;
        Ok(Status::OK)
    } else {
        Ok(StatusReply::ReplyDefault.into())
    }
}

fn action_render(instance: ImageEffectInstance, in_args: ActionRenderIn) -> OutboundResult {
    let data = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;
    let s_param = &data.parameter_suite.0;

    let instance_props = unsafe { instance.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    let time = unsafe { in_args.get_time(s_prop) }.ok_or(StatusError::Failed)?;
    let render_window =
        unsafe { in_args.get_image_effect_render_window(s_prop) }.ok_or(StatusError::Failed)?;
    let render_window = rect_i_from_array(&render_window);

    let Some(my_data_ptr) =
        (unsafe { instance_props.get_instance_data(s_prop) }).ok_or(StatusError::Failed)?
    else {
        return Err(StatusError::Failed);
    };
    let my_data = unsafe { &*(my_data_ptr.as_ptr() as *const MyInstanceData) };

    let mut gain: f64 = Default::default();
    param_get_value_at_time!(s_param, my_data.gain_param, time, &mut gain);
    let mut apply_to_alpha: c_int = Default::default();
    param_get_value_at_time!(
        s_param,
        my_data.apply_to_alpha_param,
        time,
        &mut apply_to_alpha
    );
    let apply_to_alpha = apply_to_alpha != 0;

    let output_img = unsafe { my_data.output_clip.0.clip_get_image(s_ifx, time, None) }
        .ok_or(StatusError::Failed)?;
    let source_img = unsafe { my_data.source_clip.0.clip_get_image(s_ifx, time, None) }
        .ok_or(StatusError::Failed)?;

    let output_img_m = unsafe { ClipImageManaged::try_new(&data, output_img) }?;
    let source_img_m = unsafe { ClipImageManaged::try_new(&data, source_img) }?;

    match output_img_m.pixel_depth() {
        BitDepth::Byte => pixel_processing(
            |f| f as u8,
            |v| v as f64,
            255u8,
            gain,
            apply_to_alpha,
            &data,
            instance,
            source_img_m,
            output_img_m,
            render_window,
        )?,
        BitDepth::Short => pixel_processing(
            |f| f as u16,
            |v| v as f64,
            65535u16,
            gain,
            apply_to_alpha,
            &data,
            instance,
            source_img_m,
            output_img_m,
            render_window,
        )?,
        BitDepth::Float => pixel_processing(
            |f| f as f32,
            |v| v as f64,
            1.0f32,
            gain,
            apply_to_alpha,
            &data,
            instance,
            source_img_m,
            output_img_m,
            render_window,
        )?,
    }

    Ok(Status::OK)
}

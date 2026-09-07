mod processing;

use std::{
    ffi::{CStr, c_void},
    sync::Mutex,
};

use openfx::{
    low::{
        Status,
        enums::{
            ImageEffectPluginRenderThreadSafety, ImageEffectPropContext,
            ImageEffectPropSupportedComponents, ImageEffectPropSupportedContexts,
            ImageEffectPropSupportedPixelDepths, ParamPropDoubleType,
        },
    },
    low_plugin::{
        Host, Plugin,
        actions::image_effect::{
            ActionDescribeInContextIn, ActionIsIdentityIn, ActionRenderIn, ImageEffectAction,
        },
        property_sets::{
            EffectDescriptorPropertySet, EffectInstancePropertySet, ParamDouble1DPropertySet,
            ParamsBytePropertySet,
        },
    },
    sys::{
        generic::core::OfxPropertySetHandle,
        image_effect_v1::{
            image_effect::{OfxImageClipHandle, OfxImageEffectHandle},
            param::{OfxParamHandle, kOfxParamTypeBoolean, kOfxParamTypeDouble},
        },
    },
    sys_helpers::{
        generic::properties::{set_OfxPropInstanceData, set_OfxPropName},
        image_effect_v1::properties::get_OfxImageEffectPropRenderWindow,
    },
};

use processing::{pixel_processing, rect_i_from_array};

use crate::{
    definitions::{PLUGIN_3_GAIN_IDENTIFIER, PLUGIN_3_GAIN_LABEL, PLUGINS_GROUPING},
    helpers::shared_data::{BitDepth, GuaranteeSend, SharedData},
};

static HOST_BEFORE_ACTION_LOAD: Mutex<Option<GuaranteeSend<Host>>> = Mutex::new(None);
static SHARED_DATA: Mutex<Option<SharedData>> = Mutex::new(None);

struct MyInstanceData {
    source_clip: OfxImageClipHandle,
    output_clip: OfxImageClipHandle,

    gain_param: OfxParamHandle,
    apply_to_alpha_param: OfxParamHandle,
}

fn shared_data_lockless() -> openfx::low::Result<SharedData> {
    let data = SHARED_DATA.lock().map_err(|_| Status::Failed)?;
    let data = data.as_ref().ok_or(Status::Failed)?;
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
            ImageEffectAction::IsIdentity {
                sys_handle,
                in_args,
                sys_out_args,
            } => action_is_identity(sys_handle as OfxImageEffectHandle, in_args, sys_out_args),
            ImageEffectAction::Render {
                sys_handle,
                in_args,
                ..
            } => action_render(sys_handle as OfxImageEffectHandle, in_args),
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
    let mut data = SHARED_DATA.lock().map_err(|_| Status::Failed)?;
    if data.take().is_none() {
        Err(Status::Failed)
    } else {
        Ok(())
    }
}

fn action_describe(descriptor: OfxImageEffectHandle) -> openfx::low::Result<()> {
    let data = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;

    let props = unsafe { data.get_property_set_from_image_effect(descriptor) }?;
    let props = EffectDescriptorPropertySet::from(props);

    unsafe {
        props.set_label(s_prop, Some(PLUGIN_3_GAIN_LABEL))?;
        props.set_image_effect_plugin_grouping(s_prop, Some(PLUGINS_GROUPING))?;
        props.set_image_effect_supported_contexts(
            s_prop,
            &[ImageEffectPropSupportedContexts::Filter],
        )?;
        props.set_image_effect_supported_pixel_depths(
            s_prop,
            &[
                ImageEffectPropSupportedPixelDepths::Float,
                ImageEffectPropSupportedPixelDepths::Short,
                ImageEffectPropSupportedPixelDepths::Byte,
            ],
        )?;
        props.set_image_effect_plugin_render_thread_safety(
            s_prop,
            ImageEffectPluginRenderThreadSafety::FullySafe,
        )?;
        props.set_image_effect_plugin_host_frame_threading(s_prop, true)?;
    }

    Ok(())
}

fn action_describe_in_context(
    descriptor: OfxImageEffectHandle,
    in_args: ActionDescribeInContextIn,
) -> openfx::low::Result<()> {
    let data = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = data.image_effect_suite_helper();

    let context = unsafe { in_args.get_image_effect_context(s_prop) }?;
    if context != ImageEffectPropContext::Filter {
        return Err(Status::ErrUnsupported);
    }

    for name in [c"Output", c"Source"] {
        let props = unsafe { s_ifx.clip_define(descriptor, name) }?;

        (unsafe {
            props.set_image_effect_supported_components(
                s_prop,
                &[
                    ImageEffectPropSupportedComponents::RGBA,
                    ImageEffectPropSupportedComponents::Alpha,
                    ImageEffectPropSupportedComponents::RGB,
                ],
            )
        })?;
    }

    let param_set = unsafe { data.make_param_set_helper_for_image_effect(descriptor) }?;

    {
        let param_props = param_set.param_define(kOfxParamTypeDouble, GAIN_PARAM_NAME)?;
        let param_props = ParamDouble1DPropertySet::from(param_props);

        unsafe {
            param_props.set_param_double_type(s_prop, ParamPropDoubleType::Scale)?;
            param_props.set_param_default_double(s_prop, &[1.0])?;
            param_props.set_param_min_double(s_prop, &[0.0])?;
            param_props.set_param_display_min_double(s_prop, &[0.0])?;
            param_props.set_param_display_max_double(s_prop, &[10.0])?;
            param_props.set_label(s_prop, Some(c"Gain"))?;
            param_props.set_param_hint(s_prop, Some(c"How much to multiply the image by."))?;
        }
    }

    {
        let param_props =
            param_set.param_define(kOfxParamTypeBoolean, APPLY_TO_ALPHA_PARAM_NAME)?;
        let param_props = ParamsBytePropertySet::from(param_props);

        unsafe {
            param_props.set_param_default_int(s_prop, &[0])?;
            param_props.set_label(s_prop, Some(c"Apply To Alpha"))?;
            param_props.set_param_hint(
                s_prop,
                Some(c"Whether to apply the gain value to alpha as well."),
            )?;
        }
    }

    Ok(())
}

fn action_create_instance(instance: OfxImageEffectHandle) -> openfx::low::Result<()> {
    let data = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = data.image_effect_suite_helper();

    let instance_props = unsafe { data.get_property_set_from_image_effect(instance) }?;

    let source_clip = unsafe { s_ifx.clip_get_handle(instance, c"Source") }?;
    let output_clip = unsafe { s_ifx.clip_get_handle(instance, c"Output") }?;

    let param_set = unsafe { data.make_param_set_helper_for_image_effect(instance) }?;
    let gain_param = param_set.param_get_handle(GAIN_PARAM_NAME)?;
    let apply_to_alpha_param = param_set.param_get_handle(APPLY_TO_ALPHA_PARAM_NAME)?;

    let my_data = MyInstanceData {
        source_clip,
        output_clip,
        gain_param,
        apply_to_alpha_param,
    };
    let my_data_ptr = Box::into_raw(Box::new(my_data)) as *mut c_void;

    // SAFETY: the pointee is kept alive by `Box::into_raw` until it is
    // reclaimed with `Box::from_raw` in `action_destroy_instance`.
    match unsafe { set_OfxPropInstanceData(s_prop.sys_ptr(), instance_props, my_data_ptr) } {
        Ok(_) => Ok(()),
        Err(err) => {
            drop(unsafe { Box::from_raw(my_data_ptr as *mut MyInstanceData) });
            Err(Status::from(err))
        }
    }
}

fn action_destroy_instance(instance: OfxImageEffectHandle) -> openfx::low::Result<()> {
    let data = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;

    let props = unsafe { data.get_property_set_from_image_effect(instance) }?;
    let props = EffectInstancePropertySet::from(props);

    let Some(my_data_ptr) = (unsafe { props.get_instance_data(s_prop)? }) else {
        return Err(Status::Failed);
    };

    drop(unsafe { Box::from_raw(my_data_ptr.as_ptr() as *mut MyInstanceData) });

    Ok(())
}

fn action_is_identity(
    effect: OfxImageEffectHandle,
    in_args: ActionIsIdentityIn,
    out_args: OfxPropertySetHandle,
) -> openfx::low::Result<()> {
    let data = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_param = data.parameter_suite_helper();

    let instance_props = unsafe { data.get_property_set_from_image_effect(effect) }?;
    let instance_props = EffectInstancePropertySet::from(instance_props);

    let Some(my_data_ptr) = (unsafe { instance_props.get_instance_data(s_prop)? }) else {
        return Err(Status::Failed);
    };
    let my_data = unsafe { &*(my_data_ptr.as_ptr() as *const MyInstanceData) };

    let time = unsafe { in_args.get_time(s_prop) }?;
    let gain = unsafe { s_param.param_get_value_at_time_double(my_data.gain_param, time) }?;

    if (gain - 1.0).abs() < 0.000000001 {
        (unsafe { set_OfxPropName(s_prop.sys_ptr(), out_args, c"Source".as_ptr()) })?;
        Ok(())
    } else {
        Err(Status::ReplyDefault)
    }
}

fn action_render(
    instance: OfxImageEffectHandle,
    in_args: ActionRenderIn,
) -> openfx::low::Result<()> {
    let data = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_param = data.parameter_suite_helper();

    let instance_props = unsafe { data.get_property_set_from_image_effect(instance) }?;
    let instance_props = EffectInstancePropertySet::from(instance_props);

    let time = unsafe { in_args.get_time(s_prop) }?;
    let render_window =
        unsafe { get_OfxImageEffectPropRenderWindow(s_prop.sys_ptr(), in_args.sys_handle()) }?;
    let render_window = rect_i_from_array(&render_window);

    let Some(my_data_ptr) = (unsafe { instance_props.get_instance_data(s_prop)? }) else {
        return Err(Status::Failed);
    };
    let my_data = unsafe { &*(my_data_ptr.as_ptr() as *const MyInstanceData) };

    let gain = unsafe { s_param.param_get_value_at_time_double(my_data.gain_param, time) }?;
    let apply_to_alpha =
        unsafe { s_param.param_get_value_at_time_int(my_data.apply_to_alpha_param, time) }? != 0;

    let Some(output_img_m) =
        unsafe { data.make_clip_image_managed(my_data.output_clip, time, None) }?
    else {
        return Err(Status::Failed);
    };
    let Some(source_img_m) =
        unsafe { data.make_clip_image_managed(my_data.source_clip, time, None) }?
    else {
        return Err(Status::Failed);
    };

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
        ),
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
        ),
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
        ),
    }
}

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
        objects::{ImageEffectDescriptor, ImageEffectInstance},
        property_sets::ParamDouble1DPropertySet,
    },
    sys::{
        generic::core::OfxPropertySetHandle,
        image_effect_v1::{
            image_effect::OfxImageClipHandle,
            param::{OfxParamHandle, kOfxParamTypeDouble},
        },
    },
    sys_helpers::{
        generic::properties::{set_OfxPropInstanceData, set_OfxPropName},
        image_effect_v1::properties::get_OfxImageEffectPropRenderWindow,
    },
};

use processing::{pixel_processing, rect_i_from_array};

use crate::{
    definitions::{PLUGIN_4_SATURATION_IDENTIFIER, PLUGIN_4_SATURATION_LABEL, PLUGINS_GROUPING},
    helpers::shared_data::{BitDepth, GuaranteeSend, SharedData},
};

static HOST_BEFORE_ACTION_LOAD: Mutex<Option<GuaranteeSend<Host>>> = Mutex::new(None);
static SHARED_DATA: Mutex<Option<SharedData>> = Mutex::new(None);

struct MyInstanceData {
    #[expect(unused)]
    is_general_context: bool,

    source_clip: OfxImageClipHandle,
    output_clip: OfxImageClipHandle,
    mask_clip: Option<OfxImageClipHandle>,

    saturation_param: OfxParamHandle,
}

fn shared_data_lockless() -> openfx::low::Result<SharedData> {
    let data = SHARED_DATA.lock().map_err(|_| Status::Failed)?;
    let data = data.as_ref().ok_or(Status::Failed)?;
    Ok(data.clone())
}

const SATURATION_PARAM_NAME: &CStr = c"saturation";

pub struct PluginExampleSaturation;
impl Plugin for PluginExampleSaturation {
    const PLUGIN_IDENTIFIER: &'static CStr = PLUGIN_4_SATURATION_IDENTIFIER;
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
            ImageEffectAction::IsIdentity {
                handle,
                in_args,
                sys_out_args,
            } => action_is_identity(handle, in_args, sys_out_args),
            ImageEffectAction::Render {
                handle, in_args, ..
            } => action_render(handle, in_args),
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

fn action_describe(descriptor: ImageEffectDescriptor) -> openfx::low::Result<()> {
    let data = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;

    let props = unsafe { data.get_property_set_from_image_effect_descriptor(&descriptor) }?;

    unsafe {
        props.set_label(s_prop, Some(PLUGIN_4_SATURATION_LABEL))?;
        props.set_image_effect_plugin_grouping(s_prop, Some(PLUGINS_GROUPING))?;
        props.set_image_effect_supported_contexts(
            s_prop,
            &[
                ImageEffectPropSupportedContexts::Filter,
                ImageEffectPropSupportedContexts::General,
            ],
        )?;
        props.set_image_effect_supported_pixel_depths(
            s_prop,
            &[
                ImageEffectPropSupportedPixelDepths::Byte,
                ImageEffectPropSupportedPixelDepths::Short,
                ImageEffectPropSupportedPixelDepths::Float,
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
    descriptor: ImageEffectDescriptor,
    in_args: ActionDescribeInContextIn,
) -> openfx::low::Result<()> {
    let data = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = data.image_effect_suite_helper();

    let context = unsafe { in_args.get_image_effect_context(s_prop) }?;
    if context != ImageEffectPropContext::Filter && context != ImageEffectPropContext::General {
        return Err(Status::ErrUnsupported);
    }

    for name in [c"Output", c"Source"] {
        let props = unsafe { s_ifx.clip_define(&descriptor, name) }?;

        (unsafe {
            props.set_image_effect_supported_components(
                s_prop,
                &[
                    ImageEffectPropSupportedComponents::RGBA,
                    ImageEffectPropSupportedComponents::RGB,
                ],
            )
        })?;
    }
    if context == ImageEffectPropContext::General {
        let props = unsafe { s_ifx.clip_define(&descriptor, c"Mask") }?;

        unsafe {
            props.set_image_effect_supported_components(
                s_prop,
                &[ImageEffectPropSupportedComponents::Alpha],
            )?;
            props.set_image_clip_optional(s_prop, true)?;
            props.set_image_clip_is_mask(s_prop, true)?;
        }
    }

    let param_set = unsafe { data.make_param_set_helper_for_image_effect_descriptor(&descriptor) }?;

    {
        let param_props = param_set.param_define(kOfxParamTypeDouble, SATURATION_PARAM_NAME)?;
        let param_props = ParamDouble1DPropertySet::from(param_props);

        unsafe {
            param_props.set_param_double_type(s_prop, ParamPropDoubleType::Scale)?;
            param_props.set_param_default_double(s_prop, &[1.0])?;
            param_props.set_param_display_min_double(s_prop, &[-2.0])?;
            param_props.set_param_display_max_double(s_prop, &[2.0])?;
            param_props.set_label(s_prop, Some(c"Saturation"))?;
            param_props.set_param_hint(s_prop, Some(c"How saturated the image should be."))?;
        }
    }

    Ok(())
}

fn action_create_instance(instance: ImageEffectInstance) -> openfx::low::Result<()> {
    let data = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = data.image_effect_suite_helper();

    let instance_props = unsafe { data.get_property_set_from_image_effect_instance(&instance) }?;

    let context = unsafe { instance_props.get_image_effect_context(s_prop) }?;
    let is_general_context = context == ImageEffectPropContext::General;

    let source_clip = unsafe { s_ifx.clip_get_handle(&instance, c"Source") }?;
    let output_clip = unsafe { s_ifx.clip_get_handle(&instance, c"Output") }?;
    let mask_clip = if is_general_context {
        Some(unsafe { s_ifx.clip_get_handle(&instance, c"Mask") }?)
    } else {
        None
    };

    let param_set = unsafe { data.make_param_set_helper_for_image_effect_instance(&instance) }?;
    let saturation_param = param_set.param_get_handle(SATURATION_PARAM_NAME)?;

    let my_data = MyInstanceData {
        is_general_context,
        source_clip,
        output_clip,
        mask_clip,
        saturation_param,
    };
    let my_data_ptr = Box::into_raw(Box::new(my_data)) as *mut c_void;

    // SAFETY: the pointee is kept alive by `Box::into_raw` until it is
    // reclaimed with `Box::from_raw` in `action_destroy_instance`.
    match unsafe {
        set_OfxPropInstanceData(s_prop.sys_ptr(), instance_props.sys_handle(), my_data_ptr)
    } {
        Ok(_) => Ok(()),
        Err(err) => {
            drop(unsafe { Box::from_raw(my_data_ptr as *mut MyInstanceData) });
            Err(Status::from(err))
        }
    }
}

fn action_destroy_instance(instance: ImageEffectInstance) -> openfx::low::Result<()> {
    let data = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;

    let props = unsafe { data.get_property_set_from_image_effect_instance(&instance) }?;

    let Some(my_data_ptr) = (unsafe { props.get_instance_data(s_prop)? }) else {
        return Err(Status::Failed);
    };

    drop(unsafe { Box::from_raw(my_data_ptr.as_ptr() as *mut MyInstanceData) });

    Ok(())
}

fn action_is_identity(
    effect: ImageEffectInstance,
    in_args: ActionIsIdentityIn,
    out_args: OfxPropertySetHandle,
) -> openfx::low::Result<()> {
    let data = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_param = data.parameter_suite_helper();

    let instance_props = unsafe { data.get_property_set_from_image_effect_instance(&effect) }?;

    let Some(my_data_ptr) = (unsafe { instance_props.get_instance_data(s_prop)? }) else {
        return Err(Status::Failed);
    };
    let my_data = unsafe { &*(my_data_ptr.as_ptr() as *const MyInstanceData) };

    let time = unsafe { in_args.get_time(s_prop) }?;
    let saturation =
        unsafe { s_param.param_get_value_at_time_double(my_data.saturation_param, time) }?;

    if (saturation - 1.0).abs() < 0.000000001 {
        unsafe { set_OfxPropName(s_prop.sys_ptr(), out_args, c"Source".as_ptr()) }?;
        Ok(())
    } else {
        Err(Status::ReplyDefault)
    }
}

fn action_render(
    instance: ImageEffectInstance,
    in_args: ActionRenderIn,
) -> openfx::low::Result<()> {
    let data = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_param = data.parameter_suite_helper();

    let instance_props = unsafe { data.get_property_set_from_image_effect_instance(&instance) }?;

    let time = unsafe { in_args.get_time(s_prop) }?;
    let render_window =
        unsafe { get_OfxImageEffectPropRenderWindow(s_prop.sys_ptr(), in_args.sys_handle()) }?;
    let render_window = rect_i_from_array(&render_window);

    let Some(my_data_ptr) = (unsafe { instance_props.get_instance_data(s_prop)? }) else {
        return Err(Status::Failed);
    };
    let my_data = unsafe { &*(my_data_ptr.as_ptr() as *const MyInstanceData) };

    let saturation =
        unsafe { s_param.param_get_value_at_time_double(my_data.saturation_param, time) }?;

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
    let mask_img_m = if let Some(mask_clip) = my_data.mask_clip {
        #[expect(clippy::needless_match, clippy::manual_map)]
        match unsafe { data.make_clip_image_managed(mask_clip, time, None) }? {
            Some(mask_img_m) => Some(mask_img_m),
            // copilot:
            //
            // ```md
            // an optional but unconnected Mask clip commonly returns `None`
            // from `clip_get_image`;
            // ```
            None => {
                // return Err(Status::Failed);
                None
            }
        }
    } else {
        None
    };

    match output_img_m.pixel_depth() {
        BitDepth::Byte => pixel_processing(
            |f| f as u8,
            |v| v as f64,
            255u8,
            saturation,
            &data,
            instance,
            source_img_m,
            mask_img_m,
            output_img_m,
            render_window,
        ),
        BitDepth::Short => pixel_processing(
            |f| f as u16,
            |v| v as f64,
            65535u16,
            saturation,
            &data,
            instance,
            source_img_m,
            mask_img_m,
            output_img_m,
            render_window,
        ),
        BitDepth::Float => pixel_processing(
            |f| f as f32,
            |v| v as f64,
            1.0f32,
            saturation,
            &data,
            instance,
            source_img_m,
            mask_img_m,
            output_img_m,
            render_window,
        ),
    }
}

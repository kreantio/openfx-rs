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
            ActionDescribeInContextIn, ActionIsIdentityIn, ActionIsIdentityOut, ActionRenderIn,
            ImageEffectAction,
        },
        objects::{ImageEffectDescriptor, ImageEffectInstance},
        property_sets::ParamsDouble1DPropertySet,
    },
    sys::{
        generic::core::{kOfxPropInstanceData, kOfxPropLabel},
        image_effect_v1::param::{OfxParamHandle, kOfxParamTypeDouble},
    },
};

use processing::{pixel_processing, rect_i_from_array};

use crate::{
    definitions::{PLUGIN_4_SATURATION_IDENTIFIER, PLUGIN_4_SATURATION_LABEL, PLUGINS_GROUPING},
    helpers::shared_data::{
        BitDepth, ClipImageManaged, GuaranteeSend, GuaranteeSendClipInInstance, SharedData,
        param_get_value_at_time,
    },
};

static HOST_BEFORE_ACTION_LOAD: Mutex<Option<GuaranteeSend<Host>>> = Mutex::new(None);
static SHARED_DATA: Mutex<Option<SharedData>> = Mutex::new(None);

struct MyInstanceData {
    #[expect(unused)]
    is_general_context: bool,

    source_clip: GuaranteeSendClipInInstance,
    output_clip: GuaranteeSendClipInInstance,
    mask_clip: Option<GuaranteeSendClipInInstance>,

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
                out_args,
            } => action_is_identity(handle, in_args, out_args),
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
    let s_ifx = &data.image_effect_suite.0;

    let props = unsafe { descriptor.get_property_set(s_ifx) }?;

    unsafe {
        props.set_label(s_prop, Some(PLUGIN_4_SATURATION_LABEL))?;
        props.set_image_effect_plugin_grouping(s_prop, Some(PLUGINS_GROUPING))?;
        props.set_image_effect_supported_contexts(
            s_prop,
            &[
                ImageEffectPropSupportedContexts::Filter.into(),
                ImageEffectPropSupportedContexts::General.into(),
            ],
        )?;
        props.set_image_effect_supported_pixel_depths(
            s_prop,
            &[
                ImageEffectPropSupportedPixelDepths::Byte.into(),
                ImageEffectPropSupportedPixelDepths::Short.into(),
                ImageEffectPropSupportedPixelDepths::Float.into(),
            ],
        )?;
        props.set_image_effect_plugin_render_thread_safety(
            s_prop,
            ImageEffectPluginRenderThreadSafety::FullySafe.into(),
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
    let s_ifx = &data.image_effect_suite.0;
    let s_param = &data.parameter_suite.0;

    let context = unsafe { in_args.get_image_effect_context(s_prop) }?;
    if context != ImageEffectPropContext::Filter && context != ImageEffectPropContext::General {
        return Err(Status::ErrUnsupported);
    }

    for name in [c"Output", c"Source"] {
        let props = unsafe { descriptor.clip_define(s_ifx, name) }?;

        (unsafe {
            props.set_image_effect_supported_components(
                s_prop,
                &[
                    ImageEffectPropSupportedComponents::RGBA.into(),
                    ImageEffectPropSupportedComponents::RGB.into(),
                ],
            )
        })?;
    }
    if context == ImageEffectPropContext::General {
        let props = unsafe { descriptor.clip_define(s_ifx, c"Mask") }?;

        unsafe {
            props.set_image_effect_supported_components(
                s_prop,
                &[ImageEffectPropSupportedComponents::Alpha.into()],
            )?;
            props.set_image_clip_optional(s_prop, true)?;
            props.set_image_clip_is_mask(s_prop, true)?;
        }
    }

    let param_set = unsafe { descriptor.get_param_set(s_ifx) }?;

    {
        let param_props =
            unsafe { param_set.param_define(s_param, kOfxParamTypeDouble, SATURATION_PARAM_NAME) }?;
        let param_props = ParamsDouble1DPropertySet::from(param_props);

        unsafe {
            param_props.set_param_double_type(s_prop, ParamPropDoubleType::Scale.into())?;
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
    let s_ifx = &data.image_effect_suite.0;
    let s_param = &data.parameter_suite.0;

    let instance_props = unsafe { instance.get_property_set(s_ifx) }?;

    let context = unsafe { instance_props.get_image_effect_context(s_prop) }?;
    let is_general_context = context == ImageEffectPropContext::General;

    let source_clip = unsafe { instance.clip_get_clip_handle(s_ifx, c"Source") }?;
    let output_clip = unsafe { instance.clip_get_clip_handle(s_ifx, c"Output") }?;
    let mask_clip = if is_general_context {
        Some(unsafe { instance.clip_get_clip_handle(s_ifx, c"Mask") }?)
    } else {
        None
    };

    let param_set = unsafe { instance.get_param_set(s_ifx) }?;
    let saturation_param =
        unsafe { param_set.param_get_param_handle(s_param, SATURATION_PARAM_NAME) }?;

    let my_data = MyInstanceData {
        is_general_context,
        source_clip: GuaranteeSendClipInInstance(source_clip),
        output_clip: GuaranteeSendClipInInstance(output_clip),
        mask_clip: mask_clip.map(GuaranteeSendClipInInstance),
        saturation_param,
    };
    let my_data_ptr = Box::into_raw(Box::new(my_data)) as *mut c_void;

    // SAFETY: the pointee is kept alive by `Box::into_raw` until it is
    // reclaimed with `Box::from_raw` in `action_destroy_instance`.
    match unsafe {
        s_prop.prop_set_pointer(
            instance_props.sys_handle(),
            kOfxPropInstanceData,
            std::ptr::NonNull::new(my_data_ptr),
        )
    } {
        Ok(()) => (),
        Err(e) => {
            drop(unsafe { Box::from_raw(my_data_ptr as *mut MyInstanceData) });
            return Err(e);
        }
    };

    Ok(())
}

fn action_destroy_instance(instance: ImageEffectInstance) -> openfx::low::Result<()> {
    let data = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;

    let props = unsafe { instance.get_property_set(s_ifx) }?;

    let Some(my_data_ptr) = (unsafe { props.get_instance_data(s_prop)? }) else {
        return Err(Status::Failed);
    };

    drop(unsafe { Box::from_raw(my_data_ptr.as_ptr() as *mut MyInstanceData) });

    Ok(())
}

fn action_is_identity(
    effect: ImageEffectInstance,
    in_args: ActionIsIdentityIn,
    out_args: ActionIsIdentityOut,
) -> openfx::low::Result<()> {
    let data = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;
    let s_param = &data.parameter_suite.0;

    let instance_props = unsafe { effect.get_property_set(s_ifx) }?;

    let Some(my_data_ptr) = (unsafe { instance_props.get_instance_data(s_prop)? }) else {
        return Err(Status::Failed);
    };
    let my_data = unsafe { &*(my_data_ptr.as_ptr() as *const MyInstanceData) };

    let time = unsafe { in_args.get_time(s_prop) }?;
    let mut saturation: f64 = Default::default();
    param_get_value_at_time!(s_param, my_data.saturation_param, time, &mut saturation);

    if (saturation - 1.0).abs() < 0.000000001 {
        (unsafe { s_prop.prop_set_string(out_args.sys_handle(), kOfxPropLabel, Some(c"Source")) })?;
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
    let s_ifx = &data.image_effect_suite.0;
    let s_param = &data.parameter_suite.0;

    let instance_props = unsafe { instance.get_property_set(s_ifx) }?;

    let time = unsafe { in_args.get_time(s_prop) }?;
    let render_window = unsafe { in_args.get_image_effect_render_window(s_prop) }?;
    let render_window = rect_i_from_array(&render_window);

    let Some(my_data_ptr) = (unsafe { instance_props.get_instance_data(s_prop)? }) else {
        return Err(Status::Failed);
    };
    let my_data = unsafe { &*(my_data_ptr.as_ptr() as *const MyInstanceData) };

    let mut saturation: f64 = Default::default();
    param_get_value_at_time!(s_param, my_data.saturation_param, time, &mut saturation);

    let output_img = unsafe { my_data.output_clip.0.clip_get_image(s_ifx, time, None) }?;
    let source_img = unsafe { my_data.source_clip.0.clip_get_image(s_ifx, time, None) }?;
    let mask_img = if let Some(mask_clip) = my_data.mask_clip {
        match unsafe { mask_clip.0.clip_get_image(s_ifx, time, None) } {
            Ok(img) => Some(img),
            Err(Status::Failed) => None, // unconnected
            Err(err) => return Err(err),
        }
    } else {
        None
    };

    let output_img_m = unsafe { ClipImageManaged::try_new(&data, output_img) }?;
    let source_img_m = unsafe { ClipImageManaged::try_new(&data, source_img) }?;
    let mask_img_m = if let Some(mask_img) = mask_img {
        Some(unsafe { ClipImageManaged::try_new(&data, mask_img) }?)
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

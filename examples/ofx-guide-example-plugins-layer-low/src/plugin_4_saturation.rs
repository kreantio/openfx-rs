mod processing;

use std::{
    ffi::{CStr, c_void},
    sync::Mutex,
};

use openfx::{
    low::{Status, enums::ImageEffectPropContext},
    low_plugin::{
        Host, Plugin,
        actions::image_effect::{
            ActionDescribeInContextIn, ActionIsIdentityIn, ActionRenderIn, ImageEffectAction,
        },
    },
    sys::{
        generic::core::{
            OfxPropertySetHandle, OfxRectI, OfxStatus, kOfxBitDepthByte, kOfxBitDepthFloat,
            kOfxBitDepthShort, kOfxStatFailed,
        },
        image_effect_v1::{
            image_effect::{
                OfxImageClipHandle, OfxImageEffectHandle, kOfxImageComponentAlpha,
                kOfxImageComponentRGB, kOfxImageComponentRGBA, kOfxImageEffectContextFilter,
                kOfxImageEffectContextGeneral, kOfxImageEffectRenderFullySafe,
            },
            param::{OfxParamHandle, kOfxParamDoubleTypeScale, kOfxParamTypeDouble},
        },
    },
    sys_helpers::{
        generic::properties::{
            get_OfxPropInstanceData, set_OfxPropInstanceData, set_OfxPropLabel, set_OfxPropName,
        },
        image_effect_v1::properties::{
            get_OfxImageEffectPropContext, get_OfxImageEffectPropRenderWindow,
            set_OfxImageClipPropIsMask, set_OfxImageClipPropOptional,
            set_OfxImageEffectPluginPropGrouping, set_OfxImageEffectPluginPropHostFrameThreading,
            set_OfxImageEffectPluginRenderThreadSafety, set_OfxImageEffectPropSupportedComponents,
            set_OfxImageEffectPropSupportedContexts, set_OfxImageEffectPropSupportedPixelDepths,
            set_OfxParamPropDefault_Double, set_OfxParamPropDisplayMax_Double,
            set_OfxParamPropDisplayMin_Double, set_OfxParamPropDoubleType, set_OfxParamPropHint,
        },
    },
};

use processing::{pixel_processing, rect_i_from_array};

use crate::{
    definitions::{PLUGIN_4_SATURATION_IDENTIFIER, PLUGIN_4_SATURATION_LABEL, PLUGINS_GROUPING},
    helpers::{
        GuaranteeSend, SharedData,
        shared_data_helper::{BitDepth, ClipImageManaged, SharedDataHelper},
    },
};

static HOST_BEFORE_ACTION_LOAD: Mutex<Option<GuaranteeSend<Host>>> = Mutex::new(None);
static SHARED_DATA: Mutex<Option<SharedData<'static>>> = Mutex::new(None);

struct MyInstanceData {
    #[expect(unused)]
    is_general_context: bool,

    source_clip: OfxImageClipHandle,
    output_clip: OfxImageClipHandle,
    mask_clip: Option<OfxImageClipHandle>,

    saturation_param: OfxParamHandle,
}

fn shared_data_lockless() -> Result<SharedData<'static>, OfxStatus> {
    let data = SHARED_DATA.lock().map_err(|_| kOfxStatFailed)?;
    let data = data.as_ref().ok_or(kOfxStatFailed)?;
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
        .map_err(|_| kOfxStatFailed)?
        .take()
        .ok_or(kOfxStatFailed)?;

    let mut data = SHARED_DATA.lock().map_err(|_| kOfxStatFailed)?;
    if data.is_some() {
        Err(Status::Failed)
    } else {
        *data = Some(SharedData::try_new(host)?);
        Ok(())
    }
}

fn action_unload() -> openfx::low::Result<()> {
    let mut data = SHARED_DATA.lock().map_err(|_| kOfxStatFailed)?;
    if data.take().is_none() {
        Err(Status::Failed)
    } else {
        Ok(())
    }
}

fn action_describe(descriptor: OfxImageEffectHandle) -> openfx::low::Result<()> {
    let data = shared_data_lockless()?;
    let data = unsafe { SharedDataHelper::try_new(&data) }?;

    let s_prop = data.inner().property_suite;

    let descriptor = unsafe { data.get_property_set_from_image_effect(descriptor) }?;

    unsafe {
        set_OfxPropLabel(s_prop, descriptor, PLUGIN_4_SATURATION_LABEL.as_ptr())?;
        set_OfxImageEffectPluginPropGrouping(s_prop, descriptor, PLUGINS_GROUPING.as_ptr())?;
        set_OfxImageEffectPropSupportedContexts(
            s_prop,
            descriptor,
            &[
                kOfxImageEffectContextFilter.as_ptr(),
                kOfxImageEffectContextGeneral.as_ptr(),
            ],
        )?;
        set_OfxImageEffectPropSupportedPixelDepths(
            s_prop,
            descriptor,
            &[
                kOfxBitDepthByte.as_ptr(),
                kOfxBitDepthShort.as_ptr(),
                kOfxBitDepthFloat.as_ptr(),
            ],
        )?;
        set_OfxImageEffectPluginRenderThreadSafety(
            s_prop,
            descriptor,
            kOfxImageEffectRenderFullySafe.as_ptr(),
        )?;
        set_OfxImageEffectPluginPropHostFrameThreading(s_prop, descriptor, 1)?;
    }

    Ok(())
}

fn action_describe_in_context(
    descriptor: OfxImageEffectHandle,
    in_args: ActionDescribeInContextIn,
) -> openfx::low::Result<()> {
    let data = shared_data_lockless()?;
    let data = unsafe { SharedDataHelper::try_new(&data) }?;

    let s_prop = data.inner().property_suite;
    let s_ifx = data.image_effect_suite_helper();

    let context = unsafe { in_args.get_image_effect_context(s_prop) }?;
    if context != ImageEffectPropContext::Filter && context != ImageEffectPropContext::General {
        return Err(Status::ErrUnsupported);
    }

    for name in [c"Output", c"Source"] {
        let props = unsafe { s_ifx.clip_define(descriptor, name) }?;

        unsafe {
            set_OfxImageEffectPropSupportedComponents(
                s_prop,
                props,
                &[
                    kOfxImageComponentRGBA.as_ptr(),
                    kOfxImageComponentRGB.as_ptr(),
                ],
            )
        }?;
    }
    if context == ImageEffectPropContext::General {
        let props = unsafe { s_ifx.clip_define(descriptor, c"Mask") }?;

        unsafe {
            set_OfxImageEffectPropSupportedComponents(
                s_prop,
                props,
                &[kOfxImageComponentAlpha.as_ptr()],
            )?;
            set_OfxImageClipPropOptional(s_prop, props, 1)?;
            set_OfxImageClipPropIsMask(s_prop, props, 1)?;
        }
    }

    let param_set = unsafe { data.make_param_set_helper_for_image_effect(descriptor) }?;

    {
        let param_props = param_set.param_define(kOfxParamTypeDouble, SATURATION_PARAM_NAME)?;
        unsafe {
            set_OfxParamPropDoubleType(s_prop, param_props, kOfxParamDoubleTypeScale.as_ptr())?;
            set_OfxParamPropDefault_Double(s_prop, param_props, &[1.0])?;
            set_OfxParamPropDisplayMin_Double(s_prop, param_props, &[-2.0])?;
            set_OfxParamPropDisplayMax_Double(s_prop, param_props, &[2.0])?;
            set_OfxPropLabel(s_prop, param_props, c"Saturation".as_ptr())?;
            set_OfxParamPropHint(
                s_prop,
                param_props,
                c"How saturated the image should be.".as_ptr(),
            )?;
        }
    }

    Ok(())
}

fn action_create_instance(instance: OfxImageEffectHandle) -> openfx::low::Result<()> {
    let data = shared_data_lockless()?;
    let data = unsafe { SharedDataHelper::try_new(&data) }?;

    let s_prop = data.inner().property_suite;
    let s_ifx = data.image_effect_suite_helper();

    let instance_props = unsafe { data.get_property_set_from_image_effect(instance) }?;

    let context = unsafe { get_OfxImageEffectPropContext(s_prop, instance_props) }?;
    if context.is_null() {
        return Err(Status::ErrUnsupported);
    }
    let context = unsafe { CStr::from_ptr(context) };
    let is_general_context = context == kOfxImageEffectContextGeneral;

    let source_clip = unsafe { s_ifx.clip_get_handle(instance, c"Source") }?;
    let output_clip = unsafe { s_ifx.clip_get_handle(instance, c"Output") }?;
    let mask_clip = if is_general_context {
        Some(unsafe { s_ifx.clip_get_handle(instance, c"Mask") }?)
    } else {
        None
    };

    let param_set = unsafe { data.make_param_set_helper_for_image_effect(instance) }?;
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
    match unsafe { set_OfxPropInstanceData(s_prop, instance_props, my_data_ptr) } {
        Ok(_) => Ok(()),
        Err(err) => {
            drop(unsafe { Box::from_raw(my_data_ptr as *mut MyInstanceData) });
            Err(Status::from(err))
        }
    }
}

fn action_destroy_instance(instance: OfxImageEffectHandle) -> openfx::low::Result<()> {
    let data = shared_data_lockless()?;
    let data = unsafe { SharedDataHelper::try_new(&data) }?;

    let s_prop = data.inner().property_suite;

    let instance_props = unsafe { data.get_property_set_from_image_effect(instance) }?;
    let my_data_ptr = unsafe { get_OfxPropInstanceData(s_prop, instance_props) }?;
    if my_data_ptr.is_null() {
        return Err(Status::Failed);
    }

    drop(unsafe { Box::from_raw(my_data_ptr as *mut MyInstanceData) });

    Ok(())
}

fn action_is_identity(
    effect: OfxImageEffectHandle,
    in_args: ActionIsIdentityIn,
    out_args: OfxPropertySetHandle,
) -> openfx::low::Result<()> {
    let data = shared_data_lockless()?;
    let data = unsafe { SharedDataHelper::try_new(&data) }?;

    let s_prop = data.inner().property_suite;
    let s_param = data.parameter_suite_helper();

    let instance_props = unsafe { data.get_property_set_from_image_effect(effect) }?;
    let my_data_ptr = unsafe { get_OfxPropInstanceData(s_prop, instance_props) }?;
    if my_data_ptr.is_null() {
        return Err(Status::Failed);
    }
    let my_data = unsafe { &*(my_data_ptr as *const MyInstanceData) };

    let time = unsafe { in_args.get_time(s_prop) }?;
    let saturation =
        unsafe { s_param.param_get_value_at_time_double(my_data.saturation_param, time) }?;

    if (saturation - 1.0).abs() < 0.000000001 {
        unsafe { set_OfxPropName(s_prop, out_args, c"Source".as_ptr()) }?;
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
    let data = unsafe { SharedDataHelper::try_new(&data) }?;

    let s_prop = data.inner().property_suite;
    let s_param = data.parameter_suite_helper();

    let instance_props = unsafe { data.get_property_set_from_image_effect(instance) }?;

    let time = unsafe { in_args.get_time(s_prop) }?;
    let render_window =
        unsafe { get_OfxImageEffectPropRenderWindow(s_prop, in_args.sys_handle()) }?;
    let render_window = rect_i_from_array(&render_window);

    let my_data_ptr = unsafe { get_OfxPropInstanceData(s_prop, instance_props) }?;
    if my_data_ptr.is_null() {
        return Err(Status::Failed);
    }
    let my_data = unsafe { &*(my_data_ptr as *const MyInstanceData) };

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
                // return Err(OfxStat::kOfxStatFailed);
                None
            }
        }
    } else {
        None
    };

    fn inner(
        saturation: f64,
        data: &SharedDataHelper,
        instance: OfxImageEffectHandle,
        source_img: ClipImageManaged,
        mask_img: Option<ClipImageManaged>,
        output_img: ClipImageManaged,
        render_window: OfxRectI,
    ) -> openfx::low::Result<()> {
        match output_img.pixel_depth() {
            BitDepth::Byte => pixel_processing(
                |f| f as u8,
                |v| v as f64,
                255u8,
                saturation,
                data,
                instance,
                source_img,
                mask_img,
                output_img,
                render_window,
            ),
            BitDepth::Short => pixel_processing(
                |f| f as u16,
                |v| v as f64,
                65535u16,
                saturation,
                data,
                instance,
                source_img,
                mask_img,
                output_img,
                render_window,
            ),
            BitDepth::Float => pixel_processing(
                |f| f as f32,
                |v| v as f64,
                1.0f32,
                saturation,
                data,
                instance,
                source_img,
                mask_img,
                output_img,
                render_window,
            ),
        }?;

        Ok(())
    }

    inner(
        saturation,
        &data,
        instance,
        source_img_m,
        mask_img_m,
        output_img_m,
        render_window,
    )
}

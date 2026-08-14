mod processing;

use std::{
    ffi::{CStr, c_char, c_int, c_void},
    sync::{Mutex, OnceLock},
};

use openfx::{
    generic::sys::core::{
        OfxHost, OfxPropertySetHandle, OfxRectI, OfxStatus, OfxTime, kOfxActionCreateInstance,
        kOfxActionDescribe, kOfxActionDestroyInstance, kOfxActionLoad, kOfxActionUnload,
        kOfxBitDepthByte, kOfxBitDepthFloat, kOfxBitDepthShort, kOfxPropInstanceData,
        kOfxPropLabel, kOfxPropName, kOfxPropTime, kOfxStatErrUnsupported, kOfxStatFailed,
        kOfxStatOK, kOfxStatReplyDefault,
    },
    image_effect_v1::{
        sys::{
            image_effect::{
                OfxImageClipHandle, OfxImageEffectHandle, kOfxImageClipPropIsMask,
                kOfxImageClipPropOptional, kOfxImageComponentAlpha, kOfxImageComponentRGB,
                kOfxImageComponentRGBA, kOfxImageEffectActionDescribeInContext,
                kOfxImageEffectActionIsIdentity, kOfxImageEffectActionRender,
                kOfxImageEffectContextFilter, kOfxImageEffectContextGeneral,
                kOfxImageEffectPluginPropGrouping, kOfxImageEffectPluginPropHostFrameThreading,
                kOfxImageEffectPluginRenderThreadSafety, kOfxImageEffectPropContext,
                kOfxImageEffectPropRenderWindow, kOfxImageEffectPropSupportedComponents,
                kOfxImageEffectPropSupportedContexts, kOfxImageEffectPropSupportedPixelDepths,
                kOfxImageEffectRenderFullySafe,
            },
            param::{
                OfxParamHandle, kOfxParamDoubleTypeScale, kOfxParamPropDefault,
                kOfxParamPropDisplayMax, kOfxParamPropDisplayMin, kOfxParamPropDoubleType,
                kOfxParamPropHint, kOfxParamTypeDouble,
            },
        },
        sys_helpers::Plugin,
    },
};

use processing::{pixel_processing, rect_i_from_array};

use crate::{
    definitions::{PLUGIN_4_SATURATION_IDENTIFIER, PLUGIN_4_SATURATION_LABEL, PLUGINS_GROUPING},
    helpers::{
        SaferHostStruct, SharedData,
        shared_data_helper::{BitDepth, ClipImageManaged, SharedDataHelper},
    },
};

static HOST_STRUCT: OnceLock<SaferHostStruct<'static>> = OnceLock::new();

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

    extern "C" fn set_host(host_struct: *mut OfxHost) {
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

    extern "C" fn main_entry(
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
            _ if action == kOfxImageEffectActionRender => action_render(effect, in_args, out_args),
            _ => Err(kOfxStatReplyDefault),
        };

        match result {
            Ok(_) => kOfxStatOK,
            Err(status) => status,
        }
    }
}

fn action_load() -> Result<(), OfxStatus> {
    let host_struct = HOST_STRUCT.get().ok_or(kOfxStatFailed)?.clone();

    let mut data = SHARED_DATA.lock().map_err(|_| kOfxStatFailed)?;
    if data.is_some() {
        Err(kOfxStatFailed)
    } else {
        *data = Some(SharedData::try_new(host_struct)?);
        Ok(())
    }
}

fn action_unload() -> Result<(), OfxStatus> {
    let mut data = SHARED_DATA.lock().map_err(|_| kOfxStatFailed)?;
    if data.take().is_none() {
        Err(kOfxStatFailed)
    } else {
        Ok(())
    }
}

fn action_describe(descriptor: OfxImageEffectHandle) -> Result<(), OfxStatus> {
    let data = shared_data_lockless()?;
    let data = unsafe { SharedDataHelper::try_new(&data) }?;

    let descriptor = unsafe { data.make_property_set_helper_for_image_effect(descriptor) }?;

    descriptor.prop_set_string(kOfxPropLabel, 0, PLUGIN_4_SATURATION_LABEL)?;
    descriptor.prop_set_string(kOfxImageEffectPluginPropGrouping, 0, PLUGINS_GROUPING)?;
    for (i, context) in [
        kOfxImageEffectContextFilter,
        // FIXME: not working in Natron.
        kOfxImageEffectContextGeneral,
    ]
    .iter()
    .enumerate()
    {
        descriptor.prop_set_string(kOfxImageEffectPropSupportedContexts, i as c_int, context)?;
    }

    for (i, bp) in [kOfxBitDepthFloat, kOfxBitDepthShort, kOfxBitDepthByte]
        .iter()
        .enumerate()
    {
        descriptor.prop_set_string(kOfxImageEffectPropSupportedPixelDepths, i as c_int, bp)?;
    }

    descriptor.prop_set_string(
        kOfxImageEffectPluginRenderThreadSafety,
        0,
        kOfxImageEffectRenderFullySafe,
    )?;
    descriptor.prop_set_int(kOfxImageEffectPluginPropHostFrameThreading, 0, 1)?;

    Ok(())
}

fn action_describe_in_context(
    descriptor: OfxImageEffectHandle,
    in_args: OfxPropertySetHandle,
) -> Result<(), OfxStatus> {
    let data = shared_data_lockless()?;
    let data = unsafe { SharedDataHelper::try_new(&data) }?;

    let s_prop = data.property_suite_helper();
    let s_ifx = data.image_effect_suite_helper();

    let in_args = unsafe { s_prop.make_property_set_helper(in_args) };

    let context = in_args.prop_get_string(kOfxImageEffectPropContext, 0)?;
    if context != Some(kOfxImageEffectContextFilter)
        && context != Some(kOfxImageEffectContextGeneral)
    {
        return Err(kOfxStatErrUnsupported);
    }

    for name in [c"Output", c"Source"] {
        let props = unsafe { s_ifx.clip_define(descriptor, name) }?;
        let props = unsafe { s_prop.make_property_set_helper(props) };

        for (i, comp) in [kOfxImageComponentRGBA, kOfxImageComponentRGB]
            .iter()
            .enumerate()
        {
            props.prop_set_string(kOfxImageEffectPropSupportedComponents, i as c_int, comp)?;
        }
    }
    if context == Some(kOfxImageEffectContextGeneral) {
        let props = unsafe { s_ifx.clip_define(descriptor, c"Mask") }?;
        let props = unsafe { s_prop.make_property_set_helper(props) };

        props.prop_set_string(
            kOfxImageEffectPropSupportedComponents,
            0,
            kOfxImageComponentAlpha,
        )?;
        props.prop_set_int(kOfxImageClipPropOptional, 0, 1)?;
        props.prop_set_int(kOfxImageClipPropIsMask, 0, 1)?;
    }

    let param_set = unsafe { data.make_param_set_helper_for_image_effect(descriptor) }?;

    {
        let param_props = param_set.param_define(kOfxParamTypeDouble, SATURATION_PARAM_NAME)?;
        let param_props = unsafe { s_prop.make_property_set_helper(param_props) };
        param_props.prop_set_string(kOfxParamPropDoubleType, 0, kOfxParamDoubleTypeScale)?;
        param_props.prop_set_double(kOfxParamPropDefault, 0, 1.0)?;
        param_props.prop_set_double(kOfxParamPropDisplayMin, 0, -2.0)?;
        param_props.prop_set_double(kOfxParamPropDisplayMax, 0, 2.0)?;
        param_props.prop_set_string(kOfxPropLabel, 0, c"Saturation")?;
        param_props.prop_set_string(kOfxParamPropHint, 0, c"How saturated the image should be.")?;
    }

    Ok(())
}

fn action_create_instance(instance: OfxImageEffectHandle) -> Result<(), OfxStatus> {
    let data = shared_data_lockless()?;
    let data = unsafe { SharedDataHelper::try_new(&data) }?;

    let s_ifx = data.image_effect_suite_helper();

    let instance_props = unsafe { data.make_property_set_helper_for_image_effect(instance) }?;

    let context = instance_props.prop_get_string(kOfxImageEffectPropContext, 0)?;
    let is_general_context = context == Some(kOfxImageEffectContextGeneral);

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
    match unsafe { instance_props.prop_set_pointer(kOfxPropInstanceData, 0, my_data_ptr) } {
        Ok(_) => Ok(()),
        Err(err) => {
            drop(unsafe { Box::from_raw(my_data_ptr as *mut MyInstanceData) });
            Err(err)
        }
    }
}

fn action_destroy_instance(instance: OfxImageEffectHandle) -> Result<(), OfxStatus> {
    let data = shared_data_lockless()?;
    let data = unsafe { SharedDataHelper::try_new(&data) }?;

    let instance_props = unsafe { data.make_property_set_helper_for_image_effect(instance) }?;
    let my_data_ptr = instance_props.prop_get_pointer(kOfxPropInstanceData, 0)?;
    if my_data_ptr.is_null() {
        return Err(kOfxStatFailed);
    }

    drop(unsafe { Box::from_raw(my_data_ptr as *mut MyInstanceData) });

    Ok(())
}

fn action_is_identity(
    effect: OfxImageEffectHandle,
    in_args: OfxPropertySetHandle,
    out_args: OfxPropertySetHandle,
) -> Result<(), OfxStatus> {
    let data = shared_data_lockless()?;
    let data = unsafe { SharedDataHelper::try_new(&data) }?;

    let s_prop = data.property_suite_helper();
    let s_param = data.parameter_suite_helper();

    let instance_props = unsafe { data.make_property_set_helper_for_image_effect(effect) }?;
    let my_data_ptr = instance_props.prop_get_pointer(kOfxPropInstanceData, 0)?;
    if my_data_ptr.is_null() {
        return Err(kOfxStatFailed);
    }
    let my_data = unsafe { &*(my_data_ptr as *const MyInstanceData) };

    let in_args = unsafe { s_prop.make_property_set_helper(in_args) };
    let out_args = unsafe { s_prop.make_property_set_helper(out_args) };

    let time = in_args.prop_get_double(kOfxPropTime, 0)?;
    let saturation =
        unsafe { s_param.param_get_value_at_time_double(my_data.saturation_param, time) }?;

    if (saturation - 1.0).abs() < 0.000000001 {
        out_args.prop_set_string(kOfxPropName, 0, c"Source")?;
        Ok(())
    } else {
        Err(kOfxStatReplyDefault)
    }
}

fn action_render(
    instance: OfxImageEffectHandle,
    in_args: OfxPropertySetHandle,
    _out_args: OfxPropertySetHandle,
) -> Result<(), OfxStatus> {
    let data = shared_data_lockless()?;
    let data = unsafe { SharedDataHelper::try_new(&data) }?;

    let s_prop = data.property_suite_helper();
    let s_param = data.parameter_suite_helper();

    let in_args = unsafe { s_prop.make_property_set_helper(in_args) };

    let time: OfxTime = in_args.prop_get_double(kOfxPropTime, 0)?;
    let render_window = {
        let mut render_window: [c_int; 4] = [0; 4];
        in_args.prop_get_int_n(kOfxImageEffectPropRenderWindow, &mut render_window)?;
        rect_i_from_array(&render_window)
    };

    let instance_props = unsafe { data.make_property_set_helper_for_image_effect(instance) }?;
    let my_data_ptr = instance_props.prop_get_pointer(kOfxPropInstanceData, 0)?;
    if my_data_ptr.is_null() {
        return Err(kOfxStatFailed);
    }
    let my_data = unsafe { &*(my_data_ptr as *const MyInstanceData) };

    let saturation =
        unsafe { s_param.param_get_value_at_time_double(my_data.saturation_param, time) }?;

    let Some(output_img_m) =
        unsafe { data.make_clip_image_managed(my_data.output_clip, time, None) }?
    else {
        return Err(kOfxStatFailed);
    };
    let Some(source_img_m) =
        unsafe { data.make_clip_image_managed(my_data.source_clip, time, None) }?
    else {
        return Err(kOfxStatFailed);
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
    ) -> Result<(), OfxStatus> {
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

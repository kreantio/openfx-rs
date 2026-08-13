mod processing;

use std::{
    ffi::{CStr, c_char, c_int, c_void},
    sync::{Mutex, OnceLock},
};

use openfx::{
    generic::sys::core::{
        OfxHost, OfxPlugin, OfxPropertySetHandle, OfxRectI, OfxStatus, OfxTime,
        kOfxActionCreateInstance, kOfxActionDescribe, kOfxActionDestroyInstance, kOfxActionLoad,
        kOfxActionUnload, kOfxBitDepthByte, kOfxBitDepthFloat, kOfxBitDepthShort,
        kOfxPropInstanceData, kOfxPropLabel, kOfxPropName, kOfxPropTime, kOfxStatErrUnsupported,
        kOfxStatFailed, kOfxStatOK, kOfxStatReplyDefault,
    },
    image_effect_v1::sys::{
        image_effect::{
            OfxImageClipHandle, OfxImageEffectHandle, kOfxImageComponentAlpha,
            kOfxImageComponentRGB, kOfxImageComponentRGBA, kOfxImageEffectActionDescribeInContext,
            kOfxImageEffectActionIsIdentity, kOfxImageEffectActionRender,
            kOfxImageEffectContextFilter, kOfxImageEffectPluginApi,
            kOfxImageEffectPluginPropGrouping, kOfxImageEffectPluginPropHostFrameThreading,
            kOfxImageEffectPluginRenderThreadSafety, kOfxImageEffectPropComponents,
            kOfxImageEffectPropContext, kOfxImageEffectPropPixelDepth,
            kOfxImageEffectPropRenderWindow, kOfxImageEffectPropSupportedComponents,
            kOfxImageEffectPropSupportedContexts, kOfxImageEffectPropSupportedPixelDepths,
            kOfxImageEffectRenderFullySafe,
        },
        param::{
            OfxParamHandle, kOfxParamDoubleTypeScale, kOfxParamPropDefault,
            kOfxParamPropDisplayMax, kOfxParamPropDisplayMin, kOfxParamPropDoubleType,
            kOfxParamPropHint, kOfxParamPropMin, kOfxParamTypeBoolean, kOfxParamTypeDouble,
        },
    },
};

use processing::{pixel_processing, rect_i_from_array};

use crate::{
    definitions::{PLUGIN_3_GAIN_IDENTIFIER, PLUGIN_3_GAIN_LABEL, PLUGINS_GROUPING},
    helpers::{SaferHostStruct, SharedData, shared_data_helper::SharedDataHelper},
};

pub static EFFECT_PLUGIN_STRUCT_GAIN: OfxPlugin = OfxPlugin {
    pluginApi: kOfxImageEffectPluginApi.as_ptr(),
    apiVersion: 1,
    pluginIdentifier: PLUGIN_3_GAIN_IDENTIFIER.as_ptr(),
    pluginVersionMajor: 1,
    pluginVersionMinor: 0,
    setHost: Some(set_host),
    mainEntry: Some(main_entry),
};

static HOST_STRUCT: OnceLock<SaferHostStruct<'static>> = OnceLock::new();

static SHARED_DATA: Mutex<Option<SharedData<'static>>> = Mutex::new(None);

struct MyInstanceData {
    source_clip: OfxImageClipHandle,
    output_clip: OfxImageClipHandle,

    gain_param: OfxParamHandle,
    apply_to_alpha_param: OfxParamHandle,
}

fn shared_data_lockless() -> Result<SharedData<'static>, OfxStatus> {
    let data = SHARED_DATA.lock().map_err(|_| kOfxStatFailed)?;
    let data = data.as_ref().ok_or(kOfxStatFailed)?;
    Ok(data.clone())
}

const GAIN_PARAM_NAME: &CStr = c"gain";
const APPLY_TO_ALPHA_PARAM_NAME: &CStr = c"applyToAlpha";

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
        _ if action == kOfxImageEffectActionRender => action_render(effect, in_args, out_args),
        _ => Err(kOfxStatReplyDefault),
    };

    match result {
        Ok(_) => kOfxStatOK,
        Err(status) => status,
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

    descriptor.prop_set_string(kOfxPropLabel, 0, PLUGIN_3_GAIN_LABEL)?;
    descriptor.prop_set_string(kOfxImageEffectPluginPropGrouping, 0, PLUGINS_GROUPING)?;
    descriptor.prop_set_string(
        kOfxImageEffectPropSupportedContexts,
        0,
        kOfxImageEffectContextFilter,
    )?;

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
    if context != Some(kOfxImageEffectContextFilter) {
        return Err(kOfxStatErrUnsupported);
    }

    for name in [c"Output", c"Source"] {
        let props = unsafe { s_ifx.clip_define(descriptor, name) }?;
        let props = unsafe { s_prop.make_property_set_helper(props) };

        for (i, comp) in [
            kOfxImageComponentRGBA,
            kOfxImageComponentAlpha,
            kOfxImageComponentRGB,
        ]
        .iter()
        .enumerate()
        {
            props.prop_set_string(kOfxImageEffectPropSupportedComponents, i as c_int, comp)?;
        }
    }

    let param_set = unsafe { data.make_param_set_helper_for_image_effect(descriptor) }?;

    {
        let param_props = param_set.param_define(kOfxParamTypeDouble, GAIN_PARAM_NAME)?;
        let param_props = unsafe { s_prop.make_property_set_helper(param_props) };
        param_props.prop_set_string(kOfxParamPropDoubleType, 0, kOfxParamDoubleTypeScale)?;
        param_props.prop_set_double(kOfxParamPropDefault, 0, 1.0)?;
        param_props.prop_set_double(kOfxParamPropMin, 0, 0.0)?;
        param_props.prop_set_double(kOfxParamPropDisplayMin, 0, 0.0)?;
        param_props.prop_set_double(kOfxParamPropDisplayMax, 0, 10.0)?;
        param_props.prop_set_string(kOfxPropLabel, 0, c"Gain")?;
        param_props.prop_set_string(kOfxParamPropHint, 0, c"How much to multiply the image by.")?;
    }

    {
        let param_props =
            param_set.param_define(kOfxParamTypeBoolean, APPLY_TO_ALPHA_PARAM_NAME)?;
        let param_props = unsafe { s_prop.make_property_set_helper(param_props) };
        param_props.prop_set_int(kOfxParamPropDefault, 0, 0)?;
        param_props.prop_set_string(kOfxPropLabel, 0, c"Apply To Alpha")?;
        param_props.prop_set_string(
            kOfxParamPropHint,
            0,
            c"Whether to apply the gain value to alpha as well.",
        )?;
    }

    Ok(())
}

fn action_create_instance(instance: OfxImageEffectHandle) -> Result<(), OfxStatus> {
    let data = shared_data_lockless()?;
    let data = unsafe { SharedDataHelper::try_new(&data) }?;

    let s_ifx = data.image_effect_suite_helper();

    let instance_props = unsafe { data.make_property_set_helper_for_image_effect(instance) }?;

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
    let gain = unsafe { s_param.param_get_value_at_time_double(my_data.gain_param, time) }?;

    if (gain - 1.0).abs() < 0.000000001 {
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

    let gain = unsafe { s_param.param_get_value_at_time_double(my_data.gain_param, time) }?;
    let apply_to_alpha =
        unsafe { s_param.param_get_value_at_time_int(my_data.apply_to_alpha_param, time) }? != 0;

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

    fn inner(
        gain: f64,
        apply_to_alpha: bool,
        data: &SharedDataHelper,
        instance: OfxImageEffectHandle,
        source_img: OfxPropertySetHandle,
        output_img: OfxPropertySetHandle,
        render_window: OfxRectI,
    ) -> Result<(), OfxStatus> {
        let s_prop = data.property_suite_helper();

        let output_img = unsafe { s_prop.make_property_set_helper(output_img) };

        let components = output_img.prop_get_string(kOfxImageEffectPropComponents, 0)?;
        let n_comps = match components {
            Some(c) if c == kOfxImageComponentRGBA => 4,
            Some(c) if c == kOfxImageComponentRGB => 3,
            Some(c) if c == kOfxImageComponentAlpha => 1,
            _ => return Err(kOfxStatErrUnsupported),
        };

        let data_type = output_img.prop_get_string(kOfxImageEffectPropPixelDepth, 0)?;
        match data_type {
            Some(c) if c == kOfxBitDepthByte => pixel_processing(
                |f| f as u8,
                |v| v as f64,
                255u8,
                gain,
                apply_to_alpha,
                data,
                instance,
                source_img,
                output_img.props(),
                render_window,
                n_comps,
            ),
            Some(c) if c == kOfxBitDepthShort => pixel_processing(
                |f| f as u16,
                |v| v as f64,
                65535u16,
                gain,
                apply_to_alpha,
                data,
                instance,
                source_img,
                output_img.props(),
                render_window,
                n_comps,
            ),
            Some(c) if c == kOfxBitDepthFloat => pixel_processing(
                |f| f as f32,
                |v| v as f64,
                1.0f32,
                gain,
                apply_to_alpha,
                data,
                instance,
                source_img,
                output_img.props(),
                render_window,
                n_comps,
            ),
            _ => return Err(kOfxStatErrUnsupported),
        }?;

        Ok(())
    }

    let result = inner(
        gain,
        apply_to_alpha,
        &data,
        instance,
        source_img_m.image_handle(),
        output_img_m.image_handle(),
        render_window,
    );

    drop(output_img_m);
    drop(source_img_m);

    result
}

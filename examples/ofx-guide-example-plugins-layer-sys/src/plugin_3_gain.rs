mod processing;

use std::{
    ffi::{CStr, c_char, c_void},
    sync::{Mutex, OnceLock},
};

use openfx::{
    generic::{
        sys::core::{
            OfxHost, OfxPropertySetHandle, OfxRectI, OfxStatus, OfxTime, kOfxActionCreateInstance,
            kOfxActionDescribe, kOfxActionDestroyInstance, kOfxActionLoad, kOfxActionUnload,
            kOfxBitDepthByte, kOfxBitDepthFloat, kOfxBitDepthShort, kOfxStatErrUnsupported,
            kOfxStatFailed, kOfxStatOK, kOfxStatReplyDefault,
        },
        sys_helpers::properties::{
            get_OfxPropInstanceData, get_OfxPropTime, set_OfxPropInstanceData, set_OfxPropLabel,
            set_OfxPropName,
        },
    },
    image_effect_v1::{
        sys::{
            image_effect::{
                OfxImageClipHandle, OfxImageEffectHandle, kOfxImageComponentAlpha,
                kOfxImageComponentRGB, kOfxImageComponentRGBA,
                kOfxImageEffectActionDescribeInContext, kOfxImageEffectActionIsIdentity,
                kOfxImageEffectActionRender, kOfxImageEffectContextFilter,
                kOfxImageEffectRenderFullySafe,
            },
            param::{
                OfxParamHandle, kOfxParamDoubleTypeScale, kOfxParamTypeBoolean, kOfxParamTypeDouble,
            },
        },
        sys_helpers::{
            Plugin,
            properties::{
                get_OfxImageEffectPropComponents, get_OfxImageEffectPropContext,
                get_OfxImageEffectPropPixelDepth, get_OfxImageEffectPropRenderWindow,
                set_OfxImageEffectPluginPropGrouping,
                set_OfxImageEffectPluginPropHostFrameThreading,
                set_OfxImageEffectPluginRenderThreadSafety,
                set_OfxImageEffectPropSupportedComponents, set_OfxImageEffectPropSupportedContexts,
                set_OfxImageEffectPropSupportedPixelDepths, set_OfxParamPropDefault_Double,
                set_OfxParamPropDefault_Int, set_OfxParamPropDisplayMax_Double,
                set_OfxParamPropDisplayMin_Double, set_OfxParamPropDoubleType,
                set_OfxParamPropHint, set_OfxParamPropMin_Double,
            },
        },
    },
};

use processing::{pixel_processing, rect_i_from_array};

use crate::{
    definitions::{PLUGIN_3_GAIN_IDENTIFIER, PLUGIN_3_GAIN_LABEL, PLUGINS_GROUPING},
    helpers::{SaferHostStruct, SharedData, shared_data_helper::SharedDataHelper},
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

pub struct PluginExampleGain;
impl Plugin for PluginExampleGain {
    const PLUGIN_IDENTIFIER: &'static CStr = PLUGIN_3_GAIN_IDENTIFIER;
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

    let s_prop = data.inner().property_suite;

    let props = unsafe { data.get_property_set_from_image_effect(descriptor) }?;

    unsafe {
        set_OfxPropLabel(s_prop, props, PLUGIN_3_GAIN_LABEL.as_ptr())?;
        set_OfxImageEffectPluginPropGrouping(s_prop, props, PLUGINS_GROUPING.as_ptr())?;
        set_OfxImageEffectPropSupportedContexts(
            s_prop,
            props,
            &[kOfxImageEffectContextFilter.as_ptr()],
        )?;
        set_OfxImageEffectPropSupportedPixelDepths(
            s_prop,
            props,
            &[
                kOfxBitDepthFloat.as_ptr(),
                kOfxBitDepthShort.as_ptr(),
                kOfxBitDepthByte.as_ptr(),
            ],
        )?;
        set_OfxImageEffectPluginRenderThreadSafety(
            s_prop,
            props,
            kOfxImageEffectRenderFullySafe.as_ptr(),
        )?;
        set_OfxImageEffectPluginPropHostFrameThreading(s_prop, props, 1)?;
    }

    Ok(())
}

fn action_describe_in_context(
    descriptor: OfxImageEffectHandle,
    in_args: OfxPropertySetHandle,
) -> Result<(), OfxStatus> {
    let data = shared_data_lockless()?;
    let data = unsafe { SharedDataHelper::try_new(&data) }?;

    let s_prop = data.inner().property_suite;
    let s_ifx = data.image_effect_suite_helper();

    let context = unsafe { get_OfxImageEffectPropContext(s_prop, in_args) }?;
    if context.is_null() || unsafe { CStr::from_ptr(context) } != kOfxImageEffectContextFilter {
        return Err(kOfxStatErrUnsupported);
    }

    for name in [c"Output", c"Source"] {
        let props = unsafe { s_ifx.clip_define(descriptor, name) }?;

        (unsafe {
            set_OfxImageEffectPropSupportedComponents(
                s_prop,
                props,
                &[
                    kOfxImageComponentRGBA.as_ptr(),
                    kOfxImageComponentAlpha.as_ptr(),
                    kOfxImageComponentRGB.as_ptr(),
                ],
            )
        })?;
    }

    let param_set = unsafe { data.make_param_set_helper_for_image_effect(descriptor) }?;

    {
        let param_props = param_set.param_define(kOfxParamTypeDouble, GAIN_PARAM_NAME)?;
        unsafe {
            set_OfxParamPropDoubleType(s_prop, param_props, kOfxParamDoubleTypeScale.as_ptr())?;
            set_OfxParamPropDefault_Double(s_prop, param_props, &[1.0])?;
            set_OfxParamPropMin_Double(s_prop, param_props, &[0.0])?;
            set_OfxParamPropDisplayMin_Double(s_prop, param_props, &[0.0])?;
            set_OfxParamPropDisplayMax_Double(s_prop, param_props, &[10.0])?;
            set_OfxPropLabel(s_prop, param_props, c"Gain".as_ptr())?;
            set_OfxParamPropHint(
                s_prop,
                param_props,
                c"How much to multiply the image by.".as_ptr(),
            )?;
        }
    }

    {
        let param_props =
            param_set.param_define(kOfxParamTypeBoolean, APPLY_TO_ALPHA_PARAM_NAME)?;
        unsafe {
            set_OfxParamPropDefault_Int(s_prop, param_props, &[0])?;
            set_OfxPropLabel(s_prop, param_props, c"Apply To Alpha".as_ptr())?;
            set_OfxParamPropHint(
                s_prop,
                param_props,
                c"Whether to apply the gain value to alpha as well.".as_ptr(),
            )?;
        }
    }

    Ok(())
}

fn action_create_instance(instance: OfxImageEffectHandle) -> Result<(), OfxStatus> {
    let data = shared_data_lockless()?;
    let data = unsafe { SharedDataHelper::try_new(&data) }?;

    let s_prop = data.inner().property_suite;
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
    match unsafe { set_OfxPropInstanceData(s_prop, instance_props, my_data_ptr) } {
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

    let s_prop = data.inner().property_suite;

    let instance_props = unsafe { data.get_property_set_from_image_effect(instance) }?;
    let my_data_ptr = unsafe { get_OfxPropInstanceData(s_prop, instance_props) }?;
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

    let s_prop = data.inner().property_suite;
    let s_param = data.parameter_suite_helper();

    let instance_props = unsafe { data.get_property_set_from_image_effect(effect) }?;
    let my_data_ptr = unsafe { get_OfxPropInstanceData(s_prop, instance_props) }?;
    if my_data_ptr.is_null() {
        return Err(kOfxStatFailed);
    }
    let my_data = unsafe { &*(my_data_ptr as *const MyInstanceData) };

    let time = unsafe { get_OfxPropTime(s_prop, in_args) }?;
    let gain = unsafe { s_param.param_get_value_at_time_double(my_data.gain_param, time) }?;

    if (gain - 1.0).abs() < 0.000000001 {
        (unsafe { set_OfxPropName(s_prop, out_args, c"Source".as_ptr()) })?;
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

    let s_prop = data.inner().property_suite;
    let s_param = data.parameter_suite_helper();

    let instance_props = unsafe { data.get_property_set_from_image_effect(instance) }?;

    let time: OfxTime = unsafe { get_OfxPropTime(s_prop, in_args) }?;
    let render_window = unsafe { get_OfxImageEffectPropRenderWindow(s_prop, in_args) }?;
    let render_window = rect_i_from_array(&render_window);

    let my_data_ptr = unsafe { get_OfxPropInstanceData(s_prop, instance_props) }?;
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
        let s_prop = data.inner().property_suite;

        let components = unsafe { get_OfxImageEffectPropComponents(s_prop, output_img) }?;
        if components.is_null() {
            return Err(kOfxStatErrUnsupported);
        }
        let n_comps = match unsafe { CStr::from_ptr(components) } {
            c if c == kOfxImageComponentRGBA => 4,
            c if c == kOfxImageComponentRGB => 3,
            c if c == kOfxImageComponentAlpha => 1,
            _ => return Err(kOfxStatErrUnsupported),
        };

        let data_type = unsafe { get_OfxImageEffectPropPixelDepth(s_prop, output_img) }?;
        if data_type.is_null() {
            return Err(kOfxStatErrUnsupported);
        }

        match unsafe { CStr::from_ptr(data_type) } {
            c if c == kOfxBitDepthByte => pixel_processing(
                |f| f as u8,
                |v| v as f64,
                255u8,
                gain,
                apply_to_alpha,
                data,
                instance,
                source_img,
                output_img,
                render_window,
                n_comps,
            ),
            c if c == kOfxBitDepthShort => pixel_processing(
                |f| f as u16,
                |v| v as f64,
                65535u16,
                gain,
                apply_to_alpha,
                data,
                instance,
                source_img,
                output_img,
                render_window,
                n_comps,
            ),
            c if c == kOfxBitDepthFloat => pixel_processing(
                |f| f as f32,
                |v| v as f64,
                1.0f32,
                gain,
                apply_to_alpha,
                data,
                instance,
                source_img,
                output_img,
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

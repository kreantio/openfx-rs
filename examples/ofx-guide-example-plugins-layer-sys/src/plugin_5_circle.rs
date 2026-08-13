mod processing;

use std::{
    ffi::{CStr, c_char, c_int, c_void},
    sync::{Arc, Mutex, OnceLock},
};

use openfx::{
    generic::sys::core::{
        OfxHost, OfxPlugin, OfxPropertySetHandle, OfxRectI, OfxStatus, OfxTime,
        kOfxActionCreateInstance, kOfxActionDescribe, kOfxActionDestroyInstance, kOfxActionLoad,
        kOfxActionUnload, kOfxBitDepthByte, kOfxBitDepthFloat, kOfxBitDepthShort,
        kOfxPropAPIVersion, kOfxPropInstanceData, kOfxPropLabel, kOfxPropName, kOfxPropTime,
        kOfxStatErrMissingHostFeature, kOfxStatErrUnsupported, kOfxStatFailed, kOfxStatOK,
        kOfxStatReplyDefault,
    },
    image_effect_v1::sys::{
        image_effect::{
            OfxImageClipHandle, OfxImageEffectHandle, kOfxImageComponentAlpha,
            kOfxImageComponentRGB, kOfxImageComponentRGBA, kOfxImageEffectActionDescribeInContext,
            kOfxImageEffectActionGetRegionOfDefinition, kOfxImageEffectActionIsIdentity,
            kOfxImageEffectActionRender, kOfxImageEffectContextFilter, kOfxImageEffectPluginApi,
            kOfxImageEffectPluginPropGrouping, kOfxImageEffectPluginPropHostFrameThreading,
            kOfxImageEffectPluginRenderThreadSafety, kOfxImageEffectPropContext,
            kOfxImageEffectPropRegionOfDefinition, kOfxImageEffectPropRenderScale,
            kOfxImageEffectPropRenderWindow, kOfxImageEffectPropSupportedComponents,
            kOfxImageEffectPropSupportedContexts, kOfxImageEffectPropSupportedPixelDepths,
            kOfxImageEffectPropSupportsMultiResolution, kOfxImageEffectRenderFullySafe,
        },
        param::{
            OfxParamHandle, kOfxParamCoordinatesNormalised, kOfxParamDoubleTypeX,
            kOfxParamDoubleTypeXYAbsolute, kOfxParamPropDefault,
            kOfxParamPropDefaultCoordinateSystem, kOfxParamPropDisplayMax, kOfxParamPropDisplayMin,
            kOfxParamPropDoubleType, kOfxParamPropHint, kOfxParamPropMin, kOfxParamTypeBoolean,
            kOfxParamTypeDouble, kOfxParamTypeDouble2D, kOfxParamTypeRGBA,
        },
    },
};
use processing::{pixel_processing, rect_d_to_array, rect_i_from_array};

use crate::{
    definitions::{PLUGIN_5_CIRCLE_IDENTIFIER, PLUGIN_5_CIRCLE_LABEL, PLUGINS_GROUPING},
    helpers::{
        SaferHostStruct, SharedData,
        shared_data_helper::{
            BitDepth, ClipImageManaged, SharedDataHelper, param_get_value_at_time,
        },
    },
};

pub static EFFECT_PLUGIN_STRUCT_CIRCLE: OfxPlugin = OfxPlugin {
    pluginApi: kOfxImageEffectPluginApi.as_ptr(),
    apiVersion: 1,
    pluginIdentifier: PLUGIN_5_CIRCLE_IDENTIFIER.as_ptr(),
    pluginVersionMajor: 1,
    pluginVersionMinor: 0,
    setHost: Some(set_host),
    mainEntry: Some(main_entry),
};

static HOST_STRUCT: OnceLock<SaferHostStruct<'static>> = OnceLock::new();

static SHARED_DATA: Mutex<Option<(SharedData<'static>, Arc<AdditionalSharedData>)>> =
    Mutex::new(None);

struct AdditionalSharedData {
    #[expect(unused)]
    api_version: [c_int; 2],
    host_supports_multi_res: bool,
}

struct InstanceData {
    source_clip: OfxImageClipHandle,
    output_clip: OfxImageClipHandle,

    radius_param: OfxParamHandle,
    centre_param: OfxParamHandle,
    colour_param: OfxParamHandle,
    grow_rod_param: Option<OfxParamHandle>,
}

fn shared_data_lockless() -> Result<(SharedData<'static>, Arc<AdditionalSharedData>), OfxStatus> {
    let data = SHARED_DATA.lock().map_err(|_| kOfxStatFailed)?;
    let data = data.as_ref().ok_or(kOfxStatFailed)?;
    Ok((data.0.clone(), data.1.clone()))
}

const RADIUS_PARAM_NAME: &CStr = c"radius";
const CENTRE_PARAM_NAME: &CStr = c"centre";
const COLOUR_PARAM_NAME: &CStr = c"colour";
const GROW_ROD_PARAM_NAME: &CStr = c"growRoD";

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
        _ if action == kOfxImageEffectActionGetRegionOfDefinition => {
            action_get_region_of_definition(effect, in_args, out_args)
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
        return Err(kOfxStatFailed);
    }

    *data = Some({
        let data = SharedData::try_new(host_struct)?;

        let additional = {
            let data = unsafe { SharedDataHelper::try_new(&data) }?;

            let host_props = data.make_property_set_helper_for_host()?;
            let var_size = host_props.prop_get_dimension(kOfxPropAPIVersion)?;
            let mut api_version = [1, 0];
            if var_size == 1 {
                api_version[0] = host_props.prop_get_int(kOfxPropAPIVersion, 0)?;
            } else {
                host_props.prop_get_int_n(kOfxPropAPIVersion, &mut api_version)?;
            }

            // we only support 1.2 and above
            if api_version[0] == 1 && api_version[1] < 2 {
                return Err(kOfxStatErrMissingHostFeature);
            }

            let host_supports_multi_res = host_props
                .prop_get_int(kOfxImageEffectPropSupportsMultiResolution, 0)
                .unwrap_or_default()
                == 1;

            AdditionalSharedData {
                api_version,
                host_supports_multi_res,
            }
        };

        (data, Arc::new(additional))
    });

    Ok(())
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
    let (data, _additional) = shared_data_lockless()?;
    let data = unsafe { SharedDataHelper::try_new(&data) }?;

    let descriptor = unsafe { data.make_property_set_helper_for_image_effect(descriptor) }?;

    descriptor.prop_set_string(kOfxPropLabel, 0, PLUGIN_5_CIRCLE_LABEL)?;
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
    let (data, additional) = shared_data_lockless()?;
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

        (unsafe {
            props.prop_set_string_n_raw(
                kOfxImageEffectPropSupportedComponents,
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
        let param_props = param_set.param_define(kOfxParamTypeDouble, RADIUS_PARAM_NAME)?;
        let param_props = unsafe { s_prop.make_property_set_helper(param_props) };
        param_props.prop_set_string(kOfxParamPropDoubleType, 0, kOfxParamDoubleTypeX)?;
        // Not supported by DaVinci Resolve. To make the plugin work there, we
        // ignore the return value here. TODO: Calculate the default value in
        // canonical coordinate if this fails.
        param_props
            .prop_set_string(
                kOfxParamPropDefaultCoordinateSystem,
                0,
                kOfxParamCoordinatesNormalised,
            )
            .ok();
        param_props.prop_set_double(kOfxParamPropDefault, 0, 0.25)?;
        param_props.prop_set_double(kOfxParamPropMin, 0, 0.0)?;
        param_props.prop_set_double(kOfxParamPropDisplayMin, 0, 0.0)?;
        param_props.prop_set_double(kOfxParamPropDisplayMax, 0, 2.0)?;
        param_props.prop_set_string(kOfxPropLabel, 0, c"Radius")?;
        param_props.prop_set_string(kOfxParamPropHint, 0, c"The radius of the circle.")?;
    }

    {
        let param_props = param_set.param_define(kOfxParamTypeDouble2D, CENTRE_PARAM_NAME)?;
        let param_props = unsafe { s_prop.make_property_set_helper(param_props) };
        param_props.prop_set_string(kOfxParamPropDoubleType, 0, kOfxParamDoubleTypeXYAbsolute)?;
        // Not supported by DaVinci Resolve. See above.
        param_props
            .prop_set_string(
                kOfxParamPropDefaultCoordinateSystem,
                0,
                kOfxParamCoordinatesNormalised,
            )
            .ok();
        param_props.prop_set_double_n(kOfxParamPropDefault, &[0.5, 0.5])?;
        param_props.prop_set_string(kOfxPropLabel, 0, c"Centre")?;
        param_props.prop_set_string(kOfxParamPropHint, 0, c"The centre of the circle.")?;
    }

    {
        let param_props = param_set.param_define(kOfxParamTypeRGBA, COLOUR_PARAM_NAME)?;
        let param_props = unsafe { s_prop.make_property_set_helper(param_props) };
        param_props.prop_set_double_n(kOfxParamPropDefault, &[1.0, 1.0, 1.0, 0.5])?;
        param_props.prop_set_string(kOfxPropLabel, 0, c"Colour")?;
        param_props.prop_set_string(kOfxParamPropHint, 0, c"The colour of the circle.")?;
    }

    if additional.host_supports_multi_res {
        let param_props = param_set.param_define(kOfxParamTypeBoolean, GROW_ROD_PARAM_NAME)?;
        let param_props = unsafe { s_prop.make_property_set_helper(param_props) };
        param_props.prop_set_int(kOfxParamPropDefault, 0, 0)?;
        param_props.prop_set_string(kOfxPropLabel, 0, c"Grow RoD")?;
        param_props.prop_set_string(
            kOfxParamPropHint,
            0,
            c"Whether to grow the output's Region of Definition to include the circle.",
        )?;
    }

    Ok(())
}

fn action_create_instance(instance: OfxImageEffectHandle) -> Result<(), OfxStatus> {
    let (data, additional) = shared_data_lockless()?;
    let data = unsafe { SharedDataHelper::try_new(&data) }?;

    let s_ifx = data.image_effect_suite_helper();

    let instance_props = unsafe { data.make_property_set_helper_for_image_effect(instance) }?;

    let source_clip = unsafe { s_ifx.clip_get_handle(instance, c"Source") }?;
    let output_clip = unsafe { s_ifx.clip_get_handle(instance, c"Output") }?;

    let param_set = unsafe { data.make_param_set_helper_for_image_effect(instance) }?;
    let radius_param = param_set.param_get_handle(RADIUS_PARAM_NAME)?;
    let centre_param = param_set.param_get_handle(CENTRE_PARAM_NAME)?;
    let colour_param = param_set.param_get_handle(COLOUR_PARAM_NAME)?;
    let grow_rod_param = if additional.host_supports_multi_res {
        Some(param_set.param_get_handle(GROW_ROD_PARAM_NAME)?)
    } else {
        None
    };

    let instance_data = InstanceData {
        source_clip,
        output_clip,
        radius_param,
        centre_param,
        colour_param,
        grow_rod_param,
    };
    let instance_data_ptr = Box::into_raw(Box::new(instance_data)) as *mut c_void;

    // SAFETY: the pointee is kept alive by `Box::into_raw` until it is
    // reclaimed with `Box::from_raw` in `action_destroy_instance`.
    match unsafe { instance_props.prop_set_pointer(kOfxPropInstanceData, 0, instance_data_ptr) } {
        Ok(_) => Ok(()),
        Err(err) => {
            drop(unsafe { Box::from_raw(instance_data_ptr as *mut InstanceData) });
            Err(err)
        }
    }
}

fn action_destroy_instance(instance: OfxImageEffectHandle) -> Result<(), OfxStatus> {
    let (data, _additional) = shared_data_lockless()?;
    let data = unsafe { SharedDataHelper::try_new(&data) }?;

    let instance_props = unsafe { data.make_property_set_helper_for_image_effect(instance) }?;
    let instance_data_ptr = instance_props.prop_get_pointer(kOfxPropInstanceData, 0)?;
    if instance_data_ptr.is_null() {
        return Err(kOfxStatFailed);
    }

    drop(unsafe { Box::from_raw(instance_data_ptr as *mut InstanceData) });

    Ok(())
}

fn action_get_region_of_definition(
    effect: OfxImageEffectHandle,
    in_args: OfxPropertySetHandle,
    out_args: OfxPropertySetHandle,
) -> Result<(), OfxStatus> {
    let (data, additional) = shared_data_lockless()?;

    if !additional.host_supports_multi_res {
        return Err(kOfxStatReplyDefault);
    }

    let data = unsafe { SharedDataHelper::try_new(&data) }?;

    let instance_data = unsafe { data.get_instance_data::<InstanceData>(effect)? };

    let s_prop = data.property_suite_helper();
    let s_ifx = data.image_effect_suite_helper();
    let s_param = data.parameter_suite_helper();

    let in_args = unsafe { s_prop.make_property_set_helper(in_args) };
    let out_args = unsafe { s_prop.make_property_set_helper(out_args) };

    let time = in_args.prop_get_double(kOfxPropTime, 0)?;

    let growing_rod = if let Some(grow_rod_param) = instance_data.grow_rod_param {
        (unsafe { s_param.param_get_value_at_time_int(grow_rod_param, time) })? != 0
    } else {
        false
    };

    if !growing_rod {
        return Err(kOfxStatReplyDefault);
    }

    let radius =
        unsafe { s_param.param_get_value_at_time_double(instance_data.radius_param, time) }?;
    let mut centre_x = 0.0;
    let mut centre_y = 0.0;
    param_get_value_at_time!(
        s_param,
        instance_data.centre_param,
        time,
        &mut centre_x,
        &mut centre_y,
    );

    let mut rod = unsafe { s_ifx.clip_get_region_of_definition(instance_data.source_clip, time) }?;

    rod.x1 = f64::min(rod.x1, centre_x - radius);
    rod.y1 = f64::min(rod.y1, centre_y - radius);
    rod.x2 = f64::max(rod.x2, centre_x + radius);
    rod.y2 = f64::max(rod.y2, centre_y + radius);

    out_args.prop_set_double_n(
        kOfxImageEffectPropRegionOfDefinition,
        &rect_d_to_array(&rod),
    )?;

    Ok(())
}

fn action_is_identity(
    effect: OfxImageEffectHandle,
    in_args: OfxPropertySetHandle,
    out_args: OfxPropertySetHandle,
) -> Result<(), OfxStatus> {
    let (data, _additional) = shared_data_lockless()?;
    let data = unsafe { SharedDataHelper::try_new(&data) }?;

    let s_prop = data.property_suite_helper();
    let s_ifx = data.image_effect_suite_helper();
    let s_param = data.parameter_suite_helper();

    let instance_data = unsafe { data.get_instance_data::<InstanceData>(effect)? };

    let in_args = unsafe { s_prop.make_property_set_helper(in_args) };
    let out_args = unsafe { s_prop.make_property_set_helper(out_args) };

    let time = in_args.prop_get_double(kOfxPropTime, 0)?;

    let radius =
        unsafe { s_param.param_get_value_at_time_double(instance_data.radius_param, time) }?;

    let is_identity = if radius < 0.0001 {
        true
    } else {
        let growing_rod = if let Some(grow_rod_param) = instance_data.grow_rod_param {
            (unsafe { s_param.param_get_value_at_time_int(grow_rod_param, time) })? != 0
        } else {
            false
        };

        if growing_rod {
            false
        } else {
            let bounds =
                unsafe { s_ifx.clip_get_region_of_definition(instance_data.source_clip, time) }?;

            let mut centre_x = 0.0;
            let mut centre_y = 0.0;
            param_get_value_at_time!(
                s_param,
                instance_data.centre_param,
                time,
                &mut centre_x,
                &mut centre_y,
            );

            centre_x + radius < bounds.x1
                || centre_x - radius > bounds.x2
                || centre_y + radius < bounds.y1
                || centre_y - radius > bounds.y2
        }
    };

    if is_identity {
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
    let (data, _additional) = shared_data_lockless()?;
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
    let render_scale = {
        let mut render_scale: [f64; 2] = [0.0; 2];
        in_args.prop_get_double_n(kOfxImageEffectPropRenderScale, &mut render_scale)?;
        render_scale
    };

    let instance_data = unsafe { data.get_instance_data::<InstanceData>(instance)? };

    let radius =
        unsafe { s_param.param_get_value_at_time_double(instance_data.radius_param, time) }?;
    let centre = {
        let mut centre_x = 0.0;
        let mut centre_y = 0.0;
        param_get_value_at_time!(
            s_param,
            instance_data.centre_param,
            time,
            &mut centre_x,
            &mut centre_y,
        );
        [centre_x, centre_y]
    };
    let colour = {
        let mut colour_r = 0.0;
        let mut colour_g = 0.0;
        let mut colour_b = 0.0;
        let mut colour_a = 0.0;
        param_get_value_at_time!(
            s_param,
            instance_data.colour_param,
            time,
            &mut colour_r,
            &mut colour_g,
            &mut colour_b,
            &mut colour_a,
        );
        [colour_r, colour_g, colour_b, colour_a]
    };

    let Some(output_img_m) =
        unsafe { data.make_clip_image_managed(instance_data.output_clip, time, None) }?
    else {
        return Err(kOfxStatFailed);
    };
    let Some(source_img_m) =
        unsafe { data.make_clip_image_managed(instance_data.source_clip, time, None) }?
    else {
        return Err(kOfxStatFailed);
    };

    #[allow(clippy::too_many_arguments)]
    fn inner(
        centre: [f64; 2],
        radius: f64,
        colour: [f64; 4],
        render_scale: [f64; 2],
        data: &SharedDataHelper,
        instance: OfxImageEffectHandle,
        source_img: ClipImageManaged,
        output_img: ClipImageManaged,
        render_window: OfxRectI,
    ) -> Result<(), OfxStatus> {
        match output_img.pixel_depth() {
            BitDepth::Byte => pixel_processing(
                |f| f as u8,
                |v| v as f64,
                |v, min, max| v.clamp(min, max),
                255u8,
                centre,
                radius,
                colour,
                render_scale,
                data,
                instance,
                source_img,
                output_img,
                render_window,
            ),
            BitDepth::Short => pixel_processing(
                |f| f as u16,
                |v| v as f64,
                |v, min, max| v.clamp(min, max),
                65535u16,
                centre,
                radius,
                colour,
                render_scale,
                data,
                instance,
                source_img,
                output_img,
                render_window,
            ),
            BitDepth::Float => pixel_processing(
                |f| f as f32,
                |v| v as f64,
                |v, min, max| v.clamp(min, max),
                1.0f32,
                centre,
                radius,
                colour,
                render_scale,
                data,
                instance,
                source_img,
                output_img,
                render_window,
            ),
        }?;

        Ok(())
    }

    inner(
        centre,
        radius,
        colour,
        render_scale,
        &data,
        instance,
        source_img_m,
        output_img_m,
        render_window,
    )
}

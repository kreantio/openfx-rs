mod processing;

use std::{
    ffi::{CStr, c_int, c_void},
    sync::{Arc, Mutex},
};

use openfx::{
    low::{Status, enums::ImageEffectPropContext},
    low_plugin::{
        Host, Plugin,
        actions::image_effect::{
            ActionDescribeInContextIn, ActionGetRegionOfDefinitionIn,
            ActionGetRegionOfDefinitionOut, ActionIsIdentityIn, ActionRenderIn, ImageEffectAction,
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
                kOfxImageEffectRenderFullySafe,
            },
            param::{
                OfxParamHandle, kOfxParamCoordinatesNormalised, kOfxParamDoubleTypeX,
                kOfxParamDoubleTypeXYAbsolute, kOfxParamTypeBoolean, kOfxParamTypeDouble,
                kOfxParamTypeDouble2D, kOfxParamTypeRGBA,
            },
        },
    },
    sys_helpers::{
        generic::properties::{
            get_OfxPropInstanceData, set_OfxPropInstanceData, set_OfxPropLabel, set_OfxPropName,
        },
        image_effect_v1::properties::{
            get_OfxImageEffectPropRenderScale, get_OfxImageEffectPropRenderWindow,
            set_OfxImageEffectPluginPropGrouping, set_OfxImageEffectPluginPropHostFrameThreading,
            set_OfxImageEffectPluginRenderThreadSafety, set_OfxImageEffectPropSupportedComponents,
            set_OfxImageEffectPropSupportedContexts, set_OfxImageEffectPropSupportedPixelDepths,
            set_OfxParamPropDefault_Double, set_OfxParamPropDefault_Int,
            set_OfxParamPropDefaultCoordinateSystem, set_OfxParamPropDisplayMax_Double,
            set_OfxParamPropDisplayMin_Double, set_OfxParamPropDoubleType, set_OfxParamPropHint,
            set_OfxParamPropMin_Double,
        },
    },
};
use processing::{pixel_processing, rect_d_to_array, rect_i_from_array};

use crate::{
    definitions::{PLUGIN_5_CIRCLE_IDENTIFIER, PLUGIN_5_CIRCLE_LABEL, PLUGINS_GROUPING},
    helpers::{
        GuaranteeSend, SharedData,
        shared_data_helper::{
            BitDepth, ClipImageManaged, SharedDataHelper, param_get_value_at_time,
        },
    },
};

static HOST_BEFORE_ACTION_LOAD: Mutex<Option<GuaranteeSend<Host>>> = Mutex::new(None);
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

pub struct PluginExampleCircle;
impl Plugin for PluginExampleCircle {
    const PLUGIN_IDENTIFIER: &'static CStr = PLUGIN_5_CIRCLE_IDENTIFIER;
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
            ImageEffectAction::GetRegionOfDefinition {
                sys_handle,
                in_args,
                out_args,
            } => action_get_region_of_definition(
                sys_handle as OfxImageEffectHandle,
                in_args,
                out_args,
            ),
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
        return Err(Status::Failed);
    }

    *data = Some({
        let data = SharedData::try_new(host)?;

        let additional = {
            let api_version = unsafe { data.host.0.host().get_api_version(data.property_suite) }?;

            // we only support 1.2 and above
            if api_version[0] == 1 && api_version[1] < 2 {
                return Err(Status::ErrMissingHostFeature);
            }

            let host_supports_multi_res = unsafe {
                data.host
                    .0
                    .host()
                    .get_image_effect_supports_multi_resolution(data.property_suite)
            }?;

            AdditionalSharedData {
                api_version: [api_version[0], api_version[1]],
                host_supports_multi_res,
            }
        };

        (data, Arc::new(additional))
    });

    Ok(())
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
    let (data, _additional) = shared_data_lockless()?;
    let data = unsafe { SharedDataHelper::try_new(&data) }?;

    let s_prop = data.inner().property_suite;

    let descriptor = unsafe { data.get_property_set_from_image_effect(descriptor) }?;

    unsafe {
        set_OfxPropLabel(s_prop, descriptor, PLUGIN_5_CIRCLE_LABEL.as_ptr())?;
        set_OfxImageEffectPluginPropGrouping(s_prop, descriptor, PLUGINS_GROUPING.as_ptr())?;
        set_OfxImageEffectPropSupportedContexts(
            s_prop,
            descriptor,
            &[kOfxImageEffectContextFilter.as_ptr()],
        )?;
        set_OfxImageEffectPropSupportedPixelDepths(
            s_prop,
            descriptor,
            &[
                kOfxBitDepthFloat.as_ptr(),
                kOfxBitDepthShort.as_ptr(),
                kOfxBitDepthByte.as_ptr(),
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
    let (data, additional) = shared_data_lockless()?;
    let data = unsafe { SharedDataHelper::try_new(&data) }?;

    let s_prop = data.inner().property_suite;
    let s_ifx = data.image_effect_suite_helper();

    let context = unsafe { in_args.get_image_effect_context(s_prop) }?;
    if context != ImageEffectPropContext::Filter {
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
                    kOfxImageComponentAlpha.as_ptr(),
                    kOfxImageComponentRGB.as_ptr(),
                ],
            )
        }?;
    }

    let param_set = unsafe { data.make_param_set_helper_for_image_effect(descriptor) }?;

    {
        let param_props = param_set.param_define(kOfxParamTypeDouble, RADIUS_PARAM_NAME)?;
        unsafe {
            set_OfxParamPropDoubleType(s_prop, param_props, kOfxParamDoubleTypeX.as_ptr())?;
            // Not supported by DaVinci Resolve. To make the plugin work there,
            // we ignore the return value here. TODO: Calculate the default value
            // in canonical coordinate if this fails.
            set_OfxParamPropDefaultCoordinateSystem(
                s_prop,
                param_props,
                kOfxParamCoordinatesNormalised.as_ptr(),
            )
            .ok();
            set_OfxParamPropDefault_Double(s_prop, param_props, &[0.25])?;
            set_OfxParamPropMin_Double(s_prop, param_props, &[0.0])?;
            set_OfxParamPropDisplayMin_Double(s_prop, param_props, &[0.0])?;
            set_OfxParamPropDisplayMax_Double(s_prop, param_props, &[2.0])?;
            set_OfxPropLabel(s_prop, param_props, c"Radius".as_ptr())?;
            set_OfxParamPropHint(s_prop, param_props, c"The radius of the circle.".as_ptr())?;
        }
    }

    {
        let param_props = param_set.param_define(kOfxParamTypeDouble2D, CENTRE_PARAM_NAME)?;
        unsafe {
            set_OfxParamPropDoubleType(
                s_prop,
                param_props,
                kOfxParamDoubleTypeXYAbsolute.as_ptr(),
            )?;
            // Not supported by DaVinci Resolve. See above.
            set_OfxParamPropDefaultCoordinateSystem(
                s_prop,
                param_props,
                kOfxParamCoordinatesNormalised.as_ptr(),
            )
            .ok();
            set_OfxParamPropDefault_Double(s_prop, param_props, &[0.5, 0.5])?;
            set_OfxPropLabel(s_prop, param_props, c"Centre".as_ptr())?;
            set_OfxParamPropHint(s_prop, param_props, c"The centre of the circle.".as_ptr())?;
        }
    }

    {
        let param_props = param_set.param_define(kOfxParamTypeRGBA, COLOUR_PARAM_NAME)?;
        unsafe {
            set_OfxParamPropDefault_Double(s_prop, param_props, &[1.0, 1.0, 1.0, 0.5])?;
            set_OfxPropLabel(s_prop, param_props, c"Colour".as_ptr())?;
            set_OfxParamPropHint(s_prop, param_props, c"The colour of the circle.".as_ptr())?;
        }
    }

    if additional.host_supports_multi_res {
        let param_props = param_set.param_define(kOfxParamTypeBoolean, GROW_ROD_PARAM_NAME)?;
        unsafe {
            set_OfxParamPropDefault_Int(s_prop, param_props, &[0])?;
            set_OfxPropLabel(s_prop, param_props, c"Grow RoD".as_ptr())?;
            set_OfxParamPropHint(
                s_prop,
                param_props,
                c"Whether to grow the output's Region of Definition to include the circle."
                    .as_ptr(),
            )?;
        }
    }

    Ok(())
}

fn action_create_instance(instance: OfxImageEffectHandle) -> openfx::low::Result<()> {
    let (data, additional) = shared_data_lockless()?;
    let data = unsafe { SharedDataHelper::try_new(&data) }?;

    let s_prop = data.inner().property_suite;
    let s_ifx = data.image_effect_suite_helper();

    let instance_props = unsafe { data.get_property_set_from_image_effect(instance) }?;

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
    match unsafe { set_OfxPropInstanceData(s_prop, instance_props, instance_data_ptr) } {
        Ok(_) => Ok(()),
        Err(err) => {
            drop(unsafe { Box::from_raw(instance_data_ptr as *mut InstanceData) });
            Err(Status::from(err))
        }
    }
}

fn action_destroy_instance(instance: OfxImageEffectHandle) -> openfx::low::Result<()> {
    let (data, _additional) = shared_data_lockless()?;
    let data = unsafe { SharedDataHelper::try_new(&data) }?;

    let s_prop = data.inner().property_suite;

    let instance_props = unsafe { data.get_property_set_from_image_effect(instance) }?;
    let instance_data_ptr = unsafe { get_OfxPropInstanceData(s_prop, instance_props) }?;
    if instance_data_ptr.is_null() {
        return Err(Status::Failed);
    }

    drop(unsafe { Box::from_raw(instance_data_ptr as *mut InstanceData) });

    Ok(())
}

fn action_get_region_of_definition(
    effect: OfxImageEffectHandle,
    in_args: ActionGetRegionOfDefinitionIn,
    out_args: ActionGetRegionOfDefinitionOut,
) -> openfx::low::Result<()> {
    let (data, additional) = shared_data_lockless()?;

    if !additional.host_supports_multi_res {
        return Err(Status::ReplyDefault);
    }

    let data = unsafe { SharedDataHelper::try_new(&data) }?;

    let instance_data = unsafe { data.get_instance_data::<InstanceData>(effect)? };

    let s_prop = data.inner().property_suite;
    let s_ifx = data.image_effect_suite_helper();
    let s_param = data.parameter_suite_helper();

    let time = unsafe { in_args.get_time(s_prop) }?;

    let growing_rod = if let Some(grow_rod_param) = instance_data.grow_rod_param {
        (unsafe { s_param.param_get_value_at_time_int(grow_rod_param, time) })? != 0
    } else {
        false
    };

    if !growing_rod {
        return Err(Status::ReplyDefault);
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

    unsafe { out_args.set_image_effect_region_of_definition(s_prop, rect_d_to_array(&rod)) }?;

    Ok(())
}

fn action_is_identity(
    effect: OfxImageEffectHandle,
    in_args: ActionIsIdentityIn,
    out_args: OfxPropertySetHandle,
) -> openfx::low::Result<()> {
    let (data, _additional) = shared_data_lockless()?;
    let data = unsafe { SharedDataHelper::try_new(&data) }?;

    let s_prop = data.inner().property_suite;
    let s_ifx = data.image_effect_suite_helper();
    let s_param = data.parameter_suite_helper();

    let instance_data = unsafe { data.get_instance_data::<InstanceData>(effect)? };

    let time = unsafe { in_args.get_time(s_prop) }?;

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
    let (data, _additional) = shared_data_lockless()?;
    let data = unsafe { SharedDataHelper::try_new(&data) }?;

    let s_prop = data.inner().property_suite;
    let s_param = data.parameter_suite_helper();

    let time = unsafe { in_args.get_time(s_prop) }?;
    let render_window =
        unsafe { get_OfxImageEffectPropRenderWindow(s_prop, in_args.sys_handle()) }?;
    let render_window = rect_i_from_array(&render_window);
    let render_scale = unsafe { get_OfxImageEffectPropRenderScale(s_prop, in_args.sys_handle()) }?;

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
        return Err(Status::Failed);
    };
    let Some(source_img_m) =
        unsafe { data.make_clip_image_managed(instance_data.source_clip, time, None) }?
    else {
        return Err(Status::Failed);
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
    ) -> openfx::low::Result<()> {
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

mod processing;

use std::{
    ffi::{CStr, c_int, c_void},
    sync::{Arc, Mutex},
};

use openfx::{
    low::{
        OutboundResult2, Status, StatusError, StatusReply,
        enums::{
            ImageEffectPluginRenderThreadSafety, ImageEffectPropContext,
            ImageEffectPropSupportedComponents, ImageEffectPropSupportedContexts,
            ImageEffectPropSupportedPixelDepths, ParamPropDefaultCoordinateSystem,
            ParamPropDoubleType,
        },
    },
    low_plugin::{
        Host, Plugin,
        actions::image_effect::{
            ActionDescribeInContextIn, ActionGetRegionOfDefinitionIn,
            ActionGetRegionOfDefinitionOut, ActionIsIdentityIn, ActionIsIdentityOut,
            ActionRenderIn, ImageEffectAction,
        },
        objects::{ImageEffectDescriptor, ImageEffectInstance},
        property_sets::{
            ParamsBytePropertySet, ParamsDouble1DPropertySet, ParamsDouble2D3DPropertySet,
            ParamsNormalizedSpatialPropertySet,
        },
    },
    sys::{
        generic::core::{kOfxPropInstanceData, kOfxPropLabel},
        image_effect_v1::param::{
            OfxParamHandle, kOfxParamTypeBoolean, kOfxParamTypeDouble, kOfxParamTypeDouble2D,
            kOfxParamTypeRGBA,
        },
    },
};
use processing::{pixel_processing, rect_d_to_array, rect_i_from_array};

use crate::{
    definitions::{PLUGIN_5_CIRCLE_IDENTIFIER, PLUGIN_5_CIRCLE_LABEL, PLUGINS_GROUPING},
    helpers::shared_data::{
        BitDepth, ClipImageManaged, GuaranteeSend, GuaranteeSendClipInInstance, SharedData,
        param_get_value_at_time,
    },
};

static HOST_BEFORE_ACTION_LOAD: Mutex<Option<GuaranteeSend<Host>>> = Mutex::new(None);
static SHARED_DATA: Mutex<Option<(SharedData, Arc<AdditionalSharedData>)>> = Mutex::new(None);

struct AdditionalSharedData {
    #[expect(unused)]
    api_version: [c_int; 2],
    host_supports_multi_res: bool,
}

struct MyInstanceData {
    source_clip: GuaranteeSendClipInInstance,
    output_clip: GuaranteeSendClipInInstance,

    radius_param: OfxParamHandle,
    centre_param: OfxParamHandle,
    colour_param: OfxParamHandle,
    grow_rod_param: Option<OfxParamHandle>,
}

fn shared_data_lockless() -> Result<(SharedData, Arc<AdditionalSharedData>), StatusError> {
    let data = SHARED_DATA.lock().map_err(|_| StatusError::Failed)?;
    let data = data.as_ref().ok_or(StatusError::Failed)?;
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

    fn main_entry(action: ImageEffectAction) -> OutboundResult2 {
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
            ImageEffectAction::GetRegionOfDefinition {
                handle,
                in_args,
                out_args,
            } => action_get_region_of_definition(handle, in_args, out_args),
            ImageEffectAction::Render {
                handle, in_args, ..
            } => action_render(handle, in_args),
            _ => Ok(StatusReply::ReplyDefault.into()),
        }
    }
}

fn action_load() -> OutboundResult2 {
    let host = HOST_BEFORE_ACTION_LOAD
        .lock()
        .map_err(|_| StatusError::Failed)?
        .take()
        .ok_or(StatusError::Failed)?;

    let mut data = SHARED_DATA.lock().map_err(|_| StatusError::Failed)?;
    if data.is_some() {
        return Err(StatusError::Failed);
    }

    *data = Some({
        let data = SharedData::try_new(host)?;

        let additional = {
            let api_version = unsafe { data.host.host().get_api_version(&data.property_suite.0) }
                .ok_or(StatusError::Failed)?;

            // we only support 1.2 and above
            if api_version[0] == 1 && api_version[1] < 2 {
                return Err(StatusError::ErrMissingHostFeature);
            }

            let host_supports_multi_res = unsafe {
                data.host
                    .host()
                    .get_image_effect_supports_multi_resolution(&data.property_suite.0)
            }
            .ok_or(StatusError::Failed)?;

            AdditionalSharedData {
                api_version: [api_version[0], api_version[1]],
                host_supports_multi_res,
            }
        };

        (data, Arc::new(additional))
    });

    Ok(Status::OK)
}

fn action_unload() -> OutboundResult2 {
    let mut data = SHARED_DATA.lock().map_err(|_| StatusError::Failed)?;
    if data.take().is_none() {
        Err(StatusError::Failed)
    } else {
        Ok(Status::OK)
    }
}

fn action_describe(descriptor: ImageEffectDescriptor) -> OutboundResult2 {
    let (data, _additional) = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;

    let props = unsafe { descriptor.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    unsafe {
        props
            .set_label(s_prop, Some(PLUGIN_5_CIRCLE_LABEL))
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
) -> OutboundResult2 {
    let (data, additional) = shared_data_lockless()?;

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
            unsafe { param_set.param_define(s_param, kOfxParamTypeDouble, RADIUS_PARAM_NAME) }
                .ok_or(StatusError::Failed)?;
        let param_props_d = ParamsDouble1DPropertySet::from(param_props);
        let param_props_ns = ParamsNormalizedSpatialPropertySet::from(param_props);

        unsafe {
            param_props_d
                .set_param_double_type(s_prop, ParamPropDoubleType::X.into())
                .ok_or(StatusError::Failed)?;
            // Not supported by DaVinci Resolve. To make the plugin work there,
            // we ignore the return value here. TODO: Calculate the default value
            // in canonical coordinate if this fails.
            param_props_ns
                .set_param_default_coordinate_system(
                    s_prop,
                    ParamPropDefaultCoordinateSystem::Normalised.into(),
                )
                .ok();
            param_props_d
                .set_param_default_double(s_prop, &[0.25])
                .ok_or(StatusError::Failed)?;
            param_props_d
                .set_param_min_double(s_prop, &[0.0])
                .ok_or(StatusError::Failed)?;
            param_props_d
                .set_param_display_min_double(s_prop, &[0.0])
                .ok_or(StatusError::Failed)?;
            param_props_d
                .set_param_display_max_double(s_prop, &[2.0])
                .ok_or(StatusError::Failed)?;
            param_props_d
                .set_label(s_prop, Some(c"Radius"))
                .ok_or(StatusError::Failed)?;
            param_props_d
                .set_param_hint(s_prop, Some(c"The radius of the circle."))
                .ok_or(StatusError::Failed)?;
        }
    }

    {
        let param_props =
            unsafe { param_set.param_define(s_param, kOfxParamTypeDouble2D, CENTRE_PARAM_NAME) }
                .ok_or(StatusError::Failed)?;
        let param_props_d = ParamsDouble2D3DPropertySet::from(param_props);
        let param_props_ns = ParamsNormalizedSpatialPropertySet::from(param_props);

        unsafe {
            param_props_d
                .set_param_double_type(s_prop, ParamPropDoubleType::XYAbsolute.into())
                .ok_or(StatusError::Failed)?;
            param_props_ns
                .set_param_default_coordinate_system(
                    s_prop,
                    ParamPropDefaultCoordinateSystem::Normalised.into(),
                )
                .ok();
            param_props_d
                .set_param_default_double(s_prop, &[0.5, 0.5])
                .ok_or(StatusError::Failed)?;
            param_props_d
                .set_label(s_prop, Some(c"Centre"))
                .ok_or(StatusError::Failed)?;
            param_props_d
                .set_param_hint(s_prop, Some(c"The centre of the circle."))
                .ok_or(StatusError::Failed)?;
        }
    }

    {
        let param_props =
            unsafe { param_set.param_define(s_param, kOfxParamTypeRGBA, COLOUR_PARAM_NAME) }
                .ok_or(StatusError::Failed)?;
        let param_props_d = ParamsDouble2D3DPropertySet::from(param_props);

        unsafe {
            param_props_d
                .set_param_default_double(s_prop, &[1.0, 1.0, 1.0, 0.5])
                .ok_or(StatusError::Failed)?;
            param_props_d
                .set_label(s_prop, Some(c"Colour"))
                .ok_or(StatusError::Failed)?;
            param_props_d
                .set_param_hint(s_prop, Some(c"The colour of the circle."))
                .ok_or(StatusError::Failed)?;
        }
    }

    if additional.host_supports_multi_res {
        let param_props =
            unsafe { param_set.param_define(s_param, kOfxParamTypeBoolean, GROW_ROD_PARAM_NAME) }
                .ok_or(StatusError::Failed)?;
        let param_props = ParamsBytePropertySet::from(param_props);

        unsafe {
            param_props
                .set_param_default_int(s_prop, &[0])
                .ok_or(StatusError::Failed)?;
            param_props
                .set_label(s_prop, Some(c"Grow RoD"))
                .ok_or(StatusError::Failed)?;
            param_props
                .set_param_hint(
                    s_prop,
                    Some(
                        c"Whether to grow the output's Region of Definition to include the circle.",
                    ),
                )
                .ok_or(StatusError::Failed)?;
        }
    }

    Ok(Status::OK)
}

fn action_create_instance(instance: ImageEffectInstance) -> OutboundResult2 {
    let (data, additional) = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;
    let s_param = &data.parameter_suite.0;

    let instance_props = unsafe { instance.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    let source_clip =
        unsafe { instance.clip_get_clip_handle(s_ifx, c"Source") }.ok_or(StatusError::Failed)?;
    let output_clip =
        unsafe { instance.clip_get_clip_handle(s_ifx, c"Output") }.ok_or(StatusError::Failed)?;

    let param_set = unsafe { instance.get_param_set(s_ifx) }.ok_or(StatusError::Failed)?;
    let radius_param = unsafe { param_set.param_get_param_handle(s_param, RADIUS_PARAM_NAME) }
        .ok_or(StatusError::Failed)?;
    let centre_param = unsafe { param_set.param_get_param_handle(s_param, CENTRE_PARAM_NAME) }
        .ok_or(StatusError::Failed)?;
    let colour_param = unsafe { param_set.param_get_param_handle(s_param, COLOUR_PARAM_NAME) }
        .ok_or(StatusError::Failed)?;
    let grow_rod_param = if additional.host_supports_multi_res {
        Some(
            unsafe { param_set.param_get_param_handle(s_param, GROW_ROD_PARAM_NAME) }
                .ok_or(StatusError::Failed)?,
        )
    } else {
        None
    };

    let my_data = MyInstanceData {
        source_clip: GuaranteeSendClipInInstance(source_clip),
        output_clip: GuaranteeSendClipInInstance(output_clip),
        radius_param,
        centre_param,
        colour_param,
        grow_rod_param,
    };
    let my_data_ptr = Box::into_raw(Box::new(my_data)) as *mut c_void;

    // SAFETY: the pointee is kept alive by `Box::into_raw` until it is
    // reclaimed with `Box::from_raw` in `action_destroy_instance`.
    unsafe {
        if !s_prop
            .prop_set_pointer(
                instance_props.sys_handle(),
                kOfxPropInstanceData,
                std::ptr::NonNull::new(my_data_ptr),
            )
            .is_ok()
        {
            drop(Box::from_raw(my_data_ptr as *mut MyInstanceData));
            return Err(StatusError::Failed);
        }
    }

    Ok(Status::OK)
}

fn action_destroy_instance(instance: ImageEffectInstance) -> OutboundResult2 {
    let (data, _additional) = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;

    let props = unsafe { instance.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    let Some(my_data_ptr) =
        unsafe { props.get_instance_data(s_prop) }.ok_or(StatusError::Failed)?
    else {
        return Err(StatusError::Failed);
    };

    drop(unsafe { Box::from_raw(my_data_ptr.as_ptr() as *mut MyInstanceData) });

    Ok(Status::OK)
}

fn action_get_region_of_definition(
    effect: ImageEffectInstance,
    in_args: ActionGetRegionOfDefinitionIn,
    out_args: ActionGetRegionOfDefinitionOut,
) -> OutboundResult2 {
    let (data, additional) = shared_data_lockless()?;

    if !additional.host_supports_multi_res {
        return Ok(StatusReply::ReplyDefault.into());
    }

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;
    let s_param = &data.parameter_suite.0;

    let instance_props = unsafe { effect.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    let time = unsafe { in_args.get_time(s_prop) }.ok_or(StatusError::Failed)?;

    let Some(my_data_ptr) =
        unsafe { instance_props.get_instance_data(s_prop) }.ok_or(StatusError::Failed)?
    else {
        return Err(StatusError::Failed);
    };
    let my_data = unsafe { &*(my_data_ptr.as_ptr() as *const MyInstanceData) };

    let mut growing_rod = false;
    if let Some(grow_rod_param) = my_data.grow_rod_param {
        let mut growing_rod_: c_int = Default::default();
        param_get_value_at_time!(s_param, grow_rod_param, time, &mut growing_rod_);
        growing_rod = growing_rod_ != 0;
    };

    if !growing_rod {
        return Ok(StatusReply::ReplyDefault.into());
    }

    let mut radius: f64 = Default::default();
    param_get_value_at_time!(s_param, my_data.radius_param, time, &mut radius);
    let mut centre_x = 0.0;
    let mut centre_y = 0.0;
    param_get_value_at_time!(
        s_param,
        my_data.centre_param,
        time,
        &mut centre_x,
        &mut centre_y
    );

    let mut rod = unsafe {
        my_data
            .source_clip
            .0
            .clip_get_region_of_definition(s_ifx, time)
    }
    .ok_or(StatusError::Failed)?;

    rod.x1 = f64::min(rod.x1, centre_x - radius);
    rod.y1 = f64::min(rod.y1, centre_y - radius);
    rod.x2 = f64::max(rod.x2, centre_x + radius);
    rod.y2 = f64::max(rod.y2, centre_y + radius);

    unsafe { out_args.set_image_effect_region_of_definition(s_prop, rect_d_to_array(&rod)) }
        .ok_or(StatusError::Failed)?;

    Ok(Status::OK)
}

fn action_is_identity(
    effect: ImageEffectInstance,
    in_args: ActionIsIdentityIn,
    out_args: ActionIsIdentityOut,
) -> OutboundResult2 {
    let (data, _additional) = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;
    let s_param = &data.parameter_suite.0;

    let instance_props = unsafe { effect.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    let time = unsafe { in_args.get_time(s_prop) }.ok_or(StatusError::Failed)?;

    let Some(my_data_ptr) =
        unsafe { instance_props.get_instance_data(s_prop) }.ok_or(StatusError::Failed)?
    else {
        return Err(StatusError::Failed);
    };
    let my_data = unsafe { &*(my_data_ptr.as_ptr() as *const MyInstanceData) };

    let mut radius: f64 = Default::default();
    param_get_value_at_time!(s_param, my_data.radius_param, time, &mut radius);

    let is_identity = if radius < 0.0001 {
        true
    } else {
        let mut growing_rod = false;
        if let Some(grow_rod_param) = my_data.grow_rod_param {
            let mut growing_rod_: c_int = Default::default();
            param_get_value_at_time!(s_param, grow_rod_param, time, &mut growing_rod_);
            growing_rod = growing_rod_ != 0;
        };

        if growing_rod {
            false
        } else {
            let bounds = unsafe {
                my_data
                    .source_clip
                    .0
                    .clip_get_region_of_definition(s_ifx, time)
            }
            .ok_or(StatusError::Failed)?;

            let mut centre_x = 0.0;
            let mut centre_y = 0.0;
            param_get_value_at_time!(
                s_param,
                my_data.centre_param,
                time,
                &mut centre_x,
                &mut centre_y
            );

            centre_x + radius < bounds.x1
                || centre_x - radius > bounds.x2
                || centre_y + radius < bounds.y1
                || centre_y - radius > bounds.y2
        }
    };

    if is_identity {
        (unsafe { s_prop.prop_set_string(out_args.sys_handle(), kOfxPropLabel, Some(c"Source")) })
            .ok_or(StatusError::Failed)?;
        Ok(Status::OK)
    } else {
        Ok(StatusReply::ReplyDefault.into())
    }
}

fn action_render(instance: ImageEffectInstance, in_args: ActionRenderIn) -> OutboundResult2 {
    let (data, _additional) = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;
    let s_param = &data.parameter_suite.0;

    let instance_props = unsafe { instance.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    let time = unsafe { in_args.get_time(s_prop) }.ok_or(StatusError::Failed)?;
    let render_window =
        unsafe { in_args.get_image_effect_render_window(s_prop) }.ok_or(StatusError::Failed)?;
    let render_window = rect_i_from_array(&render_window);
    let render_scale =
        unsafe { in_args.get_image_effect_render_scale(s_prop) }.ok_or(StatusError::Failed)?;

    let Some(my_data_ptr) =
        unsafe { instance_props.get_instance_data(s_prop) }.ok_or(StatusError::Failed)?
    else {
        return Err(StatusError::Failed);
    };
    let my_data = unsafe { &*(my_data_ptr.as_ptr() as *const MyInstanceData) };

    let mut radius: f64 = Default::default();
    param_get_value_at_time!(s_param, my_data.radius_param, time, &mut radius);
    let centre = {
        let mut centre_x = 0.0;
        let mut centre_y = 0.0;
        param_get_value_at_time!(
            s_param,
            my_data.centre_param,
            time,
            &mut centre_x,
            &mut centre_y
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
            my_data.colour_param,
            time,
            &mut colour_r,
            &mut colour_g,
            &mut colour_b,
            &mut colour_a
        );
        [colour_r, colour_g, colour_b, colour_a]
    };

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
            |v, min, max| v.clamp(min, max),
            255u8,
            centre,
            radius,
            colour,
            render_scale,
            &data,
            instance,
            source_img_m,
            output_img_m,
            render_window,
        )?,
        BitDepth::Short => pixel_processing(
            |f| f as u16,
            |v| v as f64,
            |v, min, max| v.clamp(min, max),
            65535u16,
            centre,
            radius,
            colour,
            render_scale,
            &data,
            instance,
            source_img_m,
            output_img_m,
            render_window,
        )?,
        BitDepth::Float => pixel_processing(
            |f| f as f32,
            |v| v as f64,
            |v, min, max| v.clamp(min, max),
            1.0f32,
            centre,
            radius,
            colour,
            render_scale,
            &data,
            instance,
            source_img_m,
            output_img_m,
            render_window,
        )?,
    };

    Ok(Status::OK)
}

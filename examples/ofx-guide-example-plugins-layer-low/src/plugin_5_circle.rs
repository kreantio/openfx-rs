mod processing;

use std::{
    ffi::{CStr, c_int, c_void},
    sync::{Arc, Mutex},
};

use openfx::{
    low::{
        Status,
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
            ActionGetRegionOfDefinitionOut, ActionIsIdentityIn, ActionRenderIn, ImageEffectAction,
        },
        objects::{ImageEffectDescriptor, ImageEffectInstance},
        property_sets::{
            ParamBytePropertySet, ParamDouble1DPropertySet, ParamDouble2D3DPropertySet,
            ParamNormalizedSpatialPropertySet,
        },
    },
    sys::{
        generic::core::{OfxPropertySetHandle, kOfxStatFailed},
        image_effect_v1::param::{
            OfxParamHandle, kOfxParamTypeBoolean, kOfxParamTypeDouble, kOfxParamTypeDouble2D,
            kOfxParamTypeRGBA,
        },
    },
    sys_helpers::{
        generic::properties::{set_OfxPropInstanceData, set_OfxPropName},
        image_effect_v1::properties::{
            get_OfxImageEffectPropRenderScale, get_OfxImageEffectPropRenderWindow,
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

fn shared_data_lockless() -> openfx::low::Result<(SharedData, Arc<AdditionalSharedData>)> {
    let data = SHARED_DATA.lock().map_err(|_| Status::Failed)?;
    let data = data.as_ref().ok_or(Status::Failed)?;
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
            ImageEffectAction::GetRegionOfDefinition {
                handle,
                in_args,
                out_args,
            } => action_get_region_of_definition(handle, in_args, out_args),
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
            let api_version = unsafe { data.host.host().get_api_version(&data.property_suite.0) }?;

            // we only support 1.2 and above
            if api_version[0] == 1 && api_version[1] < 2 {
                return Err(Status::ErrMissingHostFeature);
            }

            let host_supports_multi_res = unsafe {
                data.host
                    .host()
                    .get_image_effect_supports_multi_resolution(&data.property_suite.0)
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

fn action_describe(descriptor: ImageEffectDescriptor) -> openfx::low::Result<()> {
    let (data, _additional) = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;

    let props = unsafe { descriptor.get_property_set(s_ifx) }?;

    unsafe {
        props.set_label(s_prop, Some(PLUGIN_5_CIRCLE_LABEL))?;
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
    descriptor: ImageEffectDescriptor,
    in_args: ActionDescribeInContextIn,
) -> openfx::low::Result<()> {
    let (data, additional) = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;

    let context = unsafe { in_args.get_image_effect_context(s_prop) }?;
    if context != ImageEffectPropContext::Filter {
        return Err(Status::ErrUnsupported);
    }

    for name in [c"Output", c"Source"] {
        let props = unsafe { descriptor.clip_define(s_ifx, name) }?;

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

    let param_set = unsafe { data.make_param_set_helper_for_image_effect_descriptor(&descriptor) }?;

    {
        let param_props = param_set.param_define(kOfxParamTypeDouble, RADIUS_PARAM_NAME)?;
        let param_props_d = ParamDouble1DPropertySet::from(param_props);
        let param_props_ns = ParamNormalizedSpatialPropertySet::from(param_props);

        unsafe {
            param_props_d.set_param_double_type(s_prop, ParamPropDoubleType::X)?;
            // Not supported by DaVinci Resolve. To make the plugin work there,
            // we ignore the return value here. TODO: Calculate the default value
            // in canonical coordinate if this fails.
            param_props_ns
                .set_param_default_coordinate_system(
                    s_prop,
                    ParamPropDefaultCoordinateSystem::Normalised,
                )
                .ok();
            param_props_d.set_param_default_double(s_prop, &[0.25])?;
            param_props_d.set_param_min_double(s_prop, &[0.0])?;
            param_props_d.set_param_display_min_double(s_prop, &[0.0])?;
            param_props_d.set_param_display_max_double(s_prop, &[2.0])?;
            param_props_d.set_label(s_prop, Some(c"Radius"))?;
            param_props_d.set_param_hint(s_prop, Some(c"The radius of the circle."))?;
        }
    }

    {
        let param_props = param_set.param_define(kOfxParamTypeDouble2D, CENTRE_PARAM_NAME)?;
        let param_props_d = ParamDouble2D3DPropertySet::from(param_props);
        let param_props_ns = ParamNormalizedSpatialPropertySet::from(param_props);

        unsafe {
            param_props_d.set_param_double_type(s_prop, ParamPropDoubleType::XYAbsolute)?;
            param_props_ns
                .set_param_default_coordinate_system(
                    s_prop,
                    ParamPropDefaultCoordinateSystem::Normalised,
                )
                .ok();
            param_props_d.set_param_default_double(s_prop, &[0.5, 0.5])?;
            param_props_d.set_label(s_prop, Some(c"Centre"))?;
            param_props_d.set_param_hint(s_prop, Some(c"The centre of the circle."))?;
        }
    }

    {
        let param_props = param_set.param_define(kOfxParamTypeRGBA, COLOUR_PARAM_NAME)?;
        let param_props_d = ParamDouble2D3DPropertySet::from(param_props);

        unsafe {
            param_props_d.set_param_default_double(s_prop, &[1.0, 1.0, 1.0, 0.5])?;
            param_props_d.set_label(s_prop, Some(c"Colour"))?;
            param_props_d.set_param_hint(s_prop, Some(c"The colour of the circle."))?;
        }
    }

    if additional.host_supports_multi_res {
        let param_props = param_set.param_define(kOfxParamTypeBoolean, GROW_ROD_PARAM_NAME)?;
        let param_props = ParamBytePropertySet::from(param_props);

        unsafe {
            param_props.set_param_default_int(s_prop, &[0])?;
            param_props.set_label(s_prop, Some(c"Grow RoD"))?;
            param_props.set_param_hint(
                s_prop,
                Some(c"Whether to grow the output's Region of Definition to include the circle."),
            )?;
        }
    }

    Ok(())
}

fn action_create_instance(instance: ImageEffectInstance) -> openfx::low::Result<()> {
    let (data, additional) = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;

    let instance_props = unsafe { instance.get_property_set(s_ifx) }?;

    let source_clip = unsafe { instance.clip_get_clip_handle(s_ifx, c"Source") }?;
    let output_clip = unsafe { instance.clip_get_clip_handle(s_ifx, c"Output") }?;

    let param_set = unsafe { data.make_param_set_helper_for_image_effect_instance(&instance) }?;
    let radius_param = param_set.param_get_handle(RADIUS_PARAM_NAME)?;
    let centre_param = param_set.param_get_handle(CENTRE_PARAM_NAME)?;
    let colour_param = param_set.param_get_handle(COLOUR_PARAM_NAME)?;
    let grow_rod_param = if additional.host_supports_multi_res {
        Some(param_set.param_get_handle(GROW_ROD_PARAM_NAME)?)
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
    let (data, _additional) = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;

    let props = unsafe { instance.get_property_set(s_ifx) }?;

    let Some(my_data_ptr) = (unsafe { props.get_instance_data(s_prop)? }) else {
        return Err(Status::Failed);
    };

    drop(unsafe { Box::from_raw(my_data_ptr.as_ptr() as *mut MyInstanceData) });

    Ok(())
}

fn action_get_region_of_definition(
    effect: ImageEffectInstance,
    in_args: ActionGetRegionOfDefinitionIn,
    out_args: ActionGetRegionOfDefinitionOut,
) -> openfx::low::Result<()> {
    let (data, additional) = shared_data_lockless()?;

    if !additional.host_supports_multi_res {
        return Err(Status::ReplyDefault);
    }

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;
    let s_param = data.parameter_suite_helper();

    let instance_props = unsafe { effect.get_property_set(s_ifx) }?;

    let time = unsafe { in_args.get_time(s_prop) }?;

    let Some(my_data_ptr) = (unsafe { instance_props.get_instance_data(s_prop)? }) else {
        return Err(Status::Failed);
    };
    let my_data = unsafe { &*(my_data_ptr.as_ptr() as *const MyInstanceData) };

    let growing_rod = if let Some(grow_rod_param) = my_data.grow_rod_param {
        (unsafe { s_param.param_get_value_at_time_int(grow_rod_param, time) })? != 0
    } else {
        false
    };

    if !growing_rod {
        return Err(Status::ReplyDefault);
    }

    let radius = unsafe { s_param.param_get_value_at_time_double(my_data.radius_param, time) }?;
    let mut centre_x = 0.0;
    let mut centre_y = 0.0;
    param_get_value_at_time!(
        s_param,
        my_data.centre_param,
        time,
        &mut centre_x,
        &mut centre_y,
    );

    let mut rod = unsafe {
        my_data
            .source_clip
            .0
            .clip_get_region_of_definition(s_ifx, time)
    }?;

    rod.x1 = f64::min(rod.x1, centre_x - radius);
    rod.y1 = f64::min(rod.y1, centre_y - radius);
    rod.x2 = f64::max(rod.x2, centre_x + radius);
    rod.y2 = f64::max(rod.y2, centre_y + radius);

    unsafe { out_args.set_image_effect_region_of_definition(s_prop, rect_d_to_array(&rod)) }?;

    Ok(())
}

fn action_is_identity(
    effect: ImageEffectInstance,
    in_args: ActionIsIdentityIn,
    out_args: OfxPropertySetHandle,
) -> openfx::low::Result<()> {
    let (data, _additional) = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;
    let s_param = data.parameter_suite_helper();

    let instance_props = unsafe { effect.get_property_set(s_ifx) }?;

    let time = unsafe { in_args.get_time(s_prop) }?;

    let Some(my_data_ptr) = (unsafe { instance_props.get_instance_data(s_prop)? }) else {
        return Err(Status::Failed);
    };
    let my_data = unsafe { &*(my_data_ptr.as_ptr() as *const MyInstanceData) };

    let radius = unsafe { s_param.param_get_value_at_time_double(my_data.radius_param, time) }?;

    let is_identity = if radius < 0.0001 {
        true
    } else {
        let growing_rod = if let Some(grow_rod_param) = my_data.grow_rod_param {
            (unsafe { s_param.param_get_value_at_time_int(grow_rod_param, time) })? != 0
        } else {
            false
        };

        if growing_rod {
            false
        } else {
            let bounds = unsafe {
                my_data
                    .source_clip
                    .0
                    .clip_get_region_of_definition(s_ifx, time)
            }?;

            let mut centre_x = 0.0;
            let mut centre_y = 0.0;
            param_get_value_at_time!(
                s_param,
                my_data.centre_param,
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
    let (data, _additional) = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;
    let s_param = data.parameter_suite_helper();

    let instance_props = unsafe { instance.get_property_set(s_ifx) }?;

    let time = unsafe { in_args.get_time(s_prop) }?;
    let render_window =
        unsafe { get_OfxImageEffectPropRenderWindow(s_prop.sys_ptr(), in_args.sys_handle()) }?;
    let render_window = rect_i_from_array(&render_window);
    let render_scale =
        unsafe { get_OfxImageEffectPropRenderScale(s_prop.sys_ptr(), in_args.sys_handle()) }?;

    let Some(my_data_ptr) = (unsafe { instance_props.get_instance_data(s_prop)? }) else {
        return Err(Status::Failed);
    };
    let my_data = unsafe { &*(my_data_ptr.as_ptr() as *const MyInstanceData) };

    let radius = unsafe { s_param.param_get_value_at_time_double(my_data.radius_param, time) }?;
    let centre = {
        let mut centre_x = 0.0;
        let mut centre_y = 0.0;
        param_get_value_at_time!(
            s_param,
            my_data.centre_param,
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
            my_data.colour_param,
            time,
            &mut colour_r,
            &mut colour_g,
            &mut colour_b,
            &mut colour_a,
        );
        [colour_r, colour_g, colour_b, colour_a]
    };

    let output_img = unsafe { my_data.output_clip.0.clip_get_image(s_ifx, time, None) }?;
    let source_img = unsafe { my_data.source_clip.0.clip_get_image(s_ifx, time, None) }?;

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
            &data,
            instance,
            source_img_m,
            output_img_m,
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
            &data,
            instance,
            source_img_m,
            output_img_m,
            render_window,
        ),
    }
}

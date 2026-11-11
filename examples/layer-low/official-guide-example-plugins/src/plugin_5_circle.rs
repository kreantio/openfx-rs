//! This example is a loose port of: https://github.com/AcademySoftwareFoundation/openfx/blob/b2d2eea3a64e511bfabb80e56f902d96adf4a05e/Documentation/sources/Guide/Code/Example5/circle.cpp
//!
//! Copyright notice of the original code:
//! ```text
//! Copyright OpenFX and contributors to the OpenFX project.
//! SPDX-License-Identifier: BSD-3-Clause
//! ```
//!
//! Author of the original code: Bruno Nicoletti (2014)
//!
//! This plugin will take you through the basics of defining and using
//! parameters as well as how to use instance data.
//!
//! Known issues:
//! - [`openfx::low::enums::ParamPropDefaultCoordinateSystem::Normalised`] is
//!   not supported by DaVinci Resolve (21.0.4 BUILD 5).

use std::{
    ffi::{CStr, c_int, c_void},
    sync::{Arc, Mutex},
};

use openfx::{
    low::{
        OutboundResult, Status, StatusError, StatusReply,
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
        parameters::{ParamBoolean, ParamDouble, ParamDouble2D, ParamRGBA},
    },
    sys::core::{OfxRectI, kOfxPropLabel},
};

use crate::{
    definitions::{PLUGIN_5_CIRCLE_IDENTIFIER, PLUGIN_5_CIRCLE_LABEL, PLUGINS_GROUPING},
    helpers::{
        internal_utils::{
            BitDepth, ClipImageManaged, blend, rect_d_to_array, rect_i_from_array, set_labels,
        },
        shared_data::{GuaranteeSend, GuaranteeSendClipInstance, SharedData},
    },
};

// name of our params
const RADIUS_PARAM_NAME: &CStr = c"radius";
const CENTRE_PARAM_NAME: &CStr = c"centre";
const COLOUR_PARAM_NAME: &CStr = c"colour";
const GROW_ROD_PARAM_NAME: &CStr = c"growRoD";

static HOST_BEFORE_ACTION_LOAD: Mutex<Option<GuaranteeSend<Host>>> = Mutex::new(None);
static GLOBAL_DATA: Mutex<Option<(SharedData, Arc<AdditionalSharedData>)>> = Mutex::new(None);

struct AdditionalSharedData {
    /// Version of the API the host is running.
    #[expect(unused)]
    api_version: [c_int; 2],
    /// Does the host support multi resolution images?
    host_supports_multi_res: bool,
}

/// Our instance data, where we are caching away clip and param objects.
struct MyInstanceData {
    // wrapper objects to the clips we deal with
    source_clip: GuaranteeSendClipInstance,
    output_clip: GuaranteeSendClipInstance,

    // wrapper objects to our parameters
    radius_param: ParamDouble,
    centre_param: ParamDouble2D,
    colour_param: ParamRGBA,
    grow_rod_param: Option<ParamBoolean>,
}

fn shared_data_lockless() -> Result<(SharedData, Arc<AdditionalSharedData>), StatusError> {
    let data = GLOBAL_DATA.lock().map_err(|_| StatusError::Failed)?;
    let data = data.as_ref().ok_or(StatusError::Failed)?;
    Ok((data.0.clone(), data.1.clone()))
}

/// The high-level representation of the [`openfx::sys::core::OfxPlugin`] struct
/// that is passed back to the host application to initiate bootstrapping of
/// plugin communications.
pub struct PluginExampleCircle;
impl Plugin for PluginExampleCircle {
    // The unique ID of this plugin.
    const PLUGIN_IDENTIFIER: &'static CStr = PLUGIN_5_CIRCLE_IDENTIFIER;
    // The major version number of this plugin.
    const PLUGIN_VERSION_MAJOR: std::ffi::c_uint = 1;
    // The minor version number of this plugin.
    const PLUGIN_VERSION_MINOR: std::ffi::c_uint = 0;

    /// Call back passed to the host in the [`openfx::sys::core::OfxPlugin`]
    /// struct to set our host pointer.
    ///
    /// This must be called AFTER both `OfxGetNumberOfPlugins` and
    /// `OfxGetPlugin`, but before the `pluginMain` entry point (i.e.,
    /// [`Self::main_entry`]) is ever touched.
    fn set_host(host: Host) {
        let mut lock = HOST_BEFORE_ACTION_LOAD
            .lock()
            .expect("Failed to lock HOST_BEFORE_ACTION_LOAD.");
        if lock.is_some() {
            panic!("HOST_BEFORE_ACTION_LOAD has already been set.");
        }
        lock.replace(GuaranteeSend(host));
    }

    /// The main entry point function, the host calls this to get the plugin to
    /// do things.
    fn main_entry(action: ImageEffectAction) -> OutboundResult {
        match action {
            // The very first action called on a plugin.
            ImageEffectAction::Load { .. } => action_load(),
            // The very last action called.
            // NOTE: This is not part of the original example. We use it to
            // explicitly drop `GLOBAL_DATA`.
            ImageEffectAction::Unload { .. } => action_unload(),
            // The first action called to describe what the plugin does.
            ImageEffectAction::Describe { handle, .. } => action_describe(handle),
            // The second action called to describe what the plugin does in a
            // specific context.
            ImageEffectAction::DescribeInContext {
                handle, in_args, ..
            } => action_describe_in_context(handle, in_args),
            // The action called when an instance of a plugin is created.
            ImageEffectAction::CreateInstance { handle, .. } => action_create_instance(handle),
            // The action called when an instance of a plugin is destroyed.
            ImageEffectAction::DestroyInstance { handle, .. } => action_destroy_instance(handle),
            // Check to see if our param settings cause nothing to happen.
            ImageEffectAction::IsIdentity {
                handle,
                in_args,
                out_args,
            } => action_is_identity(handle, in_args, out_args),
            // NOTE: Unlike the original example, the `host_supports_multi_res`
            // check is inside the `action_get_region_of_definition` function.
            ImageEffectAction::GetRegionOfDefinition {
                handle,
                in_args,
                out_args,
            } => action_get_region_of_definition(handle, in_args, out_args),
            // The action called to render a frame.
            ImageEffectAction::Render {
                handle, in_args, ..
            } => action_render(handle, in_args),
            // Other actions to take the default value.
            _ => Ok(StatusReply::ReplyDefault.into()),
        }
    }
}

/// The first *action* called after the binary is loaded (bootstrapper
/// functions (e.g., `OfxGetNumberOfPlugins`) are, however, called before any
/// action is called.)
fn action_load() -> OutboundResult {
    let host = HOST_BEFORE_ACTION_LOAD
        .lock()
        .map_err(|_| StatusError::Failed)?
        .take()
        .ok_or(StatusError::Failed)?;

    let mut data = GLOBAL_DATA.lock().map_err(|_| StatusError::Failed)?;
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

            // does the host support multi-resolution images?
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

/// The last action called before a plugin binary is unloaded.
fn action_unload() -> OutboundResult {
    let mut data = GLOBAL_DATA.lock().map_err(|_| StatusError::Failed)?;
    if data.take().is_none() {
        Err(StatusError::Failed)
    } else {
        Ok(Status::OK)
    }
}

/// The plugin's basic description routine.
fn action_describe(descriptor: ImageEffectDescriptor) -> OutboundResult {
    let (data, _additional) = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;

    // get the property set object for the plugin
    let props = unsafe { descriptor.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    unsafe {
        // set some labels and the group it belongs to.
        // NOTE: The original example only sets `kOfxPropLabel`. Instead, we set
        // all three labels at once so that the plugin's label displays
        // correctly in DaVinci Resolve's Cut tab.
        set_labels(&data.property_suite.0, &props, PLUGIN_5_CIRCLE_LABEL)?;
        props
            .set_image_effect_plugin_grouping(s_prop, Some(PLUGINS_GROUPING))
            .ok_or(StatusError::Failed)?;

        // define the image effects contexts we can be used in, in this case a
        // simple filter
        props
            .set_image_effect_supported_contexts(
                s_prop,
                &[ImageEffectPropSupportedContexts::Filter.into()],
            )
            .ok_or(StatusError::Failed)?;

        // set the bit depths the plugin can handle
        {
            use ImageEffectPropSupportedPixelDepths as D;
            props
                .set_image_effect_supported_pixel_depths(
                    s_prop,
                    &[D::Float.into(), D::Short.into(), D::Byte.into()],
                )
                .ok_or(StatusError::Failed)?;
        }

        // say that a single instance of this plugin can be rendered in multiple
        // threads
        props
            .set_image_effect_plugin_render_thread_safety(
                s_prop,
                ImageEffectPluginRenderThreadSafety::FullySafe.into(),
            )
            .ok_or(StatusError::Failed)?;

        // say that the host should manage SMP threading over a single frame
        props
            .set_image_effect_plugin_host_frame_threading(s_prop, true)
            .ok_or(StatusError::Failed)?;
    }

    Ok(Status::OK)
}

/// Describe the plugin in context.
fn action_describe_in_context(
    descriptor: ImageEffectDescriptor,
    in_args: ActionDescribeInContextIn,
) -> OutboundResult {
    let (data, additional) = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;
    let s_param = &data.parameter_suite.0;

    // get the context we are being described for
    let context = unsafe { in_args.get_image_effect_context(s_prop) }.ok_or(StatusError::Failed)?;
    if context != ImageEffectPropContext::Filter {
        return Err(StatusError::ErrUnsupported);
    }

    // define the mandated single output clip & the mandated single source clip
    for name in [c"Output", c"Source"] {
        let props = unsafe { descriptor.clip_define(s_ifx, name) }.ok_or(StatusError::Failed)?;

        // set the component types we can handle on our output & our main input
        (unsafe {
            use ImageEffectPropSupportedComponents as C;
            props.set_image_effect_supported_components(
                s_prop,
                &[C::RGBA.into(), C::Alpha.into(), C::RGB.into()],
            )
        })
        .ok_or(StatusError::Failed)?;
    }

    // first get the wrapper object to the parameter set
    let param_set = unsafe { descriptor.get_param_set(s_ifx) }.ok_or(StatusError::Failed)?;

    {
        // set the properties on the radius param
        let param_props = unsafe { param_set.param_define_double(s_param, RADIUS_PARAM_NAME) }
            .ok_or(StatusError::Failed)?;

        unsafe {
            param_props
                .set_param_double_type(s_prop, ParamPropDoubleType::X.into())
                .ok_or(StatusError::Failed)?;
            // Not supported by DaVinci Resolve. To make the plugin work there,
            // we ignore the return value here. TODO: Calculate the default value
            // in canonical coordinate if this fails.
            param_props
                .set_param_default_coordinate_system(
                    s_prop,
                    ParamPropDefaultCoordinateSystem::Normalised.into(),
                )
                .ok();
            param_props
                .set_param_default(s_prop, 0.25)
                .ok_or(StatusError::Failed)?;
            param_props
                .set_param_min(s_prop, 0.0)
                .ok_or(StatusError::Failed)?;
            param_props
                .set_param_display_min(s_prop, 0.0)
                .ok_or(StatusError::Failed)?;
            param_props
                .set_param_display_max(s_prop, 2.0)
                .ok_or(StatusError::Failed)?;
            param_props
                .set_label(s_prop, Some(c"Radius"))
                .ok_or(StatusError::Failed)?;
            param_props
                .set_param_hint(s_prop, Some(c"The radius of the circle."))
                .ok_or(StatusError::Failed)?;
        }
    }

    {
        // set the properties on the centre param
        let param_props = unsafe { param_set.param_define_double_2d(s_param, CENTRE_PARAM_NAME) }
            .ok_or(StatusError::Failed)?;

        unsafe {
            param_props
                .set_param_double_type(s_prop, ParamPropDoubleType::XYAbsolute.into())
                .ok_or(StatusError::Failed)?;
            param_props
                .set_param_default_coordinate_system(
                    s_prop,
                    ParamPropDefaultCoordinateSystem::Normalised.into(),
                )
                .ok();
            param_props
                .set_param_default(s_prop, [0.5, 0.5])
                .ok_or(StatusError::Failed)?;
            param_props
                .set_label(s_prop, Some(c"Centre"))
                .ok_or(StatusError::Failed)?;
            param_props
                .set_param_hint(s_prop, Some(c"The centre of the circle."))
                .ok_or(StatusError::Failed)?;
        }
    }

    {
        // set the properties on the colour param
        let param_props = unsafe { param_set.param_define_rgba(s_param, COLOUR_PARAM_NAME) }
            .ok_or(StatusError::Failed)?;

        unsafe {
            param_props
                .set_param_default(s_prop, [1.0, 1.0, 1.0, 0.5])
                .ok_or(StatusError::Failed)?;
            param_props
                .set_label(s_prop, Some(c"Colour"))
                .ok_or(StatusError::Failed)?;
            param_props
                .set_param_hint(s_prop, Some(c"The colour of the circle."))
                .ok_or(StatusError::Failed)?;
        }
    }

    // and define the 'grow RoD' parameter and set its properties
    if additional.host_supports_multi_res {
        let param_props = unsafe { param_set.param_define_boolean(s_param, GROW_ROD_PARAM_NAME) }
            .ok_or(StatusError::Failed)?;

        unsafe {
            param_props
                .set_param_default(s_prop, false)
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

/// Instance construction.
fn action_create_instance(instance: ImageEffectInstance) -> OutboundResult {
    let (data, additional) = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;
    let s_param = &data.parameter_suite.0;

    let instance_props = unsafe { instance.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    // the clip objects wrapping the respective handles
    let source_clip =
        unsafe { instance.clip_get_clip_handle(s_ifx, c"Source") }.ok_or(StatusError::Failed)?;
    let output_clip =
        unsafe { instance.clip_get_clip_handle(s_ifx, c"Output") }.ok_or(StatusError::Failed)?;

    let param_set = unsafe { instance.get_param_set(s_ifx) }.ok_or(StatusError::Failed)?;
    // the param objects wrapping the respective handles
    let radius_param =
        unsafe { param_set.param_get_param_handle_double(s_param, RADIUS_PARAM_NAME) }
            .ok_or(StatusError::Failed)?;
    let centre_param =
        unsafe { param_set.param_get_param_handle_double_2d(s_param, CENTRE_PARAM_NAME) }
            .ok_or(StatusError::Failed)?;
    let colour_param = unsafe { param_set.param_get_param_handle_rgba(s_param, COLOUR_PARAM_NAME) }
        .ok_or(StatusError::Failed)?;
    let grow_rod_param = if additional.host_supports_multi_res {
        Some(
            unsafe { param_set.param_get_param_handle_boolean(s_param, GROW_ROD_PARAM_NAME) }
                .ok_or(StatusError::Failed)?,
        )
    } else {
        None
    };

    // To avoid continual lookup, put our objects into our instance data, those
    // objects are guaranteed to be valid for the duration of the instance.
    let my_data = MyInstanceData {
        source_clip: GuaranteeSendClipInstance(source_clip),
        output_clip: GuaranteeSendClipInstance(output_clip),
        radius_param,
        centre_param,
        colour_param,
        grow_rod_param,
    };
    let my_data_ptr = Box::into_raw(Box::new(my_data)) as *mut c_void;

    // Set my private instance data.
    // SAFETY: the pointee is kept alive by `Box::into_raw` until it is
    // reclaimed with `Box::from_raw` in `action_destroy_instance`.
    if let r =
        unsafe { instance_props.set_instance_data(s_prop, std::ptr::NonNull::new(my_data_ptr)) }
        && !r.is_ok()
    {
        drop(unsafe { Box::from_raw(my_data_ptr as *mut MyInstanceData) });
        return Err(StatusError::Failed);
    }

    Ok(Status::OK)
}

/// Instance destruction.
fn action_destroy_instance(instance: ImageEffectInstance) -> OutboundResult {
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

/// Tells the host what region we are capable of filling.
fn action_get_region_of_definition(
    effect: ImageEffectInstance,
    in_args: ActionGetRegionOfDefinitionIn,
    out_args: ActionGetRegionOfDefinitionOut,
) -> OutboundResult {
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
        growing_rod = unsafe { grow_rod_param.param_get_value_at_time(s_param, time) }
            .ok_or(StatusError::Failed)?;
    };

    // are we growing the RoD to include the circle?
    if !growing_rod {
        return Ok(StatusReply::ReplyDefault.into());
    }

    let radius = unsafe { my_data.radius_param.param_get_value_at_time(s_param, time) }
        .ok_or(StatusError::Failed)?;
    let [centre_x, centre_y] =
        unsafe { my_data.centre_param.param_get_value_at_time(s_param, time) }
            .ok_or(StatusError::Failed)?;

    // get the source rod
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

    // set the rod in the out args
    unsafe { out_args.set_image_effect_region_of_definition(s_prop, rect_d_to_array(&rod)) }
        .ok_or(StatusError::Failed)?;

    // and say we trapped the action
    Ok(Status::OK)
}

/// Are the settings of the effect making it redundant and so not do anything to
/// the image data?
fn action_is_identity(
    effect: ImageEffectInstance,
    in_args: ActionIsIdentityIn,
    out_args: ActionIsIdentityOut,
) -> OutboundResult {
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

    let radius = unsafe { my_data.radius_param.param_get_value_at_time(s_param, time) }
        .ok_or(StatusError::Failed)?;

    let is_identity = if radius < 0.0001 {
        // if the radius is zero then we don't draw anything and it has no
        // effect
        true
    } else {
        // if we are drawing out side of the RoD and we aren't growing to
        // include it, we have no effect

        let mut growing_rod = false;
        if let Some(grow_rod_param) = my_data.grow_rod_param {
            growing_rod = unsafe { grow_rod_param.param_get_value_at_time(s_param, time) }
                .ok_or(StatusError::Failed)?;
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

            let [centre_x, centre_y] =
                unsafe { my_data.centre_param.param_get_value_at_time(s_param, time) }
                    .ok_or(StatusError::Failed)?;

            centre_x + radius < bounds.x1
                || centre_x - radius > bounds.x2
                || centre_y + radius < bounds.y1
                || centre_y - radius > bounds.y2
        }
    };

    if is_identity {
        // we set the name of the input clip to pull default images from
        (unsafe { s_prop.prop_set_string(out_args.sys_handle(), kOfxPropLabel, Some(c"Source")) })
            .ok_or(StatusError::Failed)?;
        // and say we trapped the action and we are at the identity
        Ok(Status::OK)
    } else {
        // say we aren't at the identity
        Ok(StatusReply::ReplyDefault.into())
    }
}

/// Render an output image.
fn action_render(instance: ImageEffectInstance, in_args: ActionRenderIn) -> OutboundResult {
    let (data, _additional) = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;
    let s_param = &data.parameter_suite.0;

    let instance_props = unsafe { instance.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    // get the render window, render scale and the time from the `in_args`
    let time = unsafe { in_args.get_time(s_prop) }.ok_or(StatusError::Failed)?;
    let render_window =
        unsafe { in_args.get_image_effect_render_window(s_prop) }.ok_or(StatusError::Failed)?;
    let render_window = rect_i_from_array(&render_window);
    let render_scale =
        unsafe { in_args.get_image_effect_render_scale(s_prop) }.ok_or(StatusError::Failed)?;

    // get our instance data which has out clip and param objects
    let Some(my_data_ptr) =
        unsafe { instance_props.get_instance_data(s_prop) }.ok_or(StatusError::Failed)?
    else {
        return Err(StatusError::Failed);
    };
    let my_data = unsafe { &*(my_data_ptr.as_ptr() as *const MyInstanceData) };

    // get our param values
    let radius = unsafe { my_data.radius_param.param_get_value_at_time(s_param, time) }
        .ok_or(StatusError::Failed)?;
    let centre = unsafe { my_data.centre_param.param_get_value_at_time(s_param, time) }
        .ok_or(StatusError::Failed)?;
    let colour = unsafe { my_data.colour_param.param_get_value_at_time(s_param, time) }
        .ok_or(StatusError::Failed)?;

    // fetch the image to write into, from the output clip
    let output_img =
        unsafe { ClipImageManaged::try_new_from_clip(&data, &my_data.output_clip.0, time) }?;

    // fetch the image to read from, from the source clip
    let source_img =
        unsafe { ClipImageManaged::try_new_from_clip(&data, &my_data.source_clip.0, time) }?;

    // now do our render depending on the data type
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
            &data,
            instance,
            source_img,
            output_img,
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
            source_img,
            output_img,
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
            source_img,
            output_img,
            render_window,
        )?,
    };

    // all was well
    Ok(Status::OK)
}

/// Iterate over our pixels and process them.
#[allow(clippy::too_many_arguments)]
pub fn pixel_processing<T>(
    from_f64: fn(f64) -> T,
    into_f64: fn(T) -> f64,
    clamp: fn(T, T, T) -> T,
    max: T,
    centre: [f64; 2],
    radius: f64,
    colour: [f64; 4],
    render_scale: [f64; 2],
    data: &SharedData,
    instance: ImageEffectInstance,
    source_img: ClipImageManaged,
    output_img: ClipImageManaged,
    render_window: OfxRectI,
) -> Result<(), StatusError>
where
    T: Copy + Default + std::ops::Add<Output = T>,
{
    // pixel aspect of our output
    let par = output_img.pixel_aspect_ratio();

    let colour_quantised = if into_f64(max) == 1.0 {
        // don't clamp floating point values
        colour.map(from_f64)
    } else {
        colour.map(|c| clamp(from_f64(c * into_f64(max)), T::default(), max))
    };

    // now do some processing
    for y in render_window.y1..render_window.y2 {
        if y % 20 == 0
            && unsafe {
                data.image_effect_suite
                    .sys_ref()
                    .abort
                    .is_some_and(|abort| abort(instance.sys_handle()) != 0)
            }
        {
            return Ok(());
        }

        // get our y coord in canonical space
        let y_canonical = (y as f64 + 0.5) / render_scale[1];
        // how far are we from the centre in y, canonical
        let dy = y_canonical - centre[1];

        // get the row start for the output image
        let Some(dst_pix) = output_img.raw_address(render_window.x1, y) else {
            return Err(StatusError::Failed);
        };
        let mut dst_pix = dst_pix as *mut T;

        for x in render_window.x1..render_window.x2 {
            // get our x pixel coord in canonical space,
            let x_canonical = (x as f64 + 0.5) * par / render_scale[0];
            // how far are we from the centre in x, canonical
            let dx = x_canonical - centre[0];
            // distance to the centre of our circle, canonical
            let d = (dx * dx + dy * dy).sqrt();

            // this will hold the antialiased value
            let mut alpha = colour[3];

            // Is the square of the distance to the centre less than the square
            // of the radius?
            if d < radius {
                if d > radius - 1.0 {
                    // we are within 1 pixel of the edge, modulate our alpha
                    // with an anti-aliasing value
                    alpha *= radius - d;
                }
            } else {
                // outside, so alpha is 0
                alpha = 0.0;
            }

            // get the source pixel
            let src_pix = source_img.raw_address(x, y).map(|ptr| ptr as *const T);

            fn comp<T>(pix: Option<*const T>, i: usize) -> T
            where
                T: Copy + Default,
            {
                if let Some(pix) = pix {
                    unsafe { *pix.add(i) }
                } else {
                    T::default()
                }
            }

            // scale each component around that average
            for i in 0..output_img.n_comps() {
                // blend the source pixel with the circle colour based on the
                // intensity of the calculated alpha
                let blended = blend(
                    into_f64(comp(src_pix, i as usize)),
                    into_f64(colour_quantised[i as usize]),
                    alpha,
                );
                unsafe {
                    *dst_pix.add(i as usize) = from_f64(blended);
                }
            }
            unsafe { dst_pix = dst_pix.offset(output_img.n_comps() as isize) };
        }
    }

    Ok(())
}

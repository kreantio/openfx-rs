//! This example is a loose port of: https://github.com/AcademySoftwareFoundation/openfx/blob/b2d2eea3a64e511bfabb80e56f902d96adf4a05e/Documentation/sources/Guide/Code/Example3/gain.cpp
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

mod processing;

use std::{
    ffi::{CStr, c_void},
    sync::Mutex,
};

use openfx::{
    low::{
        OutboundResult, Status, StatusError, StatusReply,
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
        parameters::{ParamBoolean, ParamDouble},
    },
    sys::core::kOfxPropLabel,
};

use processing::pixel_processing;

use crate::{
    definitions::{PLUGIN_3_GAIN_IDENTIFIER, PLUGIN_3_GAIN_LABEL, PLUGINS_GROUPING},
    helpers::{
        internal_utils::{BitDepth, ClipImageManaged, rect_i_from_array, set_labels},
        shared_data::{GuaranteeSend, GuaranteeSendClipInstance, SharedData},
    },
};

static HOST_BEFORE_ACTION_LOAD: Mutex<Option<GuaranteeSend<Host>>> = Mutex::new(None);
static SHARED_DATA: Mutex<Option<SharedData>> = Mutex::new(None);

struct MyInstanceData {
    // wrapper objects to the clips we deal with
    source_clip: GuaranteeSendClipInstance,
    output_clip: GuaranteeSendClipInstance,

    // wrapper objects to our parameters
    gain_param: ParamDouble,
    apply_to_alpha_param: ParamBoolean,
}

fn shared_data_lockless() -> Result<SharedData, StatusError> {
    let data = SHARED_DATA.lock().map_err(|_| StatusError::Failed)?;
    let data = data.as_ref().ok_or(StatusError::Failed)?;
    Ok(data.clone())
}

const GAIN_PARAM_NAME: &CStr = c"gain";
const APPLY_TO_ALPHA_PARAM_NAME: &CStr = c"applyToAlpha";

pub struct PluginExampleGain;
impl Plugin for PluginExampleGain {
    // The unique ID of this plugin.
    const PLUGIN_IDENTIFIER: &'static CStr = PLUGIN_3_GAIN_IDENTIFIER;
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
            // explicitly drop `SHARED_DATA`.
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

    let mut data = SHARED_DATA.lock().map_err(|_| StatusError::Failed)?;
    if data.is_some() {
        Err(StatusError::Failed)
    } else {
        *data = Some(SharedData::try_new(host)?);
        Ok(Status::OK)
    }
}

/// The last action called before a plugin binary is unloaded.
fn action_unload() -> OutboundResult {
    let mut data = SHARED_DATA.lock().map_err(|_| StatusError::Failed)?;
    if data.take().is_none() {
        Err(StatusError::Failed)
    } else {
        Ok(Status::OK)
    }
}

/// The plugin's basic description routine.
fn action_describe(descriptor: ImageEffectDescriptor) -> OutboundResult {
    let data = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;

    // get the property set handle for the plugin
    let props = unsafe { descriptor.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    unsafe {
        // set some labels and the group it belongs to
        // NOTE: The original example only sets `kOfxPropLabel`. Instead, we set
        // all three labels at once so that the plugin's label displays
        // correctly in DaVinci Resolve's Cut tab.
        set_labels(&data.property_suite.0, &props, PLUGIN_3_GAIN_LABEL)?;
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
    let data = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;
    let s_param = &data.parameter_suite.0;

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

    // first get the handle to the parameter set
    let param_set = unsafe { descriptor.get_param_set(s_ifx) }.ok_or(StatusError::Failed)?;

    {
        // properties on our parameter
        let param_props = unsafe { param_set.param_define_double(s_param, GAIN_PARAM_NAME) }
            .ok_or(StatusError::Failed)?;

        // now define a 'gain' parameter and set its properties
        unsafe {
            param_props
                .set_param_double_type(s_prop, ParamPropDoubleType::Scale.into())
                .ok_or(StatusError::Failed)?;
            param_props
                .set_param_default(s_prop, 1.0)
                .ok_or(StatusError::Failed)?;
            param_props
                .set_param_min(s_prop, 0.0)
                .ok_or(StatusError::Failed)?;
            param_props
                .set_param_display_min(s_prop, 0.0)
                .ok_or(StatusError::Failed)?;
            param_props
                .set_param_display_max(s_prop, 10.0)
                .ok_or(StatusError::Failed)?;
            param_props
                .set_label(s_prop, Some(c"Gain"))
                .ok_or(StatusError::Failed)?;
            param_props
                .set_param_hint(s_prop, Some(c"How much to multiply the image by."))
                .ok_or(StatusError::Failed)?;
        }
    }

    {
        let param_props =
            unsafe { param_set.param_define_boolean(s_param, APPLY_TO_ALPHA_PARAM_NAME) }
                .ok_or(StatusError::Failed)?;

        // and define the 'applyToAlpha' parameters and set its properties
        unsafe {
            param_props
                .set_param_default(s_prop, false)
                .ok_or(StatusError::Failed)?;
            param_props
                .set_label(s_prop, Some(c"Apply To Alpha"))
                .ok_or(StatusError::Failed)?;
            param_props
                .set_param_hint(
                    s_prop,
                    Some(c"Whether to apply the gain value to alpha as well."),
                )
                .ok_or(StatusError::Failed)?;
        }
    }

    Ok(Status::OK)
}

/// Instance construction.
fn action_create_instance(instance: ImageEffectInstance) -> OutboundResult {
    let data = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;
    let s_param = &data.parameter_suite.0;

    let instance_props = unsafe { instance.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    // the source and output clip objects wrapping the respective handles
    let source_clip =
        unsafe { instance.clip_get_clip_handle(s_ifx, c"Source") }.ok_or(StatusError::Failed)?;
    let output_clip =
        unsafe { instance.clip_get_clip_handle(s_ifx, c"Output") }.ok_or(StatusError::Failed)?;

    let param_set = unsafe { instance.get_param_set(s_ifx) }.ok_or(StatusError::Failed)?;
    //the param objects wrapping the respective handles
    let gain_param = unsafe { param_set.param_get_param_handle_double(s_param, GAIN_PARAM_NAME) }
        .ok_or(StatusError::Failed)?;
    let apply_to_alpha_param =
        unsafe { param_set.param_get_param_handle_boolean(s_param, APPLY_TO_ALPHA_PARAM_NAME) }
            .ok_or(StatusError::Failed)?;

    // To avoid continual lookup, put our objects into our instance data, those
    // objects are guaranteed to be valid for the duration of the instance.
    let my_data = MyInstanceData {
        source_clip: GuaranteeSendClipInstance(source_clip),
        output_clip: GuaranteeSendClipInstance(output_clip),
        gain_param,
        apply_to_alpha_param,
    };
    let my_data_ptr = Box::into_raw(Box::new(my_data)) as *mut c_void;

    // Set my private instance data
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
    let data = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;

    let props = unsafe { instance.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    let Some(my_data_ptr) =
        (unsafe { props.get_instance_data(s_prop) }).ok_or(StatusError::Failed)?
    else {
        return Err(StatusError::Failed);
    };

    drop(unsafe { Box::from_raw(my_data_ptr.as_ptr() as *mut MyInstanceData) });

    Ok(Status::OK)
}

fn action_is_identity(
    effect: ImageEffectInstance,
    in_args: ActionIsIdentityIn,
    out_args: ActionIsIdentityOut,
) -> OutboundResult {
    let data = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;
    let s_param = &data.parameter_suite.0;

    let instance_props = unsafe { effect.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    let Some(my_data_ptr) =
        (unsafe { instance_props.get_instance_data(s_prop) }).ok_or(StatusError::Failed)?
    else {
        return Err(StatusError::Failed);
    };
    let my_data = unsafe { &*(my_data_ptr.as_ptr() as *const MyInstanceData) };

    let time = unsafe { in_args.get_time(s_prop) }.ok_or(StatusError::Failed)?;
    let gain = unsafe { my_data.gain_param.param_get_value_at_time(s_param, time) }
        .ok_or(StatusError::Failed)?;

    if (gain - 1.0).abs() < 0.000000001 {
        (unsafe { s_prop.prop_set_string(out_args.sys_handle(), kOfxPropLabel, Some(c"Source")) })
            .ok_or(StatusError::Failed)?;
        Ok(Status::OK)
    } else {
        Ok(StatusReply::ReplyDefault.into())
    }
}

/// Render an output image.
fn action_render(instance: ImageEffectInstance, in_args: ActionRenderIn) -> OutboundResult {
    let data = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;
    let s_param = &data.parameter_suite.0;

    let instance_props = unsafe { instance.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    // get the render window and the time from the `in_args`
    let time = unsafe { in_args.get_time(s_prop) }.ok_or(StatusError::Failed)?;
    let render_window =
        unsafe { in_args.get_image_effect_render_window(s_prop) }.ok_or(StatusError::Failed)?;
    let render_window = rect_i_from_array(&render_window);

    // get our instance data which has out clip and param objects
    let Some(my_data_ptr) =
        (unsafe { instance_props.get_instance_data(s_prop) }).ok_or(StatusError::Failed)?
    else {
        return Err(StatusError::Failed);
    };
    let my_data = unsafe { &*(my_data_ptr.as_ptr() as *const MyInstanceData) };

    // get our param values
    let gain = unsafe { my_data.gain_param.param_get_value_at_time(s_param, time) }
        .ok_or(StatusError::Failed)?;
    let apply_to_alpha = unsafe {
        my_data
            .apply_to_alpha_param
            .param_get_value_at_time(s_param, time)
    }
    .ok_or(StatusError::Failed)?;

    // the property sets holding our images
    let output_img = unsafe { my_data.output_clip.0.clip_get_image(s_ifx, time, None) }
        .ok_or(StatusError::Failed)?;
    let source_img = unsafe { my_data.source_clip.0.clip_get_image(s_ifx, time, None) }
        .ok_or(StatusError::Failed)?;

    let output_img_m = unsafe { ClipImageManaged::try_new(&data, output_img) }?;
    let source_img_m = unsafe { ClipImageManaged::try_new(&data, source_img) }?;

    // now do our render depending on the data type
    match output_img_m.pixel_depth() {
        BitDepth::Byte => pixel_processing(
            |f| f as u8,
            |v| v as f64,
            255u8,
            gain,
            apply_to_alpha,
            &data,
            instance,
            source_img_m,
            output_img_m,
            render_window,
        )?,
        BitDepth::Short => pixel_processing(
            |f| f as u16,
            |v| v as f64,
            65535u16,
            gain,
            apply_to_alpha,
            &data,
            instance,
            source_img_m,
            output_img_m,
            render_window,
        )?,
        BitDepth::Float => pixel_processing(
            |f| f as f32,
            |v| v as f64,
            1.0f32,
            gain,
            apply_to_alpha,
            &data,
            instance,
            source_img_m,
            output_img_m,
            render_window,
        )?,
    }

    // all was well
    Ok(Status::OK)
}

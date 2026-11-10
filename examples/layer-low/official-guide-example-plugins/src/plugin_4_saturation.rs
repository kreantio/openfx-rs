//! This example is a loose port of: https://github.com/AcademySoftwareFoundation/openfx/blob/b2d2eea3a64e511bfabb80e56f902d96adf4a05e/Documentation/sources/Guide/Code/Example4/saturation.cpp
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

use std::{
    ffi::{CStr, c_void},
    sync::Mutex,
};

use openfx::{
    low::{
        InboundResult, OutboundResult, Status, StatusError, StatusReply,
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
        parameters::ParamDouble,
    },
    sys::core::{OfxRectI, kOfxPropLabel},
};

use crate::{
    definitions::{PLUGIN_4_SATURATION_IDENTIFIER, PLUGIN_4_SATURATION_LABEL, PLUGINS_GROUPING},
    helpers::{
        internal_utils::{BitDepth, ClipImageManaged, rect_i_from_array, set_labels},
        shared_data::{GuaranteeSend, GuaranteeSendClipInstance, SharedData},
    },
};

static HOST_BEFORE_ACTION_LOAD: Mutex<Option<GuaranteeSend<Host>>> = Mutex::new(None);
static SHARED_DATA: Mutex<Option<SharedData>> = Mutex::new(None);

/// Our instance data, where we are caching away clip and param objects.
struct MyInstanceData {
    /// Are we in the general context?
    #[expect(unused)]
    is_general_context: bool,

    source_clip: GuaranteeSendClipInstance,
    output_clip: GuaranteeSendClipInstance,
    mask_clip: Option<GuaranteeSendClipInstance>,

    saturation_param: ParamDouble,
}

fn shared_data_lockless() -> Result<SharedData, StatusError> {
    let data = SHARED_DATA.lock().map_err(|_| StatusError::Failed)?;
    let data = data.as_ref().ok_or(StatusError::Failed)?;
    Ok(data.clone())
}

// name of our param
const SATURATION_PARAM_NAME: &CStr = c"saturation";

pub struct PluginExampleSaturation;
impl Plugin for PluginExampleSaturation {
    // The unique ID of this plugin.
    const PLUGIN_IDENTIFIER: &'static CStr = PLUGIN_4_SATURATION_IDENTIFIER;
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
        // set some labels and the group it belongs to.
        // NOTE: The original example only sets `kOfxPropLabel`. Instead, we set
        // all three labels at once so that the plugin's label displays
        // correctly in DaVinci Resolve's Cut tab.
        set_labels(&data.property_suite.0, &props, PLUGIN_4_SATURATION_LABEL)?;
        props
            .set_image_effect_plugin_grouping(s_prop, Some(PLUGINS_GROUPING))
            .ok_or(StatusError::Failed)?;

        // define the image effects contexts we can be used in, in this case a
        // filter and a general effect
        {
            use ImageEffectPropSupportedContexts as C;
            props
                .set_image_effect_supported_contexts(s_prop, &[C::Filter.into(), C::General.into()])
                .ok_or(StatusError::Failed)?;
        }

        // set the bit depths the plugin can handle
        {
            use ImageEffectPropSupportedPixelDepths as D;
            props
                .set_image_effect_supported_pixel_depths(
                    s_prop,
                    &[D::Byte.into(), D::Short.into(), D::Float.into()],
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

    // get the context we are being described for
    let context = unsafe { in_args.get_image_effect_context(s_prop) }.ok_or(StatusError::Failed)?;
    if context != ImageEffectPropContext::Filter && context != ImageEffectPropContext::General {
        return Err(StatusError::ErrUnsupported);
    }

    // define the mandated single output clip & the mandated single source clip
    for name in [c"Output", c"Source"] {
        let props = unsafe { descriptor.clip_define(s_ifx, name) }.ok_or(StatusError::Failed)?;

        (unsafe {
            props.set_image_effect_supported_components(
                s_prop,
                &[
                    ImageEffectPropSupportedComponents::RGBA.into(),
                    ImageEffectPropSupportedComponents::RGB.into(),
                ],
            )
        })
        .ok_or(StatusError::Failed)?;
    }

    if context == ImageEffectPropContext::General {
        let props = unsafe { descriptor.clip_define(s_ifx, c"Mask") }.ok_or(StatusError::Failed)?;

        unsafe {
            // set the component types we can handle on our mask input
            props
                .set_image_effect_supported_components(
                    s_prop,
                    &[ImageEffectPropSupportedComponents::Alpha.into()],
                )
                .ok_or(StatusError::Failed)?;
            props
                .set_image_clip_optional(s_prop, true)
                .ok_or(StatusError::Failed)?;
            props
                .set_image_clip_is_mask(s_prop, true)
                .ok_or(StatusError::Failed)?;
        }
    }

    // first get the wrapper object to the parameter set
    let param_set = unsafe { descriptor.get_param_set(s_ifx) }.ok_or(StatusError::Failed)?;

    {
        // now define a 'saturation' parameter and set its properties
        let param_props = unsafe { param_set.param_define_double(s_param, SATURATION_PARAM_NAME) }
            .ok_or(StatusError::Failed)?;

        unsafe {
            param_props
                .set_param_double_type(s_prop, ParamPropDoubleType::Scale.into())
                .ok_or(StatusError::Failed)?;
            param_props
                .set_param_default(s_prop, 1.0)
                .ok_or(StatusError::Failed)?;
            param_props
                .set_param_display_min(s_prop, -2.0)
                .ok_or(StatusError::Failed)?;
            param_props
                .set_param_display_max(s_prop, 2.0)
                .ok_or(StatusError::Failed)?;
            param_props
                .set_label(s_prop, Some(c"Saturation"))
                .ok_or(StatusError::Failed)?;
            param_props
                .set_param_hint(s_prop, Some(c"How saturated the image should be."))
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

    // is this instance made for the general context?
    let context =
        unsafe { instance_props.get_image_effect_context(s_prop) }.ok_or(StatusError::Failed)?;
    let is_general_context = context == ImageEffectPropContext::General;

    // the clip objects wrapping the respective handles
    let source_clip =
        unsafe { instance.clip_get_clip_handle(s_ifx, c"Source") }.ok_or(StatusError::Failed)?;
    let output_clip =
        unsafe { instance.clip_get_clip_handle(s_ifx, c"Output") }.ok_or(StatusError::Failed)?;
    let mask_clip = if is_general_context {
        Some(unsafe { instance.clip_get_clip_handle(s_ifx, c"Mask") }.ok_or(StatusError::Failed)?)
    } else {
        None
    };

    let param_set = unsafe { instance.get_param_set(s_ifx) }.ok_or(StatusError::Failed)?;
    // the param object(s) wrapping the respective handles
    let saturation_param =
        unsafe { param_set.param_get_param_handle_double(s_param, SATURATION_PARAM_NAME) }
            .ok_or(StatusError::Failed)?;

    // To avoid continual lookup, put our objects into our instance data, those
    // objects are guaranteed to be valid for the duration of the instance.
    let my_data = MyInstanceData {
        is_general_context,
        source_clip: GuaranteeSendClipInstance(source_clip),
        output_clip: GuaranteeSendClipInstance(output_clip),
        mask_clip: mask_clip.map(GuaranteeSendClipInstance),
        saturation_param,
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
    let saturation = unsafe {
        my_data
            .saturation_param
            .param_get_value_at_time(s_param, time)
    }
    .ok_or(StatusError::Failed)?;

    if (saturation - 1.0).abs() < 0.000000001 {
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

    // get our param value(s)
    let saturation = unsafe {
        my_data
            .saturation_param
            .param_get_value_at_time(s_param, time)
    }
    .ok_or(StatusError::Failed)?;

    // fetch the image to write into, from the output clip
    let output_img =
        unsafe { ClipImageManaged::try_new_from_clip(&data, &my_data.output_clip.0, time) }?;

    // fetch the image to read from, from the source clip
    let source_img =
        unsafe { ClipImageManaged::try_new_from_clip(&data, &my_data.source_clip.0, time) }?;

    // fetch the mask image at render time from the mask clip, it may not be
    // there as we might in the filter context or it might not be attached as it
    // is optional, so don't worry if we don't have one.
    let mask_img = if let Some(mask_clip) = my_data.mask_clip {
        match unsafe { mask_clip.0.clip_get_image(s_ifx, time, None) } {
            InboundResult::Ok(mask_img) => {
                Some(unsafe { ClipImageManaged::try_new_from_property_set(&data, mask_img) }?)
            }
            InboundResult::KnownError(StatusError::Failed) => None, // unconnected
            _ => return Err(StatusError::Failed),
        }
    } else {
        None
    };

    // now do our render depending on the data type
    match output_img.pixel_depth() {
        BitDepth::Byte => pixel_processing(
            |f| f as u8,
            |v| v as f64,
            255u8,
            saturation,
            &data,
            instance,
            source_img,
            mask_img,
            output_img,
            render_window,
        )?,
        BitDepth::Short => pixel_processing(
            |f| f as u16,
            |v| v as f64,
            65535u16,
            saturation,
            &data,
            instance,
            source_img,
            mask_img,
            output_img,
            render_window,
        )?,
        BitDepth::Float => pixel_processing(
            |f| f as f32,
            |v| v as f64,
            1.0f32,
            saturation,
            &data,
            instance,
            source_img,
            mask_img,
            output_img,
            render_window,
        )?,
    }

    // all was well
    Ok(Status::OK)
}

/// Iterate over our pixels and process them.
#[allow(clippy::too_many_arguments)]
pub fn pixel_processing<T>(
    from_f64: fn(f64) -> T,
    into_f64: fn(T) -> f64,
    max: T,
    saturation: f64,
    data: &SharedData,
    instance: ImageEffectInstance,
    source_img: ClipImageManaged,
    mask_img: Option<ClipImageManaged>,
    output_img: ClipImageManaged,
    render_window: OfxRectI,
) -> Result<(), StatusError>
where
    T: Copy + Default + std::ops::Add<Output = T>,
{
    /// Blend v2 into v1 by the given blend factor.
    fn blend(v1: f64, v2: f64, blend: f64) -> f64 {
        v1 + (v2 - v1) * blend
    }

    // do some processing.
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

        // get the row start for the output image
        let Some(dst_pix) = output_img.raw_address(render_window.x1, y) else {
            return Err(StatusError::Failed);
        };
        let mut dst_pix = dst_pix as *mut T;

        for x in render_window.x1..render_window.x2 {
            // get the source pixel
            let src_pix = source_img.raw_address(x, y).map(|ptr| ptr as *mut T);

            // get the amount to mask by, no mask image means we do the full
            // effect everywhere
            let mask_amount = if let Some(mask_img) = &mask_img {
                // get our mask pixel address
                let mask_pix = mask_img.raw_address(x, y).map(|ptr| ptr as *mut T);
                if let Some(mask_pix) = mask_pix {
                    (unsafe { (into_f64)(*mask_pix) }) / (into_f64)(max)
                } else {
                    0.0
                }
            } else {
                1.0
            };

            if let Some(mut src_pix) = src_pix {
                if mask_amount == 0.0 {
                    // we have a mask input, but the mask is zero here, so no
                    // effect happens, copy source to output
                    for _ in 0..output_img.n_comps() {
                        unsafe {
                            *dst_pix = *src_pix;
                            dst_pix = dst_pix.offset(1);
                            src_pix = src_pix.offset(1);
                        }
                    }
                } else {
                    // we have a non zero mask or no mask at all

                    let rgb = [
                        into_f64(unsafe { *src_pix }),
                        into_f64(unsafe { *src_pix.offset(1) }),
                        into_f64(unsafe { *src_pix.offset(2) }),
                    ];

                    // find the average of the R, G and B
                    let average = (rgb[0] + rgb[1] + rgb[2]) / 3.0;

                    // scale each component around that average
                    for (i, c) in rgb.iter().enumerate() {
                        let value = (*c - average) * saturation + average;
                        let value = value.clamp(0.0, into_f64(max));
                        // use the mask to control how much original we should
                        // have
                        let blended = blend(rgb[i], value, mask_amount);
                        unsafe { *dst_pix.add(i) = from_f64(blended) };
                    }

                    // if we have an alpha, just copy it
                    if output_img.n_comps() == 4 {
                        unsafe {
                            *dst_pix.add(3) = *src_pix.add(3);
                        }
                    }

                    unsafe { dst_pix = dst_pix.offset(output_img.n_comps() as isize) };
                }
            } else {
                // we don't have a pixel in the source image, set output to zero
                for _ in 0..output_img.n_comps() {
                    unsafe {
                        *dst_pix = T::default();
                        dst_pix = dst_pix.offset(1);
                    }
                }
            }
        }
    }

    Ok(())
}

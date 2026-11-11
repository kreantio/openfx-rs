//! This example is a loose port of: https://github.com/AcademySoftwareFoundation/openfx/blob/b2d2eea3a64e511bfabb80e56f902d96adf4a05e/Documentation/sources/Guide/Code/Example2/invert.cpp
//!
//! Copyright notice of the original code:
//! ```text
//! Copyright OpenFX and contributors to the OpenFX project.
//! SPDX-License-Identifier: BSD-3-Clause
//! ```
//!
//! Author of the original code: Bruno Nicoletti (2014)
//!
//! This plugin will take you through the basics of grabbing and processing
//! images with an OFX plugin by grabbing and input image and inverting it.

use std::{ffi::CStr, sync::Mutex};

use openfx::{
    low::{
        OutboundResult, Status, StatusError, StatusReply,
        enums::{
            ImageEffectPluginRenderThreadSafety, ImageEffectPropContext,
            ImageEffectPropSupportedComponents, ImageEffectPropSupportedContexts,
            ImageEffectPropSupportedPixelDepths,
        },
    },
    low_plugin::{
        Host, Plugin,
        actions::image_effect::{ActionDescribeInContextIn, ActionRenderIn, ImageEffectAction},
        objects::{ImageEffectDescriptor, ImageEffectInstance},
    },
    sys::core::OfxRectI,
};

use crate::{
    definitions::{PLUGIN_2_INVERT_IDENTIFIER, PLUGIN_2_INVERT_LABEL, PLUGINS_GROUPING},
    helpers::{
        internal_utils::{
            BitDepth, ClipImageManaged, pixel_address, rect_i_from_array, set_labels,
        },
        shared_data::{GuaranteeSend, SharedData},
    },
};

static HOST_BEFORE_ACTION_LOAD: Mutex<Option<GuaranteeSend<Host>>> = Mutex::new(None);
static SHARED_DATA: Mutex<Option<SharedData>> = Mutex::new(None);

fn shared_data_lockless() -> Result<SharedData, StatusError> {
    let data = SHARED_DATA.lock().map_err(|_| StatusError::Failed)?;
    let data = data.as_ref().ok_or(StatusError::Failed)?;
    Ok(data.clone())
}

/// The high-level representation of the [`openfx::sys::core::OfxPlugin`] struct
/// that is passed back to the host application to initiate bootstrapping of
/// plugin communications.
pub struct PluginExampleInvert;
impl Plugin for PluginExampleInvert {
    // The unique ID of this plugin.
    const PLUGIN_IDENTIFIER: &'static CStr = PLUGIN_2_INVERT_IDENTIFIER;
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
            // The action called to render a frame.
            ImageEffectAction::Render {
                handle, in_args, ..
            } => action_render(handle, in_args),
            // NOTE: Handling the following two actions is not part of the
            // original example. We need this because, otherwise, if
            // `StatusReply::ReplyDefault` were returned for these actions,
            // DaVinci Resolve would ignore the plugin.
            ImageEffectAction::CreateInstance { .. }
            | ImageEffectAction::DestroyInstance { .. } => Ok(Status::OK),
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
        // Inside `SharedData::try_new`, suites are fetched out of the host via
        // it's fetch suite function.
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

    // get the property set object for the plugin
    let props = unsafe { descriptor.get_property_set(&data.image_effect_suite.0) }
        .ok_or(StatusError::Failed)?;

    unsafe {
        // set some labels and the group it belongs to.
        // NOTE: The original example only sets `kOfxPropLabel`. Instead, we set
        // all three labels at once so that the plugin's label displays
        // correctly in DaVinci Resolve's Cut tab.
        set_labels(&data.property_suite.0, &props, PLUGIN_2_INVERT_LABEL)?;
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

    Ok(Status::OK)
}

/// Render an output image.
fn action_render(instance: ImageEffectInstance, in_args: ActionRenderIn) -> OutboundResult {
    let data = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;

    // get the render window and the time from the `in_args`
    let time = unsafe { in_args.get_time(s_prop) }.ok_or(StatusError::Failed)?;
    let render_window =
        unsafe { in_args.get_image_effect_render_window(s_prop) }.ok_or(StatusError::Failed)?;
    let render_window = rect_i_from_array(&render_window);

    // fetch output clip
    let output_clip =
        unsafe { instance.clip_get_clip_handle(s_ifx, c"Output") }.ok_or(StatusError::Failed)?;
    // fetch main input clip
    let source_clip =
        unsafe { instance.clip_get_clip_handle(s_ifx, c"Source") }.ok_or(StatusError::Failed)?;

    // the property sets holding our images
    let output_img = unsafe { ClipImageManaged::try_new_from_clip(&data, &output_clip, time) }?;
    let source_img = unsafe { ClipImageManaged::try_new_from_clip(&data, &source_clip, time) }?;

    // now do our render depending on the data type
    match output_img.pixel_depth() {
        BitDepth::Byte => pixel_processing(
            255u8,
            &data,
            instance,
            source_img,
            output_img,
            render_window,
        )?,
        BitDepth::Short => pixel_processing(
            65535u16,
            &data,
            instance,
            source_img,
            output_img,
            render_window,
        )?,
        BitDepth::Float => pixel_processing(
            1.0f32,
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
fn pixel_processing<T>(
    max: T,
    data: &SharedData,
    instance: ImageEffectInstance,
    source_img: ClipImageManaged,
    output_img: ClipImageManaged,
    render_window: OfxRectI,
) -> Result<(), StatusError>
where
    T: std::ops::Sub<Output = T> + Copy + Default,
{
    let n_comps = output_img.n_comps();

    let dst_row_bytes = output_img.row_bytes();
    let dst_bounds = output_img.bounds();
    let dst_ptr = output_img.data_ptr();
    let dst_ptr = dst_ptr.as_ptr() as *mut T;

    let src_row_bytes = source_img.row_bytes();
    let src_bounds = source_img.bounds();
    let src_ptr = source_img.data_ptr();
    let src_ptr = src_ptr.as_ptr() as *mut T;

    // do some inverting
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
        let Some(dst_pix) = pixel_address(
            render_window.x1,
            y,
            dst_ptr,
            dst_bounds,
            dst_row_bytes,
            n_comps,
        ) else {
            return Err(StatusError::Failed);
        };
        let mut dst_pix = dst_pix;

        for x in render_window.x1..render_window.x2 {
            // get the source pixel
            let src_pix = pixel_address(x, y, src_ptr, src_bounds, src_row_bytes, n_comps);

            if let Some(src_pix) = src_pix {
                // we have one, iterate each component in the pixels
                let mut src_pix = src_pix;
                for i in 0..n_comps {
                    unsafe {
                        *dst_pix = if i != 3 {
                            max - *src_pix // invert
                        } else {
                            *src_pix // We don't invert alpha.
                        };
                        dst_pix = dst_pix.offset(1);
                        src_pix = src_pix.offset(1);
                    }
                }
            } else {
                // we don't have a pixel in the source image, set output to
                // black
                for _ in 0..n_comps {
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

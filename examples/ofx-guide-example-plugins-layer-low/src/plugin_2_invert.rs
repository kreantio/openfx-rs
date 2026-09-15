use std::{
    ffi::{CStr, c_int},
    sync::Mutex,
};

use openfx::{
    low::{
        Status,
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
    sys::generic::core::{OfxRectI, kOfxStatFailed},
    sys_helpers::image_effect_v1::properties::get_OfxImageEffectPropRenderWindow,
};

use crate::{
    definitions::{PLUGIN_2_INVERT_IDENTIFIER, PLUGIN_2_INVERT_LABEL, PLUGINS_GROUPING},
    helpers::shared_data::{BitDepth, ClipImageManaged, GuaranteeSend, SharedData},
};

static HOST_BEFORE_ACTION_LOAD: Mutex<Option<GuaranteeSend<Host>>> = Mutex::new(None);
static SHARED_DATA: Mutex<Option<SharedData>> = Mutex::new(None);

fn shared_data_lockless() -> openfx::low::Result<SharedData> {
    let data = SHARED_DATA.lock().map_err(|_| Status::Failed)?;
    let data = data.as_ref().ok_or(Status::Failed)?;
    Ok(data.clone())
}

pub struct PluginExampleInvert;
impl Plugin for PluginExampleInvert {
    const PLUGIN_IDENTIFIER: &'static CStr = PLUGIN_2_INVERT_IDENTIFIER;
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
            ImageEffectAction::Render {
                handle, in_args, ..
            } => action_render(handle, in_args),
            ImageEffectAction::CreateInstance { .. }
            | ImageEffectAction::DestroyInstance { .. } => Ok(()),
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
        Err(Status::Failed)
    } else {
        *data = Some(SharedData::try_new(host)?);
        Ok(())
    }
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
    let data = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;

    let props = unsafe { descriptor.get_property_set(&data.image_effect_suite.0) }?;

    unsafe {
        props.set_label(s_prop, Some(PLUGIN_2_INVERT_LABEL))?;
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
    let data = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;

    let context = unsafe { in_args.get_image_effect_context(s_prop) }?;
    if context != ImageEffectPropContext::Filter {
        return Err(Status::ErrUnsupported);
    }

    for name in [c"Output", c"Source"] {
        let props = unsafe { s_ifx.clip_define(&descriptor, name) }?;

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

    Ok(())
}

/// Look up a pixel in the image. Returns `None` if the pixel was not in the
/// bounds of the image.
fn pixel_address<T>(
    x: c_int,
    y: c_int,
    base_address: *mut T,
    bounds: OfxRectI,
    row_bytes: c_int,
    n_comps_per_pixel: c_int,
) -> Option<*mut T> {
    if x < bounds.x1 || x >= bounds.x2 || y < bounds.y1 || y >= bounds.y2 {
        return None;
    }

    let x_offset = (x - bounds.x1) as isize;
    let y_offset = (y - bounds.y1) as isize;

    let row_start_address =
        unsafe { (base_address as *mut u8).offset(y_offset * row_bytes as isize) as *mut T };

    Some(unsafe { row_start_address.offset(x_offset * n_comps_per_pixel as isize) })
}

fn pixel_processing<T>(
    max: T,
    data: &SharedData,
    instance: ImageEffectInstance,
    source_img: ClipImageManaged,
    output_img: ClipImageManaged,
    render_window: OfxRectI,
) -> openfx::low::Result<()>
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

        let Some(dst_pix) = pixel_address(
            render_window.x1,
            y,
            dst_ptr,
            dst_bounds,
            dst_row_bytes,
            n_comps,
        ) else {
            return Err(Status::Failed);
        };
        let mut dst_pix = dst_pix;

        for x in render_window.x1..render_window.x2 {
            let src_pix = pixel_address(x, y, src_ptr, src_bounds, src_row_bytes, n_comps);

            if let Some(src_pix) = src_pix {
                let mut src_pix = src_pix;
                for i in 0..n_comps {
                    unsafe {
                        *dst_pix = if i != 3 { max - *src_pix } else { *src_pix };
                        dst_pix = dst_pix.offset(1);
                        src_pix = src_pix.offset(1);
                    }
                }
            } else {
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

fn rect_i_from_array(arr: &[c_int; 4]) -> OfxRectI {
    OfxRectI {
        x1: arr[0],
        y1: arr[1],
        x2: arr[2],
        y2: arr[3],
    }
}

fn action_render(
    instance: ImageEffectInstance,
    in_args: ActionRenderIn,
) -> openfx::low::Result<()> {
    let data = shared_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;

    let time = unsafe { in_args.get_time(s_prop) }?;
    let render_window =
        unsafe { get_OfxImageEffectPropRenderWindow(s_prop.sys_ptr(), in_args.sys_handle()) }?;
    let render_window = rect_i_from_array(&render_window);

    let output_clip = unsafe { s_ifx.clip_get_clip_handle(&instance, c"Output") }?;
    let source_clip = unsafe { s_ifx.clip_get_clip_handle(&instance, c"Source") }?;

    let output_img = unsafe { output_clip.clip_get_image(s_ifx, time, None) }?;
    let source_img = unsafe { source_clip.clip_get_image(s_ifx, time, None) }?;

    let output_img_m = unsafe { ClipImageManaged::try_new(&data, output_img) }?;
    let source_img_m = unsafe { ClipImageManaged::try_new(&data, source_img) }?;

    match output_img_m.pixel_depth() {
        BitDepth::Byte => pixel_processing(
            255u8,
            &data,
            instance,
            source_img_m,
            output_img_m,
            render_window,
        ),
        BitDepth::Short => pixel_processing(
            65535u16,
            &data,
            instance,
            source_img_m,
            output_img_m,
            render_window,
        ),
        BitDepth::Float => pixel_processing(
            1.0f32,
            &data,
            instance,
            source_img_m,
            output_img_m,
            render_window,
        ),
    }
}

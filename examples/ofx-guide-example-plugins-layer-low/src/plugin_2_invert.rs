use std::{
    ffi::{CStr, c_int},
    sync::Mutex,
};

use openfx::{
    low::{Status, enums::ImageEffectPropContext},
    low_plugin::{
        Host, Plugin,
        actions::image_effect::{ActionDescribeInContextIn, ActionRenderIn, ImageEffectAction},
    },
    sys::{
        generic::core::{
            OfxPropertySetHandle, OfxRectI, OfxStatus, kOfxBitDepthByte, kOfxBitDepthFloat,
            kOfxBitDepthShort, kOfxStatFailed,
        },
        image_effect_v1::image_effect::{
            OfxImageEffectHandle, kOfxImageComponentAlpha, kOfxImageComponentRGB,
            kOfxImageComponentRGBA, kOfxImageEffectContextFilter, kOfxImageEffectRenderFullySafe,
        },
    },
    sys_helpers::{
        generic::properties::set_OfxPropLabel,
        image_effect_v1::properties::{
            get_OfxImageEffectPropComponents, get_OfxImageEffectPropPixelDepth,
            get_OfxImageEffectPropRenderWindow, get_OfxImagePropBounds, get_OfxImagePropData,
            get_OfxImagePropRowBytes, set_OfxImageEffectPluginPropGrouping,
            set_OfxImageEffectPluginPropHostFrameThreading,
            set_OfxImageEffectPluginRenderThreadSafety, set_OfxImageEffectPropSupportedComponents,
            set_OfxImageEffectPropSupportedContexts, set_OfxImageEffectPropSupportedPixelDepths,
        },
    },
};

use crate::{
    definitions::{PLUGIN_2_INVERT_IDENTIFIER, PLUGIN_2_INVERT_LABEL, PLUGINS_GROUPING},
    helpers::{GuaranteeSend, SharedData, shared_data_helper::SharedDataHelper},
};

static HOST_BEFORE_ACTION_LOAD: Mutex<Option<GuaranteeSend<Host>>> = Mutex::new(None);
static SHARED_DATA: Mutex<Option<SharedData<'static>>> = Mutex::new(None);

fn shared_data_lockless() -> Result<SharedData<'static>, OfxStatus> {
    let data = SHARED_DATA.lock().map_err(|_| kOfxStatFailed)?;
    let data = data.as_ref().ok_or(kOfxStatFailed)?;
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
            ImageEffectAction::Describe { sys_handle, .. } => {
                action_describe(sys_handle as OfxImageEffectHandle)
            }
            ImageEffectAction::DescribeInContext {
                sys_handle,
                in_args,
                ..
            } => action_describe_in_context(sys_handle as OfxImageEffectHandle, in_args),
            ImageEffectAction::Render {
                sys_handle,
                in_args,
                ..
            } => action_render(sys_handle as OfxImageEffectHandle, in_args),
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

fn action_describe(descriptor: OfxImageEffectHandle) -> openfx::low::Result<()> {
    let data = shared_data_lockless()?;
    let data = unsafe { SharedDataHelper::try_new(&data) }?;

    let s_prop = data.inner().property_suite;

    let props = unsafe { data.get_property_set_from_image_effect(descriptor) }?;

    unsafe {
        set_OfxPropLabel(s_prop, props, PLUGIN_2_INVERT_LABEL.as_ptr())?;
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
    in_args: ActionDescribeInContextIn,
) -> openfx::low::Result<()> {
    let data = shared_data_lockless()?;
    let data = unsafe { SharedDataHelper::try_new(&data) }?;

    let s_prop = data.inner().property_suite;
    let image_effect_suite_helper = data.image_effect_suite_helper();

    let context = unsafe { in_args.get_image_effect_context(s_prop) }?;
    if context != ImageEffectPropContext::Filter {
        return Err(Status::ErrUnsupported);
    }

    for name in [c"Output", c"Source"] {
        let props = unsafe { image_effect_suite_helper.clip_define(descriptor, name) }?;

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
    data: &SharedDataHelper,
    instance: OfxImageEffectHandle,
    source_img: OfxPropertySetHandle,
    output_img: OfxPropertySetHandle,
    render_window: OfxRectI,
    n_comps: c_int,
) -> openfx::low::Result<()>
where
    T: std::ops::Sub<Output = T> + Copy + Default,
{
    let s_prop = data.inner().property_suite;

    let dst_row_bytes = unsafe { get_OfxImagePropRowBytes(s_prop, output_img) }?;
    let dst_bounds = unsafe { get_OfxImagePropBounds(s_prop, output_img) }?;
    let dst_bounds = rect_i_from_array(&dst_bounds);
    let dst_ptr = unsafe { get_OfxImagePropData(s_prop, output_img) }? as *mut T;
    if dst_ptr.is_null() {
        return Err(Status::Failed);
    }

    let src_row_bytes = unsafe { get_OfxImagePropRowBytes(s_prop, source_img) }?;
    let src_bounds = unsafe { get_OfxImagePropBounds(s_prop, source_img) }?;
    let src_bounds = rect_i_from_array(&src_bounds);
    let src_ptr = unsafe { get_OfxImagePropData(s_prop, source_img) }? as *mut T;
    if src_ptr.is_null() {
        return Err(Status::Failed);
    }

    for y in render_window.y1..render_window.y2 {
        if y % 20 == 0
            && data
                .inner()
                .image_effect_suite
                .abort
                .is_some_and(|abort| unsafe { abort(instance) } != 0)
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
    instance: OfxImageEffectHandle,
    in_args: ActionRenderIn,
) -> openfx::low::Result<()> {
    let data = shared_data_lockless()?;
    let data = unsafe { SharedDataHelper::try_new(&data) }?;

    let s_prop = data.inner().property_suite;
    let image_effect_suite_helper = data.image_effect_suite_helper();

    let time = unsafe { in_args.get_time(s_prop) }?;
    let render_window =
        unsafe { get_OfxImageEffectPropRenderWindow(s_prop, in_args.sys_handle()) }?;
    let render_window = rect_i_from_array(&render_window);

    let output_clip = unsafe { image_effect_suite_helper.clip_get_handle(instance, c"Output") }?;
    let source_clip = unsafe { image_effect_suite_helper.clip_get_handle(instance, c"Source") }?;

    let Some(output_img_m) = unsafe { data.make_clip_image_managed(output_clip, time, None) }?
    else {
        return Err(Status::Failed);
    };
    let Some(source_img_m) = unsafe { data.make_clip_image_managed(source_clip, time, None) }?
    else {
        return Err(Status::Failed);
    };

    fn inner(
        data: &SharedDataHelper,
        instance: OfxImageEffectHandle,
        source_img: OfxPropertySetHandle,
        output_img: OfxPropertySetHandle,
        render_window: OfxRectI,
    ) -> openfx::low::Result<()> {
        let s_prop = data.inner().property_suite;

        let components = unsafe { get_OfxImageEffectPropComponents(s_prop, output_img) }?;
        if components.is_null() {
            return Err(Status::ErrUnsupported);
        }
        let n_comps = match unsafe { CStr::from_ptr(components) } {
            c if c == kOfxImageComponentRGBA => 4,
            c if c == kOfxImageComponentRGB => 3,
            c if c == kOfxImageComponentAlpha => 1,
            _ => return Err(Status::ErrUnsupported),
        };

        let data_type = unsafe { get_OfxImageEffectPropPixelDepth(s_prop, output_img) }?;
        if data_type.is_null() {
            return Err(Status::ErrUnsupported);
        }
        match unsafe { CStr::from_ptr(data_type) } {
            c if c == kOfxBitDepthByte => pixel_processing(
                255u8,
                data,
                instance,
                source_img,
                output_img,
                render_window,
                n_comps,
            ),
            c if c == kOfxBitDepthShort => pixel_processing(
                65535u16,
                data,
                instance,
                source_img,
                output_img,
                render_window,
                n_comps,
            ),
            c if c == kOfxBitDepthFloat => pixel_processing(
                1.0f32,
                data,
                instance,
                source_img,
                output_img,
                render_window,
                n_comps,
            ),
            _ => return Err(Status::Unlicensed),
        }?;

        Ok(())
    }

    let result = inner(
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

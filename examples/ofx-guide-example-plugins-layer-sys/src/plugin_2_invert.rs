use std::{
    ffi::{CStr, c_char, c_int, c_void},
    sync::{Mutex, OnceLock},
};

use openfx::{
    generic::{
        sys::core::{
            OfxHost, OfxPropertySetHandle, OfxRectI, OfxStatus, OfxTime, kOfxActionCreateInstance,
            kOfxActionDescribe, kOfxActionDestroyInstance, kOfxActionLoad, kOfxActionUnload,
            kOfxBitDepthByte, kOfxBitDepthFloat, kOfxBitDepthShort, kOfxStatErrUnsupported,
            kOfxStatFailed, kOfxStatOK, kOfxStatReplyDefault,
        },
        sys_helpers::properties::{get_OfxPropTime, set_OfxPropLabel},
    },
    image_effect_v1::{
        sys::image_effect::{
            OfxImageEffectHandle, kOfxImageComponentAlpha, kOfxImageComponentRGB,
            kOfxImageComponentRGBA, kOfxImageEffectActionDescribeInContext,
            kOfxImageEffectActionRender, kOfxImageEffectContextFilter,
            kOfxImageEffectRenderFullySafe,
        },
        sys_helpers::{
            Plugin,
            properties::{
                get_OfxImageEffectPropComponents, get_OfxImageEffectPropContext,
                get_OfxImageEffectPropPixelDepth, get_OfxImageEffectPropRenderWindow,
                get_OfxImagePropBounds, get_OfxImagePropData, get_OfxImagePropRowBytes,
                set_OfxImageEffectPluginPropGrouping,
                set_OfxImageEffectPluginPropHostFrameThreading,
                set_OfxImageEffectPluginRenderThreadSafety,
                set_OfxImageEffectPropSupportedComponents, set_OfxImageEffectPropSupportedContexts,
                set_OfxImageEffectPropSupportedPixelDepths,
            },
        },
    },
};

use crate::{
    definitions::{PLUGIN_2_INVERT_IDENTIFIER, PLUGIN_2_INVERT_LABEL, PLUGINS_GROUPING},
    helpers::{SaferHostStruct, SharedData, shared_data_helper::SharedDataHelper},
};

static HOST_STRUCT: OnceLock<SaferHostStruct> = OnceLock::new();

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

    extern "C" fn set_host(host_struct: *mut OfxHost) {
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

    extern "C" fn main_entry(
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
            _ if action == kOfxImageEffectActionRender => action_render(effect, in_args, out_args),
            _ if action == kOfxActionCreateInstance || action == kOfxActionDestroyInstance => {
                // We need to handle these actions (even if it's just a no-op) for DaVinci resolve to properly load our plugin
                // If not handled, it'll load the plugin but will never show the controls or actually render anything
                Ok(())
            }
            _ => Err(kOfxStatReplyDefault),
        };

        match result {
            Ok(_) => kOfxStatOK,
            Err(status) => status,
        }
    }
}

fn action_load() -> Result<(), OfxStatus> {
    let host_struct = HOST_STRUCT.get().ok_or(kOfxStatFailed)?.clone();

    let mut data = SHARED_DATA.lock().map_err(|_| kOfxStatFailed)?;
    if data.is_some() {
        Err(kOfxStatFailed)
    } else {
        *data = Some(SharedData::try_new(host_struct)?);
        Ok(())
    }
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
    in_args: OfxPropertySetHandle,
) -> Result<(), OfxStatus> {
    let data = shared_data_lockless()?;
    let data = unsafe { SharedDataHelper::try_new(&data) }?;

    let s_prop = data.inner().property_suite;
    let image_effect_suite_helper = data.image_effect_suite_helper();

    let context = unsafe { get_OfxImageEffectPropContext(s_prop, in_args) }?;
    if context.is_null() || unsafe { CStr::from_ptr(context) } != kOfxImageEffectContextFilter {
        return Err(kOfxStatErrUnsupported);
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
) -> Result<(), OfxStatus>
where
    T: std::ops::Sub<Output = T> + Copy + Default,
{
    let s_prop = data.inner().property_suite;

    let dst_row_bytes = unsafe { get_OfxImagePropRowBytes(s_prop, output_img) }?;
    let dst_bounds = unsafe { get_OfxImagePropBounds(s_prop, output_img) }?;
    let dst_bounds = rect_i_from_array(&dst_bounds);
    let dst_ptr = unsafe { get_OfxImagePropData(s_prop, output_img) }? as *mut T;
    if dst_ptr.is_null() {
        return Err(kOfxStatFailed);
    }

    let src_row_bytes = unsafe { get_OfxImagePropRowBytes(s_prop, source_img) }?;
    let src_bounds = unsafe { get_OfxImagePropBounds(s_prop, source_img) }?;
    let src_bounds = rect_i_from_array(&src_bounds);
    let src_ptr = unsafe { get_OfxImagePropData(s_prop, source_img) }? as *mut T;
    if src_ptr.is_null() {
        return Err(kOfxStatFailed);
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
            return Err(kOfxStatFailed);
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
    in_args: OfxPropertySetHandle,
    _out_args: OfxPropertySetHandle,
) -> Result<(), OfxStatus> {
    let data = shared_data_lockless()?;
    let data = unsafe { SharedDataHelper::try_new(&data) }?;

    let s_prop = data.inner().property_suite;
    let image_effect_suite_helper = data.image_effect_suite_helper();

    let time: OfxTime = unsafe { get_OfxPropTime(s_prop, in_args) }?;
    let render_window = unsafe { get_OfxImageEffectPropRenderWindow(s_prop, in_args) }?;
    let render_window = rect_i_from_array(&render_window);

    let output_clip = unsafe { image_effect_suite_helper.clip_get_handle(instance, c"Output") }?;
    let source_clip = unsafe { image_effect_suite_helper.clip_get_handle(instance, c"Source") }?;

    let Some(output_img_m) = unsafe { data.make_clip_image_managed(output_clip, time, None) }?
    else {
        return Err(kOfxStatFailed);
    };
    let Some(source_img_m) = unsafe { data.make_clip_image_managed(source_clip, time, None) }?
    else {
        return Err(kOfxStatFailed);
    };

    fn inner(
        data: &SharedDataHelper,
        instance: OfxImageEffectHandle,
        source_img: OfxPropertySetHandle,
        output_img: OfxPropertySetHandle,
        render_window: OfxRectI,
    ) -> Result<(), OfxStatus> {
        let s_prop = data.inner().property_suite;

        let components = unsafe { get_OfxImageEffectPropComponents(s_prop, output_img) }?;
        if components.is_null() {
            return Err(kOfxStatErrUnsupported);
        }
        let n_comps = match unsafe { CStr::from_ptr(components) } {
            c if c == kOfxImageComponentRGBA => 4,
            c if c == kOfxImageComponentRGB => 3,
            c if c == kOfxImageComponentAlpha => 1,
            _ => return Err(kOfxStatErrUnsupported),
        };

        let data_type = unsafe { get_OfxImageEffectPropPixelDepth(s_prop, output_img) }?;
        if data_type.is_null() {
            return Err(kOfxStatErrUnsupported);
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
            _ => return Err(kOfxStatErrUnsupported),
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

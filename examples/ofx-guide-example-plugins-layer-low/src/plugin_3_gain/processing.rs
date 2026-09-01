use std::ffi::c_int;

use openfx::{
    generic::sys::core::{OfxPropertySetHandle, OfxRectI, OfxStatus, kOfxStatFailed},
    image_effect_v1::{
        sys::image_effect::OfxImageEffectHandle,
        sys_helpers::properties::{
            get_OfxImagePropBounds, get_OfxImagePropData, get_OfxImagePropRowBytes,
        },
    },
};

use crate::helpers::shared_data_helper::SharedDataHelper;

#[allow(clippy::too_many_arguments)]
pub fn pixel_processing<T>(
    from_f64: fn(f64) -> T,
    into_f64: fn(T) -> f64,
    max: T,
    gain: f64,
    apply_to_alpha: bool,
    data: &SharedDataHelper,
    instance: OfxImageEffectHandle,
    source_img: OfxPropertySetHandle,
    output_img: OfxPropertySetHandle,
    render_window: OfxRectI,
    n_comps: c_int,
) -> Result<(), OfxStatus>
where
    T: Copy + Default,
{
    let s_prop = data.inner().property_suite;

    let dst_row_bytes = unsafe { get_OfxImagePropRowBytes(s_prop, source_img) }?;
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
                        *dst_pix = if i != 3 || apply_to_alpha {
                            let mut value = (into_f64)(*src_pix) * gain;
                            if (into_f64)(max) != 1.0 {
                                value = value.clamp(0.0, (into_f64)(max));
                            }
                            from_f64(value)
                        } else {
                            *src_pix
                        };
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

pub fn rect_i_from_array(arr: &[c_int; 4]) -> OfxRectI {
    OfxRectI {
        x1: arr[0],
        y1: arr[1],
        x2: arr[2],
        y2: arr[3],
    }
}

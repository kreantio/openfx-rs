use std::ffi::c_int;

use openfx::{
    low::Status,
    sys::{generic::core::OfxRectI, image_effect_v1::image_effect::OfxImageEffectHandle},
};

use crate::helpers::shared_data::{ClipImageManaged, SharedData};

#[allow(clippy::too_many_arguments)]
pub fn pixel_processing<T>(
    from_f64: fn(f64) -> T,
    into_f64: fn(T) -> f64,
    max: T,
    gain: f64,
    apply_to_alpha: bool,
    data: &SharedData,
    instance: OfxImageEffectHandle,
    source_img: ClipImageManaged,
    output_img: ClipImageManaged,
    render_window: OfxRectI,
) -> openfx::low::Result<()>
where
    T: Copy + Default,
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
                data.0
                    .image_effect_suite
                    .0
                    .sys_ref()
                    .abort
                    .is_some_and(|abort| abort(instance) != 0)
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

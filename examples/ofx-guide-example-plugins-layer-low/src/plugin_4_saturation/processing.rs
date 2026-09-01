use std::ffi::c_int;

use openfx::{
    generic::sys::core::{OfxRectI, OfxStatus, kOfxStatFailed},
    image_effect_v1::sys::image_effect::OfxImageEffectHandle,
};

use crate::helpers::shared_data_helper::{ClipImageManaged, SharedDataHelper};

#[allow(clippy::too_many_arguments)]
pub fn pixel_processing<T>(
    from_f64: fn(f64) -> T,
    into_f64: fn(T) -> f64,
    max: T,
    saturation: f64,
    data: &SharedDataHelper,
    instance: OfxImageEffectHandle,
    source_img: ClipImageManaged,
    mask_img: Option<ClipImageManaged>,
    output_img: ClipImageManaged,
    render_window: OfxRectI,
) -> Result<(), OfxStatus>
where
    T: Copy + Default + std::ops::Add<Output = T>,
{
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

        let Some(dst_pix) = output_img.raw_address(render_window.x1, y) else {
            return Err(kOfxStatFailed);
        };
        let mut dst_pix = dst_pix as *mut T;

        for x in render_window.x1..render_window.x2 {
            let src_pix = source_img.raw_address(x, y).map(|ptr| ptr as *mut T);

            let mask_amount = if let Some(mask_img) = &mask_img {
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
                    for _ in 0..output_img.n_comps() {
                        unsafe {
                            *dst_pix = *src_pix;
                            dst_pix = dst_pix.offset(1);
                            src_pix = src_pix.offset(1);
                        }
                    }
                } else {
                    let rgb = [
                        into_f64(unsafe { *src_pix }),
                        into_f64(unsafe { *src_pix.offset(1) }),
                        into_f64(unsafe { *src_pix.offset(2) }),
                    ];

                    let average = (rgb[0] + rgb[1] + rgb[2]) / 3.0;

                    for (i, c) in rgb.iter().enumerate() {
                        let value = (*c - average) * saturation + average;
                        let value = value.clamp(0.0, into_f64(max));
                        let blended = blend(rgb[i], value, mask_amount);
                        unsafe { *dst_pix.add(i) = from_f64(blended) };
                    }

                    if output_img.n_comps() == 4 {
                        unsafe {
                            *dst_pix.add(3) = *src_pix.add(3);
                        }
                    }

                    unsafe { dst_pix = dst_pix.offset(output_img.n_comps() as isize) };
                }
            } else {
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

pub fn rect_i_from_array(arr: &[c_int; 4]) -> OfxRectI {
    OfxRectI {
        x1: arr[0],
        y1: arr[1],
        x2: arr[2],
        y2: arr[3],
    }
}

fn blend(v1: f64, v2: f64, blend: f64) -> f64 {
    v1 + (v2 - v1) * blend
}

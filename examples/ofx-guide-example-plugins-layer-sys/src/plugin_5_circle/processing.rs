use std::ffi::c_int;

use openfx::{
    generic::sys::core::{OfxRectD, OfxRectI, OfxStatus, kOfxStatFailed},
    image_effect_v1::sys::image_effect::OfxImageEffectHandle,
};

use crate::helpers::shared_data_helper::{ClipImageManaged, SharedDataHelper};

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
    data: &SharedDataHelper,
    instance: OfxImageEffectHandle,
    source_img: ClipImageManaged,
    output_img: ClipImageManaged,
    render_window: OfxRectI,
) -> Result<(), OfxStatus>
where
    T: Copy + Default + std::ops::Add<Output = T>,
{
    let par = output_img.pixel_aspect_ratio();

    let colour_quantised = if into_f64(max) == 1.0 {
        // don't clamp floating point values
        colour.map(from_f64)
    } else {
        colour.map(|c| clamp(from_f64(c * into_f64(max)), T::default(), max))
    };

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

        let y_canonical = (y as f64 + 0.5) / render_scale[1];
        let dy = y_canonical - centre[1];

        let Some(dst_pix) = output_img.raw_address(render_window.x1, y) else {
            return Err(kOfxStatFailed);
        };
        let mut dst_pix = dst_pix as *mut T;

        for x in render_window.x1..render_window.x2 {
            let x_canonical = (x as f64 + 0.5) * par / render_scale[0];
            let dx = x_canonical - centre[0];
            let d = (dx * dx + dy * dy).sqrt();
            let mut alpha = colour[3];

            if d < radius {
                if d > radius - 1.0 {
                    alpha *= radius - d;
                }
            } else {
                alpha = 0.0;
            }

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

            for i in 0..output_img.n_comps() {
                unsafe {
                    *dst_pix.add(i as usize) = from_f64(blend(
                        into_f64(comp(src_pix, i as usize)),
                        into_f64(colour_quantised[i as usize]),
                        alpha,
                    ));
                }
            }
            unsafe { dst_pix = dst_pix.offset(output_img.n_comps() as isize) };
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

pub fn rect_d_to_array(rect: &OfxRectD) -> [f64; 4] {
    [rect.x1, rect.y1, rect.x2, rect.y2]
}

fn blend(v1: f64, v2: f64, blend: f64) -> f64 {
    v1 + (v2 - v1) * blend
}

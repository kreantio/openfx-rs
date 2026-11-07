use std::ffi::c_int;

use openfx::{
    low::StatusError,
    low_plugin::{property_sets::ImageEffectDescriptorPropertySet, suites::PropertySuiteV1},
    sys::core::{OfxRectD, OfxRectI},
};

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

pub unsafe fn set_labels(
    s_prop: &PropertySuiteV1,
    props: &ImageEffectDescriptorPropertySet,
    label: &std::ffi::CStr,
) -> Result<(), StatusError> {
    (unsafe {
        props
            .set_label(s_prop, Some(label))
            .ok_or(StatusError::Failed)
    })?;
    (unsafe {
        props
            .set_short_label(s_prop, Some(label))
            .ok_or(StatusError::Failed)
    })?;
    // If unset, DaVinci Resolve will display an empty label on the side panel.
    (unsafe {
        props
            .set_long_label(s_prop, Some(label))
            .ok_or(StatusError::Failed)
    })?;

    Ok(())
}

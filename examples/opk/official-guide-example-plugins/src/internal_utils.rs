use openfx::{
    low::StatusError,
    low_plugin::{property_sets::ImageEffectDescriptorPropertySet, suites::PropertySuiteV1},
};

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

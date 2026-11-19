pub mod guarantee_send;
pub mod managed_clip_images;
pub mod plugin_data;
pub mod renderer_info;
pub mod sys_type_utils;
pub mod types;

#[cfg(feature = "opengl")]
pub mod gl;
#[cfg(feature = "opengl")]
pub mod gl_loader;

#[macro_export]
macro_rules! set_labels {
    ($s_prop:expr, $props:expr, $label:expr) => {{
        {
            $props
                .set_label($s_prop, Some($label))
                .ok_or(StatusError::Failed)
        }?;
        {
            $props
                .set_short_label($s_prop, Some($label))
                .ok_or(StatusError::Failed)
        }?;
        // If unset, DaVinci Resolve will display an empty label on the side panel.
        {
            $props
                .set_long_label($s_prop, Some($label))
                .ok_or(StatusError::Failed)
        }?;
    }};
}

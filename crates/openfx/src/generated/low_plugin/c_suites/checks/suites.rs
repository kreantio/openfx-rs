const _: () = {
    use crate::low_plugin::suites::DialogSuiteV1;
    let _ = DialogSuiteV1::notify_redraw_pending;
    let _ = DialogSuiteV1::request_dialog;
};
const _: () = {
    use crate::low_plugin::suites::DrawSuiteV1;
    let _ = DrawSuiteV1::draw;
    let _ = DrawSuiteV1::draw_text;
    let _ = DrawSuiteV1::get_colour;
    let _ = DrawSuiteV1::set_colour;
    let _ = DrawSuiteV1::set_line_stipple;
    let _ = DrawSuiteV1::set_line_width;
};
const _: () = {
    use crate::low_plugin::suites::ImageEffectOpenGLRenderSuiteV1;
    let _ = ImageEffectOpenGLRenderSuiteV1::clip_free_texture;
    let _ = ImageEffectOpenGLRenderSuiteV1::clip_load_texture;
    let _ = ImageEffectOpenGLRenderSuiteV1::flush_resources;
};
const _: () = {
    use crate::low_plugin::suites::ImageEffectSuiteV1;
    let _ = ImageEffectSuiteV1::abort;
    let _ = ImageEffectSuiteV1::clip_define;
    let _ = ImageEffectSuiteV1::clip_get_handle;
    let _ = ImageEffectSuiteV1::clip_get_image;
    let _ = ImageEffectSuiteV1::clip_get_property_set;
    let _ = ImageEffectSuiteV1::clip_get_region_of_definition;
    let _ = ImageEffectSuiteV1::clip_release_image;
    let _ = ImageEffectSuiteV1::get_param_set;
    let _ = ImageEffectSuiteV1::get_property_set;
    let _ = ImageEffectSuiteV1::image_memory_alloc;
    let _ = ImageEffectSuiteV1::image_memory_free;
    let _ = ImageEffectSuiteV1::image_memory_lock;
    let _ = ImageEffectSuiteV1::image_memory_unlock;
};
const _: () = {
    use crate::low_plugin::suites::InteractSuiteV1;
    let _ = InteractSuiteV1::interact_get_property_set;
    let _ = InteractSuiteV1::interact_redraw;
    let _ = InteractSuiteV1::interact_swap_buffers;
};
const _: () = {
    use crate::low_plugin::suites::MemorySuiteV1;
    let _ = MemorySuiteV1::memory_alloc;
    let _ = MemorySuiteV1::memory_free;
};
const _: () = {
    use crate::low_plugin::suites::MessageSuiteV1;
    let _ = MessageSuiteV1::sys_message;
};
const _: () = {
    use crate::low_plugin::suites::MessageSuiteV2;
    let _ = MessageSuiteV2::sys_clear_persistent_message;
    let _ = MessageSuiteV2::sys_message;
    let _ = MessageSuiteV2::sys_set_persistent_message;
};
const _: () = {
    use crate::low_plugin::suites::MultiThreadSuiteV1;
    let _ = MultiThreadSuiteV1::multi_thread_index;
    let _ = MultiThreadSuiteV1::multi_thread_is_spawned_thread;
    let _ = MultiThreadSuiteV1::multi_thread_num_cpus;
    let _ = MultiThreadSuiteV1::mutex_create;
    let _ = MultiThreadSuiteV1::mutex_destroy;
    let _ = MultiThreadSuiteV1::mutex_lock;
    let _ = MultiThreadSuiteV1::mutex_try_lock;
    let _ = MultiThreadSuiteV1::mutex_unlock;
    let _ = MultiThreadSuiteV1::sys_multi_thread;
};
const _: () = {
    use crate::low_plugin::suites::ParameterSuiteV1;
    let _ = ParameterSuiteV1::param_copy;
    let _ = ParameterSuiteV1::param_define;
    let _ = ParameterSuiteV1::param_delete_all_keys;
    let _ = ParameterSuiteV1::param_delete_key;
    let _ = ParameterSuiteV1::param_edit_begin;
    let _ = ParameterSuiteV1::param_edit_end;
    let _ = ParameterSuiteV1::param_get_handle;
    let _ = ParameterSuiteV1::param_get_key_index;
    let _ = ParameterSuiteV1::param_get_key_time;
    let _ = ParameterSuiteV1::param_get_num_keys;
    let _ = ParameterSuiteV1::param_get_property_set;
    let _ = ParameterSuiteV1::param_set_get_property_set;
};
const _: () = {
    use crate::low_plugin::suites::ParametricParameterSuiteV1;
    let _ = ParametricParameterSuiteV1::parametric_param_add_control_point;
    let _ = ParametricParameterSuiteV1::parametric_param_delete_all_control_points;
    let _ = ParametricParameterSuiteV1::parametric_param_delete_control_point;
    let _ = ParametricParameterSuiteV1::parametric_param_get_n_control_points;
    let _ = ParametricParameterSuiteV1::parametric_param_get_nth_control_point;
    let _ = ParametricParameterSuiteV1::parametric_param_get_value;
    let _ = ParametricParameterSuiteV1::parametric_param_set_nth_control_point;
};
const _: () = {
    use crate::low_plugin::suites::ProgressSuiteV1;
    let _ = ProgressSuiteV1::progress_end;
    let _ = ProgressSuiteV1::progress_start;
    let _ = ProgressSuiteV1::progress_update;
};
const _: () = {
    use crate::low_plugin::suites::ProgressSuiteV2;
    let _ = ProgressSuiteV2::progress_end;
    let _ = ProgressSuiteV2::progress_start;
    let _ = ProgressSuiteV2::progress_update;
};
const _: () = {
    use crate::low_plugin::suites::PropertySuiteV1;
    let _ = PropertySuiteV1::prop_get_dimension;
    let _ = PropertySuiteV1::prop_get_double;
    let _ = PropertySuiteV1::prop_get_double_n;
    let _ = PropertySuiteV1::prop_get_int;
    let _ = PropertySuiteV1::prop_get_int_n;
    let _ = PropertySuiteV1::prop_get_pointer;
    let _ = PropertySuiteV1::prop_get_pointer_n;
    let _ = PropertySuiteV1::prop_reset;
    let _ = PropertySuiteV1::prop_set_double;
    let _ = PropertySuiteV1::prop_set_double_n;
    let _ = PropertySuiteV1::prop_set_int;
    let _ = PropertySuiteV1::prop_set_int_n;
    let _ = PropertySuiteV1::prop_set_pointer;
    let _ = PropertySuiteV1::prop_set_pointer_n;
    let _ = PropertySuiteV1::prop_set_string;
    let _ = PropertySuiteV1::prop_set_string_n;
};
const _: () = {
    use crate::low_plugin::suites::TimeLineSuiteV1;
    let _ = TimeLineSuiteV1::get_time;
    let _ = TimeLineSuiteV1::get_time_bounds;
    let _ = TimeLineSuiteV1::goto_time;
};

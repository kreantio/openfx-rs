openfx_internal_macros::low_make_suite_struct!(
    DialogSuiteV1 : OfxDialogSuiteV1 : notify_redraw_pending, request_dialog,
);
openfx_internal_macros::low_make_suite_struct!(
    DrawSuiteV1 : OfxDrawSuiteV1 : get_colour, draw, draw_text, set_colour,
    set_line_stipple, set_line_width,
);
openfx_internal_macros::low_make_suite_struct!(
    ImageEffectOpenGLRenderSuiteV1 : OfxImageEffectOpenGLRenderSuiteV1 :
    clip_free_texture, flush_resources, clip_load_texture,
);
openfx_internal_macros::low_make_suite_struct!(
    ImageEffectSuiteV1 : OfxImageEffectSuiteV1 : image_memory_alloc, get_property_set,
    image_memory_free, clip_release_image, clip_get_handle, clip_get_image,
    get_param_set, clip_get_property_set, clip_define, clip_get_region_of_definition,
    abort, image_memory_lock, image_memory_unlock,
);
openfx_internal_macros::low_make_suite_struct!(
    InteractSuiteV1 : OfxInteractSuiteV1 : interact_get_property_set,
    interact_swap_buffers, interact_redraw,
);
openfx_internal_macros::low_make_suite_struct!(
    MemorySuiteV1 : OfxMemorySuiteV1 : memory_free, memory_alloc,
);
openfx_internal_macros::low_make_suite_struct!(
    MessageSuiteV1 : OfxMessageSuiteV1 : message,
);
openfx_internal_macros::low_make_suite_struct!(
    MessageSuiteV2 : OfxMessageSuiteV2 : message, clear_persistent_message,
    set_persistent_message,
);
openfx_internal_macros::low_make_suite_struct!(
    MultiThreadSuiteV1 : OfxMultiThreadSuiteV1 : multi_thread_index, mutex_lock,
    mutex_create, mutex_try_lock, mutex_un_lock, multi_thread_is_spawned_thread,
    multi_thread_num_cp_us, multi_thread, mutex_destroy,
);
openfx_internal_macros::low_make_suite_struct!(
    OpenCLProgramSuiteV1 : OfxOpenCLProgramSuiteV1 : compile_program,
);
openfx_internal_macros::low_make_suite_struct!(
    ParameterSuiteV1 : OfxParameterSuiteV1 : param_get_value, param_set_get_property_set,
    param_delete_key, param_get_num_keys, param_set_value_at_time, param_get_integral,
    param_get_key_time, param_get_value_at_time, param_edit_end, param_get_key_index,
    param_get_handle, param_delete_all_keys, param_copy, param_set_value, param_define,
    param_edit_begin, param_get_property_set, param_get_derivative,
);
openfx_internal_macros::low_make_suite_struct!(
    ParametricParameterSuiteV1 : OfxParametricParameterSuiteV1 :
    parametric_param_get_value, parametric_param_delete_control_point,
    parametric_param_delete_all_control_points, parametric_param_add_control_point,
    parametric_param_set_nth_control_point, parametric_param_get_n_control_points,
    parametric_param_get_nth_control_point,
);
openfx_internal_macros::low_make_suite_struct!(
    ProgressSuiteV1 : OfxProgressSuiteV1 : progress_start, progress_update, progress_end,
);
openfx_internal_macros::low_make_suite_struct!(
    ProgressSuiteV2 : OfxProgressSuiteV2 : progress_start, progress_end, progress_update,
);
openfx_internal_macros::low_make_suite_struct!(
    PropertySuiteV1 : OfxPropertySuiteV1 : prop_get_dimension, prop_get_string,
    prop_get_double_n, prop_get_double, prop_get_string_n, prop_get_int, prop_set_double,
    prop_set_string_n, prop_get_int_n, prop_set_pointer, prop_set_int_n, prop_reset,
    prop_get_pointer, prop_set_int, prop_set_string, prop_set_pointer_n,
    prop_get_pointer_n, prop_set_double_n,
);
openfx_internal_macros::low_make_suite_struct!(
    TimeLineSuiteV1 : OfxTimeLineSuiteV1 : goto_time, get_time_bounds, get_time,
);

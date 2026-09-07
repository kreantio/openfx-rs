openfx_internal_macros::low_make_suite_struct!(
    DialogSuiteV1 : OfxDialogSuiteV1 : notify_redraw_pending, request_dialog,
);
openfx_internal_macros::low_make_suite_struct!(
    DrawSuiteV1 : OfxDrawSuiteV1 : draw, draw_text, get_colour, set_colour,
    set_line_stipple, set_line_width,
);
openfx_internal_macros::low_make_suite_struct!(
    ImageEffectOpenGLRenderSuiteV1 : OfxImageEffectOpenGLRenderSuiteV1 :
    clip_free_texture, clip_load_texture, flush_resources,
);
openfx_internal_macros::low_make_suite_struct!(
    ImageEffectSuiteV1 : OfxImageEffectSuiteV1 : abort, clip_define, clip_get_handle,
    clip_get_image, clip_get_property_set, clip_get_region_of_definition,
    clip_release_image, get_param_set, get_property_set, image_memory_alloc,
    image_memory_free, image_memory_lock, image_memory_unlock,
);
openfx_internal_macros::low_make_suite_struct!(
    InteractSuiteV1 : OfxInteractSuiteV1 : interact_get_property_set, interact_redraw,
    interact_swap_buffers,
);
openfx_internal_macros::low_make_suite_struct!(
    MemorySuiteV1 : OfxMemorySuiteV1 : memory_alloc, memory_free,
);
openfx_internal_macros::low_make_suite_struct!(
    MessageSuiteV1 : OfxMessageSuiteV1 : message,
);
openfx_internal_macros::low_make_suite_struct!(
    MessageSuiteV2 : OfxMessageSuiteV2 : clear_persistent_message, message,
    set_persistent_message,
);
openfx_internal_macros::low_make_suite_struct!(
    MultiThreadSuiteV1 : OfxMultiThreadSuiteV1 : multi_thread, multi_thread_index,
    multi_thread_is_spawned_thread, multi_thread_num_cp_us, mutex_create, mutex_destroy,
    mutex_lock, mutex_try_lock, mutex_un_lock,
);
openfx_internal_macros::low_make_suite_struct!(
    OpenCLProgramSuiteV1 : OfxOpenCLProgramSuiteV1 : compile_program,
);
openfx_internal_macros::low_make_suite_struct!(
    ParameterSuiteV1 : OfxParameterSuiteV1 : param_copy, param_define,
    param_delete_all_keys, param_delete_key, param_edit_begin, param_edit_end,
    param_get_handle, param_get_key_index, param_get_key_time, param_get_num_keys,
    param_get_property_set, param_set_get_property_set,
);
openfx_internal_macros::low_make_suite_struct!(
    ParametricParameterSuiteV1 : OfxParametricParameterSuiteV1 :
    parametric_param_add_control_point, parametric_param_delete_all_control_points,
    parametric_param_delete_control_point, parametric_param_get_n_control_points,
    parametric_param_get_nth_control_point, parametric_param_get_value,
    parametric_param_set_nth_control_point,
);
openfx_internal_macros::low_make_suite_struct!(
    ProgressSuiteV1 : OfxProgressSuiteV1 : progress_end, progress_start, progress_update,
);
openfx_internal_macros::low_make_suite_struct!(
    ProgressSuiteV2 : OfxProgressSuiteV2 : progress_end, progress_start, progress_update,
);
openfx_internal_macros::low_make_suite_struct!(
    PropertySuiteV1 : OfxPropertySuiteV1 : prop_get_dimension, prop_get_double,
    prop_get_double_n, prop_get_int, prop_get_int_n, prop_get_pointer,
    prop_get_pointer_n, prop_get_string, prop_get_string_n, prop_reset, prop_set_double,
    prop_set_double_n, prop_set_int, prop_set_int_n, prop_set_pointer,
    prop_set_pointer_n, prop_set_string, prop_set_string_n,
);
openfx_internal_macros::low_make_suite_struct!(
    TimeLineSuiteV1 : OfxTimeLineSuiteV1 : get_time, get_time_bounds, goto_time,
);

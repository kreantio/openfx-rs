openfx_internal_macros::low_make_object_struct!(
    DrawContext : OfxDrawContextHandle : draw, draw_text, get_colour, set_colour,
    set_line_stipple, set_line_width,
);
openfx_internal_macros::low_make_object_struct!(
    ImageClipDescriptor : OfxImageClipHandle : clip_get_property_set,
);
openfx_internal_macros::low_make_object_struct!(
    ImageClipInstance : OfxImageClipHandle : clip_get_image, clip_get_property_set,
    clip_get_region_of_definition, clip_load_texture,
);
openfx_internal_macros::low_make_object_struct!(
    ImageEffectDescriptor : OfxImageEffectHandle : clip_define, get_param_set,
    get_property_set,
);
openfx_internal_macros::low_make_object_struct!(
    ImageEffectInstance : OfxImageEffectHandle : abort, clip_get_handle, get_param_set,
    get_property_set, image_memory_alloc,
);
openfx_internal_macros::low_make_object_struct!(
    ImageMemory : OfxImageMemoryHandle : image_memory_free, image_memory_lock,
    image_memory_unlock,
);
openfx_internal_macros::low_make_object_struct!(
    InteractDescriptor : OfxInteractHandle : interact_get_property_set,
);
openfx_internal_macros::low_make_object_struct!(
    InteractInstance : OfxInteractHandle : interact_get_property_set, interact_redraw,
    interact_swap_buffers,
);
openfx_internal_macros::low_make_object_struct!(
    Mutex : OfxMutexHandle : mutex_destroy, mutex_lock, mutex_try_lock, mutex_un_lock,
);
openfx_internal_macros::low_make_object_struct!(
    ParamSetDescriptor : OfxParamSetHandle : param_define, param_set_get_property_set,
);
openfx_internal_macros::low_make_object_struct!(
    ParamSetInstance : OfxParamSetHandle : param_edit_begin, param_edit_end,
    param_get_handle, param_set_get_property_set,
);

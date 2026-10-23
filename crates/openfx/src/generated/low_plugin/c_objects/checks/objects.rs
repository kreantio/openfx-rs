const _: () = {
    use crate::low_plugin::objects::DrawContext;
    let _ = DrawContext::draw;
    let _ = DrawContext::draw_text;
    let _ = DrawContext::get_colour;
    let _ = DrawContext::set_colour;
    let _ = DrawContext::set_line_stipple;
    let _ = DrawContext::set_line_width;
};
const _: () = {
    use crate::low_plugin::objects::ImageClipDescriptor;
    let _ = ImageClipDescriptor::clip_get_property_set;
};
const _: () = {
    use crate::low_plugin::objects::ImageClipInstance;
    let _ = ImageClipInstance::clip_get_image;
    let _ = ImageClipInstance::clip_get_property_set;
    let _ = ImageClipInstance::clip_get_region_of_definition;
    let _ = ImageClipInstance::clip_load_texture;
};
const _: () = {
    use crate::low_plugin::objects::ImageEffectDescriptor;
    let _ = ImageEffectDescriptor::clip_define;
    let _ = ImageEffectDescriptor::get_param_set;
    let _ = ImageEffectDescriptor::get_property_set;
};
const _: () = {
    use crate::low_plugin::objects::ImageEffectInstance;
    let _ = ImageEffectInstance::abort;
    let _ = ImageEffectInstance::clip_get_handle;
    let _ = ImageEffectInstance::get_param_set;
    let _ = ImageEffectInstance::get_property_set;
    let _ = ImageEffectInstance::image_memory_alloc;
};
const _: () = {
    use crate::low_plugin::objects::ImageMemory;
    let _ = ImageMemory::image_memory_free;
    let _ = ImageMemory::image_memory_lock;
    let _ = ImageMemory::image_memory_unlock;
};
const _: () = {
    use crate::low_plugin::objects::InteractDescriptor;
    let _ = InteractDescriptor::interact_get_property_set;
};
const _: () = {
    use crate::low_plugin::objects::InteractInstance;
    let _ = InteractInstance::interact_get_property_set;
    let _ = InteractInstance::interact_redraw;
    let _ = InteractInstance::interact_swap_buffers;
};
const _: () = {
    use crate::low_plugin::objects::Mutex;
    let _ = Mutex::mutex_destroy;
    let _ = Mutex::mutex_lock;
    let _ = Mutex::mutex_try_lock;
    let _ = Mutex::mutex_un_lock;
};
const _: () = {
    use crate::low_plugin::objects::ParamSetDescriptor;
    let _ = ParamSetDescriptor::param_define;
    let _ = ParamSetDescriptor::param_set_get_property_set;
};
const _: () = {
    use crate::low_plugin::objects::ParamSetInstance;
    let _ = ParamSetInstance::param_edit_begin;
    let _ = ParamSetInstance::param_edit_end;
    let _ = ParamSetInstance::param_get_handle;
    let _ = ParamSetInstance::param_set_get_property_set;
};

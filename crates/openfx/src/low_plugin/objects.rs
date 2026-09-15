//! This module contains structs that wrap object handles, excluding
//! property sets and parameters.

use std::ffi::{CStr, c_void};

use crate::{
    low::enums::{DrawLineStipplePattern, DrawPrimitive, DrawTextAlignment, StandardColour},
    low_plugin::{
        property_sets::{
            ImageClipDescriptorPropertySet, ImageClipInstancePropertySet,
            ImageEffectDescriptorPropertySet, ImageEffectInstancePropertySet,
            ImageInstancePropertySet, InteractDescriptorPropertySet, InteractInstancePropertySet,
            ParamSetPropertySet,
        },
        suites::{
            DrawSuiteV1, ImageEffectOpenGLRenderSuiteV1, ImageEffectSuiteV1, InteractSuiteV1,
            MultiThreadSuiteV1, ParameterSuiteV1,
        },
    },
    sys_umbrella::{
        OfxParamHandle, OfxPointD, OfxPropertySetHandle, OfxRGBAColourF, OfxRectD, OfxTime,
    },
};

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/generated/code_from_c/low_objects_plugin.rs",
));

impl DrawContext {
    /// ## SAFETY
    ///
    /// `self` and `draw_suite` must be valid.
    pub unsafe fn draw(
        &self,
        draw_suite: &DrawSuiteV1,
        primitive: DrawPrimitive,
        points: &[OfxPointD],
    ) -> crate::low::Result<()> {
        unsafe { draw_suite.draw(self, primitive, points) }
    }

    /// ## SAFETY
    ///
    /// `self` and `draw_suite` must be valid.
    pub unsafe fn draw_text(
        &self,
        draw_suite: &DrawSuiteV1,
        text: &CStr,
        pos: &OfxPointD,
        alignment: DrawTextAlignment,
    ) -> crate::low::Result<()> {
        unsafe { draw_suite.draw_text(self, text, pos, alignment) }
    }

    /// ## SAFETY
    ///
    /// `self` and `draw_suite` must be valid.
    pub unsafe fn get_colour(
        &self,
        draw_suite: &DrawSuiteV1,
        std_colour: StandardColour,
    ) -> crate::low::Result<OfxRGBAColourF> {
        unsafe { draw_suite.get_colour(self, std_colour) }
    }

    /// ## SAFETY
    ///
    /// `self` and `draw_suite` must be valid.
    pub unsafe fn set_colour(
        &self,
        draw_suite: &DrawSuiteV1,
        colour: OfxRGBAColourF,
    ) -> crate::low::Result<()> {
        unsafe { draw_suite.set_colour(self, colour) }
    }

    /// ## SAFETY
    ///
    /// `self` and `draw_suite` must be valid.
    pub unsafe fn set_line_stipple(
        &self,
        draw_suite: &DrawSuiteV1,
        pattern: DrawLineStipplePattern,
    ) -> crate::low::Result<()> {
        unsafe { draw_suite.set_line_stipple(self, pattern) }
    }

    /// ## SAFETY
    ///
    /// `self` and `draw_suite` must be valid.
    pub unsafe fn set_line_width(
        &self,
        draw_suite: &DrawSuiteV1,
        width: f32,
    ) -> crate::low::Result<()> {
        unsafe { draw_suite.set_line_width(self, width) }
    }
}

impl ImageClipDescriptor {
    /// ## SAFETY
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn clip_get_property_set(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
    ) -> crate::low::Result<ImageClipDescriptorPropertySet> {
        unsafe { image_effect_suite.clip_get_descriptor_property_set(self) }
    }
}

impl ImageClipInstance {
    /// ## SAFETY
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn clip_get_image(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
        time: OfxTime,
        region: Option<&OfxRectD>,
    ) -> crate::low::Result<ImageInstancePropertySet> {
        unsafe { image_effect_suite.clip_get_image(self, time, region) }
    }

    /// ## SAFETY
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn clip_get_property_set(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
    ) -> crate::low::Result<ImageClipInstancePropertySet> {
        unsafe { image_effect_suite.clip_get_instance_property_set(self) }
    }

    /// ## SAFETY
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn clip_get_region_of_definition(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
        time: OfxTime,
    ) -> crate::low::Result<OfxRectD> {
        unsafe { image_effect_suite.clip_get_region_of_definition(self, time) }
    }

    /// ## SAFETY
    ///
    /// `self` and `image_effect_open_gl_render_suite` must be valid.
    /// ## TODO
    ///
    /// See [ImageEffectOpenGLRenderSuiteV1::clip_free_texture].
    pub unsafe fn clip_load_texture(
        &self,
        image_effect_open_gl_render_suite: &ImageEffectOpenGLRenderSuiteV1,
        time: OfxTime,
        format: &CStr,
        region: &OfxRectD,
    ) -> crate::low::Result<OfxPropertySetHandle> {
        unsafe { image_effect_open_gl_render_suite.clip_load_texture(self, time, format, region) }
    }
}

impl ImageEffectDescriptor {
    /// ## SAFETY
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn clip_define(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
        name: &CStr,
    ) -> crate::low::Result<ImageClipDescriptorPropertySet> {
        unsafe { image_effect_suite.clip_define(self, name) }
    }

    /// ## SAFETY
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn get_param_set(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
    ) -> crate::low::Result<ParamSetDescriptor> {
        unsafe { image_effect_suite.get_param_set_descriptor(self) }
    }

    /// ## SAFETY
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn get_property_set(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
    ) -> crate::low::Result<ImageEffectDescriptorPropertySet> {
        unsafe { image_effect_suite.get_descriptor_property_set(self) }
    }
}

impl ImageEffectInstance {
    /// ## SAFETY
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn abort(&self, image_effect_suite: &ImageEffectSuiteV1) -> bool {
        unsafe { image_effect_suite.abort(self) }
    }

    /// ## SAFETY
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn clip_get_handle(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
        name: &CStr,
    ) -> crate::low::Result<(ImageClipInstance, ImageClipInstancePropertySet)> {
        unsafe { image_effect_suite.clip_get_handle(self, name) }
    }

    /// ## SAFETY
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn clip_get_clip_handle(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
        name: &CStr,
    ) -> crate::low::Result<ImageClipInstance> {
        unsafe { image_effect_suite.clip_get_clip_handle(self, name) }
    }

    /// ## SAFETY
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn get_param_set(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
    ) -> crate::low::Result<ParamSetInstance> {
        unsafe { image_effect_suite.get_param_set_instance(self) }
    }

    /// ## SAFETY
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn get_property_set(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
    ) -> crate::low::Result<ImageEffectInstancePropertySet> {
        unsafe { image_effect_suite.get_instance_property_set(self) }
    }

    /// ## SAFETY
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn image_memory_alloc(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
        n_bytes: usize,
    ) -> crate::low::Result<ImageMemory> {
        unsafe { image_effect_suite.image_memory_alloc(self, n_bytes) }
    }
}

impl ImageMemory {
    /// ## SAFETY
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn image_memory_free(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
    ) -> crate::low::Result<()> {
        unsafe { image_effect_suite.image_memory_free(self) }
    }

    /// ## SAFETY
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn image_memory_lock(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
    ) -> crate::low::Result<*mut c_void> {
        unsafe { image_effect_suite.image_memory_lock(self) }
    }

    /// ## SAFETY
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn image_memory_unlock(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
    ) -> crate::low::Result<()> {
        unsafe { image_effect_suite.image_memory_unlock(self) }
    }
}

impl InteractDescriptor {
    /// ## SAFETY
    ///
    /// `self` and `interact_suite` must be valid.
    pub unsafe fn interact_get_property_set(
        &self,
        interact_suite: &InteractSuiteV1,
    ) -> crate::low::Result<InteractDescriptorPropertySet> {
        unsafe { interact_suite.interacte_get_descriptor_property_set(self) }
    }
}

impl InteractInstance {
    /// ## SAFETY
    ///
    /// `self` and `interact_suite` must be valid.
    pub unsafe fn interact_get_property_set(
        &self,
        interact_suite: &InteractSuiteV1,
    ) -> crate::low::Result<InteractInstancePropertySet> {
        unsafe { interact_suite.interacte_get_instance_property_set(self) }
    }

    /// ## SAFETY
    ///
    /// `self` and `interact_suite` must be valid.
    pub unsafe fn interact_redraw(
        &self,
        interact_suite: &InteractSuiteV1,
    ) -> crate::low::Result<()> {
        unsafe { interact_suite.interact_redraw(self) }
    }

    /// ## SAFETY
    ///
    /// `self` and `interact_suite` must be valid.
    pub unsafe fn interact_swap_buffers(
        &self,
        interact_suite: &InteractSuiteV1,
    ) -> crate::low::Result<()> {
        unsafe { interact_suite.interact_swap_buffers(self) }
    }
}

impl Mutex {
    /// ## SAFETY
    ///
    /// `self` and `multi_thread_suite` must be valid.
    pub unsafe fn mutex_destroy(
        &self,
        multi_thread_suite: &MultiThreadSuiteV1,
    ) -> crate::low::Result<()> {
        unsafe { multi_thread_suite.mutex_destroy(self) }
    }

    /// ## SAFETY
    ///
    /// `self` and `multi_thread_suite` must be valid.
    pub unsafe fn mutex_lock(
        &self,
        multi_thread_suite: &MultiThreadSuiteV1,
    ) -> crate::low::Result<()> {
        unsafe { multi_thread_suite.mutex_lock(self) }
    }

    /// ## SAFETY
    ///
    /// `self` and `multi_thread_suite` must be valid.
    pub unsafe fn mutex_try_lock(
        &self,
        multi_thread_suite: &MultiThreadSuiteV1,
    ) -> crate::low::Result<()> {
        unsafe { multi_thread_suite.mutex_try_lock(self) }
    }

    /// ## SAFETY
    ///
    /// `self` and `multi_thread_suite` must be valid.
    pub unsafe fn mutex_un_lock(
        &self,
        multi_thread_suite: &MultiThreadSuiteV1,
    ) -> crate::low::Result<()> {
        unsafe { multi_thread_suite.mutex_unlock(self) }
    }
}

impl ParamSetDescriptor {
    /// ## SAFETY
    ///
    /// `self` and `parameter_suite` must be valid.
    pub(crate) unsafe fn param_define(
        &self,
        parameter_suite: &ParameterSuiteV1,
        param_type: &CStr,
        name: &CStr,
    ) -> crate::low::Result<OfxPropertySetHandle> {
        unsafe { parameter_suite.param_define(*self, param_type, name) }
    }

    /// ## SAFETY
    ///
    /// `self` and `parameter_suite` must be valid.
    pub unsafe fn param_set_get_property_set(
        &self,
        parameter_suite: &ParameterSuiteV1,
    ) -> crate::low::Result<ParamSetPropertySet> {
        unsafe { parameter_suite.param_set_get_descriptor_property_set(*self) }
    }
}

impl ParamSetInstance {
    /// ## SAFETY
    ///
    /// `self` and `parameter_suite` must be valid.
    pub unsafe fn param_edit_begin(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> crate::low::Result<()> {
        unsafe { parameter_suite.param_edit_begin(*self, name) }
    }

    /// ## SAFETY
    ///
    /// `self` and `parameter_suite` must be valid.
    pub unsafe fn param_edit_end(
        &self,
        parameter_suite: &ParameterSuiteV1,
    ) -> crate::low::Result<()> {
        unsafe { parameter_suite.param_edit_end(*self) }
    }

    /// ## SAFETY
    ///
    /// `self` and `parameter_suite` must be valid.
    pub(crate) unsafe fn param_get_handle(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> crate::low::Result<(OfxParamHandle, OfxPropertySetHandle)> {
        unsafe { parameter_suite.param_get_handle(*self, name) }
    }

    #[expect(unused)]
    /// ## SAFETY
    ///
    /// `self` and `parameter_suite` must be valid.
    pub(crate) unsafe fn param_get_param_handle(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> crate::low::Result<OfxParamHandle> {
        unsafe { parameter_suite.param_get_param_handle(*self, name) }
    }

    /// ## SAFETY
    ///
    /// `self` and `parameter_suite` must be valid.
    pub unsafe fn param_set_get_property_set(
        &self,
        parameter_suite: &ParameterSuiteV1,
    ) -> crate::low::Result<ParamSetPropertySet> {
        unsafe { parameter_suite.param_set_get_instance_property_set(*self) }
    }
}

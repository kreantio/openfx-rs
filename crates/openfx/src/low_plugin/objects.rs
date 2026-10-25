//! This module contains structs that wrap object handles, excluding
//! property sets and parameters.

use std::ffi::{CStr, c_void};

use crate::{
    low::{
        InboundResult2,
        enums::{DrawLineStipplePattern, DrawPrimitive, DrawTextAlignment, StandardColour},
    },
    low_plugin::{
        property_sets::{
            ImageClipDescriptorPropertySet, ImageClipInstancePropertySet,
            ImageEffectDescriptorPropertySet, ImageEffectInstancePropertySet,
            ImageInstancePropertySet, InteractDescriptorPropertySet, InteractInstancePropertySet,
            OpenGLTexturePropertySet, ParamSetPropertySet,
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

pub use crate::generated::low_plugin::c_objects::objects::*;

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/generated/low_plugin/c_objects/checks/objects.rs",
));

impl DrawContext {
    /// ## Safety
    ///
    /// `self` and `draw_suite` must be valid.
    pub unsafe fn draw(
        &self,
        draw_suite: &DrawSuiteV1,
        primitive: DrawPrimitive,
        points: &[OfxPointD],
    ) -> InboundResult2<()> {
        unsafe { draw_suite.draw(self, primitive, points) }
    }

    /// ## Safety
    ///
    /// `self` and `draw_suite` must be valid.
    pub unsafe fn draw_text(
        &self,
        draw_suite: &DrawSuiteV1,
        text: &CStr,
        pos: &OfxPointD,
        alignment: DrawTextAlignment,
    ) -> InboundResult2<()> {
        unsafe { draw_suite.draw_text(self, text, pos, alignment) }
    }

    /// ## Safety
    ///
    /// `self` and `draw_suite` must be valid.
    pub unsafe fn get_colour(
        &self,
        draw_suite: &DrawSuiteV1,
        std_colour: StandardColour,
    ) -> InboundResult2<OfxRGBAColourF> {
        unsafe { draw_suite.get_colour(self, std_colour) }
    }

    /// ## Safety
    ///
    /// `self` and `draw_suite` must be valid.
    pub unsafe fn set_colour(
        &self,
        draw_suite: &DrawSuiteV1,
        colour: OfxRGBAColourF,
    ) -> InboundResult2<()> {
        unsafe { draw_suite.set_colour(self, colour) }
    }

    /// ## Safety
    ///
    /// `self` and `draw_suite` must be valid.
    pub unsafe fn set_line_stipple(
        &self,
        draw_suite: &DrawSuiteV1,
        pattern: DrawLineStipplePattern,
    ) -> InboundResult2<()> {
        unsafe { draw_suite.set_line_stipple(self, pattern) }
    }

    /// ## Safety
    ///
    /// `self` and `draw_suite` must be valid.
    pub unsafe fn set_line_width(
        &self,
        draw_suite: &DrawSuiteV1,
        width: f32,
    ) -> InboundResult2<()> {
        unsafe { draw_suite.set_line_width(self, width) }
    }
}

impl ImageClipDescriptor {
    /// ## Safety
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn clip_get_property_set(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
    ) -> InboundResult2<ImageClipDescriptorPropertySet> {
        unsafe { image_effect_suite.clip_get_descriptor_property_set(self) }
    }
}

impl ImageClipInstance {
    /// ## Safety
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn clip_get_image(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
        time: OfxTime,
        region: Option<&OfxRectD>,
    ) -> InboundResult2<ImageInstancePropertySet> {
        unsafe { image_effect_suite.clip_get_image(self, time, region) }
    }

    /// ## Safety
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn clip_get_property_set(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
    ) -> InboundResult2<ImageClipInstancePropertySet> {
        unsafe { image_effect_suite.clip_get_instance_property_set(self) }
    }

    /// ## Safety
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn clip_get_region_of_definition(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
        time: OfxTime,
    ) -> InboundResult2<OfxRectD> {
        unsafe { image_effect_suite.clip_get_region_of_definition(self, time) }
    }

    /// ## Safety
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
    ) -> InboundResult2<OpenGLTexturePropertySet> {
        unsafe { image_effect_open_gl_render_suite.clip_load_texture(self, time, format, region) }
    }
}

impl ImageEffectDescriptor {
    /// ## Safety
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn clip_define(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
        name: &CStr,
    ) -> InboundResult2<ImageClipDescriptorPropertySet> {
        unsafe { image_effect_suite.clip_define(self, name) }
    }

    /// ## Safety
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn get_param_set(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
    ) -> InboundResult2<ParamSetDescriptor> {
        unsafe { image_effect_suite.get_param_set_descriptor(self) }
    }

    /// ## Safety
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn get_property_set(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
    ) -> InboundResult2<ImageEffectDescriptorPropertySet> {
        unsafe { image_effect_suite.get_descriptor_property_set(self) }
    }
}

impl ImageEffectInstance {
    /// ## Safety
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn abort(&self, image_effect_suite: &ImageEffectSuiteV1) -> bool {
        unsafe { image_effect_suite.abort(self) }
    }

    /// ## Safety
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn clip_get_handle(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
        name: &CStr,
    ) -> InboundResult2<(ImageClipInstance, ImageClipInstancePropertySet)> {
        unsafe { image_effect_suite.clip_get_handle(self, name) }
    }

    /// ## Safety
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn clip_get_clip_handle(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
        name: &CStr,
    ) -> InboundResult2<ImageClipInstance> {
        unsafe { image_effect_suite.clip_get_clip_handle(self, name) }
    }

    /// ## Safety
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn get_param_set(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
    ) -> InboundResult2<ParamSetInstance> {
        unsafe { image_effect_suite.get_param_set_instance(self) }
    }

    /// ## Safety
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn get_property_set(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
    ) -> InboundResult2<ImageEffectInstancePropertySet> {
        unsafe { image_effect_suite.get_instance_property_set(self) }
    }

    /// ## Safety
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn image_memory_alloc(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
        n_bytes: usize,
    ) -> InboundResult2<ImageMemory> {
        unsafe { image_effect_suite.image_memory_alloc(self, n_bytes) }
    }
}

impl ImageMemory {
    /// ## Safety
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn image_memory_free(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
    ) -> InboundResult2<()> {
        unsafe { image_effect_suite.image_memory_free(self) }
    }

    /// ## Safety
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn image_memory_lock(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
    ) -> InboundResult2<*mut c_void> {
        unsafe { image_effect_suite.image_memory_lock(self) }
    }

    /// ## Safety
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn image_memory_unlock(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
    ) -> InboundResult2<()> {
        unsafe { image_effect_suite.image_memory_unlock(self) }
    }
}

impl InteractDescriptor {
    /// ## Safety
    ///
    /// `self` and `interact_suite` must be valid.
    pub unsafe fn interact_get_property_set(
        &self,
        interact_suite: &InteractSuiteV1,
    ) -> InboundResult2<InteractDescriptorPropertySet> {
        unsafe { interact_suite.interacte_get_descriptor_property_set(self) }
    }
}

impl InteractInstance {
    /// ## Safety
    ///
    /// `self` and `interact_suite` must be valid.
    pub unsafe fn interact_get_property_set(
        &self,
        interact_suite: &InteractSuiteV1,
    ) -> InboundResult2<InteractInstancePropertySet> {
        unsafe { interact_suite.interacte_get_instance_property_set(self) }
    }

    /// ## Safety
    ///
    /// `self` and `interact_suite` must be valid.
    pub unsafe fn interact_redraw(&self, interact_suite: &InteractSuiteV1) -> InboundResult2<()> {
        unsafe { interact_suite.interact_redraw(self) }
    }

    /// ## Safety
    ///
    /// `self` and `interact_suite` must be valid.
    pub unsafe fn interact_swap_buffers(
        &self,
        interact_suite: &InteractSuiteV1,
    ) -> InboundResult2<()> {
        unsafe { interact_suite.interact_swap_buffers(self) }
    }
}

impl Mutex {
    /// ## Safety
    ///
    /// `self` and `multi_thread_suite` must be valid.
    pub unsafe fn mutex_destroy(
        &self,
        multi_thread_suite: &MultiThreadSuiteV1,
    ) -> InboundResult2<()> {
        unsafe { multi_thread_suite.mutex_destroy(self) }
    }

    /// ## Safety
    ///
    /// `self` and `multi_thread_suite` must be valid.
    pub unsafe fn mutex_lock(&self, multi_thread_suite: &MultiThreadSuiteV1) -> InboundResult2<()> {
        unsafe { multi_thread_suite.mutex_lock(self) }
    }

    /// ## Safety
    ///
    /// `self` and `multi_thread_suite` must be valid.
    pub unsafe fn mutex_try_lock(
        &self,
        multi_thread_suite: &MultiThreadSuiteV1,
    ) -> InboundResult2<()> {
        unsafe { multi_thread_suite.mutex_try_lock(self) }
    }

    /// ## Safety
    ///
    /// `self` and `multi_thread_suite` must be valid.
    pub unsafe fn mutex_un_lock(
        &self,
        multi_thread_suite: &MultiThreadSuiteV1,
    ) -> InboundResult2<()> {
        unsafe { multi_thread_suite.mutex_unlock(self) }
    }
}

impl ParamSetDescriptor {
    /// It is recommended to use the typed versions of this function where
    /// applicable.
    ///
    /// TODO: Implement the typed versions of this function.
    ///
    /// ## Safety
    ///
    /// `self` and `parameter_suite` must be valid.
    pub unsafe fn param_define(
        &self,
        parameter_suite: &ParameterSuiteV1,
        param_type: &CStr,
        name: &CStr,
    ) -> InboundResult2<OfxPropertySetHandle> {
        unsafe { parameter_suite.param_define(self, param_type, name) }
    }

    /// It is recommended to use the typed versions of this function where
    /// applicable.
    ///
    /// TODO: Implement the typed versions of this function.
    ///
    /// ## Safety
    ///
    /// `self` and `parameter_suite` must be valid.
    pub unsafe fn param_define_no_output(
        &self,
        parameter_suite: &ParameterSuiteV1,
        param_type: &CStr,
        name: &CStr,
    ) -> InboundResult2<()> {
        unsafe { parameter_suite.param_define_no_output(self, param_type, name) }
    }

    /// ## Safety
    ///
    /// `self` and `parameter_suite` must be valid.
    pub unsafe fn param_set_get_property_set(
        &self,
        parameter_suite: &ParameterSuiteV1,
    ) -> InboundResult2<ParamSetPropertySet> {
        unsafe { parameter_suite.param_set_get_descriptor_property_set(self) }
    }
}

impl ParamSetInstance {
    /// ## Safety
    ///
    /// `self` and `parameter_suite` must be valid.
    pub unsafe fn param_edit_begin(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult2<()> {
        unsafe { parameter_suite.param_edit_begin(self, name) }
    }

    /// ## Safety
    ///
    /// `self` and `parameter_suite` must be valid.
    pub unsafe fn param_edit_end(&self, parameter_suite: &ParameterSuiteV1) -> InboundResult2<()> {
        unsafe { parameter_suite.param_edit_end(self) }
    }

    /// It is recommended to use the typed versions of this function where
    /// applicable.
    ///
    /// TODO: Implement the typed versions of this function.
    ///
    /// ## Safety
    ///
    /// `self` and `parameter_suite` must be valid.
    pub unsafe fn param_get_handle(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult2<(OfxParamHandle, OfxPropertySetHandle)> {
        unsafe { parameter_suite.param_get_handle(self, name) }
    }

    /// It is recommended to use the typed versions of this function where
    /// applicable.
    ///
    /// TODO: Implement the typed versions of this function.
    ///
    /// ## Safety
    ///
    /// `self` and `parameter_suite` must be valid.
    pub unsafe fn param_get_param_handle(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult2<OfxParamHandle> {
        unsafe { parameter_suite.param_get_param_handle(self, name) }
    }

    /// ## Safety
    ///
    /// `self` and `parameter_suite` must be valid.
    pub unsafe fn param_set_get_property_set(
        &self,
        parameter_suite: &ParameterSuiteV1,
    ) -> InboundResult2<ParamSetPropertySet> {
        unsafe { parameter_suite.param_set_get_instance_property_set(self) }
    }
}

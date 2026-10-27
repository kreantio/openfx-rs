//! This module contains structs that wrap object handles, excluding
//! property sets and parameters.

use std::ffi::{CStr, c_void};

use crate::{
    low::{
        InboundResult,
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
    /// See: [`crate::sys_umbrella::OfxDrawSuiteV1::draw`]
    ///
    /// ## Safety
    ///
    /// `self` and `draw_suite` must be valid.
    pub unsafe fn draw(
        &self,
        draw_suite: &DrawSuiteV1,
        primitive: DrawPrimitive,
        points: &[OfxPointD],
    ) -> InboundResult<()> {
        unsafe { draw_suite.draw(self, primitive, points) }
    }

    /// See: [`crate::sys_umbrella::OfxDrawSuiteV1::drawText`]
    ///
    /// ## Safety
    ///
    /// `self` and `draw_suite` must be valid.
    pub unsafe fn draw_text(
        &self,
        draw_suite: &DrawSuiteV1,
        text: &CStr,
        pos: &OfxPointD,
        alignment: DrawTextAlignment,
    ) -> InboundResult<()> {
        unsafe { draw_suite.draw_text(self, text, pos, alignment) }
    }

    /// See: [`crate::sys_umbrella::OfxDrawSuiteV1::getColour`]
    ///
    /// ## Safety
    ///
    /// `self` and `draw_suite` must be valid.
    pub unsafe fn get_colour(
        &self,
        draw_suite: &DrawSuiteV1,
        std_colour: StandardColour,
    ) -> InboundResult<OfxRGBAColourF> {
        unsafe { draw_suite.get_colour(self, std_colour) }
    }

    /// See: [`crate::sys_umbrella::OfxDrawSuiteV1::setColour`]
    ///
    /// ## Safety
    ///
    /// `self` and `draw_suite` must be valid.
    pub unsafe fn set_colour(
        &self,
        draw_suite: &DrawSuiteV1,
        colour: OfxRGBAColourF,
    ) -> InboundResult<()> {
        unsafe { draw_suite.set_colour(self, colour) }
    }

    /// See: [`crate::sys_umbrella::OfxDrawSuiteV1::setLineStipple`]
    ///
    /// ## Safety
    ///
    /// `self` and `draw_suite` must be valid.
    pub unsafe fn set_line_stipple(
        &self,
        draw_suite: &DrawSuiteV1,
        pattern: DrawLineStipplePattern,
    ) -> InboundResult<()> {
        unsafe { draw_suite.set_line_stipple(self, pattern) }
    }

    /// See: [`crate::sys_umbrella::OfxDrawSuiteV1::setLineWidth`]
    ///
    /// ## Safety
    ///
    /// `self` and `draw_suite` must be valid.
    pub unsafe fn set_line_width(&self, draw_suite: &DrawSuiteV1, width: f32) -> InboundResult<()> {
        unsafe { draw_suite.set_line_width(self, width) }
    }
}

impl ImageClipDescriptor {
    /// See: [`crate::sys_umbrella::OfxImageEffectSuiteV1::clipGetPropertySet`]
    ///
    /// ## Safety
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn clip_get_property_set(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
    ) -> InboundResult<ImageClipDescriptorPropertySet> {
        unsafe { image_effect_suite.clip_get_descriptor_property_set(self) }
    }
}

impl ImageClipInstance {
    /// See: [`crate::sys_umbrella::OfxImageEffectSuiteV1::clipGetImage`]
    ///
    /// ## Safety
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn clip_get_image(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
        time: OfxTime,
        region: Option<&OfxRectD>,
    ) -> InboundResult<ImageInstancePropertySet> {
        unsafe { image_effect_suite.clip_get_image(self, time, region) }
    }

    /// See: [`crate::sys_umbrella::OfxImageEffectSuiteV1::clipGetPropertySet`]
    ///
    /// ## Safety
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn clip_get_property_set(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
    ) -> InboundResult<ImageClipInstancePropertySet> {
        unsafe { image_effect_suite.clip_get_instance_property_set(self) }
    }

    /// See: [`crate::sys_umbrella::OfxImageEffectSuiteV1::clipGetRegionOfDefinition`]
    ///
    /// ## Safety
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn clip_get_region_of_definition(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
        time: OfxTime,
    ) -> InboundResult<OfxRectD> {
        unsafe { image_effect_suite.clip_get_region_of_definition(self, time) }
    }

    /// See: [`crate::sys_umbrella::OfxImageEffectOpenGLRenderSuiteV1::clipLoadTexture`]
    ///
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
    ) -> InboundResult<OpenGLTexturePropertySet> {
        unsafe { image_effect_open_gl_render_suite.clip_load_texture(self, time, format, region) }
    }
}

impl ImageEffectDescriptor {
    /// See: [`crate::sys_umbrella::OfxImageEffectSuiteV1::clipDefine`]
    ///
    /// ## Safety
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn clip_define(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
        name: &CStr,
    ) -> InboundResult<ImageClipDescriptorPropertySet> {
        unsafe { image_effect_suite.clip_define(self, name) }
    }

    /// See: [`crate::sys_umbrella::OfxImageEffectSuiteV1::getParamSet`]
    ///
    /// ## Safety
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn get_param_set(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
    ) -> InboundResult<ParamSetDescriptor> {
        unsafe { image_effect_suite.get_param_set_descriptor(self) }
    }

    /// See: [`crate::sys_umbrella::OfxImageEffectSuiteV1::getPropertySet`]
    ///
    /// ## Safety
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn get_property_set(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
    ) -> InboundResult<ImageEffectDescriptorPropertySet> {
        unsafe { image_effect_suite.get_descriptor_property_set(self) }
    }
}

impl ImageEffectInstance {
    /// See: [`crate::sys_umbrella::OfxImageEffectSuiteV1::abort`]
    ///
    /// ## Safety
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn abort(&self, image_effect_suite: &ImageEffectSuiteV1) -> bool {
        unsafe { image_effect_suite.abort(self) }
    }

    /// See: [`crate::sys_umbrella::OfxImageEffectSuiteV1::clipGetHandle`]
    ///
    /// ## Safety
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn clip_get_handle(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
        name: &CStr,
    ) -> InboundResult<(ImageClipInstance, ImageClipInstancePropertySet)> {
        unsafe { image_effect_suite.clip_get_handle(self, name) }
    }

    /// See: [`crate::sys_umbrella::OfxImageEffectSuiteV1::clipGetHandle`]
    ///
    /// ## Safety
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn clip_get_clip_handle(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
        name: &CStr,
    ) -> InboundResult<ImageClipInstance> {
        unsafe { image_effect_suite.clip_get_clip_handle(self, name) }
    }

    /// See: [`crate::sys_umbrella::OfxImageEffectSuiteV1::getParamSet`]
    ///
    /// ## Safety
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn get_param_set(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
    ) -> InboundResult<ParamSetInstance> {
        unsafe { image_effect_suite.get_param_set_instance(self) }
    }

    /// See: [`crate::sys_umbrella::OfxImageEffectSuiteV1::getPropertySet`]
    ///
    /// ## Safety
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn get_property_set(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
    ) -> InboundResult<ImageEffectInstancePropertySet> {
        unsafe { image_effect_suite.get_instance_property_set(self) }
    }

    /// See: [`crate::sys_umbrella::OfxImageEffectSuiteV1::imageMemoryAlloc`]
    ///
    /// ## Safety
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn image_memory_alloc(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
        n_bytes: usize,
    ) -> InboundResult<ImageMemory> {
        unsafe { image_effect_suite.image_memory_alloc(self, n_bytes) }
    }
}

impl ImageMemory {
    /// See: [`crate::sys_umbrella::OfxImageEffectSuiteV1::imageMemoryFree`]
    ///
    /// ## Safety
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn image_memory_free(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
    ) -> InboundResult<()> {
        unsafe { image_effect_suite.image_memory_free(self) }
    }

    /// See: [`crate::sys_umbrella::OfxImageEffectSuiteV1::imageMemoryLock`]
    ///
    /// ## Safety
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn image_memory_lock(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
    ) -> InboundResult<*mut c_void> {
        unsafe { image_effect_suite.image_memory_lock(self) }
    }

    /// See: [`crate::sys_umbrella::OfxImageEffectSuiteV1::imageMemoryUnlock`]
    ///
    /// ## Safety
    ///
    /// `self` and `image_effect_suite` must be valid.
    pub unsafe fn image_memory_unlock(
        &self,
        image_effect_suite: &ImageEffectSuiteV1,
    ) -> InboundResult<()> {
        unsafe { image_effect_suite.image_memory_unlock(self) }
    }
}

impl InteractDescriptor {
    /// See: [`crate::sys_umbrella::OfxInteractSuiteV1::interactGetPropertySet`]
    ///
    /// ## Safety
    ///
    /// `self` and `interact_suite` must be valid.
    pub unsafe fn interact_get_property_set(
        &self,
        interact_suite: &InteractSuiteV1,
    ) -> InboundResult<InteractDescriptorPropertySet> {
        unsafe { interact_suite.interacte_get_descriptor_property_set(self) }
    }
}

impl InteractInstance {
    /// See: [`crate::sys_umbrella::OfxInteractSuiteV1::interactGetPropertySet`]
    ///
    /// ## Safety
    ///
    /// `self` and `interact_suite` must be valid.
    pub unsafe fn interact_get_property_set(
        &self,
        interact_suite: &InteractSuiteV1,
    ) -> InboundResult<InteractInstancePropertySet> {
        unsafe { interact_suite.interacte_get_instance_property_set(self) }
    }

    /// See: [`crate::sys_umbrella::OfxInteractSuiteV1::interactRedraw`]
    ///
    /// ## Safety
    ///
    /// `self` and `interact_suite` must be valid.
    pub unsafe fn interact_redraw(&self, interact_suite: &InteractSuiteV1) -> InboundResult<()> {
        unsafe { interact_suite.interact_redraw(self) }
    }

    /// See: [`crate::sys_umbrella::OfxInteractSuiteV1::interactSwapBuffers`]
    ///
    /// ## Safety
    ///
    /// `self` and `interact_suite` must be valid.
    pub unsafe fn interact_swap_buffers(
        &self,
        interact_suite: &InteractSuiteV1,
    ) -> InboundResult<()> {
        unsafe { interact_suite.interact_swap_buffers(self) }
    }
}

impl Mutex {
    /// See: [`crate::sys_umbrella::OfxMultiThreadSuiteV1::mutexDestroy`]
    ///
    /// ## Safety
    ///
    /// `self` and `multi_thread_suite` must be valid.
    pub unsafe fn mutex_destroy(
        &self,
        multi_thread_suite: &MultiThreadSuiteV1,
    ) -> InboundResult<()> {
        unsafe { multi_thread_suite.mutex_destroy(self) }
    }

    /// See: [`crate::sys_umbrella::OfxMultiThreadSuiteV1::mutexLock`]
    ///
    /// ## Safety
    ///
    /// `self` and `multi_thread_suite` must be valid.
    pub unsafe fn mutex_lock(&self, multi_thread_suite: &MultiThreadSuiteV1) -> InboundResult<()> {
        unsafe { multi_thread_suite.mutex_lock(self) }
    }

    /// See: [`crate::sys_umbrella::OfxMultiThreadSuiteV1::mutexTryLock`]
    ///
    /// ## Safety
    ///
    /// `self` and `multi_thread_suite` must be valid.
    pub unsafe fn mutex_try_lock(
        &self,
        multi_thread_suite: &MultiThreadSuiteV1,
    ) -> InboundResult<()> {
        unsafe { multi_thread_suite.mutex_try_lock(self) }
    }

    /// See: [`crate::sys_umbrella::OfxMultiThreadSuiteV1::mutexUnLock`]
    ///
    /// ## Safety
    ///
    /// `self` and `multi_thread_suite` must be valid.
    pub unsafe fn mutex_un_lock(
        &self,
        multi_thread_suite: &MultiThreadSuiteV1,
    ) -> InboundResult<()> {
        unsafe { multi_thread_suite.mutex_unlock(self) }
    }
}

impl ParamSetDescriptor {
    /// See: [`crate::sys_umbrella::OfxParameterSuiteV1::paramDefine`]
    ///
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
    ) -> InboundResult<OfxPropertySetHandle> {
        unsafe { parameter_suite.param_define(self, param_type, name) }
    }

    /// See: [`crate::sys_umbrella::OfxParameterSuiteV1::paramDefine`]
    ///
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
    ) -> InboundResult<()> {
        unsafe { parameter_suite.param_define_no_output(self, param_type, name) }
    }

    /// See: [`crate::sys_umbrella::OfxParameterSuiteV1::paramSetGetPropertySet`]
    ///
    /// ## Safety
    ///
    /// `self` and `parameter_suite` must be valid.
    pub unsafe fn param_set_get_property_set(
        &self,
        parameter_suite: &ParameterSuiteV1,
    ) -> InboundResult<ParamSetPropertySet> {
        unsafe { parameter_suite.param_set_get_descriptor_property_set(self) }
    }
}

impl ParamSetInstance {
    /// See: [`crate::sys_umbrella::OfxParameterSuiteV1::paramEditBegin`]
    ///
    /// ## Safety
    ///
    /// `self` and `parameter_suite` must be valid.
    pub unsafe fn param_edit_begin(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<()> {
        unsafe { parameter_suite.param_edit_begin(self, name) }
    }

    /// See: [`crate::sys_umbrella::OfxParameterSuiteV1::paramEditEnd`]
    ///
    /// ## Safety
    ///
    /// `self` and `parameter_suite` must be valid.
    pub unsafe fn param_edit_end(&self, parameter_suite: &ParameterSuiteV1) -> InboundResult<()> {
        unsafe { parameter_suite.param_edit_end(self) }
    }

    /// See: [`crate::sys_umbrella::OfxParameterSuiteV1::paramGetHandle`]
    ///
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
    ) -> InboundResult<(OfxParamHandle, OfxPropertySetHandle)> {
        unsafe { parameter_suite.param_get_handle(self, name) }
    }

    /// See: [`crate::sys_umbrella::OfxParameterSuiteV1::paramGetHandle`]
    ///
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
    ) -> InboundResult<OfxParamHandle> {
        unsafe { parameter_suite.param_get_param_handle(self, name) }
    }

    /// See: [`crate::sys_umbrella::OfxParameterSuiteV1::paramSetGetPropertySet`]
    ///
    /// ## Safety
    ///
    /// `self` and `parameter_suite` must be valid.
    pub unsafe fn param_set_get_property_set(
        &self,
        parameter_suite: &ParameterSuiteV1,
    ) -> InboundResult<ParamSetPropertySet> {
        unsafe { parameter_suite.param_set_get_instance_property_set(self) }
    }
}

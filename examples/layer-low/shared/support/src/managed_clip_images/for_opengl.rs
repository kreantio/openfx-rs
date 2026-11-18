use openfx::{
    low::StatusError,
    low_plugin::{
        objects::ImageClipInstance, property_sets::OpenGLTexturePropertySet,
        suites::ImageEffectOpenGLRenderSuiteV1,
    },
    sys::core::OfxTime,
};

use crate::{guarantee_send::GuaranteeSend, plugin_data::PluginData};

pub struct ManagedClipImageForOpengl {
    opengl_suite: ImageEffectOpenGLRenderSuiteV1,
    props: OpenGLTexturePropertySet,

    texture_index: u32,
    texture_target: u32,
}
impl ManagedClipImageForOpengl {
    pub fn try_new_from_clip(
        data: &PluginData,
        opengl_suite: GuaranteeSend<ImageEffectOpenGLRenderSuiteV1>,
        clip: &ImageClipInstance,
        frame: OfxTime,
    ) -> Result<Self, StatusError> {
        let prop_set = unsafe { clip.clip_load_texture(&opengl_suite.0, frame, None, None) }
            .ok_or(StatusError::Failed)?;

        let ret = Self::try_new_from_property_set(data, &opengl_suite.0, prop_set);
        if !ret.is_ok() {
            unsafe { Self::release_image(&opengl_suite.0, &prop_set) };
        }
        ret
    }

    fn try_new_from_property_set(
        data: &PluginData,
        opengl_suite: &ImageEffectOpenGLRenderSuiteV1,
        props: OpenGLTexturePropertySet,
    ) -> Result<Self, StatusError> {
        let s_prop = &data.property_suite.0;

        let texture_index = unsafe { props.get_image_effect_open_gl_texture_index(s_prop) }
            .ok_or(StatusError::Failed)?;
        let texture_target = unsafe { props.get_image_effect_open_gl_texture_target(s_prop) }
            .ok_or(StatusError::Failed)?;

        Ok(Self {
            opengl_suite: *opengl_suite,
            props,
            texture_index: texture_index as u32,
            texture_target: texture_target as u32,
        })
    }

    /// ## TODO
    ///
    /// Figure out how to properly handle non-OK statuses.
    unsafe fn release_image(
        image_effect_suite: &ImageEffectOpenGLRenderSuiteV1,
        props: &OpenGLTexturePropertySet,
    ) {
        let _ = unsafe { image_effect_suite.clip_free_texture(props) };
    }

    pub fn texture_index(&self) -> u32 {
        self.texture_index
    }
    pub fn texture_target(&self) -> u32 {
        self.texture_target
    }
}
impl Drop for ManagedClipImageForOpengl {
    fn drop(&mut self) {
        unsafe { Self::release_image(&self.opengl_suite, &self.props) };
    }
}

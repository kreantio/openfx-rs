//! This example is a loose port of: https://github.com/AcademySoftwareFoundation/openfx/blob/be1dce87e6214696744a04887ceb55e1f4b742ad/Examples/OpenGL/opengl.cpp
//!
//! Copyright notice of the original code:
//! ```text
//! Copyright OpenFX and contributors to the OpenFX project.
//! SPDX-License-Identifier: BSD-3-Clause
//! ```
//!
//! Direct GPU processing using OpenGL.
//!
//! Divergences from the original example:
//! - The original example linked directly against the OpenGL library on macOS,
//!   whereas we use dynamic loading instead.
//!
//! Known issues:
//! - Currently, due to limitations of the maintainer(s), only macOS (15, Apple
//!   Silicon) is verified to work with this plugin.

use std::{
    ffi::{CStr, c_void},
    sync::{Arc, Mutex},
};

use openfx::{
    low::{
        OutboundResult, Status, StatusError, StatusReply,
        enums::{
            ImageEffectPropCPURenderSupported, ImageEffectPropOpenGLRenderSupported,
            ImageEffectPropSupportedComponents, ImageEffectPropSupportedContexts,
            ImageEffectPropSupportedPixelDepths, OpenGLPropPixelDepth, ParamPropDoubleType,
        },
    },
    low_plugin::{
        Host, Plugin,
        actions::image_effect::{ActionDescribeInContextIn, ActionRenderIn, ImageEffectAction},
        objects::{
            ImageClipInstance, ImageEffectDescriptor, ImageEffectInstance, ParamSetDescriptor,
        },
        parameters::ParamDouble,
        property_sets::OpenGLTexturePropertySet,
        suites::{ImageEffectOpenGLRenderSuiteV1, ParameterSuiteV1, PropertySuiteV1},
    },
    sys::{
        core::OfxTime,
        image_effect::{kOfxImageEffectOutputClipName, kOfxImageEffectSimpleSourceClipName},
    },
};

use crate::{
    definitions::{PLUGIN_OPENGL_IDENTIFIER, PLUGIN_OPENGL_LABEL, PLUGINS_GROUPING},
    helpers::{
        internal_utils::{rect_i_from_array, set_labels},
        shared_data::{GuaranteeSend, GuaranteeSendClipInstance, SharedData},
    },
};

mod gl;
mod gl_loader;

static HOST_BEFORE_ACTION_LOAD: Mutex<Option<GuaranteeSend<Host>>> = Mutex::new(None);
static GLOBAL_DATA: Mutex<Option<(SharedData, Arc<AdditionalSharedData>)>> = Mutex::new(None);

struct AdditionalSharedData {
    opengl_suite: GuaranteeSend<ImageEffectOpenGLRenderSuiteV1>,
}

struct MyInstanceData {
    // wrapper objects to the clips we deal with
    source_clip: GuaranteeSendClipInstance,
    output_clip: GuaranteeSendClipInstance,

    // wrapper objects to our parameters
    scale_param: ParamDouble,
    source_scale_param: ParamDouble,
}

fn global_data_lockless() -> Result<(SharedData, Arc<AdditionalSharedData>), StatusError> {
    let data = GLOBAL_DATA.lock().map_err(|_| StatusError::Failed)?;
    let data = data.as_ref().ok_or(StatusError::Failed)?;
    Ok((data.0.clone(), data.1.clone()))
}

pub struct PluginExampleOpenGL;
impl Plugin for PluginExampleOpenGL {
    const PLUGIN_IDENTIFIER: &'static CStr = PLUGIN_OPENGL_IDENTIFIER;
    const PLUGIN_VERSION_MAJOR: std::ffi::c_uint = 1;
    const PLUGIN_VERSION_MINOR: std::ffi::c_uint = 0;

    fn set_host(host: Host) {
        let mut lock = HOST_BEFORE_ACTION_LOAD
            .lock()
            .expect("Failed to lock HOST_BEFORE_ACTION_LOAD.");
        if lock.is_some() {
            panic!("HOST_BEFORE_ACTION_LOAD has already been set.");
        }
        lock.replace(GuaranteeSend(host));
    }

    fn main_entry(action: ImageEffectAction) -> OutboundResult {
        match action {
            ImageEffectAction::Load { .. } => action_load(),
            ImageEffectAction::Unload { .. } => action_unload(),
            ImageEffectAction::OpenGLContextAttached { handle, .. } => {
                action_opengl_context_attached(handle)
            }
            ImageEffectAction::Describe { handle, .. } => action_describe(handle),
            ImageEffectAction::DescribeInContext {
                handle, in_args, ..
            } => action_describe_in_context(handle, in_args),
            ImageEffectAction::CreateInstance { handle, .. } => action_create_instance(handle),
            ImageEffectAction::DestroyInstance { handle, .. } => action_destroy_instance(handle),
            ImageEffectAction::Render {
                handle, in_args, ..
            } => action_render(handle, in_args),
            _ => Ok(StatusReply::ReplyDefault.into()),
        }
    }
}

fn action_load() -> OutboundResult {
    let host = HOST_BEFORE_ACTION_LOAD
        .lock()
        .map_err(|_| StatusError::Failed)?
        .take()
        .ok_or(StatusError::Failed)?;

    let mut data = GLOBAL_DATA.lock().map_err(|_| StatusError::Failed)?;
    if data.is_some() {
        return Err(StatusError::Failed);
    }

    *data = Some({
        let additional = {
            let opengl_suite = unsafe { host.fetch_image_effect_open_gl_render_suite_v1() }
                .ok_or(StatusError::ErrMissingHostFeature)?;

            AdditionalSharedData {
                opengl_suite: GuaranteeSend(opengl_suite),
            }
        };

        let data = SharedData::try_new(host)?;

        (data, Arc::new(additional))
    });

    Ok(Status::OK)
}

fn action_unload() -> OutboundResult {
    let mut data = GLOBAL_DATA.lock().map_err(|_| StatusError::Failed)?;
    if data.take().is_none() {
        Err(StatusError::Failed)
    } else {
        Ok(Status::OK)
    }
}

fn action_opengl_context_attached(_handle: ImageEffectInstance) -> OutboundResult {
    gl_loader::load().map_err(|error| {
        eprintln!("Failed to initialize OpenGL: {error}");
        StatusError::Failed
    })?;

    let missing_fns = gl::missing_functions();
    if !missing_fns.is_empty() {
        eprintln!("Missing OpenGL functions: {:?}", missing_fns);
        return Err(StatusError::Failed);
    }

    Ok(Status::OK)
}

fn action_describe(descriptor: ImageEffectDescriptor) -> OutboundResult {
    let (data, _additional) = global_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;

    let props = unsafe { descriptor.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    unsafe {
        set_labels(&data.property_suite.0, &props, PLUGIN_OPENGL_LABEL)?;
        props
            .set_image_effect_plugin_grouping(s_prop, Some(PLUGINS_GROUPING))
            .ok_or(StatusError::Failed)?;

        {
            use ImageEffectPropSupportedContexts as C;
            props
                .set_image_effect_supported_contexts(s_prop, &[C::Filter.into()])
                .ok_or(StatusError::Failed)?;
        }

        props
            .set_image_effect_cpu_render_supported(
                s_prop,
                ImageEffectPropCPURenderSupported::False.into(),
            )
            .ok(); // `ErrUnknown` on Natron.
        props
            .set_image_effect_open_gl_render_supported(
                s_prop,
                ImageEffectPropOpenGLRenderSupported::True.into(),
            )
            .ok_or(StatusError::Failed)?;

        props
            .set_image_effect_supports_multiple_clip_depths(s_prop, true)
            .ok_or(StatusError::Failed)?;

        {
            use ImageEffectPropSupportedPixelDepths as D;
            props
                .set_image_effect_supported_pixel_depths(
                    s_prop,
                    &[D::Byte.into(), D::Short.into(), D::Float.into()],
                )
                .ok_or(StatusError::Failed)?;
        }
        {
            use OpenGLPropPixelDepth as D;
            props
                .set_open_gl_pixel_depth(
                    s_prop,
                    &[D::Byte.into(), D::Short.into(), D::Float.into()],
                )
                .ok_or(StatusError::Failed)?;
        }
    }

    Ok(Status::OK)
}

fn action_describe_in_context(
    descriptor: ImageEffectDescriptor,
    _in_args: ActionDescribeInContextIn,
) -> OutboundResult {
    let (data, _additional) = global_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;
    let s_param = &data.parameter_suite.0;

    for name in [
        kOfxImageEffectOutputClipName,
        kOfxImageEffectSimpleSourceClipName,
    ] {
        let props = unsafe { descriptor.clip_define(s_ifx, name) }.ok_or(StatusError::Failed)?;

        // set the component types we can handle on our output & our main input
        (unsafe {
            use ImageEffectPropSupportedComponents as C;
            props.set_image_effect_supported_components(s_prop, &[C::RGBA.into(), C::Alpha.into()])
        })
        .ok_or(StatusError::Failed)?;
    }

    fn define_param(
        s_prop: &PropertySuiteV1,
        s_param: &ParameterSuiteV1,
        effect_params: &ParamSetDescriptor,
        name: &CStr,
        label: &CStr,
        script_name: &CStr,
        hint: &CStr,
    ) -> Result<(), StatusError> {
        let props = unsafe { effect_params.param_define_double(s_param, name) }
            .ok_or(StatusError::Failed)?;

        unsafe {
            props
                .set_param_double_type(s_prop, ParamPropDoubleType::Scale.into())
                .ok_or(StatusError::Failed)?;
            props
                .set_param_default(s_prop, 1.0)
                .ok_or(StatusError::Failed)?;
            props
                .set_param_min(s_prop, 0.0)
                .ok_or(StatusError::Failed)?;
            props
                .set_param_display_min(s_prop, 0.0)
                .ok_or(StatusError::Failed)?;
            props
                .set_param_increment(s_prop, 0.01)
                .ok_or(StatusError::Failed)?;
            props
                .set_param_hint(s_prop, Some(hint))
                .ok_or(StatusError::Failed)?;
            props
                .set_param_script_name(s_prop, Some(script_name))
                .ok_or(StatusError::Failed)?;
            props
                .set_label(s_prop, Some(label))
                .ok_or(StatusError::Failed)?;
        }

        Ok(())
    }

    let param_set = unsafe { descriptor.get_param_set(s_ifx) }.ok_or(StatusError::Failed)?;

    define_param(
        s_prop,
        s_param,
        &param_set,
        c"scale",
        c"scale",
        c"scale",
        c"Scales the red rect",
    )?;
    define_param(
        s_prop,
        s_param,
        &param_set,
        c"source_scale",
        c"source_scale",
        c"source_scale",
        c"Scales the source image",
    )?;

    unsafe {
        let props = param_set
            .param_define_page(s_param, c"Main")
            .ok_or(StatusError::Failed)?;
        props
            .set_param_page_child(s_prop, &[Some(c"scale"), Some(c"source_scale")])
            .ok_or(StatusError::Failed)?;
    }

    Ok(Status::OK)
}

fn action_create_instance(instance: ImageEffectInstance) -> OutboundResult {
    let (data, _additional) = global_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;
    let s_param = &data.parameter_suite.0;

    let instance_props = unsafe { instance.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    let source_clip =
        unsafe { instance.clip_get_clip_handle(s_ifx, c"Source") }.ok_or(StatusError::Failed)?;
    let output_clip =
        unsafe { instance.clip_get_clip_handle(s_ifx, c"Output") }.ok_or(StatusError::Failed)?;

    let param_set = unsafe { instance.get_param_set(s_ifx) }.ok_or(StatusError::Failed)?;
    let scale_param = unsafe { param_set.param_get_param_handle_double(s_param, c"scale") }
        .ok_or(StatusError::Failed)?;
    let source_scale_param =
        unsafe { param_set.param_get_param_handle_double(s_param, c"source_scale") }
            .ok_or(StatusError::Failed)?;

    let my_data = MyInstanceData {
        source_clip: GuaranteeSendClipInstance(source_clip),
        output_clip: GuaranteeSendClipInstance(output_clip),
        scale_param,
        source_scale_param,
    };
    let my_data_ptr = Box::into_raw(Box::new(my_data)) as *mut c_void;

    // Set my private instance data.
    // SAFETY: the pointee is kept alive by `Box::into_raw` until it is
    // reclaimed with `Box::from_raw` in `action_destroy_instance`.
    if let r =
        unsafe { instance_props.set_instance_data(s_prop, std::ptr::NonNull::new(my_data_ptr)) }
        && !r.is_ok()
    {
        drop(unsafe { Box::from_raw(my_data_ptr as *mut MyInstanceData) });
        return Err(StatusError::Failed);
    }

    Ok(Status::OK)
}

fn action_destroy_instance(instance: ImageEffectInstance) -> OutboundResult {
    let (data, _additional) = global_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;

    let props = unsafe { instance.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    let Some(my_data_ptr) =
        unsafe { props.get_instance_data(s_prop) }.ok_or(StatusError::Failed)?
    else {
        return Err(StatusError::Failed);
    };

    drop(unsafe { Box::from_raw(my_data_ptr.as_ptr() as *mut MyInstanceData) });

    Ok(Status::OK)
}

fn action_render(instance: ImageEffectInstance, in_args: ActionRenderIn) -> OutboundResult {
    let (data, additional) = global_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;
    let s_param = &data.parameter_suite.0;

    let instance_props = unsafe { instance.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    // get the render window and the time from the `in_args`
    let time = unsafe { in_args.get_time(s_prop) }.ok_or(StatusError::Failed)?;
    let render_window =
        unsafe { in_args.get_image_effect_render_window(s_prop) }.ok_or(StatusError::Failed)?;
    let render_window = rect_i_from_array(&render_window);

    // For this test, we only process in OpenGL mode.
    if !unsafe { in_args.get_image_effect_open_gl_enabled(s_prop) }.ok_or(StatusError::Failed)? {
        return Err(StatusError::ErrImageFormat);
    }

    let Some(my_data_ptr) =
        unsafe { instance_props.get_instance_data(s_prop) }.ok_or(StatusError::Failed)?
    else {
        return Err(StatusError::Failed);
    };
    let my_data = unsafe { &*(my_data_ptr.as_ptr() as *const MyInstanceData) };

    let _output_img = unsafe {
        ImageManaged::try_new_from_clip(
            &data,
            additional.opengl_suite,
            &my_data.output_clip.0,
            time,
        )
    }?;
    let source_img = unsafe {
        ImageManaged::try_new_from_clip(
            &data,
            additional.opengl_suite,
            &my_data.source_clip.0,
            time,
        )
    }?;

    let scale = unsafe {
        my_data
            .scale_param
            .param_get_value_at_time(s_param, time)
            .ok_or(StatusError::Failed)
    }? as f32;
    let source_scale = unsafe {
        my_data
            .source_scale_param
            .param_get_value_at_time(s_param, time)
            .ok_or(StatusError::Failed)
    }? as f32;

    let w = (render_window.x2 - render_window.x1) as f32;
    let h = (render_window.y2 - render_window.y1) as f32;

    unsafe {
        gl::PushAttrib(gl::inner::ALL_ATTRIB_BITS);
        gl::Disable(gl::inner::BLEND);

        // Draw black into dest to start
        gl::Begin(gl::inner::QUADS);
        gl::Color4f(0.0, 0.0, 0.0, 1.0); // Set the colour to opaque black
        gl::Vertex2f(0.0, 0.0);
        gl::Vertex2f(0.0, h);
        gl::Vertex2f(w, h);
        gl::Vertex2f(w, 0.0);
        gl::End();

        // Copy source texture to output by drawing a big textured quad

        // set up texture (how much of this is needed?)
        gl::Enable(source_img.texture_target);
        gl::BindTexture(source_img.texture_target, source_img.texture_index);
        gl::TexParameteri(
            source_img.texture_target,
            gl::inner::TEXTURE_WRAP_S,
            gl::inner::REPEAT as i32,
        );
        gl::TexParameteri(
            source_img.texture_target,
            gl::inner::TEXTURE_WRAP_T,
            gl::inner::REPEAT as i32,
        );
        gl::TexParameteri(
            source_img.texture_target,
            gl::inner::TEXTURE_MAG_FILTER,
            gl::inner::LINEAR as i32,
        );
        gl::TexParameteri(
            source_img.texture_target,
            gl::inner::TEXTURE_MIN_FILTER,
            gl::inner::LINEAR as i32,
        );
        gl::TexEnvf(
            gl::inner::TEXTURE_ENV,
            gl::inner::TEXTURE_ENV_MODE,
            gl::inner::MODULATE as f32,
        );

        // textures are oriented with Y up (standard orientation)
        let tymin = 0.0f32;
        let tymax = 1.0f32;

        // now draw the textured quad containing the source
        //gl::Begin(gl::QUADS); // <- in the original example code
        gl::Color4f(1.0, 1.0, 1.0, 1.0);
        gl::Begin(gl::inner::QUADS);
        gl::TexCoord2f(0.0, tymin);
        gl::Vertex2f(0.0, 0.0);
        gl::TexCoord2f(1.0, tymin);
        gl::Vertex2f(w * source_scale, 0.0);
        gl::TexCoord2f(1.0, tymax);
        gl::Vertex2f(w * source_scale, h * source_scale);
        gl::TexCoord2f(0.0, tymax);
        gl::Vertex2f(0.0, h * source_scale);
        gl::End();

        gl::Disable(source_img.texture_target);

        // Now draw some stuff on top of it to show we really did something
        const WIDTH: f32 = 200.0;
        const HEIGHT: f32 = 100.0;
        gl::Begin(gl::inner::QUADS);
        gl::Color3f(1.0, 0.0, 0.0); //Set the colour to red
        gl::Vertex2f(10.0, 10.0);
        gl::Vertex2f(10.0, HEIGHT * scale);
        gl::Vertex2f(WIDTH * scale, HEIGHT * scale);
        gl::Vertex2f(WIDTH * scale, 10.0);
        gl::End();

        // done.
        gl::PopAttrib();
    }

    Ok(Status::OK)
}

struct ImageManaged {
    opengl_suite: ImageEffectOpenGLRenderSuiteV1,
    props: OpenGLTexturePropertySet,

    texture_index: u32,
    texture_target: u32,
}
impl ImageManaged {
    pub unsafe fn try_new_from_clip(
        shared_data: &SharedData,
        opengl_suite: GuaranteeSend<ImageEffectOpenGLRenderSuiteV1>,
        clip: &ImageClipInstance,
        frame: OfxTime,
    ) -> Result<Self, StatusError> {
        let prop_set = unsafe { clip.clip_load_texture(&opengl_suite.0, frame, None, None) }
            .ok_or(StatusError::Failed)?;

        let ret =
            unsafe { Self::try_new_from_property_set(shared_data, &opengl_suite.0, prop_set) };
        if !ret.is_ok() {
            unsafe { Self::release_image(&opengl_suite.0, &prop_set) };
        }
        ret
    }

    unsafe fn try_new_from_property_set(
        shared_data: &SharedData,
        opengl_suite: &ImageEffectOpenGLRenderSuiteV1,
        props: OpenGLTexturePropertySet,
    ) -> Result<Self, StatusError> {
        let s_prop = &shared_data.property_suite.0;

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
}
impl Drop for ImageManaged {
    fn drop(&mut self) {
        unsafe { Self::release_image(&self.opengl_suite, &self.props) };
    }
}

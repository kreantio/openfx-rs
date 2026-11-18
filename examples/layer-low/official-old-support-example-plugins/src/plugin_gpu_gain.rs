//! This example is a loose port of: https://github.com/AcademySoftwareFoundation/openfx/blob/db28d1a18c26874179a0cde1e261633c836681dd/Support/Plugins/GPUGain/GPUGain.cpp
//!
//! Copyright notice of the original code:
//! ```text
//! Copyright OpenFX and contributors to the OpenFX project.
//! SPDX-License-Identifier: BSD-3-Clause
//! ```
//!
//! ## Divergences from the original example
//!
//! ### Metal
//!
//! The mutex's lifespan in the Metal implementation is shorter than in the
//! original example. Here, it is released immediately after the pipeline state
//! is retrieved, rather than being held for the entire duration of the
//! `run_metal_kernel` function. The original implementation and the OpenCL port
//! work similarly in this regard.
//!
//! ### CUDA
//!
//! The original example uses `nvcc` at build time to compile CUDA kernels. This
//! port instead uses `nvrtc` to compile CUDA kernels at runtime. This way, you
//! can cross-compile the code containing the CUDA implementation on macOS for
//! other platforms.
//!
//! Please note that, unlike the other ports, this port's code is not directly
//! derived from the original example, and the person who wrote this
//! implementation (with AI assistance, but without agentic activities) is not
//! skilled in CUDA. Please review it carefully if you intend to use it as a
//! reference.
//!
//! According to the documentation for
//! [openfx::sys::gpu_render::kOfxImageEffectPropCudaStream], a plugin should
//! synchronize when the stream is not provided. The original
//! example doesn't seem to do so. This port does.
//!
//! ### General
//!
//! The original example declares support for the general context, but since it
//! strictly uses only a single source clip, general context support is
//! unnecessary. Therefore, this port does not declare support for the general
//! context.
//!
//! A `renderer_info` parameter is added to help debugging.
//!
//! We have plugin variants that force the use of specific renderers, such as
//! Metal (with Buffers).
//!
//! ## Known issues
//!
//! The OpenCL image rendering logic is unverified, because the maintainer(s)
//! don't have access to a host that supports it. In fact, the maintainer(s)
//! don't even know which hosts support it.
//!
//! Similarly, because the maintainer(s) don't have access to a host that
//! doesn't support CUDA streams, they can't verify that the CUDA implementation
//! works correctly when the host does not provide CUDA streams.
//!
//! ## TODO
//!
//! - Port the comments from the original example.

use std::{
    ffi::{CStr, c_void},
    ptr::NonNull,
    sync::{Arc, Mutex},
};

use openfx::{
    low::{
        InboundResult, OutboundResult, Status, StatusError, StatusReply,
        enums::{
            ImageEffectPropComponents, ImageEffectPropOpenCLRenderSupported,
            ImageEffectPropOpenCLSupported, ImageEffectPropSupportedComponents,
            ImageEffectPropSupportedContexts, ImageEffectPropSupportedPixelDepths,
            ParamPropDoubleType, PropChangeReason,
        },
    },
    low_plugin::{
        Host, Plugin,
        actions::{
            core::ActionInstanceChangedIn,
            image_effect::{
                ActionBeginSequenceRenderIn, ActionDescribeInContextIn, ActionEndSequenceRenderIn,
                ActionIsIdentityIn, ActionIsIdentityOut, ActionRenderIn, ImageEffectAction,
            },
        },
        objects::{ImageEffectDescriptor, ImageEffectInstance, ParamSetDescriptor},
        parameters::{ParamBoolean, ParamDouble, ParamDoublePropertySet, ParamString},
        property_sets::KnownEnumProperty,
        suites::{ParameterSuiteV1, PropertySuiteV1},
    },
    sys::{
        core::{OfxTime, kOfxPropLabel},
        gpu_render::kOfxImageEffectPropOpenCLRenderSupported,
        image_effect::{
            kOfxImageEffectOutputClipName, kOfxImageEffectSimpleSourceClipName, kOfxTypeClip,
        },
        param::kOfxTypeParameter,
    },
};

use openfx_layer_low_examples_support::{
    guarantee_send::{GuaranteeSend, GuaranteeSendClipInstance},
    managed_clip_images::{for_cpu::ManagedClipImageForCpu, for_opencl::ManagedClipImageForOpencl},
    plugin_data::{PluginData, PluginDataaFeatures},
    renderer_info::{Renderer, UsedRenderer},
    set_labels,
    sys_type_utils::rect_i_from_array,
};

use crate::{
    TRACING_DISPATCH,
    definitions::{PLUGINS_GROUPING, PluginInfo},
};

mod opencl_kernels;

#[cfg(not(target_os = "macos"))]
mod cuda_kernel;

#[cfg(target_os = "macos")]
mod metal_kernel;

const SUPPORTS_TILES: bool = false;
const SUPPORTS_MULTI_RESOLUTION: bool = false;
const SUPPORTS_MULTIPLE_CLIP_PARS: bool = false;

const COMPONENT_SCALES_GROUP_NAME: &CStr = c"componentScales";
const SCALE_PARAM_NAME: &CStr = c"scale";
const COMPONENT_SCALES_ENABLED_PARAM_NAME: &CStr = c"scaleComponents";
const SCALE_R_PARAM_NAME: &CStr = c"scaleR";
const SCALE_G_PARAM_NAME: &CStr = c"scaleG";
const SCALE_B_PARAM_NAME: &CStr = c"scaleB";
const SCALE_A_PARAM_NAME: &CStr = c"scaleA";
const RENDERER_INFO_PARAM_NAME: &CStr = c"rendererInfo";

struct AdditionalSharedData {}

struct MyInstanceData {
    source_clip: GuaranteeSendClipInstance,
    output_clip: GuaranteeSendClipInstance,

    scale_param: ParamDouble,
    scale_r_param: ParamDouble,
    scale_r_param_props: ParamDoublePropertySet,
    scale_g_param: ParamDouble,
    scale_g_param_props: ParamDoublePropertySet,
    scale_b_param: ParamDouble,
    scale_b_param_props: ParamDoublePropertySet,
    scale_a_param: ParamDouble,
    scale_a_param_props: ParamDoublePropertySet,
    component_scales_enabled_param: ParamBoolean,

    used_renderer: UsedRenderer,
    last_used_renderer: UsedRenderer,
    renderer_info_param: ParamString,
}

struct PluginVariant {
    info: PluginInfo,
    force_renderer: Option<Renderer>,

    host_before_action_load: Mutex<Option<GuaranteeSend<Host>>>,
    global_data: Mutex<Option<(PluginData, Arc<AdditionalSharedData>)>>,
}
impl PluginVariant {
    const fn new(info: PluginInfo, force_renderer: Option<Renderer>) -> Self {
        Self {
            info,
            force_renderer,
            host_before_action_load: Mutex::new(None),
            global_data: Mutex::new(None),
        }
    }

    fn set_host(&self, host: Host) {
        let mut lock = self
            .host_before_action_load
            .lock()
            .expect("Failed to lock HOST_BEFORE_ACTION_LOAD.");
        if lock.is_some() {
            panic!("HOST_BEFORE_ACTION_LOAD has already been set.");
        }
        lock.replace(GuaranteeSend(host));
    }

    fn global_data_lockless(&self) -> Result<(PluginData, Arc<AdditionalSharedData>), StatusError> {
        let data = self.global_data.lock().map_err(|_| StatusError::Failed)?;
        let data = data.as_ref().ok_or(StatusError::Failed)?;
        Ok((data.0.clone(), data.1.clone()))
    }
}

macro_rules! plugin_variant {
    ($mod_name:ident, $struct_name:ident, $info_path:path, $force_renderer:expr) => {
        mod $mod_name {
            use super::*;
            use $info_path as PLUGIN_INFO;

            pub struct $struct_name;
            static PLUGIN_VARIANT: PluginVariant = PluginVariant::new(PLUGIN_INFO, $force_renderer);
            impl Plugin for $struct_name {
                const PLUGIN_IDENTIFIER: &'static CStr = PLUGIN_INFO.identifier;
                const PLUGIN_VERSION_MAJOR: std::ffi::c_uint = 1;
                const PLUGIN_VERSION_MINOR: std::ffi::c_uint = 0;

                fn set_host(host: Host) {
                    set_host(&PLUGIN_VARIANT, host);
                }

                fn main_entry(action: ImageEffectAction) -> OutboundResult {
                    main_entry(&PLUGIN_VARIANT, action)
                }
            }
        }
        pub use $mod_name::$struct_name;
    };
}

plugin_variant!(
    plugin_variant_normal,
    PluginExampleGpuGain,
    crate::definitions::PLUGIN_GPU_GAIN_INFO,
    None
);

plugin_variant!(
    plugin_variant_force_opencl_buffers,
    PluginExampleGpuGainForceOpenclBuffers,
    crate::definitions::PLUGIN_GPU_GAIN_FORCE_OPENCL_BUFFERS_INFO,
    Some(Renderer::OpenclWithBuffers)
);

plugin_variant!(
    plugin_variant_force_opencl_images,
    PluginExampleGpuGainForceOpenclImages,
    crate::definitions::PLUGIN_GPU_GAIN_FORCE_OPENCL_IMAGES_INFO,
    Some(Renderer::OpenclWithImages)
);

#[cfg(target_os = "macos")]
plugin_variant!(
    plugin_variant_force_metal_buffers,
    PluginExampleGpuGainForceMetalBuffers,
    crate::definitions::PLUGIN_GPU_GAIN_FORCE_METAL_BUFFERS_INFO,
    Some(Renderer::MetalWithBuffers)
);

#[cfg(not(target_os = "macos"))]
plugin_variant!(
    plugin_variant_force_cuda,
    PluginExampleGpuGainForceCuda,
    crate::definitions::PLUGIN_GPU_GAIN_FORCE_CUDA_INFO,
    Some(Renderer::Cuda)
);

plugin_variant!(
    plugin_variant_force_cpu,
    PluginExampleGpuGainForceCpu,
    crate::definitions::PLUGIN_GPU_GAIN_FORCE_CPU_INFO,
    Some(Renderer::Cpu)
);

fn set_host(var: &PluginVariant, host: Host) {
    match TRACING_DISPATCH.get() {
        Some(dispatch) => {
            let _guard = tracing::dispatcher::set_default(dispatch);
            var.set_host(host);
        }
        None => var.set_host(host),
    }
}

fn main_entry(var: &PluginVariant, action: ImageEffectAction) -> OutboundResult {
    return match TRACING_DISPATCH.get() {
        Some(dispatch) => {
            let _guard = tracing::dispatcher::set_default(dispatch);
            tracing::debug!("main_entry: Entering Action {:?}", action.sys_action());
            let ret = _main_entry(var, action);
            tracing::debug!("main_entry: Exiting Action with {ret:?}");
            ret
        }
        None => _main_entry(var, action),
    };

    fn _main_entry(var: &PluginVariant, action: ImageEffectAction) -> OutboundResult {
        match action {
            ImageEffectAction::Load { .. } => action_load(var),
            ImageEffectAction::Unload { .. } => action_unload(var),
            ImageEffectAction::Describe { handle, .. } => action_describe(var, handle),
            ImageEffectAction::DescribeInContext {
                handle, in_args, ..
            } => action_describe_in_context(var, handle, in_args),
            ImageEffectAction::CreateInstance { handle, .. } => action_create_instance(var, handle),
            ImageEffectAction::DestroyInstance { handle, .. } => {
                action_destroy_instance(var, handle)
            }
            ImageEffectAction::InstanceChanged {
                handle, in_args, ..
            } => action_instance_changed(var, handle, in_args),
            ImageEffectAction::IsIdentity {
                handle,
                in_args,
                out_args,
            } => action_is_identity(var, handle, in_args, out_args),
            ImageEffectAction::BeginSequenceRender {
                handle, in_args, ..
            } => action_begin_sequence_render(var, handle, in_args),
            ImageEffectAction::EndSequenceRender {
                handle, in_args, ..
            } => action_end_sequence_render(var, handle, in_args),
            ImageEffectAction::Render {
                handle, in_args, ..
            } => action_render(var, handle, in_args),
            _ => Ok(StatusReply::ReplyDefault.into()),
        }
    }
}

fn action_load(var: &PluginVariant) -> OutboundResult {
    let host = var
        .host_before_action_load
        .lock()
        .map_err(|_| StatusError::Failed)?
        .take()
        .ok_or(StatusError::Failed)?;

    let mut data = var.global_data.lock().map_err(|_| StatusError::Failed)?;
    if data.is_some() {
        return Err(StatusError::Failed);
    }

    *data = Some({
        let additional = { AdditionalSharedData {} };

        let data = PluginData::try_new(
            host,
            #[allow(clippy::needless_update)]
            PluginDataaFeatures {
                multi_thread_suite: true,
                ..Default::default()
            },
        )?;

        (data, Arc::new(additional))
    });

    Ok(Status::OK)
}

fn action_unload(var: &PluginVariant) -> OutboundResult {
    let mut data = var.global_data.lock().map_err(|_| StatusError::Failed)?;
    if data.take().is_none() {
        Err(StatusError::Failed)
    } else {
        Ok(Status::OK)
    }
}

fn action_describe(var: &PluginVariant, descriptor: ImageEffectDescriptor) -> OutboundResult {
    let (data, _additional) = var.global_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;

    let props = unsafe { descriptor.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    unsafe {
        // Basic labels
        set_labels!(&data.property_suite.0, &props, var.info.label);
        props
            .set_image_effect_plugin_grouping(s_prop, Some(PLUGINS_GROUPING))
            .ok_or(StatusError::Failed)?;
        if let stat = props.set_plugin_description(s_prop, Some(var.info.description))
            && !stat.is_ok()
        {
            // Apparently DaVinci Resolve fails this (`ErrUnknown`), even though
            // this example originated from DaVinci Resolve's SDK (according to
            // git history). What?
            tracing::warn!("Failed to set plugin description: {stat:?}",);
        }

        // Add the supported contexts, only filter at the moment
        {
            use ImageEffectPropSupportedContexts as C;
            props
                .set_image_effect_supported_contexts(s_prop, &[C::Filter.into()])
                .ok_or(StatusError::Failed)?;
        }

        // Add supported pixel depths
        {
            use ImageEffectPropSupportedPixelDepths as D;
            props
                .set_image_effect_supported_pixel_depths(s_prop, &[D::Float.into()])
                .ok_or(StatusError::Failed)?;
        }

        // Set a few flags
        props
            .set_image_effect_plugin_single_instance(s_prop, false)
            .ok_or(StatusError::Failed)?;
        props
            .set_image_effect_plugin_host_frame_threading(s_prop, false)
            .ok_or(StatusError::Failed)?;
        props
            .set_image_effect_supports_multi_resolution(s_prop, SUPPORTS_MULTI_RESOLUTION)
            .ok_or(StatusError::Failed)?;
        props
            .set_image_effect_supports_tiles(s_prop, SUPPORTS_TILES)
            .ok_or(StatusError::Failed)?;
        props
            .set_image_effect_temporal_clip_access(s_prop, false)
            .ok_or(StatusError::Failed)?;
        props
            .set_image_effect_plugin_field_render_twice_always(s_prop, false)
            .ok_or(StatusError::Failed)?;
        // FIXME: name
        props
            .set_image_effect_supports_multiple_clip_pa_rs(s_prop, SUPPORTS_MULTIPLE_CLIP_PARS)
            .ok_or(StatusError::Failed)?;

        // Setup OpenCL render capability flags
        if var
            .force_renderer
            .is_none_or(|r| r == Renderer::OpenclWithBuffers)
        {
            // for OpenCL buffer rendering
            //
            // TODO: Add the missing metadata upstream.
            if let stat = s_prop.prop_set_string(
                props.sys_handle(),
                kOfxImageEffectPropOpenCLRenderSupported,
                Some(ImageEffectPropOpenCLRenderSupported::True.as_c_str()),
            ) && !stat.is_ok()
            {
                tracing::warn!("Failed to set OpenCL Buffers render support: {stat:?}");
            }
        }

        if var
            .force_renderer
            .is_none_or(|r| r == Renderer::OpenclWithImages)
        {
            // for OpenCL image rendering
            if let stat = props.set_image_effect_open_cl_supported(
                s_prop,
                ImageEffectPropOpenCLSupported::True.into(),
            ) && !stat.is_ok()
            {
                tracing::warn!("Failed to set OpenCL Images render support: {stat:?}");
            }
        }

        // Setup CUDA render capability flags on non-Apple system
        #[cfg(not(target_os = "macos"))]
        if var.force_renderer.is_none_or(|r| r == Renderer::Cuda) {
            use openfx::{
                low::enums::{
                    ImageEffectPropCudaRenderSupported, ImageEffectPropCudaStreamSupported,
                },
                sys::gpu_render::{
                    kOfxImageEffectPropCudaRenderSupported, kOfxImageEffectPropCudaStreamSupported,
                },
            };

            // TODO: Add the missing metadata upstream.
            if let stat = s_prop.prop_set_string(
                props.sys_handle(),
                kOfxImageEffectPropCudaRenderSupported,
                Some(ImageEffectPropCudaRenderSupported::True.as_c_str()),
            ) && !stat.is_ok()
            {
                tracing::warn!("Failed to set CUDA render support: {stat:?}");
            }
            // TODO: Add the missing metadata upstream.
            if let stat = s_prop.prop_set_string(
                props.sys_handle(),
                kOfxImageEffectPropCudaStreamSupported,
                Some(ImageEffectPropCudaStreamSupported::True.as_c_str()),
            ) && !stat.is_ok()
            {
                tracing::warn!("Failed to set CUDA render's stream support: {stat:?}");
            }
        }

        // Setup Metal render capability flags only on Apple system
        #[cfg(target_os = "macos")]
        if var
            .force_renderer
            .is_none_or(|r| r == Renderer::MetalWithBuffers)
        {
            use openfx::{
                low::enums::ImageEffectPropMetalRenderSupported,
                sys::gpu_render::kOfxImageEffectPropMetalRenderSupported,
            };

            // TODO: Add the missing metadata upstream.
            if let stat = s_prop.prop_set_string(
                props.sys_handle(),
                kOfxImageEffectPropMetalRenderSupported,
                Some(ImageEffectPropMetalRenderSupported::True.as_c_str()),
            ) && !stat.is_ok()
            {
                tracing::warn!("Failed to set Metal render support: {stat:?}");
            }
        }
    }

    Ok(Status::OK)
}

fn action_describe_in_context(
    var: &PluginVariant,
    descriptor: ImageEffectDescriptor,
    _in_args: ActionDescribeInContextIn,
) -> OutboundResult {
    let (data, _additional) = var.global_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;
    let s_param = &data.parameter_suite.0;

    // Create the mandated source clip & output clip
    for name in [
        kOfxImageEffectOutputClipName,
        kOfxImageEffectSimpleSourceClipName,
    ] {
        let props = unsafe { descriptor.clip_define(s_ifx, name) }.ok_or(StatusError::Failed)?;

        unsafe {
            {
                use ImageEffectPropSupportedComponents as C;
                props
                    .set_image_effect_supported_components(s_prop, &[C::RGBA.into()])
                    .ok_or(StatusError::Failed)?;
            }
            props
                .set_image_effect_temporal_clip_access(s_prop, false)
                .ok_or(StatusError::Failed)?;
            props
                .set_image_effect_supports_tiles(s_prop, SUPPORTS_TILES)
                .ok_or(StatusError::Failed)?;
            if name == kOfxImageEffectSimpleSourceClipName {
                props
                    .set_image_clip_is_mask(s_prop, false)
                    .ok_or(StatusError::Failed)?;
            }
        }
    }

    let param_set = unsafe { descriptor.get_param_set(s_ifx) }.ok_or(StatusError::Failed)?;

    unsafe {
        // Make a page to put things in it
        let page_param_props = param_set
            .param_define_page(s_param, c"Controls")
            .ok_or(StatusError::Failed)?;
        let mut page_children: Vec<Option<&CStr>> = vec![];

        // Group param to group the scales
        {
            let param_props = param_set
                .param_define_group(s_param, COMPONENT_SCALES_GROUP_NAME)
                .ok_or(StatusError::Failed)?;
            param_props
                .set_param_hint(s_prop, Some(c"Scales on the individual component"))
                .ok_or(StatusError::Failed)?;
            set_labels!(&data.property_suite.0, &param_props, c"Components");
        }

        // Make overall scale params
        define_scale_param(
            s_prop,
            s_param,
            &param_set,
            SCALE_PARAM_NAME,
            c"scale",
            c"Scales all component in the image",
            None,
        )?;
        page_children.push(Some(SCALE_PARAM_NAME));

        // Add a boolean to enable the component scale
        {
            let param_props = param_set
                .param_define_boolean(s_param, COMPONENT_SCALES_ENABLED_PARAM_NAME)
                .ok_or(StatusError::Failed)?;
            param_props
                .set_param_default(s_prop, true)
                .ok_or(StatusError::Failed)?;
            param_props
                .set_param_hint(s_prop, Some(c"Enables scales on individual components"))
                .ok_or(StatusError::Failed)?;
            set_labels!(&data.property_suite.0, &param_props, c"Scale Components");
            param_props
                .set_param_parent(s_prop, Some(COMPONENT_SCALES_GROUP_NAME))
                .ok_or(StatusError::Failed)?;
        }
        page_children.push(Some(COMPONENT_SCALES_ENABLED_PARAM_NAME));

        // Make the four component scale params
        for (name, label, hint) in [
            (
                SCALE_R_PARAM_NAME,
                c"red",
                c"Scales the red component of the image",
            ),
            (
                SCALE_G_PARAM_NAME,
                c"green",
                c"Scales the green component of the image",
            ),
            (
                SCALE_B_PARAM_NAME,
                c"blue",
                c"Scales the blue component of the image",
            ),
            (
                SCALE_A_PARAM_NAME,
                c"alpha",
                c"Scales the alpha component of the image",
            ),
        ] {
            define_scale_param(
                s_prop,
                s_param,
                &param_set,
                name,
                label,
                hint,
                Some(COMPONENT_SCALES_GROUP_NAME),
            )?;
            page_children.push(Some(name));
        }

        {
            let param_props = param_set
                .param_define_string(s_param, RENDERER_INFO_PARAM_NAME)
                .ok_or(StatusError::Failed)?;
            param_props
                .set_param_enabled(s_prop, false)
                .ok_or(StatusError::Failed)?;
            set_labels!(&data.property_suite.0, &param_props, c"Renderer Info");
        }
        page_children.push(Some(RENDERER_INFO_PARAM_NAME));

        page_param_props
            .set_param_page_child(s_prop, &page_children)
            .ok_or(StatusError::Failed)?;
    }

    Ok(Status::OK)
}

unsafe fn define_scale_param(
    s_prop: &PropertySuiteV1,
    s_param: &ParameterSuiteV1,
    effect_params: &ParamSetDescriptor,
    name: &CStr,
    label: &CStr,
    hint: &CStr,
    parent_name: Option<&CStr>,
) -> Result<(), StatusError> {
    let props =
        unsafe { effect_params.param_define_double(s_param, name) }.ok_or(StatusError::Failed)?;

    unsafe {
        set_labels!(s_prop, props, label);
        props
            .set_param_double_type(s_prop, ParamPropDoubleType::Scale.into())
            .ok_or(StatusError::Failed)?;
        props
            .set_param_script_name(s_prop, Some(name))
            .ok_or(StatusError::Failed)?;
        props
            .set_param_hint(s_prop, Some(hint))
            .ok_or(StatusError::Failed)?;
        props
            .set_param_default(s_prop, 1.0)
            .ok_or(StatusError::Failed)?;
        props
            .set_param_min(s_prop, 0.0)
            .ok_or(StatusError::Failed)?;
        props
            .set_param_max(s_prop, 10.0)
            .ok_or(StatusError::Failed)?;
        props
            .set_param_increment(s_prop, 0.1)
            .ok_or(StatusError::Failed)?;
        props
            .set_param_display_min(s_prop, 0.0)
            .ok_or(StatusError::Failed)?;
        props
            .set_param_display_max(s_prop, 10.0)
            .ok_or(StatusError::Failed)?;

        if let Some(parent_name) = parent_name {
            props
                .set_param_parent(s_prop, Some(parent_name))
                .ok_or(StatusError::Failed)?;
        }
    }

    Ok(())
}

fn action_create_instance(var: &PluginVariant, instance: ImageEffectInstance) -> OutboundResult {
    let (data, _additional) = var.global_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;
    let s_param = &data.parameter_suite.0;

    let instance_props = unsafe { instance.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    let source_clip =
        unsafe { instance.clip_get_clip_handle(s_ifx, kOfxImageEffectSimpleSourceClipName) }
            .ok_or(StatusError::Failed)?;
    let output_clip =
        unsafe { instance.clip_get_clip_handle(s_ifx, kOfxImageEffectOutputClipName) }
            .ok_or(StatusError::Failed)?;

    let param_set = unsafe { instance.get_param_set(s_ifx) }.ok_or(StatusError::Failed)?;
    let scale_param = unsafe { param_set.param_get_param_handle_double(s_param, SCALE_PARAM_NAME) }
        .ok_or(StatusError::Failed)?;
    let (scale_r_param, scale_r_param_props) =
        unsafe { param_set.param_get_handle_double(s_param, SCALE_R_PARAM_NAME) }
            .ok_or(StatusError::Failed)?;
    let (scale_g_param, scale_g_param_props) =
        unsafe { param_set.param_get_handle_double(s_param, SCALE_G_PARAM_NAME) }
            .ok_or(StatusError::Failed)?;
    let (scale_b_param, scale_b_param_props) =
        unsafe { param_set.param_get_handle_double(s_param, SCALE_B_PARAM_NAME) }
            .ok_or(StatusError::Failed)?;
    let (scale_a_param, scale_a_param_props) =
        unsafe { param_set.param_get_handle_double(s_param, SCALE_A_PARAM_NAME) }
            .ok_or(StatusError::Failed)?;
    let component_scales_enabled_param = unsafe {
        param_set.param_get_param_handle_boolean(s_param, COMPONENT_SCALES_ENABLED_PARAM_NAME)
    }
    .ok_or(StatusError::Failed)?;

    let renderer_info_param =
        unsafe { param_set.param_get_param_handle_string(s_param, RENDERER_INFO_PARAM_NAME) }
            .ok_or(StatusError::Failed)?;

    let my_data = MyInstanceData {
        source_clip: GuaranteeSendClipInstance(source_clip),
        output_clip: GuaranteeSendClipInstance(output_clip),
        scale_param,
        scale_r_param,
        scale_r_param_props,
        scale_g_param,
        scale_g_param_props,
        scale_b_param,
        scale_b_param_props,
        scale_a_param,
        scale_a_param_props,
        component_scales_enabled_param,
        used_renderer: Default::default(),
        last_used_renderer: Default::default(),
        renderer_info_param,
    };
    let my_data_ptr = Box::into_raw(Box::new(my_data)) as *mut c_void;

    // SAFETY: the pointee is kept alive by `Box::into_raw` until it is
    // reclaimed with `Box::from_raw` in `action_destroy_instance`.
    if let r = unsafe { instance_props.set_instance_data(s_prop, NonNull::new(my_data_ptr)) }
        && !r.is_ok()
    {
        drop(unsafe { Box::from_raw(my_data_ptr as *mut MyInstanceData) });
        return Err(StatusError::Failed);
    }

    Ok(Status::OK)
}

fn action_destroy_instance(var: &PluginVariant, instance: ImageEffectInstance) -> OutboundResult {
    let (data, _additional) = var.global_data_lockless()?;

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

fn action_instance_changed(
    var: &PluginVariant,
    instance: ImageEffectInstance,
    in_args: ActionInstanceChangedIn,
) -> OutboundResult {
    let (data, _additional) = var.global_data_lockless()?;

    let s_prop = &data.property_suite.0;

    if matches!(
        unsafe { in_args.get_change_reason_known(s_prop) }.ok_or(StatusError::Failed)?,
        Some(PropChangeReason::PluginEdited)
    ) {
        return Ok(StatusReply::ReplyDefault.into());
    }

    enum ChangedType {
        Parameter,
        Clip,
    }
    let changed_type = match unsafe { in_args.get_type(s_prop) }.ok_or(StatusError::Failed)? {
        Some(ty) => {
            let ty = unsafe { CStr::from_ptr(ty.as_ptr()) };
            if ty == kOfxTypeParameter {
                ChangedType::Parameter
            } else if ty == kOfxTypeClip {
                ChangedType::Clip
            } else {
                let ty = ty.to_owned();
                let name = unsafe {
                    in_args
                        .get_name(s_prop)
                        .ok_or(StatusError::Failed)?
                        .map(|name| CStr::from_ptr(name.as_ptr()))
                };
                tracing::warn!(
                    "Instance Changed called with unknown type `{ty:?}` of object `{name:?}`"
                );
                return Ok(StatusReply::ReplyDefault.into());
            }
        }
        None => return Ok(StatusReply::ReplyDefault.into()),
    };
    let Some(changed_name) = unsafe { in_args.get_name(s_prop) }
        .ok_or(StatusError::Failed)?
        .map(|name| unsafe { CStr::from_ptr(name.as_ptr()) })
    else {
        return Ok(StatusReply::ReplyDefault.into());
    };

    match changed_type {
        ChangedType::Parameter if changed_name == COMPONENT_SCALES_ENABLED_PARAM_NAME => {
            set_enabledness(&data, &instance)?;
        }
        ChangedType::Clip if changed_name == kOfxImageEffectSimpleSourceClipName => {
            set_enabledness(&data, &instance)?;
        }
        _ => return Ok(StatusReply::ReplyDefault.into()),
    }

    Ok(Status::OK)
}

fn set_enabledness(data: &PluginData, instance: &ImageEffectInstance) -> Result<(), StatusError> {
    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;
    let s_param = &data.parameter_suite.0;

    let instance_props = unsafe { instance.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    let Some(my_data_ptr) =
        unsafe { instance_props.get_instance_data(s_prop) }.ok_or(StatusError::Failed)?
    else {
        return Err(StatusError::Failed);
    };
    let my_data = unsafe { &*(my_data_ptr.as_ptr() as *const MyInstanceData) };

    let (_, source_clip_props) =
        unsafe { instance.clip_get_handle(s_ifx, kOfxImageEffectSimpleSourceClipName) }
            .ok_or(StatusError::Failed)?;

    // the component enabledness depends on the clip being RGBA and the param being true
    let component_scales_enabled = unsafe {
        my_data
            .component_scales_enabled_param
            .param_get_value(s_param)
    }
    .ok_or(StatusError::Failed)?;
    let pixel_components = unsafe { source_clip_props.get_image_effect_components_known(s_prop) }
        .ok_or(StatusError::Failed)?;
    let enabled =
        component_scales_enabled && pixel_components == Some(ImageEffectPropComponents::RGBA);

    unsafe {
        my_data
            .scale_r_param_props
            .set_param_enabled(s_prop, enabled)
            .ok_or(StatusError::Failed)?;
        my_data
            .scale_g_param_props
            .set_param_enabled(s_prop, enabled)
            .ok_or(StatusError::Failed)?;
        my_data
            .scale_b_param_props
            .set_param_enabled(s_prop, enabled)
            .ok_or(StatusError::Failed)?;
        my_data
            .scale_a_param_props
            .set_param_enabled(s_prop, enabled)
            .ok_or(StatusError::Failed)?;
    }

    Ok(())
}

fn action_is_identity(
    var: &PluginVariant,
    instance: ImageEffectInstance,
    in_args: ActionIsIdentityIn,
    out_args: ActionIsIdentityOut,
) -> OutboundResult {
    let (data, _additional) = var.global_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;

    let instance_props = unsafe { instance.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    let Some(my_data_ptr) =
        unsafe { instance_props.get_instance_data(s_prop) }.ok_or(StatusError::Failed)?
    else {
        return Err(StatusError::Failed);
    };
    let my_data = unsafe { &*(my_data_ptr.as_ptr() as *const MyInstanceData) };

    let time = unsafe { in_args.get_time(s_prop) }.ok_or(StatusError::Failed)?;

    let Scales {
        r_scale,
        g_scale,
        b_scale,
        a_scale,
    } = Scales::retrieve_at_time(&data, my_data, time)?;

    if r_scale == 1.0 && g_scale == 1.0 && b_scale == 1.0 && a_scale == 1.0 {
        unsafe {
            s_prop.prop_set_string(
                out_args.sys_handle(),
                kOfxPropLabel,
                Some(kOfxImageEffectSimpleSourceClipName),
            )
        }
        .ok_or(StatusError::Failed)?;
        Ok(Status::OK)
    } else {
        Ok(StatusReply::ReplyDefault.into())
    }
}

fn action_begin_sequence_render(
    var: &PluginVariant,
    instance: ImageEffectInstance,
    _in_args: ActionBeginSequenceRenderIn,
) -> OutboundResult {
    let (data, _additional) = var.global_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;

    let instance_props = unsafe { instance.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    let Some(my_data_ptr) =
        unsafe { instance_props.get_instance_data(s_prop) }.ok_or(StatusError::Failed)?
    else {
        return Err(StatusError::Failed);
    };
    let my_data = unsafe { &*(my_data_ptr.as_ptr() as *const MyInstanceData) };

    my_data.last_used_renderer.assign(&my_data.used_renderer);
    my_data.used_renderer.store_no_info();

    Ok(Status::OK)
}

fn action_end_sequence_render(
    var: &PluginVariant,
    instance: ImageEffectInstance,
    _in_args: ActionEndSequenceRenderIn,
) -> OutboundResult {
    let (data, _additional) = var.global_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;
    let s_param = &data.parameter_suite.0;

    let instance_props = unsafe { instance.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    let Some(my_data_ptr) =
        unsafe { instance_props.get_instance_data(s_prop) }.ok_or(StatusError::Failed)?
    else {
        return Err(StatusError::Failed);
    };
    let my_data = unsafe { &*(my_data_ptr.as_ptr() as *const MyInstanceData) };

    if my_data.used_renderer != my_data.last_used_renderer {
        let renderer_description = my_data.used_renderer.describe();
        unsafe {
            my_data
                .renderer_info_param
                .param_set_value(s_param, &renderer_description)
        }
        .ok_or(StatusError::Failed)?;
    }

    Ok(Status::OK)
}

fn action_render(
    var: &PluginVariant,
    instance: ImageEffectInstance,
    in_args: ActionRenderIn,
) -> OutboundResult {
    let (data, _additional) = var.global_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;

    let instance_props = unsafe { instance.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    let time = unsafe { in_args.get_time(s_prop) }.ok_or(StatusError::Failed)?;
    let render_window =
        unsafe { in_args.get_image_effect_render_window(s_prop) }.ok_or(StatusError::Failed)?;
    let render_window = rect_i_from_array(&render_window);

    let Some(my_data_ptr) =
        unsafe { instance_props.get_instance_data(s_prop) }.ok_or(StatusError::Failed)?
    else {
        return Err(StatusError::Failed);
    };
    let my_data = unsafe { &*(my_data_ptr.as_ptr() as *const MyInstanceData) };

    let scales = Scales::retrieve_at_time(&data, my_data, time)?;

    if var
        .force_renderer
        .is_none_or(|r| matches!(r, Renderer::OpenclWithBuffers | Renderer::OpenclWithImages))
    {
        let is_enabled = match unsafe { in_args.get_image_effect_open_cl_enabled(s_prop) } {
            InboundResult::Ok(v) => v,
            other => {
                tracing::warn!("Failed to invoke `get_image_effect_open_cl_enabled`: {other:?}");
                false
            }
        };

        if is_enabled {
            let output_img =
                ManagedClipImageForOpencl::try_new_from_clip(&data, &my_data.output_clip.0, time)?;
            let source_img =
                ManagedClipImageForOpencl::try_new_from_clip(&data, &my_data.source_clip.0, time)?;

            if output_img.pixel_depth() != source_img.pixel_depth()
                || output_img.n_comps() != source_img.n_comps()
            {
                return Err(StatusError::ErrValue); // Follow the official example.
            }

            let cmd_q = unsafe {
                in_args
                    .get_image_effect_open_cl_command_queue(s_prop)
                    .ok_or(StatusError::Failed)?
            };
            let Some(cmd_q) = cmd_q else {
                return Err(StatusError::Failed);
            };
            processing_opencl::process_images_opencl(
                my_data,
                scales,
                source_img,
                output_img,
                cmd_q,
                var.force_renderer.as_ref(),
            )?;
            return Ok(Status::OK);
        }
    }

    #[cfg(not(target_os = "macos"))]
    if var.force_renderer.is_none_or(|r| r == Renderer::Cuda) {
        use openfx_layer_low_examples_support::managed_clip_images::for_cuda::ManagedClipImageForCuda;

        let is_enabled = match unsafe { in_args.get_image_effect_cuda_enabled(s_prop) } {
            InboundResult::Ok(v) => v,
            other => {
                tracing::warn!("Failed to invoke `get_image_effect_cuda_enabled`: {other:?}");
                false
            }
        };

        if is_enabled {
            let output_img =
                ManagedClipImageForCuda::try_new_from_clip(&data, &my_data.output_clip.0, time)?;
            let source_img =
                ManagedClipImageForCuda::try_new_from_clip(&data, &my_data.source_clip.0, time)?;

            if output_img.pixel_depth() != source_img.pixel_depth()
                || output_img.n_comps() != source_img.n_comps()
            {
                return Err(StatusError::ErrValue); // Follow the official example.
            }

            let stream = unsafe {
                in_args
                    .get_image_effect_cuda_stream(s_prop)
                    .ok_or(StatusError::Failed)?
            };
            processing_cuda::process_images_cuda(my_data, scales, source_img, output_img, stream)?;
            return Ok(Status::OK);
        }
    }

    #[cfg(target_os = "macos")]
    if var
        .force_renderer
        .is_none_or(|r| r == Renderer::MetalWithBuffers)
    {
        use openfx_layer_low_examples_support::managed_clip_images::for_metal::ManagedClipImageForMetal;

        let is_enabled = match unsafe { in_args.get_image_effect_metal_enabled(s_prop) } {
            InboundResult::Ok(v) => v,
            other => {
                tracing::warn!("Failed to invoke `get_image_effect_metal_enabled`: {other:?}");
                false
            }
        };

        if is_enabled {
            let output_img =
                ManagedClipImageForMetal::try_new_from_clip(&data, &my_data.output_clip.0, time)?;
            let source_img =
                ManagedClipImageForMetal::try_new_from_clip(&data, &my_data.source_clip.0, time)?;

            if output_img.pixel_depth() != source_img.pixel_depth()
                || output_img.n_comps() != source_img.n_comps()
            {
                return Err(StatusError::ErrValue); // Follow the official example.
            }

            let cmd_q = unsafe {
                in_args
                    .get_image_effect_metal_command_queue(s_prop)
                    .ok_or(StatusError::Failed)?
            };
            let Some(cmd_q) = cmd_q else {
                return Err(StatusError::Failed);
            };
            processing_metal::process_images_metal(my_data, scales, source_img, output_img, cmd_q)?;
            return Ok(Status::OK);
        }
    }

    if var.force_renderer.is_none_or(|r| r == Renderer::Cpu) {
        let output_img =
            ManagedClipImageForCpu::try_new_from_clip(&data, &my_data.output_clip.0, time)?;
        let source_img =
            ManagedClipImageForCpu::try_new_from_clip(&data, &my_data.source_clip.0, time)?;

        if output_img.pixel_depth() != source_img.pixel_depth()
            || output_img.n_comps() != source_img.n_comps()
        {
            return Err(StatusError::ErrValue); // Follow the official example.
        }

        processing_cpu::process_images(
            &data,
            my_data,
            &instance,
            scales,
            source_img,
            output_img,
            render_window,
        )?;

        return Ok(Status::OK);
    }

    // This follows the official OpenGL example's handling of the case where
    // no renderer is available. It is not part of the original GPU Gain
    // example. It was added to deal with a failure to force a specific
    // renderer. I really don't know whether `ErrValue` or `ErrImageFormat`
    // would be more appropriate here.
    tracing::error!("No eligible renderer available!");
    my_data.used_renderer.store_no_eligible();
    Err(StatusError::ErrImageFormat)
}

mod processing_opencl {
    use std::{ffi::c_void, ptr::NonNull};

    use openfx::low::StatusError;

    use openfx_layer_low_examples_support::{
        managed_clip_images::for_opencl::{
            ManagedClipImageForOpencl, ManagedClipImageForOpenclData,
        },
        renderer_info::Renderer,
    };

    use crate::plugin_gpu_gain::{MyInstanceData, Scales};

    pub fn process_images_opencl(
        my_data: &MyInstanceData,
        scales: Scales,
        source_img: ManagedClipImageForOpencl,
        output_img: ManagedClipImageForOpencl,
        cmd_q: NonNull<c_void>,
        force_renderer: Option<&Renderer>,
    ) -> Result<(), StatusError> {
        use crate::plugin_gpu_gain::opencl_kernels::{
            run_opencl_kernel_buffers, run_opencl_kernel_images,
        };

        let bounds = source_img.bounds();
        let width = bounds.x2 - bounds.x1;
        let height = bounds.y2 - bounds.y1;

        if let ManagedClipImageForOpenclData::Image(input) = source_img.data()
            && let ManagedClipImageForOpenclData::Image(output) = output_img.data()
        {
            tracing::debug!("Processing via OpenCL (Images)");
            if let Some(r) = force_renderer
                && r != &Renderer::OpenclWithImages
            {
                tracing::warn!(
                    "`force_renderer` not enforced: expected to render with {:?}, but this is the path for {:?}",
                    r,
                    Renderer::OpenclWithImages
                );
            }
            my_data.used_renderer.store_opencl_image();

            unsafe {
                run_opencl_kernel_images(cmd_q, width, height, scales.as_array(), *input, *output)
            }?;
        } else if let ManagedClipImageForOpenclData::Buffer(input) = source_img.data()
            && let ManagedClipImageForOpenclData::Buffer(output) = output_img.data()
        {
            tracing::debug!("Processing via OpenCL (Buffers)");
            if let Some(r) = force_renderer
                && r != &Renderer::OpenclWithBuffers
            {
                tracing::warn!(
                    "`force_renderer` not enforced: expected to render with {:?}, but this is the path for {:?}",
                    r,
                    Renderer::OpenclWithBuffers
                );
            }
            my_data.used_renderer.store_opencl_buffer();

            unsafe {
                run_opencl_kernel_buffers(cmd_q, width, height, scales.as_array(), *input, *output)
            }?;
        } else {
            tracing::error!("Unable to process via OpenCL: image configuration not supported!");
            return Err(StatusError::Failed);
        }

        Ok(())
    }
}

#[cfg(not(target_os = "macos"))]
mod processing_cuda {
    use std::{ffi::c_void, ptr::NonNull};

    use openfx::low::StatusError;

    use openfx_layer_low_examples_support::managed_clip_images::for_cuda::ManagedClipImageForCuda;

    use crate::plugin_gpu_gain::{MyInstanceData, Scales, cuda_kernel::run_cuda_kernel};

    pub fn process_images_cuda(
        my_data: &MyInstanceData,
        scales: Scales,
        source_img: ManagedClipImageForCuda,
        output_img: ManagedClipImageForCuda,
        stream: Option<NonNull<c_void>>,
    ) -> Result<(), StatusError> {
        tracing::debug!("Processing via CUDA (Buffers)");
        my_data.used_renderer.store_cuda(stream.is_some());

        let bounds = source_img.bounds();
        let width = bounds.x2 - bounds.x1;
        let height = bounds.y2 - bounds.y1;

        let input = source_img.data_ptr();
        let output = output_img.data_ptr();

        unsafe { run_cuda_kernel(stream, width, height, scales.as_array(), input, output) }?;

        Ok(())
    }
}

#[cfg(target_os = "macos")]
mod processing_metal {
    use std::{ffi::c_void, ptr::NonNull};

    use openfx::low::StatusError;

    use openfx_layer_low_examples_support::managed_clip_images::for_metal::ManagedClipImageForMetal;

    use crate::plugin_gpu_gain::{MyInstanceData, Scales, metal_kernel::run_metal_kernel};

    pub fn process_images_metal(
        my_data: &MyInstanceData,
        scales: Scales,
        source_img: ManagedClipImageForMetal,
        output_img: ManagedClipImageForMetal,
        cmd_q: NonNull<c_void>,
    ) -> Result<(), StatusError> {
        tracing::debug!("Processing via Metal (Buffers)");
        my_data.used_renderer.store_metal_buffer();

        let bounds = source_img.bounds();
        let width = bounds.x2 - bounds.x1;
        let height = bounds.y2 - bounds.y1;

        let input = source_img.data_ptr();
        let output = output_img.data_ptr();

        unsafe { run_metal_kernel(cmd_q, width, height, scales.as_array(), input, output) }?;

        Ok(())
    }
}

mod processing_cpu {
    use std::ffi::{c_int, c_uint};

    use openfx::{low::StatusError, low_plugin::objects::ImageEffectInstance, sys::core::OfxRectI};

    use openfx_layer_low_examples_support::{
        managed_clip_images::for_cpu::ManagedClipImageForCpu, plugin_data::PluginData,
    };

    use crate::plugin_gpu_gain::{MyInstanceData, Scales};

    pub fn process_images(
        data: &PluginData,
        my_data: &MyInstanceData,
        instance: &ImageEffectInstance,
        scales: Scales,
        source_img: ManagedClipImageForCpu,
        output_img: ManagedClipImageForCpu,
        proc_window: OfxRectI,
    ) -> Result<(), StatusError> {
        let dst_bounds = output_img.bounds();
        // exit gracefully in case of error
        if !(dst_bounds.x1 <= proc_window.x1
            && proc_window.x2 <= dst_bounds.x2
            && dst_bounds.y1 <= proc_window.y1
            && proc_window.y2 <= dst_bounds.y2)
            || proc_window.x1 >= proc_window.x2
            || proc_window.y1 >= proc_window.y2
        {
            return Err(StatusError::Failed);
        }

        let arg = ProcessArgument {
            data,
            instance,
            scales: &scales,
            source_img: &source_img,
            output_img: &output_img,
            proc_window,
        };

        if let Some(multi_thread_suite) = &data.multi_thread_suite {
            let s_mt = &multi_thread_suite.0;
            let n_cpus = (((proc_window.x2 - proc_window.x1).min(4096)
                * (proc_window.y2 - proc_window.y1)) as c_uint
                / 4096)
                .min(unsafe { s_mt.multi_thread_num_cpus() }.unwrap_or(1));
            tracing::debug!("Processing via CPU (with {n_cpus} thread(s))");
            my_data
                .used_renderer
                .store_cpu_multi_thread_suite(n_cpus.try_into().unwrap_or(u16::MAX));
            if n_cpus > 1 {
                unsafe { s_mt.multi_thread(process_thread, n_cpus, &arg) }
                    .ok_or(StatusError::Failed)?;
            } else {
                process_tile(arg);
            }
        } else {
            tracing::debug!("Processing via CPU (No `MultiThreadSuiteV1`)");
            my_data.used_renderer.store_cpu();
            process_tile(arg);
        }

        Ok(())
    }

    struct ProcessArgument<'a> {
        data: &'a PluginData,
        instance: &'a ImageEffectInstance,
        scales: &'a Scales,
        source_img: &'a ManagedClipImageForCpu,
        output_img: &'a ManagedClipImageForCpu,
        proc_window: OfxRectI,
    }

    extern "C" fn process_thread(thread_index: c_uint, thread_max: c_uint, arg: &ProcessArgument) {
        // slice the y range into the number of threads it has
        let dy = (arg.proc_window.y2 - arg.proc_window.y1) as c_uint;
        // the following is equivalent to std::ceil(dy/(double)nThreads);
        let mut h = dy.div_ceil(thread_max);
        if h == 0 {
            // there are more threads than lines to process
            h = 1;
        }
        if thread_index * h >= dy {
            // empty render subwindow
            return;
        }
        let y1 = arg.proc_window.y1 + (thread_index * h) as c_int;
        let step = (thread_index + 1) * h;
        let y2 = arg.proc_window.y1 + (if step < dy { step } else { dy } as c_int);

        let mut win = arg.proc_window;
        win.y1 = y1;
        win.y2 = y2;

        process_tile(ProcessArgument {
            data: arg.data,
            instance: arg.instance,
            scales: arg.scales,
            source_img: arg.source_img,
            output_img: arg.output_img,
            proc_window: win,
        });
    }

    fn process_tile(
        ProcessArgument {
            data,
            instance,
            scales,
            source_img,
            output_img,
            proc_window,
        }: ProcessArgument,
    ) {
        let scales = scales.as_array();

        for y in proc_window.y1..proc_window.y2 {
            if unsafe { instance.abort(&data.image_effect_suite) } {
                return;
            }

            let Some(dst_pix) = output_img.raw_address(proc_window.x1, y) else {
                continue;
            };
            let mut dst_pix = dst_pix as *mut f32;

            for x in proc_window.x1..proc_window.x2 {
                // do we have a source image to scale up
                if let Some(src_pix) = source_img.raw_address(x, y) {
                    let src_pix = src_pix as *const f32;
                    for (i, scale) in scales.iter().enumerate() {
                        unsafe { *dst_pix.add(i) = *src_pix.add(i) * *scale };
                    }
                } else {
                    // no src pixel here, be black and transparent
                    for i in 0..4 {
                        unsafe { *dst_pix.add(i) = 0.0 };
                    }
                };
                // increment the dst pixel
                dst_pix = unsafe { dst_pix.add(4) };
            }
        }
    }
}

struct Scales {
    r_scale: f64,
    g_scale: f64,
    b_scale: f64,
    a_scale: f64,
}
impl Scales {
    fn retrieve_at_time(
        data: &PluginData,
        my_data: &MyInstanceData,
        time: OfxTime,
    ) -> Result<Scales, StatusError> {
        let s_param = &data.parameter_suite.0;

        let mut r_scale = 1.0;
        let mut g_scale = 1.0;
        let mut b_scale = 1.0;
        let mut a_scale = 1.0;

        let component_scales_enabled = unsafe {
            my_data
                .component_scales_enabled_param
                .param_get_value_at_time(s_param, time)
        }
        .ok_or(StatusError::Failed)?;
        if component_scales_enabled {
            unsafe {
                r_scale = my_data
                    .scale_r_param
                    .param_get_value_at_time(s_param, time)
                    .ok_or(StatusError::Failed)?;
                g_scale = my_data
                    .scale_g_param
                    .param_get_value_at_time(s_param, time)
                    .ok_or(StatusError::Failed)?;
                b_scale = my_data
                    .scale_b_param
                    .param_get_value_at_time(s_param, time)
                    .ok_or(StatusError::Failed)?;
                a_scale = my_data
                    .scale_a_param
                    .param_get_value_at_time(s_param, time)
                    .ok_or(StatusError::Failed)?;
            }
        }

        let scale = unsafe { my_data.scale_param.param_get_value_at_time(s_param, time) }
            .ok_or(StatusError::Failed)?;
        r_scale *= scale;
        g_scale *= scale;
        b_scale *= scale;

        Ok(Scales {
            r_scale,
            g_scale,
            b_scale,
            a_scale,
        })
    }

    #[allow(dead_code)]
    fn as_array(&self) -> [f32; 4] {
        [
            self.r_scale as f32,
            self.g_scale as f32,
            self.b_scale as f32,
            self.a_scale as f32,
        ]
    }
}

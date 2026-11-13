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
//! The original example declares support for the general context, but since it
//! strictly uses only a single source clip, general context support is
//! unnecessary. Therefore, this port does not declare support for the general
//! context.
//!
//! ## Known issues
//!
//! Due to limitations of the maintainer(s), only the Metal part of the
//! example is implemented.
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
        OutboundResult, Status, StatusError, StatusReply,
        enums::{
            ImageEffectPropComponents, ImageEffectPropSupportedComponents,
            ImageEffectPropSupportedContexts, ImageEffectPropSupportedPixelDepths,
            ParamPropDoubleType,
        },
    },
    low_plugin::{
        Host, Plugin,
        actions::{
            core::ActionInstanceChangedIn,
            image_effect::{
                ActionDescribeInContextIn, ActionIsIdentityIn, ActionIsIdentityOut, ActionRenderIn,
                ImageEffectAction,
            },
        },
        objects::{ImageEffectDescriptor, ImageEffectInstance, ParamSetDescriptor},
        parameters::{ParamBoolean, ParamDouble, ParamDoublePropertySet},
        suites::{ParameterSuiteV1, PropertySuiteV1},
    },
    sys::{
        core::{OfxTime, kOfxPropLabel},
        image_effect::{
            kOfxImageEffectOutputClipName, kOfxImageEffectSimpleSourceClipName, kOfxTypeClip,
        },
        param::kOfxTypeParameter,
    },
};

use crate::{
    TRACING_DISPATCH,
    definitions::{
        PLUGIN_GPU_GAIN_DESCRIPTION, PLUGIN_GPU_GAIN_IDENTIFIER, PLUGIN_GPU_GAIN_LABEL,
        PLUGINS_GROUPING,
    },
    helpers::{
        internal_utils::{ClipImageManaged, rect_i_from_array, set_labels},
        shared_data::{GuaranteeSend, GuaranteeSendClipInstance, SharedData},
    },
    plugin_gpu_gain::metal_kernel::run_metal_kernel,
};

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

static HOST_BEFORE_ACTION_LOAD: Mutex<Option<GuaranteeSend<Host>>> = Mutex::new(None);
static GLOBAL_DATA: Mutex<Option<(SharedData, Arc<AdditionalSharedData>)>> = Mutex::new(None);

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
}

fn global_data_lockless() -> Result<(SharedData, Arc<AdditionalSharedData>), StatusError> {
    let data = GLOBAL_DATA.lock().map_err(|_| StatusError::Failed)?;
    let data = data.as_ref().ok_or(StatusError::Failed)?;
    Ok((data.0.clone(), data.1.clone()))
}

pub struct PluginExampleGpuGain;
impl Plugin for PluginExampleGpuGain {
    const PLUGIN_IDENTIFIER: &'static CStr = PLUGIN_GPU_GAIN_IDENTIFIER;
    const PLUGIN_VERSION_MAJOR: std::ffi::c_uint = 1;
    const PLUGIN_VERSION_MINOR: std::ffi::c_uint = 0;

    fn set_host(host: Host) {
        match TRACING_DISPATCH.get() {
            Some(dispatch) => {
                let _guard = tracing::dispatcher::set_default(dispatch);
                Self::_set_host(host);
            }
            None => Self::_set_host(host),
        }
    }

    fn main_entry(action: ImageEffectAction) -> OutboundResult {
        match TRACING_DISPATCH.get() {
            Some(dispatch) => {
                let _guard = tracing::dispatcher::set_default(dispatch);
                tracing::debug!("main_entry: Entering Action {:?}", action.sys_action());
                let ret = Self::_main_entry(action);
                tracing::debug!("main_entry: Exiting Action with {ret:?}");
                ret
            }
            None => Self::_main_entry(action),
        }
    }
}

impl PluginExampleGpuGain {
    fn _set_host(host: Host) {
        let mut lock = HOST_BEFORE_ACTION_LOAD
            .lock()
            .expect("Failed to lock HOST_BEFORE_ACTION_LOAD.");
        if lock.is_some() {
            panic!("HOST_BEFORE_ACTION_LOAD has already been set.");
        }
        lock.replace(GuaranteeSend(host));
    }

    fn _main_entry(action: ImageEffectAction) -> OutboundResult {
        match action {
            ImageEffectAction::Load { .. } => action_load(),
            ImageEffectAction::Unload { .. } => action_unload(),
            ImageEffectAction::Describe { handle, .. } => action_describe(handle),
            ImageEffectAction::DescribeInContext {
                handle, in_args, ..
            } => action_describe_in_context(handle, in_args),
            ImageEffectAction::CreateInstance { handle, .. } => action_create_instance(handle),
            ImageEffectAction::DestroyInstance { handle, .. } => action_destroy_instance(handle),
            ImageEffectAction::InstanceChanged {
                handle, in_args, ..
            } => action_instance_changed(handle, in_args),
            ImageEffectAction::IsIdentity {
                handle,
                in_args,
                out_args,
            } => action_is_identity(handle, in_args, out_args),
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
        let additional = { AdditionalSharedData {} };

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

fn action_describe(descriptor: ImageEffectDescriptor) -> OutboundResult {
    let (data, _additional) = global_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;

    let props = unsafe { descriptor.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    unsafe {
        // Basic labels
        set_labels!(&data.property_suite.0, &props, PLUGIN_GPU_GAIN_LABEL)?;
        props
            .set_image_effect_plugin_grouping(s_prop, Some(PLUGINS_GROUPING))
            .ok_or(StatusError::Failed)?;
        if let stat = props.set_plugin_description(s_prop, Some(PLUGIN_GPU_GAIN_DESCRIPTION))
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

        // Setup Metal render capability flags only on Apple system
        #[cfg(target_os = "macos")]
        {
            use openfx::{
                low::enums::ImageEffectPropMetalRenderSupported,
                low_plugin::property_sets::KnownEnumProperty as _,
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
    descriptor: ImageEffectDescriptor,
    _in_args: ActionDescribeInContextIn,
) -> OutboundResult {
    let (data, _additional) = global_data_lockless()?;

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
            set_labels!(&data.property_suite.0, &param_props, c"Components")?;
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
            set_labels!(&data.property_suite.0, &param_props, c"Scale Components")?;
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
            .set_label(s_prop, Some(label))
            .ok_or(StatusError::Failed)?;
        if let Some(parent_name) = parent_name {
            props
                .set_param_parent(s_prop, Some(parent_name))
                .ok_or(StatusError::Failed)?;
        }
    }

    Ok(())
}

fn action_create_instance(instance: ImageEffectInstance) -> OutboundResult {
    let (data, _additional) = global_data_lockless()?;

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
    };
    let my_data_ptr = Box::into_raw(Box::new(my_data)) as *mut c_void;

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

fn action_instance_changed(
    instance: ImageEffectInstance,
    in_args: ActionInstanceChangedIn,
) -> OutboundResult {
    let (data, _additional) = global_data_lockless()?;

    let s_prop = &data.property_suite.0;

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

fn set_enabledness(data: &SharedData, instance: &ImageEffectInstance) -> Result<(), StatusError> {
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
    instance: ImageEffectInstance,
    in_args: ActionIsIdentityIn,
    out_args: ActionIsIdentityOut,
) -> OutboundResult {
    let (data, _additional) = global_data_lockless()?;

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

fn action_render(instance: ImageEffectInstance, in_args: ActionRenderIn) -> OutboundResult {
    let (data, _additional) = global_data_lockless()?;

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;
    let _s_param = &data.parameter_suite.0;

    let instance_props = unsafe { instance.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    let time = unsafe { in_args.get_time(s_prop) }.ok_or(StatusError::Failed)?;
    let render_window =
        unsafe { in_args.get_image_effect_render_window(s_prop) }.ok_or(StatusError::Failed)?;
    let _render_window = rect_i_from_array(&render_window);

    let Some(my_data_ptr) =
        unsafe { instance_props.get_instance_data(s_prop) }.ok_or(StatusError::Failed)?
    else {
        return Err(StatusError::Failed);
    };
    let my_data = unsafe { &*(my_data_ptr.as_ptr() as *const MyInstanceData) };

    let output_img =
        unsafe { ClipImageManaged::try_new_from_clip(&data, &my_data.output_clip.0, time) }?;
    let source_img =
        unsafe { ClipImageManaged::try_new_from_clip(&data, &my_data.source_clip.0, time) }?;

    if output_img.pixel_depth() != source_img.pixel_depth()
        || output_img.n_comps() != source_img.n_comps()
    {
        return Err(StatusError::ErrValue);
    }

    let scales = Scales::retrieve_at_time(&data, my_data, time)?;

    #[cfg(target_os = "macos")]
    if unsafe { in_args.get_image_effect_metal_enabled(s_prop) }.ok_or(StatusError::Failed)? {
        let cmd_q = unsafe {
            in_args
                .get_image_effect_metal_command_queue(s_prop)
                .ok_or(StatusError::Failed)?
        };
        let Some(cmd_q) = cmd_q else {
            return Err(StatusError::Failed);
        };
        process_images_metal(scales, source_img, output_img, cmd_q)?;
        return Ok(Status::OK);
    }

    // TODO!!: CPU fallback
    Err(StatusError::ErrImageFormat)
}

#[cfg(target_os = "macos")]
fn process_images_metal(
    scales: Scales,
    source_img: ClipImageManaged,
    output_img: ClipImageManaged,
    cmd_q: NonNull<c_void>,
) -> Result<(), StatusError> {
    let bounds = source_img.bounds();
    let width = bounds.x2 - bounds.x1;
    let height = bounds.y2 - bounds.y1;

    let input = source_img.data_ptr();
    let output = output_img.data_ptr();

    unsafe { run_metal_kernel(cmd_q, width, height, scales.as_array(), input, output) }?;

    Ok(())
}

struct Scales {
    r_scale: f64,
    g_scale: f64,
    b_scale: f64,
    a_scale: f64,
}
impl Scales {
    fn retrieve_at_time(
        data: &SharedData,
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

    fn as_array(&self) -> [f32; 4] {
        [
            self.r_scale as f32,
            self.g_scale as f32,
            self.b_scale as f32,
            self.a_scale as f32,
        ]
    }
}

//! This example is a loose port of: https://github.com/AcademySoftwareFoundation/openfx/blob/b2d2eea3a64e511bfabb80e56f902d96adf4a05e/Documentation/sources/Guide/Code/Example1/basics.cpp
//!
//! Copyright notice of the original code:
//! ```text
//! Copyright OpenFX and contributors to the OpenFX project.
//! SPDX-License-Identifier: BSD-3-Clause
//! ```
//!
//! Author of the original code: Bruno Nicoletti (2014)
//!
//! This plugin will take you through the basics of creating an OFX plugin. It
//! exercises the basic 'bootstrapping' machinery of the API to allow a host
//! application to instantiate the plugin.
//!
//! The plugin itself does no image processing, it is a 'no-op' effect.

use std::{
    ffi::{CStr, c_void},
    sync::{
        Mutex,
        atomic::{self, AtomicBool, AtomicUsize},
    },
};

use openfx::{
    low::{
        OutboundResult, Status, StatusError, StatusReply,
        enums::{
            ImageEffectPropContext, ImageEffectPropSupportedComponents,
            ImageEffectPropSupportedContexts,
        },
    },
    low_plugin::{
        Host, Plugin,
        actions::image_effect::{
            ActionDescribeInContextIn, ActionIsIdentityOut, ImageEffectAction,
        },
        objects::{ImageEffectDescriptor, ImageEffectInstance},
    },
    sys::core::kOfxPropLabel,
};

use crate::{
    definitions::{PLUGIN_1_BASICS_IDENTIFIER, PLUGIN_1_BASICS_LABEL, PLUGINS_GROUPING},
    helpers::{
        internal_utils::set_labels,
        shared_data::{GuaranteeSend, SharedData},
    },
};

static HOST_BEFORE_ACTION_LOAD: Mutex<Option<GuaranteeSend<Host>>> = Mutex::new(None);
static SHARED_DATA: Mutex<Option<SharedData>> = Mutex::new(None);

// House keeper to make sure we are loaded and unloaded symmetrically
/// 0: not loaded; 1: loading; 2: loaded.
static G_LOADING_STATE: AtomicUsize = AtomicUsize::new(0);
static G_DESCRIBE_CALLED: AtomicBool = AtomicBool::new(false);
static G_DESCRIBE_IN_CONTEXT_CALLED: AtomicBool = AtomicBool::new(false);
static G_NUM_INSTANCES_LIVING: AtomicUsize = AtomicUsize::new(0);

fn is_loaded() -> bool {
    G_LOADING_STATE.load(atomic::Ordering::Relaxed) == 2
}

pub struct PluginExampleBasics;
impl Plugin for PluginExampleBasics {
    /// The unique ID of this plugin.
    const PLUGIN_IDENTIFIER: &'static CStr = PLUGIN_1_BASICS_IDENTIFIER;
    /// The major version number of this plugin.
    const PLUGIN_VERSION_MAJOR: std::ffi::c_uint = 1;
    /// The minor version number of this plugin.
    const PLUGIN_VERSION_MINOR: std::ffi::c_uint = 0;

    /// Call back passed to the host in the [`openfx::sys::core::OfxPlugin`]
    /// struct to set our host pointer.
    ///
    /// This must be called AFTER both `OfxGetNumberOfPlugins` and
    /// `OfxGetPlugin`, but before the `pluginMain` entry point (i.e.,
    /// [`Self::main_entry`]) is ever touched.
    fn set_host(host: Host) {
        let mut lock = HOST_BEFORE_ACTION_LOAD
            .lock()
            .expect("Failed to lock `HOST_BEFORE_ACTION_LOAD`.");
        if lock.is_some() {
            panic!("`HOST_BEFORE_ACTION_LOAD` has already been set.");
        }
        lock.replace(GuaranteeSend(host));
    }

    /// The main entry point function, the host calls this to get the plugin to
    /// do things.
    fn main_entry(action: ImageEffectAction) -> OutboundResult {
        match action {
            // The very first action called on a plugin.
            ImageEffectAction::Load { .. } => action_load(),
            // The very last action called.
            ImageEffectAction::Unload { .. } => action_unload(),
            // the first action called to describe what the plugin does.
            ImageEffectAction::Describe { handle, .. } => action_describe(handle),
            // the second action called to describe what the plugin does.
            ImageEffectAction::DescribeInContext {
                handle, in_args, ..
            } => action_describe_in_context(handle, in_args),
            // the action called when an instance of a plugin is created.
            ImageEffectAction::CreateInstance { handle, .. } => action_create_instance(handle),
            // the action called when an instance of a plugin is destroyed.
            ImageEffectAction::DestroyInstance { handle, .. } => action_destroy_instance(handle),
            // The action called by the host to see if the plugin is currently
            // a "no-op". In this example we are always a no-op.
            ImageEffectAction::IsIdentity { out_args, .. } => action_is_identity(out_args),
            // other actions to take the default value.
            _ => Ok(StatusReply::ReplyDefault.into()),
        }
    }
}

/// The first *action* called after the binary is loaded (bootstrapper
/// functions (e.g., `OfxGetNumberOfPlugins`) are, however, called before any
/// action is called.)
fn action_load() -> OutboundResult {
    if G_LOADING_STATE.fetch_add(1, atomic::Ordering::Relaxed) != 0 {
        panic!("`kOfxActionLoad` called more than once without an intervening `kOfxUnloadAction`.");
    }
    let Some(host) = HOST_BEFORE_ACTION_LOAD
        .lock()
        .map_err(|_| StatusError::Failed)?
        .take()
    else {
        panic!(
            "The `OfxHost` pointer has not been set, it should have been set in `PluginExampleBasics::set_host` before any action is called."
        );
    };

    let mut data = SHARED_DATA.lock().map_err(|_| StatusError::Failed)?;
    if data.is_some() {
        panic!("Shared data has already been initialized.");
    }

    // Inside `SharedData::try_new`, suites are fetched out of the host via it's
    // fetch suite function.
    *data = Some(SharedData::try_new(host)?);

    G_LOADING_STATE.store(2, atomic::Ordering::Relaxed);

    Ok(Status::OK)
}

/// The last action called before a plugin binary is unloaded.
fn action_unload() -> OutboundResult {
    // make sure no instances have been left alive
    if let num = G_NUM_INSTANCES_LIVING.load(atomic::Ordering::Relaxed)
        && num != 0
    {
        eprintln!(
            "`kOfxActionUnload` called while there are still {num} instances of the plugin extant."
        );
    }
    // check that we had a load called first
    if !is_loaded() {
        eprintln!(
            "`kOfxActionUnload` called without a preceding `kOfxActionLoad` call, or the load has not completed."
        );
    }
    G_LOADING_STATE.store(0, atomic::Ordering::Relaxed);

    let mut shared_data = SHARED_DATA.lock().map_err(|_| StatusError::Failed)?;
    if shared_data.take().is_none() {
        Err(StatusError::Failed)
    } else {
        Ok(Status::OK)
    }
}

/// The plugin's basic description routine.
fn action_describe(descriptor: ImageEffectDescriptor) -> OutboundResult {
    // check stuff
    assert!(is_loaded(), "`kOfxActionLoad` has not been called.");
    G_DESCRIBE_CALLED.store(true, atomic::Ordering::Relaxed);

    let data = {
        let data = SHARED_DATA.lock().map_err(|_| StatusError::Failed)?;
        let data = data.as_ref().ok_or(StatusError::Failed)?;
        data.clone()
    };

    let s_prop = &data.property_suite.0;

    // get the property set handle for the plugin
    let props = unsafe { descriptor.get_property_set(&data.image_effect_suite.0) }
        .ok_or(StatusError::Failed)?;

    unsafe {
        // set some labels and the group it belongs to
        // NOTE: The original example only sets `kOfxPropLabel`. Instead, we set
        // all three labels at once so that the plugin's label displays
        // correctly in DaVinci Resolve's Cut tab.
        set_labels(&data.property_suite.0, &props, PLUGIN_1_BASICS_LABEL)?;
        props
            .set_image_effect_plugin_grouping(s_prop, Some(PLUGINS_GROUPING))
            .ok_or(StatusError::Failed)?;

        // define the image effects contexts we can be used in, in this case a
        // simple filter
        props
            .set_image_effect_supported_contexts(
                s_prop,
                &[ImageEffectPropSupportedContexts::Filter.into()],
            )
            .ok_or(StatusError::Failed)?;
    }

    Ok(Status::OK)
}

/// Describe the plugin in context.
fn action_describe_in_context(
    descriptor: ImageEffectDescriptor,
    in_args: ActionDescribeInContextIn,
) -> OutboundResult {
    // check state
    assert!(
        G_DESCRIBE_CALLED.load(atomic::Ordering::Relaxed),
        "`action_describe_in_context` called before `action_describe`."
    );
    G_DESCRIBE_IN_CONTEXT_CALLED.store(true, atomic::Ordering::Relaxed);

    let data = {
        let data = SHARED_DATA.lock().map_err(|_| StatusError::Failed)?;
        let data = data.as_ref().ok_or(StatusError::Failed)?;
        data.clone()
    };

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;

    // get the context from the inArgs handle
    let context = unsafe { in_args.get_image_effect_context(s_prop) }.ok_or(StatusError::Failed)?;
    if context != ImageEffectPropContext::Filter {
        return Err(StatusError::ErrUnsupported);
    }

    // define the mandated single output clip
    let props = unsafe { descriptor.clip_define(s_ifx, c"Output") }.ok_or(StatusError::Failed)?;
    // set the component types we can handle on out output
    (unsafe {
        use ImageEffectPropSupportedComponents as C;
        props.set_image_effect_supported_components(s_prop, &[C::RGBA.into(), C::Alpha.into()])
    })
    .ok_or(StatusError::Failed)?;

    // define the mandated single source clip
    let props = unsafe { descriptor.clip_define(s_ifx, c"Source") }.ok_or(StatusError::Failed)?;
    // set the component types we can handle on our main input
    (unsafe {
        use ImageEffectPropSupportedComponents as C;
        props.set_image_effect_supported_components(s_prop, &[C::RGBA.into(), C::Alpha.into()])
    })
    .ok_or(StatusError::Failed)?;

    Ok(Status::OK)
}

/// Instance construction.
fn action_create_instance(instance: ImageEffectInstance) -> OutboundResult {
    // check stuff
    assert!(
        G_DESCRIBE_IN_CONTEXT_CALLED.load(atomic::Ordering::Relaxed),
        "`action_create_instance` called before `action_describe_in_context`."
    );
    G_NUM_INSTANCES_LIVING.fetch_add(1, atomic::Ordering::Relaxed);

    let data = {
        let data = SHARED_DATA.lock().map_err(|_| StatusError::Failed)?;
        let data = data.as_ref().ok_or(StatusError::Failed)?;
        data.clone()
    };

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;

    let props = unsafe { instance.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    // attach some instance data to the effect handle, it can be anything
    let my_string = Box::new(String::from(
        "This is random instance data that could be anything you want.",
    ));
    let my_string = Box::into_raw(my_string);

    if let r = unsafe {
        // set my private instance data
        props.set_instance_data(s_prop, std::ptr::NonNull::new(my_string.cast::<c_void>()))
    } && !r.is_ok()
    {
        drop(unsafe { Box::from_raw(my_string.cast::<String>()) });
        return Err(StatusError::Failed);
    }

    Ok(Status::OK)
}

/// Instance destruction.
fn action_destroy_instance(instance: ImageEffectInstance) -> OutboundResult {
    G_NUM_INSTANCES_LIVING.fetch_sub(1, atomic::Ordering::Relaxed);

    let data = {
        let data = SHARED_DATA.lock().map_err(|_| StatusError::Failed)?;
        let data = data.as_ref().ok_or(StatusError::Failed)?;
        data.clone()
    };

    let s_prop = &data.property_suite.0;
    let s_ifx = &data.image_effect_suite.0;

    let props = unsafe { instance.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

    // get my private instance data
    let my_string = unsafe { props.get_instance_data(s_prop) }.ok_or(StatusError::Failed)?;
    let Some(my_string) = my_string else {
        tracing::error!("Instance data should not be null.");
        return Err(StatusError::Failed);
    };

    drop(unsafe { Box::from_raw(my_string.as_ptr().cast::<String>()) });

    Ok(Status::OK)
}

/// Are the settings of the effect making it redundant and so not do anything to
/// the image data?
fn action_is_identity(out_args: ActionIsIdentityOut) -> OutboundResult {
    let data = {
        let data = SHARED_DATA.lock().map_err(|_| StatusError::Failed)?;
        let data = data.as_ref().ok_or(StatusError::Failed)?;
        data.clone()
    };

    let s_prop = &data.property_suite.0;

    // we set the name of the input clip to pull data from
    (unsafe { s_prop.prop_set_string(out_args.sys_handle(), kOfxPropLabel, Some(c"Source")) })
        .ok_or(StatusError::Failed)?;

    Ok(Status::OK)
}

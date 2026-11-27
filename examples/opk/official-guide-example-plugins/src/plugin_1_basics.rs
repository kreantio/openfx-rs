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

use std::ffi::CStr;

use openfx::low::{
    StatusError,
    enums::{
        ImageEffectPropContext, ImageEffectPropSupportedComponents,
        ImageEffectPropSupportedContexts,
    },
};
use opk::{actions::CommonActionOutput, plugin::Plugin, plugin_instance::PluginInstance};

use crate::{
    definitions::{PLUGIN_1_BASICS_INFO, PLUGINS_GROUPING},
    internal_utils::set_labels,
};

/// The high-level representation of the [`openfx::sys::core::OfxPlugin`] struct
/// that is passed back to the host application to initiate bootstrapping of
/// plugin communications.
pub struct PluginExampleBasics;
impl Plugin for PluginExampleBasics {
    /// The unique ID of this plugin.
    const PLUGIN_IDENTIFIER: &'static CStr = PLUGIN_1_BASICS_INFO.identifier;
    /// The major version number of this plugin.
    const PLUGIN_VERSION_MAJOR: std::ffi::c_uint = 1;
    /// The minor version number of this plugin.
    const PLUGIN_VERSION_MINOR: std::ffi::c_uint = 0;

    type Instance = PluginExampleBasicsInstance;

    /// The plugin's basic description routine.
    fn handle_describe(
        s: &opk::actions::image_effect::DescribeActionSurroundings,
    ) -> opk::actions::image_effect::DescribeActionResult {
        let descriptor = s.low_image_effect_descriptor();

        let s_prop = s.host().low_property_suite_v1()?;
        let s_ifx = s.host().low_image_effect_suite_v1()?;

        // get the property set object for the plugin
        let props = unsafe { descriptor.get_property_set(s_ifx) }.ok_or(StatusError::Failed)?;

        unsafe {
            // set some labels and the group it belongs to.
            // NOTE: The original example only sets `kOfxPropLabel`. Instead, we set
            // all three labels at once so that the plugin's label displays
            // correctly in DaVinci Resolve's Cut tab.
            set_labels(s_prop, &props, PLUGIN_1_BASICS_INFO.label)?;
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

        Ok(CommonActionOutput::Ok)
    }

    /// Describe the plugin in context.
    fn handle_describe_in_context(
        s: &opk::actions::image_effect::DescribeInContextActionSurroundings,
    ) -> opk::actions::image_effect::DescribeInContextActionResult {
        let descriptor = s.low_image_effect_descriptor();
        let in_args = s.low_in_args();

        let s_prop = s.host().low_property_suite_v1()?;
        let s_ifx = s.host().low_image_effect_suite_v1()?;

        // get the context from the `in_args`
        let context =
            unsafe { in_args.get_image_effect_context(s_prop) }.ok_or(StatusError::Failed)?;
        if context != ImageEffectPropContext::Filter {
            return Err(StatusError::ErrUnsupported.into());
        }

        // define the mandated single output clip
        let props =
            unsafe { descriptor.clip_define(s_ifx, c"Output") }.ok_or(StatusError::Failed)?;
        // set the component types we can handle on out output
        (unsafe {
            use ImageEffectPropSupportedComponents as C;
            props.set_image_effect_supported_components(s_prop, &[C::RGBA.into(), C::Alpha.into()])
        })
        .ok_or(StatusError::Failed)?;

        // define the mandated single source clip
        let props =
            unsafe { descriptor.clip_define(s_ifx, c"Source") }.ok_or(StatusError::Failed)?;
        // set the component types we can handle on our main input
        (unsafe {
            use ImageEffectPropSupportedComponents as C;
            props.set_image_effect_supported_components(s_prop, &[C::RGBA.into(), C::Alpha.into()])
        })
        .ok_or(StatusError::Failed)?;

        Ok(CommonActionOutput::Ok)
    }

    /// Instance construction.
    fn handle_create_instance(
        _s: &opk::actions::image_effect::CreateInstanceActionSurroundings,
        instance_creator: &opk::plugin_instance::PluginInstanceCreator<'_, Self::Instance>,
    ) -> opk::actions::image_effect::CreateInstanceActionResult {
        // attach some instance data to the effect handle, it can be anything
        instance_creator.create(PluginExampleBasicsInstance(String::from(
            "This is random instance data that could be anything you want.",
        )))?;

        Ok(CommonActionOutput::Ok)
    }
}

pub struct PluginExampleBasicsInstance(String);
impl PluginInstance for PluginExampleBasicsInstance {
    /// Instance destruction.
    fn handle_destroy_instance(
        self,
        _s: &opk::actions::image_effect::DestroyInstanceActionSurroundings,
    ) -> opk::actions::image_effect::DestroyInstanceActionResult {
        tracing::info!("My string: {}", self.0);

        Ok(CommonActionOutput::Ok)
    }

    /// Are the settings of the effect making it redundant and so not do anything to
    /// the image data?
    fn handle_is_identity(
        &self,
        #[allow(unused_variables)] s: &opk::actions::image_effect::IsIdentityActionSurroundings,
    ) -> opk::actions::image_effect::IsIdentityActionResult {
        let out_args = s.low_out_args();

        let s_prop = s.host().low_property_suite_v1()?;

        // we set the name of the input clip to pull data from
        unsafe { out_args.set_name(s_prop, Some(c"Source")) }.ok_or(StatusError::Failed)?;

        Ok(CommonActionOutput::Ok)
    }
}

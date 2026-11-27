use std::{
    collections::HashSet,
    ffi::{CStr, c_uint, c_void},
    marker::PhantomData,
    ptr::NonNull,
    sync::{Arc, OnceLock},
};

use openfx::{low::StatusError, low_plugin::actions::image_effect::ImageEffectAction};

use crate::{
    __TRACING_DISPATCH,
    actions::{
        self, CommonActionError, CommonActionOutput, CommonActionResult,
        image_effect::BaseActionSurroundings,
    },
    host::Host,
    plugin_instance::{
        PluginInstance, PluginInstanceCreator, PluginInstanceDestroyer, PluginInstanceRetriever,
    },
};

pub struct PluginManager<T: Plugin> {
    _plugin: PhantomData<T>,

    instance_tracker: PluginInstanceTracker,

    host: OnceLock<Host>,
}

/// `#[derive(Default)]` would require `T: Default` for [`PluginManager<T>`] to
/// implement [`Default`].
impl<T: Plugin> Default for PluginManager<T> {
    fn default() -> Self {
        Self {
            _plugin: PhantomData,
            instance_tracker: PluginInstanceTracker::default(),
            host: OnceLock::new(),
        }
    }
}

impl<T: Plugin> PluginManager<T> {
    pub fn __set_host(&self, host: openfx::low_plugin::Host) {
        match __TRACING_DISPATCH.get() {
            Some(dispatch) => {
                let _guard = tracing::dispatcher::set_default(dispatch);
                self.set_host(host);
            }
            None => {
                self.set_host(host);
            }
        }
    }
    pub fn __main_entry(&self, action: ImageEffectAction) -> openfx::low::OutboundResult {
        match __TRACING_DISPATCH.get() {
            Some(dispatch) => {
                let _guard = tracing::dispatcher::set_default(dispatch);
                self.main_entry(action)
            }
            None => self.main_entry(action),
        }
    }

    fn set_host(&self, host: openfx::low_plugin::Host) {
        if self.host.set(Host::from_low(host)).is_err() {
            #[cfg(feature = "tracing")]
            tracing::warn!("`OfxPlugin::setHost` has been called more than once! Ignoring.");
        }
    }
    fn main_entry(&self, action: ImageEffectAction) -> openfx::low::OutboundResult {
        let Some(host) = self.host.get() else {
            #[cfg(feature = "tracing")]
            tracing::warn!(
                "`OfxPlugin::setHost` has not been called before `OfxPlugin::mainEntry`. Ignoring the incoming action."
            );
            return openfx::low::OutboundResult::Err(StatusError::Failed);
        };
        let s = actions::image_effect::BaseActionSurroundings::new(host, &action);

        let result = self.handle_action(host, &s, &action);

        let result = T::handle_post_all(&s, action, result);

        let status = match result {
            Ok(output) => output.status(),
            Err(err) => err.status(),
        };

        // TODO: Add a convenient function in `openfx`.
        if let Some(status_err) = StatusError::try_from_sys(status.sys()) {
            Err(status_err)
        } else {
            Ok(status)
        }
    }

    fn handle_action(
        &self,
        host: &Host,
        s: &BaseActionSurroundings,
        action: &ImageEffectAction,
    ) -> CommonActionResult {
        match action {
            ImageEffectAction::BeginInstanceChanged {
                handle: _,
                in_args: _,
                sys_out_args: _,
            } => T::handle_begin_instance_changed(s),
            ImageEffectAction::BeginInstanceEdit {
                handle: _,
                sys_in_args: _,
                sys_out_args: _,
            } => T::handle_begin_instance_edit(s),
            ImageEffectAction::CreateInstance {
                handle,
                sys_in_args: _,
                sys_out_args: _,
            } => {
                let s_prop = host
                    .low_property_suite_v1()
                    .map_err(CommonActionError::from)?;
                let s_ifx = host.low_image_effect_suite_v1()?;
                let s = actions::image_effect::CreateInstanceActionSurroundings::new(s, handle);
                let instance_creator =
                    PluginInstanceCreator::new(s_prop, s_ifx, handle, &self.instance_tracker);
                T::handle_create_instance(&s, &instance_creator)
            }
            ImageEffectAction::Describe {
                handle,
                sys_in_args: _,
                sys_out_args: _,
            } => {
                let s = actions::image_effect::DescribeActionSurroundings::new(s, handle);
                T::handle_describe(&s)
            }
            ImageEffectAction::DestroyInstance {
                handle,
                sys_in_args: _,
                sys_out_args: _,
            } => {
                let s_prop = host
                    .low_property_suite_v1()
                    .map_err(CommonActionError::from)?;
                let s_ifx = host.low_image_effect_suite_v1()?;
                let s = actions::image_effect::DestroyInstanceActionSurroundings::new(s, handle);
                let instance_destroyer =
                    PluginInstanceDestroyer::new(s_prop, s_ifx, handle, &self.instance_tracker);
                T::handle_destroy_instance(&s, &instance_destroyer)
            }
            ImageEffectAction::Dialog {
                sys_handle: _,
                sys_in_args: _,
                sys_out_args: _,
            } => T::handle_dialog(s),
            ImageEffectAction::EndInstanceChanged {
                handle: _,
                in_args: _,
                sys_out_args: _,
            } => T::handle_end_instance_changed(s),
            ImageEffectAction::EndInstanceEdit {
                handle: _,
                sys_in_args: _,
                sys_out_args: _,
            } => T::handle_end_instance_edit(s),
            ImageEffectAction::InstanceChanged {
                handle: _,
                in_args: _,
                sys_out_args: _,
            } => T::handle_instance_changed(s),
            ImageEffectAction::Load {
                sys_handle: _,
                sys_in_args: _,
                sys_out_args: _,
            } => T::handle_load(s),
            ImageEffectAction::OpenGLContextAttached {
                handle: _,
                sys_in_args: _,
                sys_out_args: _,
            } => T::handle_opengl_context_attached(s),
            ImageEffectAction::OpenGLContextDetached {
                handle: _,
                sys_in_args: _,
                sys_out_args: _,
            } => T::handle_opengl_context_detached(s),
            ImageEffectAction::PurgeCaches {
                handle: _,
                sys_in_args: _,
                sys_out_args: _,
            } => T::handle_purge_caches(s),
            ImageEffectAction::SyncPrivateData {
                handle: _,
                sys_in_args: _,
                sys_out_args: _,
            } => T::handle_sync_private_data(s),
            ImageEffectAction::Unload {
                sys_handle: _,
                sys_in_args: _,
                sys_out_args: _,
            } => T::handle_unload(s),
            ImageEffectAction::BeginSequenceRender {
                handle: _,
                in_args: _,
                sys_out_args: _,
            } => T::handle_begin_sequence_render(s),
            ImageEffectAction::DescribeInContext {
                handle,
                in_args,
                sys_out_args: _,
            } => {
                let s = actions::image_effect::DescribeInContextActionSurroundings::new(
                    s, handle, in_args,
                );
                T::handle_describe_in_context(&s)
            }
            ImageEffectAction::EndSequenceRender {
                handle: _,
                in_args: _,
                sys_out_args: _,
            } => T::handle_end_sequence_render(s),
            ImageEffectAction::GetClipPreferences {
                handle: _,
                sys_in_args: _,
                out_args: _,
            } => T::handle_get_clip_preferences(s),
            ImageEffectAction::GetFramesNeeded {
                handle: _,
                in_args: _,
                sys_out_args: _,
            } => T::handle_get_frames_needed(s),
            ImageEffectAction::GetOutputColourspace {
                handle: _,
                in_args: _,
                out_args: _,
            } => T::handle_get_output_colourspace(s),
            ImageEffectAction::GetRegionOfDefinition {
                handle: _,
                in_args: _,
                out_args: _,
            } => T::handle_get_region_of_definition(s),
            ImageEffectAction::GetRegionsOfInterest {
                handle: _,
                in_args: _,
                sys_out_args: _,
            } => T::handle_get_regions_of_interest(s),
            ImageEffectAction::GetTimeDomain {
                handle: _,
                sys_in_args: _,
                out_args: _,
            } => T::handle_get_time_domain(s),
            ImageEffectAction::IsIdentity {
                handle,
                in_args,
                out_args,
            } => {
                let s_prop = host
                    .low_property_suite_v1()
                    .map_err(CommonActionError::from)?;
                let s_ifx = host.low_image_effect_suite_v1()?;
                let s = actions::image_effect::IsIdentityActionSurroundings::new(
                    s, handle, in_args, out_args,
                );
                let instance_retriever =
                    PluginInstanceRetriever::new(s_prop, s_ifx, handle, &self.instance_tracker);
                T::handle_is_identity(&s, &instance_retriever)
            }
            ImageEffectAction::Render {
                handle: _,
                in_args: _,
                sys_out_args: _,
            } => T::handle_render(s),
            _ => CommonActionResult::Ok(CommonActionOutput::ReplyDefault),
        }
    }
}

#[derive(Default)]
pub(crate) struct PluginInstanceTracker(Arc<parking_lot::RwLock<HashSet<PluginInstancePointer>>>);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct PluginInstancePointer(NonNull<c_void>);
unsafe impl Send for PluginInstancePointer {}
unsafe impl Sync for PluginInstancePointer {}

impl PluginInstanceTracker {
    pub fn insert(&self, ptr: NonNull<c_void>) {
        let mut tracker = self.0.write();
        tracker.insert(PluginInstancePointer(ptr));
    }
    pub fn remove(&self, ptr: NonNull<c_void>) -> bool {
        let mut tracker = self.0.write();
        tracker.remove(&PluginInstancePointer(ptr))
    }
    pub fn contains(&self, ptr: NonNull<c_void>) -> bool {
        let tracker = self.0.read();
        tracker.contains(&PluginInstancePointer(ptr))
    }
}

pub trait Plugin {
    const PLUGIN_IDENTIFIER: &'static CStr;
    const PLUGIN_VERSION_MAJOR: c_uint;
    const PLUGIN_VERSION_MINOR: c_uint;

    type Instance: PluginInstance;

    fn handle_begin_instance_changed(
        #[allow(unused_variables)]
        s: &actions::image_effect::BeginInstanceChangedActionSurroundings,
    ) -> actions::image_effect::BeginInstanceChangedActionResult {
        Ok(CommonActionOutput::ReplyDefault)
    }
    fn handle_begin_instance_edit(
        #[allow(unused_variables)] s: &actions::image_effect::BeginInstanceEditActionSurroundings,
    ) -> actions::image_effect::BeginInstanceEditActionResult {
        Ok(CommonActionOutput::ReplyDefault)
    }
    fn handle_create_instance(
        s: &actions::image_effect::CreateInstanceActionSurroundings,
        instance_creator: &PluginInstanceCreator<'_, Self::Instance>,
    ) -> actions::image_effect::CreateInstanceActionResult;
    fn handle_describe(
        #[allow(unused_variables)] s: &actions::image_effect::DescribeActionSurroundings,
    ) -> actions::image_effect::DescribeActionResult {
        Ok(CommonActionOutput::ReplyDefault)
    }
    fn handle_destroy_instance(
        #[allow(unused_variables)] s: &actions::image_effect::DestroyInstanceActionSurroundings,
        instance_destroyer: &PluginInstanceDestroyer<'_, Self::Instance>,
    ) -> actions::image_effect::DestroyInstanceActionResult {
        let Some(instance) = instance_destroyer.destroy()? else {
            return Ok(CommonActionOutput::ReplyDefault);
        };
        instance.handle_destroy_instance(s)
    }
    fn handle_dialog(
        #[allow(unused_variables)] s: &actions::image_effect::DialogActionSurroundings,
    ) -> actions::image_effect::DialogActionResult {
        Ok(CommonActionOutput::ReplyDefault)
    }
    fn handle_end_instance_changed(
        #[allow(unused_variables)] s: &actions::image_effect::EndInstanceChangedActionSurroundings,
    ) -> actions::image_effect::EndInstanceChangedActionResult {
        Ok(CommonActionOutput::ReplyDefault)
    }
    fn handle_end_instance_edit(
        #[allow(unused_variables)] s: &actions::image_effect::EndInstanceEditActionSurroundings,
    ) -> actions::image_effect::EndInstanceEditActionResult {
        Ok(CommonActionOutput::ReplyDefault)
    }
    fn handle_instance_changed(
        #[allow(unused_variables)] s: &actions::image_effect::InstanceChangedActionSurroundings,
    ) -> actions::image_effect::InstanceChangedActionResult {
        Ok(CommonActionOutput::ReplyDefault)
    }
    fn handle_load(
        #[allow(unused_variables)] s: &actions::image_effect::LoadActionSurroundings,
    ) -> actions::image_effect::LoadActionResult {
        Ok(CommonActionOutput::ReplyDefault)
    }
    fn handle_opengl_context_attached(
        #[allow(unused_variables)]
        s: &actions::image_effect::OpenGLContextAttachedActionSurroundings,
    ) -> actions::image_effect::OpenGLContextAttachedActionResult {
        Ok(CommonActionOutput::ReplyDefault)
    }
    fn handle_opengl_context_detached(
        #[allow(unused_variables)]
        s: &actions::image_effect::OpenGLContextDetachedActionSurroundings,
    ) -> actions::image_effect::OpenGLContextDetachedActionResult {
        Ok(CommonActionOutput::ReplyDefault)
    }
    fn handle_purge_caches(
        #[allow(unused_variables)] s: &actions::image_effect::PurgeCachesActionSurroundings,
    ) -> actions::image_effect::PurgeCachesActionResult {
        Ok(CommonActionOutput::ReplyDefault)
    }
    fn handle_sync_private_data(
        #[allow(unused_variables)] s: &actions::image_effect::SyncPrivateDataActionSurroundings,
    ) -> actions::image_effect::SyncPrivateDataActionResult {
        Ok(CommonActionOutput::ReplyDefault)
    }
    fn handle_unload(
        #[allow(unused_variables)] s: &actions::image_effect::UnloadActionSurroundings,
    ) -> actions::image_effect::UnloadActionResult {
        Ok(CommonActionOutput::ReplyDefault)
    }
    fn handle_begin_sequence_render(
        #[allow(unused_variables)] s: &actions::image_effect::BeginSequenceRenderActionSurroundings,
    ) -> actions::image_effect::BeginSequenceRenderActionResult {
        Ok(CommonActionOutput::ReplyDefault)
    }
    fn handle_describe_in_context(
        #[allow(unused_variables)] s: &actions::image_effect::DescribeInContextActionSurroundings,
    ) -> actions::image_effect::DescribeInContextActionResult {
        Ok(CommonActionOutput::ReplyDefault)
    }
    fn handle_end_sequence_render(
        #[allow(unused_variables)] s: &actions::image_effect::EndSequenceRenderActionSurroundings,
    ) -> actions::image_effect::EndSequenceRenderActionResult {
        Ok(CommonActionOutput::ReplyDefault)
    }
    fn handle_get_clip_preferences(
        #[allow(unused_variables)] s: &actions::image_effect::GetClipPreferencesActionSurroundings,
    ) -> actions::image_effect::GetClipPreferencesActionResult {
        Ok(CommonActionOutput::ReplyDefault)
    }
    fn handle_get_frames_needed(
        #[allow(unused_variables)] s: &actions::image_effect::GetFramesNeededActionSurroundings,
    ) -> actions::image_effect::GetFramesNeededActionResult {
        Ok(CommonActionOutput::ReplyDefault)
    }
    fn handle_get_output_colourspace(
        #[allow(unused_variables)]
        s: &actions::image_effect::GetOutputColourspaceActionSurroundings,
    ) -> actions::image_effect::GetOutputColourspaceActionResult {
        Ok(CommonActionOutput::ReplyDefault)
    }
    fn handle_get_region_of_definition(
        #[allow(unused_variables)]
        s: &actions::image_effect::GetRegionOfDefinitionActionSurroundings,
    ) -> actions::image_effect::GetRegionOfDefinitionActionResult {
        Ok(CommonActionOutput::ReplyDefault)
    }
    fn handle_get_regions_of_interest(
        #[allow(unused_variables)]
        s: &actions::image_effect::GetRegionsOfInterestActionSurroundings,
    ) -> actions::image_effect::GetRegionsOfInterestActionResult {
        Ok(CommonActionOutput::ReplyDefault)
    }
    fn handle_get_time_domain(
        #[allow(unused_variables)] s: &actions::image_effect::GetTimeDomainActionSurroundings,
    ) -> actions::image_effect::GetTimeDomainActionResult {
        Ok(CommonActionOutput::ReplyDefault)
    }
    fn handle_is_identity(
        #[allow(unused_variables)] s: &actions::image_effect::IsIdentityActionSurroundings,
        instance_retriever: &PluginInstanceRetriever<'_, Self::Instance>,
    ) -> actions::image_effect::IsIdentityActionResult {
        let Some(instance) = instance_retriever.retrieve()? else {
            return Ok(CommonActionOutput::ReplyDefault);
        };
        instance.handle_is_identity(s)
    }
    fn handle_render(
        #[allow(unused_variables)] s: &actions::image_effect::RenderActionSurroundings,
    ) -> actions::image_effect::RenderActionResult {
        Ok(CommonActionOutput::ReplyDefault)
    }

    fn handle_post_all(
        #[allow(unused_variables)] s: &actions::image_effect::BaseActionSurroundings,
        #[allow(unused_variables)] action: ImageEffectAction,
        previous_result: CommonActionResult,
    ) -> CommonActionResult {
        previous_result
    }
}

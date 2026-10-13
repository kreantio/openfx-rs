pub mod pseudo {
    pub type CustomParamInterpFunc = fn(
        handle: *const std::ffi::c_void,
        in_args: CustomParamInterpFuncIn,
        out_args: CustomParamInterpFuncOut,
    ) -> crate::low::Result<()>;
    pub use crate::low_plugin::actions::tmp::tmp_umbrella::CustomParamInterpFuncIn;
    pub use crate::low_plugin::actions::tmp::tmp_umbrella::CustomParamInterpFuncOut;
}
pub mod core {
    pub use crate::low_plugin::actions::tmp::tmp_umbrella::ActionBeginInstanceChangedIn;
    pub use crate::low_plugin::actions::tmp::tmp_umbrella::ActionEndInstanceChangedIn;
    pub use crate::low_plugin::actions::tmp::tmp_umbrella::ActionInstanceChangedIn;
}
pub mod image_effect {
    openfx_internal_macros::low_make_action_enum! {
        ImageEffectAction {
            _/_ core::Load,
            _/_ core::Unload,
            _/_ core::Describe: crate::low_plugin::objects::ImageEffectDescriptor,
            _/_ core::CreateInstance: crate::low_plugin::objects::ImageEffectInstance,
            _/_ core::DestroyInstance: crate::low_plugin::objects::ImageEffectInstance,
            i/_ core::BeginInstanceChanged: crate::low_plugin::objects::ImageEffectInstance,
            i/_ core::EndInstanceChanged: crate::low_plugin::objects::ImageEffectInstance,
            i/_ core::InstanceChanged: crate::low_plugin::objects::ImageEffectInstance,
            _/_ core::PurgeCaches: crate::low_plugin::objects::ImageEffectInstance,
            _/_ core::SyncPrivateData: crate::low_plugin::objects::ImageEffectInstance,
            _/_ core::BeginInstanceEdit: crate::low_plugin::objects::ImageEffectInstance,
            _/_ core::EndInstanceEdit: crate::low_plugin::objects::ImageEffectInstance,
            i/_ BeginSequenceRender: crate::low_plugin::objects::ImageEffectInstance,
            i/_ DescribeInContext: crate::low_plugin::objects::ImageEffectDescriptor,
            i/_ EndSequenceRender: crate::low_plugin::objects::ImageEffectInstance,
            _/o GetClipPreferences: crate::low_plugin::objects::ImageEffectInstance,
            i/o GetFramesNeeded: crate::low_plugin::objects::ImageEffectInstance,
            i/o GetOutputColourspace: crate::low_plugin::objects::ImageEffectInstance,
            i/o GetRegionOfDefinition: crate::low_plugin::objects::ImageEffectInstance,
            i/_ GetRegionsOfInterest: crate::low_plugin::objects::ImageEffectInstance,
            _/o GetTimeDomain: crate::low_plugin::objects::ImageEffectInstance,
            i/_ IsIdentity: crate::low_plugin::objects::ImageEffectInstance,
            i/_ Render: crate::low_plugin::objects::ImageEffectInstance,
        }
    }
    pub use crate::low_plugin::actions::tmp::tmp_umbrella::ActionBeginSequenceRenderIn;
    pub use crate::low_plugin::actions::tmp::tmp_umbrella::ActionDescribeInContextIn;
    pub use crate::low_plugin::actions::tmp::tmp_umbrella::ActionEndSequenceRenderIn;
    pub use crate::low_plugin::actions::tmp::tmp_umbrella::ActionGetClipPreferencesOut;
    pub use crate::low_plugin::actions::tmp::tmp_umbrella::ActionGetFramesNeededIn;
    pub use crate::low_plugin::actions::tmp::tmp_umbrella::ActionGetFramesNeededOut;
    pub use crate::low_plugin::actions::tmp::tmp_umbrella::ActionGetOutputColourspaceIn;
    pub use crate::low_plugin::actions::tmp::tmp_umbrella::ActionGetOutputColourspaceOut;
    pub use crate::low_plugin::actions::tmp::tmp_umbrella::ActionGetRegionOfDefinitionIn;
    pub use crate::low_plugin::actions::tmp::tmp_umbrella::ActionGetRegionOfDefinitionOut;
    pub use crate::low_plugin::actions::tmp::tmp_umbrella::ActionGetRegionsOfInterestIn;
    pub use crate::low_plugin::actions::tmp::tmp_umbrella::ActionGetTimeDomainOut;
    pub use crate::low_plugin::actions::tmp::tmp_umbrella::ActionIsIdentityIn;
    pub use crate::low_plugin::actions::tmp::tmp_umbrella::ActionRenderIn;
}
pub mod interact {
    openfx_internal_macros::low_make_action_enum! {
        InteractAction {
            _/_ core::Describe: crate::low_plugin::objects::InteractDescriptor,
            _/_ core::CreateInstance: crate::low_plugin::objects::InteractInstance,
            _/_ core::DestroyInstance: crate::low_plugin::objects::InteractInstance,
            i/_ Draw: crate::low_plugin::objects::InteractInstance,
            i/_ GainFocus: crate::low_plugin::objects::InteractInstance,
            i/_ KeyDown: crate::low_plugin::objects::InteractInstance,
            i/_ KeyRepeat: crate::low_plugin::objects::InteractInstance,
            i/_ KeyUp: crate::low_plugin::objects::InteractInstance,
            i/_ LoseFocus: crate::low_plugin::objects::InteractInstance,
            i/_ PenDown: crate::low_plugin::objects::InteractInstance,
            i/_ PenMotion: crate::low_plugin::objects::InteractInstance,
            i/_ PenUp: crate::low_plugin::objects::InteractInstance,
        }
    }
    pub use crate::low_plugin::actions::tmp::tmp_umbrella::ActionDrawIn;
    pub use crate::low_plugin::actions::tmp::tmp_umbrella::ActionGainFocusIn;
    pub use crate::low_plugin::actions::tmp::tmp_umbrella::ActionKeyDownIn;
    pub use crate::low_plugin::actions::tmp::tmp_umbrella::ActionKeyRepeatIn;
    pub use crate::low_plugin::actions::tmp::tmp_umbrella::ActionKeyUpIn;
    pub use crate::low_plugin::actions::tmp::tmp_umbrella::ActionLoseFocusIn;
    pub use crate::low_plugin::actions::tmp::tmp_umbrella::ActionPenDownIn;
    pub use crate::low_plugin::actions::tmp::tmp_umbrella::ActionPenMotionIn;
    pub use crate::low_plugin::actions::tmp::tmp_umbrella::ActionPenUpIn;
}

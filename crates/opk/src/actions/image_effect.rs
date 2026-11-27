use std::ffi::c_void;

use openfx::{
    low_plugin::actions::image_effect::ImageEffectAction, sys::core::OfxPropertySetStruct,
};

use crate::{actions::CommonActionResult, host::Host};

pub struct BaseActionSurroundings<'a> {
    host: &'a Host,

    sys_handle: *const c_void,
    sys_in_args: *mut OfxPropertySetStruct,
    sys_out_args: *mut OfxPropertySetStruct,
}

impl<'a> BaseActionSurroundings<'a> {
    pub fn new(host: &'a Host, action: &ImageEffectAction) -> Self {
        Self {
            host,
            sys_handle: action.sys_handle(),
            sys_in_args: action.sys_in_args(),
            sys_out_args: action.sys_out_args(),
        }
    }

    pub fn host(&self) -> &Host {
        self.host
    }

    pub fn sys_handle(&self) -> *const c_void {
        self.sys_handle
    }
    pub fn sys_in_args(&self) -> *mut OfxPropertySetStruct {
        self.sys_in_args
    }
    pub fn sys_out_args(&self) -> *mut OfxPropertySetStruct {
        self.sys_out_args
    }
}

pub type BeginInstanceChangedActionSurroundings<'a> = BaseActionSurroundings<'a>;
pub type BeginInstanceChangedActionResult = CommonActionResult;

pub type BeginInstanceEditActionSurroundings<'a> = BaseActionSurroundings<'a>;
pub type BeginInstanceEditActionResult = CommonActionResult;

pub struct CreateInstanceActionSurroundings<'a> {
    base: &'a BaseActionSurroundings<'a>,

    low_image_effect_instance: &'a openfx::low_plugin::objects::ImageEffectInstance,
}
impl<'a> CreateInstanceActionSurroundings<'a> {
    pub fn new(
        base: &'a BaseActionSurroundings<'a>,
        low_handle: &'a openfx::low_plugin::objects::ImageEffectInstance,
    ) -> Self {
        Self {
            base,
            low_image_effect_instance: low_handle,
        }
    }

    pub fn host(&self) -> &Host {
        self.base.host()
    }

    pub fn low_image_effect_instance(
        &self,
    ) -> &'a openfx::low_plugin::objects::ImageEffectInstance {
        self.low_image_effect_instance
    }
    pub fn sys_in_args(&self) -> *mut OfxPropertySetStruct {
        self.base.sys_in_args()
    }
    pub fn sys_out_args(&self) -> *mut OfxPropertySetStruct {
        self.base.sys_out_args()
    }
}
pub type CreateInstanceActionResult = CommonActionResult;

pub struct DescribeActionSurroundings<'a> {
    base: &'a BaseActionSurroundings<'a>,

    low_image_effect_descriptor: &'a openfx::low_plugin::objects::ImageEffectDescriptor,
}
impl<'a> DescribeActionSurroundings<'a> {
    pub fn new(
        base: &'a BaseActionSurroundings<'a>,
        low_handle: &'a openfx::low_plugin::objects::ImageEffectDescriptor,
    ) -> Self {
        Self {
            base,
            low_image_effect_descriptor: low_handle,
        }
    }

    pub fn host(&self) -> &Host {
        self.base.host()
    }

    pub fn low_image_effect_descriptor(
        &self,
    ) -> &'a openfx::low_plugin::objects::ImageEffectDescriptor {
        self.low_image_effect_descriptor
    }
    pub fn sys_in_args(&self) -> *mut OfxPropertySetStruct {
        self.base.sys_in_args()
    }
    pub fn sys_out_args(&self) -> *mut OfxPropertySetStruct {
        self.base.sys_out_args()
    }
}
pub type DescribeActionResult = CommonActionResult;

pub struct DestroyInstanceActionSurroundings<'a> {
    base: &'a BaseActionSurroundings<'a>,

    low_image_effect_instance: &'a openfx::low_plugin::objects::ImageEffectInstance,
}
impl<'a> DestroyInstanceActionSurroundings<'a> {
    pub fn new(
        base: &'a BaseActionSurroundings<'a>,
        low_handle: &'a openfx::low_plugin::objects::ImageEffectInstance,
    ) -> Self {
        Self {
            base,
            low_image_effect_instance: low_handle,
        }
    }

    pub fn host(&self) -> &Host {
        self.base.host()
    }

    pub fn low_image_effect_instance(
        &self,
    ) -> &'a openfx::low_plugin::objects::ImageEffectInstance {
        self.low_image_effect_instance
    }
    pub fn sys_in_args(&self) -> *mut OfxPropertySetStruct {
        self.base.sys_in_args()
    }
    pub fn sys_out_args(&self) -> *mut OfxPropertySetStruct {
        self.base.sys_out_args()
    }
}
pub type DestroyInstanceActionResult = CommonActionResult;

pub type DialogActionSurroundings<'a> = BaseActionSurroundings<'a>;
pub type DialogActionResult = CommonActionResult;

pub type EndInstanceChangedActionSurroundings<'a> = BaseActionSurroundings<'a>;
pub type EndInstanceChangedActionResult = CommonActionResult;

pub type EndInstanceEditActionSurroundings<'a> = BaseActionSurroundings<'a>;
pub type EndInstanceEditActionResult = CommonActionResult;

pub type InstanceChangedActionSurroundings<'a> = BaseActionSurroundings<'a>;
pub type InstanceChangedActionResult = CommonActionResult;

pub type LoadActionSurroundings<'a> = BaseActionSurroundings<'a>;
pub type LoadActionResult = CommonActionResult;

pub type OpenGLContextAttachedActionSurroundings<'a> = BaseActionSurroundings<'a>;
pub type OpenGLContextAttachedActionResult = CommonActionResult;

pub type OpenGLContextDetachedActionSurroundings<'a> = BaseActionSurroundings<'a>;
pub type OpenGLContextDetachedActionResult = CommonActionResult;

pub type PurgeCachesActionSurroundings<'a> = BaseActionSurroundings<'a>;
pub type PurgeCachesActionResult = CommonActionResult;

pub type SyncPrivateDataActionSurroundings<'a> = BaseActionSurroundings<'a>;
pub type SyncPrivateDataActionResult = CommonActionResult;

pub type UnloadActionSurroundings<'a> = BaseActionSurroundings<'a>;
pub type UnloadActionResult = CommonActionResult;

pub type BeginSequenceRenderActionSurroundings<'a> = BaseActionSurroundings<'a>;
pub type BeginSequenceRenderActionResult = CommonActionResult;

pub struct DescribeInContextActionSurroundings<'a> {
    base: &'a BaseActionSurroundings<'a>,

    low_image_effect_descriptor: &'a openfx::low_plugin::objects::ImageEffectDescriptor,
    low_in_args: &'a openfx::low_plugin::actions::image_effect::ActionDescribeInContextIn,
}
impl<'a> DescribeInContextActionSurroundings<'a> {
    pub fn new(
        base: &'a BaseActionSurroundings<'a>,
        low_handle: &'a openfx::low_plugin::objects::ImageEffectDescriptor,
        low_in_args: &'a openfx::low_plugin::actions::image_effect::ActionDescribeInContextIn,
    ) -> Self {
        Self {
            base,
            low_image_effect_descriptor: low_handle,
            low_in_args,
        }
    }

    pub fn host(&self) -> &Host {
        self.base.host()
    }

    pub fn low_image_effect_descriptor(
        &self,
    ) -> &'a openfx::low_plugin::objects::ImageEffectDescriptor {
        self.low_image_effect_descriptor
    }
    pub fn low_in_args(
        &self,
    ) -> &'a openfx::low_plugin::actions::image_effect::ActionDescribeInContextIn {
        self.low_in_args
    }
    pub fn sys_out_args(&self) -> *mut OfxPropertySetStruct {
        self.base.sys_out_args()
    }
}
pub type DescribeInContextActionResult = CommonActionResult;

pub type EndSequenceRenderActionSurroundings<'a> = BaseActionSurroundings<'a>;
pub type EndSequenceRenderActionResult = CommonActionResult;

pub type GetClipPreferencesActionSurroundings<'a> = BaseActionSurroundings<'a>;
pub type GetClipPreferencesActionResult = CommonActionResult;

pub type GetFramesNeededActionSurroundings<'a> = BaseActionSurroundings<'a>;
pub type GetFramesNeededActionResult = CommonActionResult;

pub type GetOutputColourspaceActionSurroundings<'a> = BaseActionSurroundings<'a>;
pub type GetOutputColourspaceActionResult = CommonActionResult;

pub type GetRegionOfDefinitionActionSurroundings<'a> = BaseActionSurroundings<'a>;
pub type GetRegionOfDefinitionActionResult = CommonActionResult;

pub type GetRegionsOfInterestActionSurroundings<'a> = BaseActionSurroundings<'a>;
pub type GetRegionsOfInterestActionResult = CommonActionResult;

pub type GetTimeDomainActionSurroundings<'a> = BaseActionSurroundings<'a>;
pub type GetTimeDomainActionResult = CommonActionResult;

pub struct IsIdentityActionSurroundings<'a> {
    base: &'a BaseActionSurroundings<'a>,

    low_image_effect_instance: &'a openfx::low_plugin::objects::ImageEffectInstance,
    low_in_args: &'a openfx::low_plugin::actions::image_effect::ActionIsIdentityIn,
    low_out_args: &'a openfx::low_plugin::actions::image_effect::ActionIsIdentityOut,
}
impl<'a> IsIdentityActionSurroundings<'a> {
    pub fn new(
        base: &'a BaseActionSurroundings<'a>,
        low_handle: &'a openfx::low_plugin::objects::ImageEffectInstance,
        low_in_args: &'a openfx::low_plugin::actions::image_effect::ActionIsIdentityIn,
        low_out_args: &'a openfx::low_plugin::actions::image_effect::ActionIsIdentityOut,
    ) -> Self {
        Self {
            base,
            low_image_effect_instance: low_handle,
            low_in_args,
            low_out_args,
        }
    }

    pub fn host(&self) -> &Host {
        self.base.host()
    }

    pub fn low_image_effect_instance(
        &self,
    ) -> &'a openfx::low_plugin::objects::ImageEffectInstance {
        self.low_image_effect_instance
    }
    pub fn low_in_args(&self) -> &'a openfx::low_plugin::actions::image_effect::ActionIsIdentityIn {
        self.low_in_args
    }
    pub fn low_out_args(
        &self,
    ) -> &'a openfx::low_plugin::actions::image_effect::ActionIsIdentityOut {
        self.low_out_args
    }
}
pub type IsIdentityActionResult = CommonActionResult;

pub type RenderActionSurroundings<'a> = BaseActionSurroundings<'a>;
pub type RenderActionResult = CommonActionResult;

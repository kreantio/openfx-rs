use std::ffi::c_void;

use openfx::{
    low_plugin::actions::image_effect::ImageEffectAction, sys::core::OfxPropertySetStruct,
};

use crate::{actions::CommonActionResult, host::Host};

pub struct CommonActionSurroundings<'a> {
    host: &'a Host,

    sys_handle: *const c_void,
    sys_in_args: *mut OfxPropertySetStruct,
    sys_out_args: *mut OfxPropertySetStruct,
}

impl<'a> CommonActionSurroundings<'a> {
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

pub type BeginInstanceChangedSurroundings<'a> = CommonActionSurroundings<'a>;
pub type BeginInstanceChangedResult = CommonActionResult;

pub type BeginInstanceEditSurroundings<'a> = CommonActionSurroundings<'a>;
pub type BeginInstanceEditResult = CommonActionResult;

pub struct CreateInstanceSurroundings<'a> {
    common: &'a CommonActionSurroundings<'a>,

    low_image_effect_instance: &'a openfx::low_plugin::objects::ImageEffectInstance,
}
impl<'a> CreateInstanceSurroundings<'a> {
    pub fn new(
        s: &'a CommonActionSurroundings<'a>,
        low_handle: &'a openfx::low_plugin::objects::ImageEffectInstance,
    ) -> Self {
        Self {
            common: s,
            low_image_effect_instance: low_handle,
        }
    }

    pub fn host(&self) -> &Host {
        self.common.host()
    }

    pub fn low_image_effect_instance(
        &self,
    ) -> &'a openfx::low_plugin::objects::ImageEffectInstance {
        self.low_image_effect_instance
    }
    pub fn sys_in_args(&self) -> *mut OfxPropertySetStruct {
        self.common.sys_in_args()
    }
    pub fn sys_out_args(&self) -> *mut OfxPropertySetStruct {
        self.common.sys_out_args()
    }
}
pub type CreateInstanceResult = CommonActionResult;

pub struct DescribeSurroundings<'a> {
    common: &'a CommonActionSurroundings<'a>,

    low_image_effect_descriptor: &'a openfx::low_plugin::objects::ImageEffectDescriptor,
}
impl<'a> DescribeSurroundings<'a> {
    pub fn new(
        s: &'a CommonActionSurroundings<'a>,
        low_handle: &'a openfx::low_plugin::objects::ImageEffectDescriptor,
    ) -> Self {
        Self {
            common: s,
            low_image_effect_descriptor: low_handle,
        }
    }

    pub fn host(&self) -> &Host {
        self.common.host()
    }

    pub fn low_image_effect_descriptor(
        &self,
    ) -> &'a openfx::low_plugin::objects::ImageEffectDescriptor {
        self.low_image_effect_descriptor
    }
    pub fn sys_in_args(&self) -> *mut OfxPropertySetStruct {
        self.common.sys_in_args()
    }
    pub fn sys_out_args(&self) -> *mut OfxPropertySetStruct {
        self.common.sys_out_args()
    }
}
pub type DescribeResult = CommonActionResult;

pub struct DestroyInstanceSurroundings<'a> {
    common: &'a CommonActionSurroundings<'a>,

    low_image_effect_instance: &'a openfx::low_plugin::objects::ImageEffectInstance,
}
impl<'a> DestroyInstanceSurroundings<'a> {
    pub fn new(
        s: &'a CommonActionSurroundings<'a>,
        low_handle: &'a openfx::low_plugin::objects::ImageEffectInstance,
    ) -> Self {
        Self {
            common: s,
            low_image_effect_instance: low_handle,
        }
    }

    pub fn host(&self) -> &Host {
        self.common.host()
    }

    pub fn low_image_effect_instance(
        &self,
    ) -> &'a openfx::low_plugin::objects::ImageEffectInstance {
        self.low_image_effect_instance
    }
    pub fn sys_in_args(&self) -> *mut OfxPropertySetStruct {
        self.common.sys_in_args()
    }
    pub fn sys_out_args(&self) -> *mut OfxPropertySetStruct {
        self.common.sys_out_args()
    }
}
pub type DestroyInstanceResult = CommonActionResult;

pub type DialogSurroundings<'a> = CommonActionSurroundings<'a>;
pub type DialogResult = CommonActionResult;

pub type EndInstanceChangedSurroundings<'a> = CommonActionSurroundings<'a>;
pub type EndInstanceChangedResult = CommonActionResult;

pub type EndInstanceEditSurroundings<'a> = CommonActionSurroundings<'a>;
pub type EndInstanceEditResult = CommonActionResult;

pub type InstanceChangedSurroundings<'a> = CommonActionSurroundings<'a>;
pub type InstanceChangedResult = CommonActionResult;

pub type LoadSurroundings<'a> = CommonActionSurroundings<'a>;
pub type LoadResult = CommonActionResult;

pub type OpenGLContextAttachedSurroundings<'a> = CommonActionSurroundings<'a>;
pub type OpenGLContextAttachedResult = CommonActionResult;

pub type OpenGLContextDetachedSurroundings<'a> = CommonActionSurroundings<'a>;
pub type OpenGLContextDetachedResult = CommonActionResult;

pub type PurgeCachesSurroundings<'a> = CommonActionSurroundings<'a>;
pub type PurgeCachesResult = CommonActionResult;

pub type SyncPrivateDataSurroundings<'a> = CommonActionSurroundings<'a>;
pub type SyncPrivateDataResult = CommonActionResult;

pub type UnloadSurroundings<'a> = CommonActionSurroundings<'a>;
pub type UnloadResult = CommonActionResult;

pub type BeginSequenceRenderSurroundings<'a> = CommonActionSurroundings<'a>;
pub type BeginSequenceRenderResult = CommonActionResult;

pub struct DescribeInContextSurroundings<'a> {
    common: &'a CommonActionSurroundings<'a>,

    low_image_effect_descriptor: &'a openfx::low_plugin::objects::ImageEffectDescriptor,
    low_in_args: &'a openfx::low_plugin::actions::image_effect::ActionDescribeInContextIn,
}
impl<'a> DescribeInContextSurroundings<'a> {
    pub fn new(
        s: &'a CommonActionSurroundings<'a>,
        low_handle: &'a openfx::low_plugin::objects::ImageEffectDescriptor,
        low_in_args: &'a openfx::low_plugin::actions::image_effect::ActionDescribeInContextIn,
    ) -> Self {
        Self {
            common: s,
            low_image_effect_descriptor: low_handle,
            low_in_args,
        }
    }

    pub fn host(&self) -> &Host {
        self.common.host()
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
        self.common.sys_out_args()
    }
}
pub type DescribeInContextResult = CommonActionResult;

pub type EndSequenceRenderSurroundings<'a> = CommonActionSurroundings<'a>;
pub type EndSequenceRenderResult = CommonActionResult;

pub type GetClipPreferencesSurroundings<'a> = CommonActionSurroundings<'a>;
pub type GetClipPreferencesResult = CommonActionResult;

pub type GetFramesNeededSurroundings<'a> = CommonActionSurroundings<'a>;
pub type GetFramesNeededResult = CommonActionResult;

pub type GetOutputColourspaceSurroundings<'a> = CommonActionSurroundings<'a>;
pub type GetOutputColourspaceResult = CommonActionResult;

pub type GetRegionOfDefinitionSurroundings<'a> = CommonActionSurroundings<'a>;
pub type GetRegionOfDefinitionResult = CommonActionResult;

pub type GetRegionsOfInterestSurroundings<'a> = CommonActionSurroundings<'a>;
pub type GetRegionsOfInterestResult = CommonActionResult;

pub type GetTimeDomainSurroundings<'a> = CommonActionSurroundings<'a>;
pub type GetTimeDomainResult = CommonActionResult;

pub struct IsIdentitySurroundings<'a> {
    common: &'a CommonActionSurroundings<'a>,

    low_image_effect_instance: &'a openfx::low_plugin::objects::ImageEffectInstance,
    low_in_args: &'a openfx::low_plugin::actions::image_effect::ActionIsIdentityIn,
    low_out_args: &'a openfx::low_plugin::actions::image_effect::ActionIsIdentityOut,
}
impl<'a> IsIdentitySurroundings<'a> {
    pub fn new(
        s: &'a CommonActionSurroundings<'a>,
        low_handle: &'a openfx::low_plugin::objects::ImageEffectInstance,
        low_in_args: &'a openfx::low_plugin::actions::image_effect::ActionIsIdentityIn,
        low_out_args: &'a openfx::low_plugin::actions::image_effect::ActionIsIdentityOut,
    ) -> Self {
        Self {
            common: s,
            low_image_effect_instance: low_handle,
            low_in_args,
            low_out_args,
        }
    }

    pub fn host(&self) -> &Host {
        self.common.host()
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
pub type IsIdentityResult = CommonActionResult;

pub type RenderSurroundings<'a> = CommonActionSurroundings<'a>;
pub type RenderResult = CommonActionResult;

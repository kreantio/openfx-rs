pub mod pseudo {
    pub type CustomParamInterpFunc = fn(
        handle: *const std::ffi::c_void,
        in_args: CustomParamInterpFuncIn,
        out_args: CustomParamInterpFuncOut,
    ) -> crate::low::Result<()>;
    openfx_internal_macros::low_make_property_set_structs! {
        CustomParamInterpFuncIn {
            r/_ param_custom_value: [String; 2] @OfxParamPropCustomValue;
            r/_ param_interpolation_amount: Double @OfxParamPropInterpolationAmount;
            r/_ param_interpolation_time: [Double; 2] @OfxParamPropInterpolationTime;
        }
        CustomParamInterpFuncOut {
            _/w param_custom_value: [String; 2] @OfxParamPropCustomValue;
            _/w param_interpolation_time: [Double; 2] @OfxParamPropInterpolationTime;
        }
    }
}
pub mod core {
    openfx_internal_macros::low_make_property_set_structs! {
        ActionBeginInstanceChangedIn {
            r/_ image_effect_thumbnail_render: Enum(ImageEffectPropThumbnailRender) @OfxImageEffectPropThumbnailRender;
            r/_ change_reason: Enum(PropChangeReason) @OfxPropChangeReason;
        }
        ActionEndInstanceChangedIn {
            r/_ change_reason: Enum(PropChangeReason) @OfxPropChangeReason;
        }
        ActionInstanceChangedIn {
            r/_ image_effect_render_scale: [Double; 2] @OfxImageEffectPropRenderScale;
            r/_ image_effect_thumbnail_render: Enum(ImageEffectPropThumbnailRender) @OfxImageEffectPropThumbnailRender;
            r/_ change_reason: Enum(PropChangeReason) @OfxPropChangeReason;
            r/_ name: String @OfxPropName;
            r/_ time: Double @OfxPropTime;
            r/_ r#type: String @OfxPropType;
        }
    }
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
    openfx_internal_macros::low_make_property_set_structs! {
        ActionBeginSequenceRenderIn {
            r/_ image_effect_cuda_enabled: Bool @OfxImageEffectPropCudaEnabled;
            r/_ image_effect_cuda_render_supported: Enum(ImageEffectPropCudaRenderSupported) @OfxImageEffectPropCudaRenderSupported;
            r/_ image_effect_cuda_stream: Pointer @OfxImageEffectPropCudaStream;
            r/_ image_effect_cuda_stream_supported: Enum(ImageEffectPropCudaStreamSupported) @OfxImageEffectPropCudaStreamSupported;
            r/_ image_effect_frame_range: [Double; 2] @OfxImageEffectPropFrameRange;
            r/_ image_effect_frame_step: Double @OfxImageEffectPropFrameStep;
            r/_ image_effect_interactive_render_status: Bool @OfxImageEffectPropInteractiveRenderStatus;
            r/_ image_effect_metal_command_queue: Pointer @OfxImageEffectPropMetalCommandQueue;
            r/_ image_effect_metal_enabled: Bool @OfxImageEffectPropMetalEnabled;
            r/_ image_effect_metal_render_supported: Enum(ImageEffectPropMetalRenderSupported) @OfxImageEffectPropMetalRenderSupported;
            r/_ image_effect_no_spatial_awareness: Enum(ImageEffectPropNoSpatialAwareness) @OfxImageEffectPropNoSpatialAwareness;
            r/_ image_effect_open_cl_command_queue: Pointer @OfxImageEffectPropOpenCLCommandQueue;
            r/_ image_effect_open_cl_enabled: Bool @OfxImageEffectPropOpenCLEnabled;
            r/_ image_effect_open_cl_image: Pointer @OfxImageEffectPropOpenCLImage;
            r/_ image_effect_open_cl_render_supported: Enum(ImageEffectPropOpenCLRenderSupported) @OfxImageEffectPropOpenCLRenderSupported;
            r/_ image_effect_open_cl_supported: Enum(ImageEffectPropOpenCLSupported) @OfxImageEffectPropOpenCLSupported;
            r/_ image_effect_open_gl_enabled: Bool @OfxImageEffectPropOpenGLEnabled;
            r/_ image_effect_open_gl_texture_index: Int @OfxImageEffectPropOpenGLTextureIndex;
            r/_ image_effect_open_gl_texture_target: Int @OfxImageEffectPropOpenGLTextureTarget;
            r/_ image_effect_render_scale: [Double; 2] @OfxImageEffectPropRenderScale;
            r/_ image_effect_sequential_render_status: Bool @OfxImageEffectPropSequentialRenderStatus;
            r/_ image_effect_thumbnail_render: Enum(ImageEffectPropThumbnailRender) @OfxImageEffectPropThumbnailRender;
            r/_ is_interactive: Bool @OfxPropIsInteractive;
        }
        ActionDescribeInContextIn {
            r/_ image_effect_context: Enum(ImageEffectPropContext) @OfxImageEffectPropContext;
        }
        ActionEndSequenceRenderIn {
            r/_ image_effect_cuda_enabled: Bool @OfxImageEffectPropCudaEnabled;
            r/_ image_effect_cuda_render_supported: Enum(ImageEffectPropCudaRenderSupported) @OfxImageEffectPropCudaRenderSupported;
            r/_ image_effect_cuda_stream: Pointer @OfxImageEffectPropCudaStream;
            r/_ image_effect_cuda_stream_supported: Enum(ImageEffectPropCudaStreamSupported) @OfxImageEffectPropCudaStreamSupported;
            r/_ image_effect_frame_range: [Double; 2] @OfxImageEffectPropFrameRange;
            r/_ image_effect_frame_step: Double @OfxImageEffectPropFrameStep;
            r/_ image_effect_interactive_render_status: Bool @OfxImageEffectPropInteractiveRenderStatus;
            r/_ image_effect_metal_command_queue: Pointer @OfxImageEffectPropMetalCommandQueue;
            r/_ image_effect_metal_enabled: Bool @OfxImageEffectPropMetalEnabled;
            r/_ image_effect_metal_render_supported: Enum(ImageEffectPropMetalRenderSupported) @OfxImageEffectPropMetalRenderSupported;
            r/_ image_effect_open_cl_command_queue: Pointer @OfxImageEffectPropOpenCLCommandQueue;
            r/_ image_effect_open_cl_enabled: Bool @OfxImageEffectPropOpenCLEnabled;
            r/_ image_effect_open_cl_image: Pointer @OfxImageEffectPropOpenCLImage;
            r/_ image_effect_open_cl_render_supported: Enum(ImageEffectPropOpenCLRenderSupported) @OfxImageEffectPropOpenCLRenderSupported;
            r/_ image_effect_open_cl_supported: Enum(ImageEffectPropOpenCLSupported) @OfxImageEffectPropOpenCLSupported;
            r/_ image_effect_open_gl_enabled: Bool @OfxImageEffectPropOpenGLEnabled;
            r/_ image_effect_open_gl_texture_index: Int @OfxImageEffectPropOpenGLTextureIndex;
            r/_ image_effect_open_gl_texture_target: Int @OfxImageEffectPropOpenGLTextureTarget;
            r/_ image_effect_render_scale: [Double; 2] @OfxImageEffectPropRenderScale;
            r/_ image_effect_sequential_render_status: Bool @OfxImageEffectPropSequentialRenderStatus;
            r/_ is_interactive: Bool @OfxPropIsInteractive;
        }
        ActionGetClipPreferencesOut {
            _/w image_clip_continuous_samples: Bool @OfxImageClipPropContinuousSamples;
            _/w image_clip_field_order: Enum(ImageClipPropFieldOrder) @OfxImageClipPropFieldOrder;
            _/w image_effect_frame_varying: Bool @OfxImageEffectFrameVarying;
            _/w image_effect_frame_rate: Double @OfxImageEffectPropFrameRate;
            _/w image_effect_pre_multiplication: Enum(ImageEffectPropPreMultiplication) @OfxImageEffectPropPreMultiplication;
        }
        ActionGetFramesNeededIn {
            r/_ image_effect_thumbnail_render: Enum(ImageEffectPropThumbnailRender) @OfxImageEffectPropThumbnailRender;
            r/_ time: Double @OfxPropTime;
        }
        ActionGetFramesNeededOut {
            _/w image_effect_frame_range: [Double; 2] @OfxImageEffectPropFrameRange;
        }
        ActionGetOutputColourspaceIn {
            r/_ image_clip_preferred_colourspaces: [String] @OfxImageClipPropPreferredColourspaces;
        }
        ActionGetOutputColourspaceOut {
            _/w image_clip_colourspace: String @OfxImageClipPropColourspace;
        }
        ActionGetRegionOfDefinitionIn {
            r/_ image_effect_render_scale: [Double; 2] @OfxImageEffectPropRenderScale;
            r/_ image_effect_thumbnail_render: Enum(ImageEffectPropThumbnailRender) @OfxImageEffectPropThumbnailRender;
            r/_ time: Double @OfxPropTime;
        }
        ActionGetRegionOfDefinitionOut {
            _/w image_effect_region_of_definition: [Double; 4] @OfxImageEffectPropRegionOfDefinition;
        }
        ActionGetRegionsOfInterestIn {
            r/_ image_effect_region_of_interest: [Double; 4] @OfxImageEffectPropRegionOfInterest;
            r/_ image_effect_render_scale: [Double; 2] @OfxImageEffectPropRenderScale;
            r/_ image_effect_thumbnail_render: Enum(ImageEffectPropThumbnailRender) @OfxImageEffectPropThumbnailRender;
            r/_ time: Double @OfxPropTime;
        }
        ActionGetTimeDomainOut {
            _/w image_effect_frame_range: [Double; 2] @OfxImageEffectPropFrameRange;
        }
        ActionIsIdentityIn {
            r/_ image_effect_field_to_render: Enum(ImageEffectPropFieldToRender) @OfxImageEffectPropFieldToRender;
            r/_ image_effect_render_scale: [Double; 2] @OfxImageEffectPropRenderScale;
            r/_ image_effect_render_window: [Int; 4] @OfxImageEffectPropRenderWindow;
            r/_ image_effect_thumbnail_render: Enum(ImageEffectPropThumbnailRender) @OfxImageEffectPropThumbnailRender;
            r/_ time: Double @OfxPropTime;
        }
        ActionRenderIn {
            r/_ image_effect_cuda_enabled: Bool @OfxImageEffectPropCudaEnabled;
            r/_ image_effect_cuda_render_supported: Enum(ImageEffectPropCudaRenderSupported) @OfxImageEffectPropCudaRenderSupported;
            r/_ image_effect_cuda_stream: Pointer @OfxImageEffectPropCudaStream;
            r/_ image_effect_cuda_stream_supported: Enum(ImageEffectPropCudaStreamSupported) @OfxImageEffectPropCudaStreamSupported;
            r/_ image_effect_interactive_render_status: Bool @OfxImageEffectPropInteractiveRenderStatus;
            r/_ image_effect_metal_command_queue: Pointer @OfxImageEffectPropMetalCommandQueue;
            r/_ image_effect_metal_enabled: Bool @OfxImageEffectPropMetalEnabled;
            r/_ image_effect_metal_render_supported: Enum(ImageEffectPropMetalRenderSupported) @OfxImageEffectPropMetalRenderSupported;
            r/_ image_effect_no_spatial_awareness: Enum(ImageEffectPropNoSpatialAwareness) @OfxImageEffectPropNoSpatialAwareness;
            r/_ image_effect_open_cl_command_queue: Pointer @OfxImageEffectPropOpenCLCommandQueue;
            r/_ image_effect_open_cl_enabled: Bool @OfxImageEffectPropOpenCLEnabled;
            r/_ image_effect_open_cl_image: Pointer @OfxImageEffectPropOpenCLImage;
            r/_ image_effect_open_cl_render_supported: Enum(ImageEffectPropOpenCLRenderSupported) @OfxImageEffectPropOpenCLRenderSupported;
            r/_ image_effect_open_cl_supported: Enum(ImageEffectPropOpenCLSupported) @OfxImageEffectPropOpenCLSupported;
            r/_ image_effect_open_gl_enabled: Bool @OfxImageEffectPropOpenGLEnabled;
            r/_ image_effect_open_gl_texture_index: Int @OfxImageEffectPropOpenGLTextureIndex;
            r/_ image_effect_open_gl_texture_target: Int @OfxImageEffectPropOpenGLTextureTarget;
            r/_ image_effect_render_quality_draft: Bool @OfxImageEffectPropRenderQualityDraft;
            r/_ image_effect_sequential_render_status: Bool @OfxImageEffectPropSequentialRenderStatus;
            r/_ image_effect_thumbnail_render: Enum(ImageEffectPropThumbnailRender) @OfxImageEffectPropThumbnailRender;
            r/_ time: Double @OfxPropTime;
        }
    }
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
    openfx_internal_macros::low_make_property_set_structs! {
        ActionDrawIn {
            r/_ image_effect_render_scale: [Double; 2] @OfxImageEffectPropRenderScale;
            r/_ interact_background_colour: [Double; 3] @OfxInteractPropBackgroundColour;
            r/_ interact_draw_context: Pointer @OfxInteractPropDrawContext;
            r/_ interact_pixel_scale: [Double; 2] @OfxInteractPropPixelScale;
            r/_ effect_instance: Pointer @OfxPropEffectInstance;
            r/_ time: Double @OfxPropTime;
        }
        ActionGainFocusIn {
            r/_ image_effect_render_scale: [Double; 2] @OfxImageEffectPropRenderScale;
            r/_ interact_background_colour: [Double; 3] @OfxInteractPropBackgroundColour;
            r/_ interact_pixel_scale: [Double; 2] @OfxInteractPropPixelScale;
            r/_ effect_instance: Pointer @OfxPropEffectInstance;
            r/_ time: Double @OfxPropTime;
        }
        ActionKeyDownIn {
            r/_ image_effect_render_scale: [Double; 2] @OfxImageEffectPropRenderScale;
            r/_ effect_instance: Pointer @OfxPropEffectInstance;
            r/_ key_string: String @OfxPropKeyString;
            r/_ key_sym: Int @OfxPropKeySym;
            r/_ time: Double @OfxPropTime;
        }
        ActionKeyRepeatIn {
            r/_ image_effect_render_scale: [Double; 2] @OfxImageEffectPropRenderScale;
            r/_ effect_instance: Pointer @OfxPropEffectInstance;
            r/_ key_string: String @OfxPropKeyString;
            r/_ key_sym: Int @OfxPropKeySym;
            r/_ time: Double @OfxPropTime;
        }
        ActionKeyUpIn {
            r/_ image_effect_render_scale: [Double; 2] @OfxImageEffectPropRenderScale;
            r/_ effect_instance: Pointer @OfxPropEffectInstance;
            r/_ key_string: String @OfxPropKeyString;
            r/_ key_sym: Int @OfxPropKeySym;
            r/_ time: Double @OfxPropTime;
        }
        ActionLoseFocusIn {
            r/_ image_effect_render_scale: [Double; 2] @OfxImageEffectPropRenderScale;
            r/_ interact_background_colour: [Double; 3] @OfxInteractPropBackgroundColour;
            r/_ interact_pixel_scale: [Double; 2] @OfxInteractPropPixelScale;
            r/_ effect_instance: Pointer @OfxPropEffectInstance;
            r/_ time: Double @OfxPropTime;
        }
        ActionPenDownIn {
            r/_ image_effect_render_scale: [Double; 2] @OfxImageEffectPropRenderScale;
            r/_ interact_background_colour: [Double; 3] @OfxInteractPropBackgroundColour;
            r/_ interact_pen_position: [Double; 2] @OfxInteractPropPenPosition;
            r/_ interact_pen_pressure: Double @OfxInteractPropPenPressure;
            r/_ interact_pen_viewport_position: [Int; 2] @OfxInteractPropPenViewportPosition;
            r/_ interact_pixel_scale: [Double; 2] @OfxInteractPropPixelScale;
            r/_ effect_instance: Pointer @OfxPropEffectInstance;
            r/_ time: Double @OfxPropTime;
        }
        ActionPenMotionIn {
            r/_ image_effect_render_scale: [Double; 2] @OfxImageEffectPropRenderScale;
            r/_ interact_background_colour: [Double; 3] @OfxInteractPropBackgroundColour;
            r/_ interact_pen_position: [Double; 2] @OfxInteractPropPenPosition;
            r/_ interact_pen_pressure: Double @OfxInteractPropPenPressure;
            r/_ interact_pen_viewport_position: [Int; 2] @OfxInteractPropPenViewportPosition;
            r/_ interact_pixel_scale: [Double; 2] @OfxInteractPropPixelScale;
            r/_ effect_instance: Pointer @OfxPropEffectInstance;
            r/_ time: Double @OfxPropTime;
        }
        ActionPenUpIn {
            r/_ image_effect_render_scale: [Double; 2] @OfxImageEffectPropRenderScale;
            r/_ interact_background_colour: [Double; 3] @OfxInteractPropBackgroundColour;
            r/_ interact_pen_position: [Double; 2] @OfxInteractPropPenPosition;
            r/_ interact_pen_pressure: Double @OfxInteractPropPenPressure;
            r/_ interact_pen_viewport_position: [Int; 2] @OfxInteractPropPenViewportPosition;
            r/_ interact_pixel_scale: [Double; 2] @OfxInteractPropPixelScale;
            r/_ effect_instance: Pointer @OfxPropEffectInstance;
            r/_ time: Double @OfxPropTime;
        }
    }
}

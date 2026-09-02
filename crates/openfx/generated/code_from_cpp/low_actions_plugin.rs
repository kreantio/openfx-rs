pub mod pseudo {
    pub type CustomParamInterpFunc = fn(
        handle: *const std::ffi::c_void,
        in_args: CustomParamInterpFuncIn,
        out_args: CustomParamInterpFuncOut,
    ) -> crate::low::Result<()>;
    openfx_internal_macros::low_make_property_set_structs! {
        CustomParamInterpFuncIn {
            r/_ custom_value: [String; 2] @OfxParamPropCustomValue;
            r/_ interpolation_amount: Double @OfxParamPropInterpolationAmount;
            r/_ interpolation_time: [Double; 2] @OfxParamPropInterpolationTime;
        }
        CustomParamInterpFuncOut {
            _/w custom_value: [String; 2] @OfxParamPropCustomValue;
            _/w interpolation_time: [Double; 2] @OfxParamPropInterpolationTime;
        }
    }
}
pub mod core {
    openfx_internal_macros::low_make_property_set_structs! {
        ActionBeginInstanceChangedIn {
            r/_ thumbnail_render: Enum(ImageEffectPropThumbnailRender) @OfxImageEffectPropThumbnailRender;
            r/_ change_reason: Enum(PropChangeReason) @OfxPropChangeReason;
        }
        ActionEndInstanceChangedIn {
            r/_ change_reason: Enum(PropChangeReason) @OfxPropChangeReason;
        }
        ActionInstanceChangedIn {
            r/_ render_scale: [Double; 2] @OfxImageEffectPropRenderScale;
            r/_ thumbnail_render: Enum(ImageEffectPropThumbnailRender) @OfxImageEffectPropThumbnailRender;
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
            _/_ core::Describe,
            _/_ core::CreateInstance,
            _/_ core::DestroyInstance,
            i/_ core::BeginInstanceChanged,
            i/_ core::EndInstanceChanged,
            i/_ core::InstanceChanged,
            _/_ core::PurgeCaches,
            _/_ core::SyncPrivateData,
            _/_ core::BeginInstanceEdit,
            _/_ core::EndInstanceEdit,
            i/_ BeginSequenceRender,
            i/_ DescribeInContext,
            i/_ EndSequenceRender,
            _/o GetClipPreferences,
            i/o GetFramesNeeded,
            i/o GetOutputColourspace,
            i/o GetRegionOfDefinition,
            i/_ GetRegionsOfInterest,
            _/o GetTimeDomain,
            i/_ IsIdentity,
            i/_ Render,
        }
    }
    openfx_internal_macros::low_make_property_set_structs! {
        ActionBeginSequenceRenderIn {
            r/_ cuda_enabled: Bool @OfxImageEffectPropCudaEnabled;
            r/_ cuda_render_supported: Enum(ImageEffectPropCudaRenderSupported) @OfxImageEffectPropCudaRenderSupported;
            r/_ cuda_stream: Pointer @OfxImageEffectPropCudaStream;
            r/_ cuda_stream_supported: Enum(ImageEffectPropCudaStreamSupported) @OfxImageEffectPropCudaStreamSupported;
            r/_ frame_range: [Double; 2] @OfxImageEffectPropFrameRange;
            r/_ frame_step: Double @OfxImageEffectPropFrameStep;
            r/_ interactive_render_status: Bool @OfxImageEffectPropInteractiveRenderStatus;
            r/_ metal_command_queue: Pointer @OfxImageEffectPropMetalCommandQueue;
            r/_ metal_enabled: Bool @OfxImageEffectPropMetalEnabled;
            r/_ metal_render_supported: Enum(ImageEffectPropMetalRenderSupported) @OfxImageEffectPropMetalRenderSupported;
            r/_ no_spatial_awareness: Enum(ImageEffectPropNoSpatialAwareness) @OfxImageEffectPropNoSpatialAwareness;
            r/_ open_cl_command_queue: Pointer @OfxImageEffectPropOpenCLCommandQueue;
            r/_ open_cl_enabled: Bool @OfxImageEffectPropOpenCLEnabled;
            r/_ open_cl_image: Pointer @OfxImageEffectPropOpenCLImage;
            r/_ open_cl_render_supported: Enum(ImageEffectPropOpenCLRenderSupported) @OfxImageEffectPropOpenCLRenderSupported;
            r/_ open_cl_supported: Enum(ImageEffectPropOpenCLSupported) @OfxImageEffectPropOpenCLSupported;
            r/_ open_gl_enabled: Bool @OfxImageEffectPropOpenGLEnabled;
            r/_ open_gl_texture_index: Int @OfxImageEffectPropOpenGLTextureIndex;
            r/_ open_gl_texture_target: Int @OfxImageEffectPropOpenGLTextureTarget;
            r/_ render_scale: [Double; 2] @OfxImageEffectPropRenderScale;
            r/_ sequential_render_status: Bool @OfxImageEffectPropSequentialRenderStatus;
            r/_ thumbnail_render: Enum(ImageEffectPropThumbnailRender) @OfxImageEffectPropThumbnailRender;
            r/_ is_interactive: Bool @OfxPropIsInteractive;
        }
        ActionDescribeInContextIn {
            r/_ context: Enum(ImageEffectPropContext) @OfxImageEffectPropContext;
        }
        ActionEndSequenceRenderIn {
            r/_ cuda_enabled: Bool @OfxImageEffectPropCudaEnabled;
            r/_ cuda_render_supported: Enum(ImageEffectPropCudaRenderSupported) @OfxImageEffectPropCudaRenderSupported;
            r/_ cuda_stream: Pointer @OfxImageEffectPropCudaStream;
            r/_ cuda_stream_supported: Enum(ImageEffectPropCudaStreamSupported) @OfxImageEffectPropCudaStreamSupported;
            r/_ frame_range: [Double; 2] @OfxImageEffectPropFrameRange;
            r/_ frame_step: Double @OfxImageEffectPropFrameStep;
            r/_ interactive_render_status: Bool @OfxImageEffectPropInteractiveRenderStatus;
            r/_ metal_command_queue: Pointer @OfxImageEffectPropMetalCommandQueue;
            r/_ metal_enabled: Bool @OfxImageEffectPropMetalEnabled;
            r/_ metal_render_supported: Enum(ImageEffectPropMetalRenderSupported) @OfxImageEffectPropMetalRenderSupported;
            r/_ open_cl_command_queue: Pointer @OfxImageEffectPropOpenCLCommandQueue;
            r/_ open_cl_enabled: Bool @OfxImageEffectPropOpenCLEnabled;
            r/_ open_cl_image: Pointer @OfxImageEffectPropOpenCLImage;
            r/_ open_cl_render_supported: Enum(ImageEffectPropOpenCLRenderSupported) @OfxImageEffectPropOpenCLRenderSupported;
            r/_ open_cl_supported: Enum(ImageEffectPropOpenCLSupported) @OfxImageEffectPropOpenCLSupported;
            r/_ open_gl_enabled: Bool @OfxImageEffectPropOpenGLEnabled;
            r/_ open_gl_texture_index: Int @OfxImageEffectPropOpenGLTextureIndex;
            r/_ open_gl_texture_target: Int @OfxImageEffectPropOpenGLTextureTarget;
            r/_ render_scale: [Double; 2] @OfxImageEffectPropRenderScale;
            r/_ sequential_render_status: Bool @OfxImageEffectPropSequentialRenderStatus;
            r/_ is_interactive: Bool @OfxPropIsInteractive;
        }
        ActionGetClipPreferencesOut {
            _/w continuous_samples: Bool @OfxImageClipPropContinuousSamples;
            _/w field_order: Enum(ImageClipPropFieldOrder) @OfxImageClipPropFieldOrder;
            _/w frame_varying: Bool @OfxImageEffectFrameVarying;
            _/w frame_rate: Double @OfxImageEffectPropFrameRate;
            _/w pre_multiplication: Enum(ImageEffectPropPreMultiplication) @OfxImageEffectPropPreMultiplication;
        }
        ActionGetFramesNeededIn {
            r/_ thumbnail_render: Enum(ImageEffectPropThumbnailRender) @OfxImageEffectPropThumbnailRender;
            r/_ time: Double @OfxPropTime;
        }
        ActionGetFramesNeededOut {
            _/w frame_range: [Double; 2] @OfxImageEffectPropFrameRange;
        }
        ActionGetOutputColourspaceIn {
            r/_ preferred_colourspaces: [String] @OfxImageClipPropPreferredColourspaces;
        }
        ActionGetOutputColourspaceOut {
            _/w colourspace: String @OfxImageClipPropColourspace;
        }
        ActionGetRegionOfDefinitionIn {
            r/_ render_scale: [Double; 2] @OfxImageEffectPropRenderScale;
            r/_ thumbnail_render: Enum(ImageEffectPropThumbnailRender) @OfxImageEffectPropThumbnailRender;
            r/_ time: Double @OfxPropTime;
        }
        ActionGetRegionOfDefinitionOut {
            _/w region_of_definition: [Double; 4] @OfxImageEffectPropRegionOfDefinition;
        }
        ActionGetRegionsOfInterestIn {
            r/_ region_of_interest: [Double; 4] @OfxImageEffectPropRegionOfInterest;
            r/_ render_scale: [Double; 2] @OfxImageEffectPropRenderScale;
            r/_ thumbnail_render: Enum(ImageEffectPropThumbnailRender) @OfxImageEffectPropThumbnailRender;
            r/_ time: Double @OfxPropTime;
        }
        ActionGetTimeDomainOut {
            _/w frame_range: [Double; 2] @OfxImageEffectPropFrameRange;
        }
        ActionIsIdentityIn {
            r/_ field_to_render: Enum(ImageEffectPropFieldToRender) @OfxImageEffectPropFieldToRender;
            r/_ render_scale: [Double; 2] @OfxImageEffectPropRenderScale;
            r/_ render_window: [Int; 4] @OfxImageEffectPropRenderWindow;
            r/_ thumbnail_render: Enum(ImageEffectPropThumbnailRender) @OfxImageEffectPropThumbnailRender;
            r/_ time: Double @OfxPropTime;
        }
        ActionRenderIn {
            r/_ cuda_enabled: Bool @OfxImageEffectPropCudaEnabled;
            r/_ cuda_render_supported: Enum(ImageEffectPropCudaRenderSupported) @OfxImageEffectPropCudaRenderSupported;
            r/_ cuda_stream: Pointer @OfxImageEffectPropCudaStream;
            r/_ cuda_stream_supported: Enum(ImageEffectPropCudaStreamSupported) @OfxImageEffectPropCudaStreamSupported;
            r/_ interactive_render_status: Bool @OfxImageEffectPropInteractiveRenderStatus;
            r/_ metal_command_queue: Pointer @OfxImageEffectPropMetalCommandQueue;
            r/_ metal_enabled: Bool @OfxImageEffectPropMetalEnabled;
            r/_ metal_render_supported: Enum(ImageEffectPropMetalRenderSupported) @OfxImageEffectPropMetalRenderSupported;
            r/_ no_spatial_awareness: Enum(ImageEffectPropNoSpatialAwareness) @OfxImageEffectPropNoSpatialAwareness;
            r/_ open_cl_command_queue: Pointer @OfxImageEffectPropOpenCLCommandQueue;
            r/_ open_cl_enabled: Bool @OfxImageEffectPropOpenCLEnabled;
            r/_ open_cl_image: Pointer @OfxImageEffectPropOpenCLImage;
            r/_ open_cl_render_supported: Enum(ImageEffectPropOpenCLRenderSupported) @OfxImageEffectPropOpenCLRenderSupported;
            r/_ open_cl_supported: Enum(ImageEffectPropOpenCLSupported) @OfxImageEffectPropOpenCLSupported;
            r/_ open_gl_enabled: Bool @OfxImageEffectPropOpenGLEnabled;
            r/_ open_gl_texture_index: Int @OfxImageEffectPropOpenGLTextureIndex;
            r/_ open_gl_texture_target: Int @OfxImageEffectPropOpenGLTextureTarget;
            r/_ render_quality_draft: Bool @OfxImageEffectPropRenderQualityDraft;
            r/_ sequential_render_status: Bool @OfxImageEffectPropSequentialRenderStatus;
            r/_ thumbnail_render: Enum(ImageEffectPropThumbnailRender) @OfxImageEffectPropThumbnailRender;
            r/_ time: Double @OfxPropTime;
        }
    }
}
pub mod interact {
    openfx_internal_macros::low_make_action_enum! {
        InteractAction {
            _/_ core::Describe,
            _/_ core::CreateInstance,
            _/_ core::DestroyInstance,
            i/_ Draw,
            i/_ GainFocus,
            i/_ KeyDown,
            i/_ KeyRepeat,
            i/_ KeyUp,
            i/_ LoseFocus,
            i/_ PenDown,
            i/_ PenMotion,
            i/_ PenUp,
        }
    }
    openfx_internal_macros::low_make_property_set_structs! {
        ActionDrawIn {
            r/_ render_scale: [Double; 2] @OfxImageEffectPropRenderScale;
            r/_ background_colour: [Double; 3] @OfxInteractPropBackgroundColour;
            r/_ draw_context: Pointer @OfxInteractPropDrawContext;
            r/_ pixel_scale: [Double; 2] @OfxInteractPropPixelScale;
            r/_ effect_instance: Pointer @OfxPropEffectInstance;
            r/_ time: Double @OfxPropTime;
        }
        ActionGainFocusIn {
            r/_ render_scale: [Double; 2] @OfxImageEffectPropRenderScale;
            r/_ background_colour: [Double; 3] @OfxInteractPropBackgroundColour;
            r/_ pixel_scale: [Double; 2] @OfxInteractPropPixelScale;
            r/_ effect_instance: Pointer @OfxPropEffectInstance;
            r/_ time: Double @OfxPropTime;
        }
        ActionKeyDownIn {
            r/_ render_scale: [Double; 2] @OfxImageEffectPropRenderScale;
            r/_ effect_instance: Pointer @OfxPropEffectInstance;
            r/_ key_string: String @OfxPropKeyString;
            r/_ key_sym: Int @OfxPropKeySym;
            r/_ time: Double @OfxPropTime;
        }
        ActionKeyRepeatIn {
            r/_ render_scale: [Double; 2] @OfxImageEffectPropRenderScale;
            r/_ effect_instance: Pointer @OfxPropEffectInstance;
            r/_ key_string: String @OfxPropKeyString;
            r/_ key_sym: Int @OfxPropKeySym;
            r/_ time: Double @OfxPropTime;
        }
        ActionKeyUpIn {
            r/_ render_scale: [Double; 2] @OfxImageEffectPropRenderScale;
            r/_ effect_instance: Pointer @OfxPropEffectInstance;
            r/_ key_string: String @OfxPropKeyString;
            r/_ key_sym: Int @OfxPropKeySym;
            r/_ time: Double @OfxPropTime;
        }
        ActionLoseFocusIn {
            r/_ render_scale: [Double; 2] @OfxImageEffectPropRenderScale;
            r/_ background_colour: [Double; 3] @OfxInteractPropBackgroundColour;
            r/_ pixel_scale: [Double; 2] @OfxInteractPropPixelScale;
            r/_ effect_instance: Pointer @OfxPropEffectInstance;
            r/_ time: Double @OfxPropTime;
        }
        ActionPenDownIn {
            r/_ render_scale: [Double; 2] @OfxImageEffectPropRenderScale;
            r/_ background_colour: [Double; 3] @OfxInteractPropBackgroundColour;
            r/_ pen_position: [Double; 2] @OfxInteractPropPenPosition;
            r/_ pen_pressure: Double @OfxInteractPropPenPressure;
            r/_ pen_viewport_position: [Int; 2] @OfxInteractPropPenViewportPosition;
            r/_ pixel_scale: [Double; 2] @OfxInteractPropPixelScale;
            r/_ effect_instance: Pointer @OfxPropEffectInstance;
            r/_ time: Double @OfxPropTime;
        }
        ActionPenMotionIn {
            r/_ render_scale: [Double; 2] @OfxImageEffectPropRenderScale;
            r/_ background_colour: [Double; 3] @OfxInteractPropBackgroundColour;
            r/_ pen_position: [Double; 2] @OfxInteractPropPenPosition;
            r/_ pen_pressure: Double @OfxInteractPropPenPressure;
            r/_ pen_viewport_position: [Int; 2] @OfxInteractPropPenViewportPosition;
            r/_ pixel_scale: [Double; 2] @OfxInteractPropPixelScale;
            r/_ effect_instance: Pointer @OfxPropEffectInstance;
            r/_ time: Double @OfxPropTime;
        }
        ActionPenUpIn {
            r/_ render_scale: [Double; 2] @OfxImageEffectPropRenderScale;
            r/_ background_colour: [Double; 3] @OfxInteractPropBackgroundColour;
            r/_ pen_position: [Double; 2] @OfxInteractPropPenPosition;
            r/_ pen_pressure: Double @OfxInteractPropPenPressure;
            r/_ pen_viewport_position: [Int; 2] @OfxInteractPropPenViewportPosition;
            r/_ pixel_scale: [Double; 2] @OfxInteractPropPixelScale;
            r/_ effect_instance: Pointer @OfxPropEffectInstance;
            r/_ time: Double @OfxPropTime;
        }
    }
}

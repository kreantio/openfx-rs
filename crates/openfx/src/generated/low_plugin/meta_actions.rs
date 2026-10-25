pub mod pseudo {
    pub type CustomParamInterpFunc = fn(
        *const std::ffi::c_void,
        in_args: CustomParamInterpFuncIn,
        out_args: CustomParamInterpFuncOut,
    ) -> crate::low::OutboundResult;
    #[openfx_internal_macros::low_make_property_set_structs]
    mod __ {
        # [propset (write = host)]
        struct CustomParamInterpFuncIn {
            # [prop (@ OfxParamPropCustomValue)]
            param_custom_value: [String; 2],
            # [prop (@ OfxParamPropInterpolationAmount)]
            param_interpolation_amount: Double,
            # [prop (@ OfxParamPropInterpolationTime)]
            param_interpolation_time: [Double; 2],
            # [prop (@ OfxPropName)]
            name: String,
            # [prop (@ OfxPropTime)]
            time: Double,
        }
        # [propset (write = plugin)]
        struct CustomParamInterpFuncOut {
            # [prop (@ OfxParamPropCustomValue)]
            param_custom_value: [String; 2],
        }
    }
}
pub mod core {
    #[openfx_internal_macros::low_make_property_set_structs]
    mod __ {
        # [propset (write = host)]
        struct ActionBeginInstanceChangedIn {
            # [prop (@ OfxImageEffectPropThumbnailRender)]
            image_effect_thumbnail_render: ty_enum!(ImageEffectPropThumbnailRender),
            # [prop (@ OfxPropChangeReason)]
            change_reason: ty_enum!(PropChangeReason),
        }
    }
    #[openfx_internal_macros::low_make_property_set_structs]
    mod __ {
        # [propset (write = host)]
        struct ActionEndInstanceChangedIn {
            # [prop (@ OfxPropChangeReason)]
            change_reason: ty_enum!(PropChangeReason),
        }
    }
    #[openfx_internal_macros::low_make_property_set_structs]
    mod __ {
        # [propset (write = host)]
        struct ActionInstanceChangedIn {
            # [prop (@ OfxImageEffectPropRenderScale)]
            image_effect_render_scale: [Double; 2],
            # [prop (@ OfxImageEffectPropThumbnailRender)]
            image_effect_thumbnail_render: ty_enum!(ImageEffectPropThumbnailRender),
            # [prop (@ OfxPropChangeReason)]
            change_reason: ty_enum!(PropChangeReason),
            # [prop (@ OfxPropName)]
            name: String,
            # [prop (@ OfxPropTime)]
            time: Double,
            # [prop (@ OfxPropType)]
            r#type: String,
        }
    }
}
pub mod image_effect {
    #[openfx_internal_macros::low_make_action_enum]
    enum ImageEffectAction {
        # [action (core , handle = ImageEffectInstance , in_args)]
        BeginInstanceChanged,
        # [action (core , handle = ImageEffectInstance)]
        BeginInstanceEdit,
        # [action (core , handle = ImageEffectInstance)]
        CreateInstance,
        # [action (core , handle = ImageEffectDescriptor)]
        Describe,
        # [action (core , handle = ImageEffectInstance)]
        DestroyInstance,
        #[action(core)]
        Dialog,
        # [action (core , handle = ImageEffectInstance , in_args)]
        EndInstanceChanged,
        # [action (core , handle = ImageEffectInstance)]
        EndInstanceEdit,
        # [action (core , handle = ImageEffectInstance , in_args)]
        InstanceChanged,
        #[action(core)]
        Load,
        # [action (core , handle = ImageEffectInstance)]
        OpenGLContextAttached,
        # [action (core , handle = ImageEffectInstance)]
        OpenGLContextDetached,
        # [action (core , handle = ImageEffectInstance)]
        PurgeCaches,
        # [action (core , handle = ImageEffectInstance)]
        SyncPrivateData,
        #[action(core)]
        Unload,
        # [action (handle = ImageEffectInstance , in_args)]
        BeginSequenceRender,
        # [action (handle = ImageEffectDescriptor , in_args)]
        DescribeInContext,
        # [action (handle = ImageEffectInstance , in_args)]
        EndSequenceRender,
        # [action (handle = ImageEffectInstance , out_args)]
        GetClipPreferences,
        # [action (handle = ImageEffectInstance , in_args)]
        GetFramesNeeded,
        # [action (handle = ImageEffectInstance , in_args , out_args)]
        GetOutputColourspace,
        # [action (handle = ImageEffectInstance , in_args , out_args)]
        GetRegionOfDefinition,
        # [action (handle = ImageEffectInstance , in_args)]
        GetRegionsOfInterest,
        # [action (handle = ImageEffectInstance , out_args)]
        GetTimeDomain,
        # [action (handle = ImageEffectInstance , in_args , out_args)]
        IsIdentity,
        # [action (handle = ImageEffectInstance , in_args)]
        Render,
    }
    #[openfx_internal_macros::low_make_property_set_structs]
    mod __ {
        # [propset (write = host)]
        struct ActionBeginSequenceRenderIn {
            # [prop (@ OfxImageEffectPropBehaviourWhenUnlicensed)]
            image_effect_behaviour_when_unlicensed:
                ty_enum!(ImageEffectPropBehaviourWhenUnlicensed),
            # [prop (@ OfxImageEffectPropCudaEnabled)]
            image_effect_cuda_enabled: Bool,
            # [prop (@ OfxImageEffectPropCudaRenderSupported)]
            image_effect_cuda_render_supported: ty_enum!(ImageEffectPropCudaRenderSupported),
            # [prop (@ OfxImageEffectPropCudaStream)]
            image_effect_cuda_stream: Pointer,
            # [prop (@ OfxImageEffectPropCudaStreamSupported)]
            image_effect_cuda_stream_supported: ty_enum!(ImageEffectPropCudaStreamSupported),
            # [prop (@ OfxImageEffectPropFrameRange)]
            image_effect_frame_range: [Double; 2],
            # [prop (@ OfxImageEffectPropFrameStep)]
            image_effect_frame_step: Double,
            # [prop (@ OfxImageEffectPropInteractiveRenderStatus)]
            image_effect_interactive_render_status: Bool,
            # [prop (@ OfxImageEffectPropMetalCommandQueue)]
            image_effect_metal_command_queue: Pointer,
            # [prop (@ OfxImageEffectPropMetalEnabled)]
            image_effect_metal_enabled: Bool,
            # [prop (@ OfxImageEffectPropMetalRenderSupported)]
            image_effect_metal_render_supported: ty_enum!(ImageEffectPropMetalRenderSupported),
            # [prop (@ OfxImageEffectPropNoSpatialAwareness)]
            image_effect_no_spatial_awareness: ty_enum!(ImageEffectPropNoSpatialAwareness),
            # [prop (@ OfxImageEffectPropOpenCLCommandQueue)]
            image_effect_open_cl_command_queue: Pointer,
            # [prop (@ OfxImageEffectPropOpenCLEnabled)]
            image_effect_open_cl_enabled: Bool,
            # [prop (@ OfxImageEffectPropOpenCLImage)]
            image_effect_open_cl_image: Pointer,
            # [prop (@ OfxImageEffectPropOpenCLRenderSupported)]
            image_effect_open_cl_render_supported: ty_enum!(ImageEffectPropOpenCLRenderSupported),
            # [prop (@ OfxImageEffectPropOpenCLSupported)]
            image_effect_open_cl_supported: ty_enum!(ImageEffectPropOpenCLSupported),
            # [prop (@ OfxImageEffectPropOpenGLEnabled)]
            image_effect_open_gl_enabled: Bool,
            # [prop (@ OfxImageEffectPropOpenGLTextureIndex)]
            image_effect_open_gl_texture_index: Int,
            # [prop (@ OfxImageEffectPropOpenGLTextureTarget)]
            image_effect_open_gl_texture_target: Int,
            # [prop (@ OfxImageEffectPropRenderScale)]
            image_effect_render_scale: [Double; 2],
            # [prop (@ OfxImageEffectPropSequentialRenderStatus)]
            image_effect_sequential_render_status: Bool,
            # [prop (@ OfxImageEffectPropThumbnailRender)]
            image_effect_thumbnail_render: ty_enum!(ImageEffectPropThumbnailRender),
            # [prop (@ OfxPropIsInteractive)]
            is_interactive: Bool,
        }
    }
    #[openfx_internal_macros::low_make_property_set_structs]
    mod __ {
        # [propset (write = host)]
        struct ActionDescribeInContextIn {
            # [prop (@ OfxImageEffectPropContext)]
            image_effect_context: ty_enum!(ImageEffectPropContext),
        }
    }
    #[openfx_internal_macros::low_make_property_set_structs]
    mod __ {
        # [propset (write = host)]
        struct ActionEndSequenceRenderIn {
            # [prop (@ OfxImageEffectPropCudaEnabled)]
            image_effect_cuda_enabled: Bool,
            # [prop (@ OfxImageEffectPropCudaRenderSupported)]
            image_effect_cuda_render_supported: ty_enum!(ImageEffectPropCudaRenderSupported),
            # [prop (@ OfxImageEffectPropCudaStream)]
            image_effect_cuda_stream: Pointer,
            # [prop (@ OfxImageEffectPropCudaStreamSupported)]
            image_effect_cuda_stream_supported: ty_enum!(ImageEffectPropCudaStreamSupported),
            # [prop (@ OfxImageEffectPropFrameRange)]
            image_effect_frame_range: [Double; 2],
            # [prop (@ OfxImageEffectPropFrameStep)]
            image_effect_frame_step: Double,
            # [prop (@ OfxImageEffectPropInteractiveRenderStatus)]
            image_effect_interactive_render_status: Bool,
            # [prop (@ OfxImageEffectPropMetalCommandQueue)]
            image_effect_metal_command_queue: Pointer,
            # [prop (@ OfxImageEffectPropMetalEnabled)]
            image_effect_metal_enabled: Bool,
            # [prop (@ OfxImageEffectPropMetalRenderSupported)]
            image_effect_metal_render_supported: ty_enum!(ImageEffectPropMetalRenderSupported),
            # [prop (@ OfxImageEffectPropOpenCLCommandQueue)]
            image_effect_open_cl_command_queue: Pointer,
            # [prop (@ OfxImageEffectPropOpenCLEnabled)]
            image_effect_open_cl_enabled: Bool,
            # [prop (@ OfxImageEffectPropOpenCLImage)]
            image_effect_open_cl_image: Pointer,
            # [prop (@ OfxImageEffectPropOpenCLRenderSupported)]
            image_effect_open_cl_render_supported: ty_enum!(ImageEffectPropOpenCLRenderSupported),
            # [prop (@ OfxImageEffectPropOpenCLSupported)]
            image_effect_open_cl_supported: ty_enum!(ImageEffectPropOpenCLSupported),
            # [prop (@ OfxImageEffectPropOpenGLEnabled)]
            image_effect_open_gl_enabled: Bool,
            # [prop (@ OfxImageEffectPropOpenGLTextureIndex)]
            image_effect_open_gl_texture_index: Int,
            # [prop (@ OfxImageEffectPropOpenGLTextureTarget)]
            image_effect_open_gl_texture_target: Int,
            # [prop (@ OfxImageEffectPropRenderScale)]
            image_effect_render_scale: [Double; 2],
            # [prop (@ OfxImageEffectPropSequentialRenderStatus)]
            image_effect_sequential_render_status: Bool,
            # [prop (@ OfxPropIsInteractive)]
            is_interactive: Bool,
        }
    }
    #[openfx_internal_macros::low_make_property_set_structs]
    mod __ {
        # [propset (write = plugin)]
        struct ActionGetClipPreferencesOut {
            # [prop (@ OfxImageClipPropContinuousSamples)]
            image_clip_continuous_samples: Bool,
            # [prop (@ OfxImageClipPropFieldOrder)]
            image_clip_field_order: ty_enum!(ImageClipPropFieldOrder),
            # [prop (@ OfxImageEffectFrameVarying)]
            image_effect_frame_varying: Bool,
            # [prop (@ OfxImageEffectPropFrameRate)]
            image_effect_frame_rate: Double,
            # [prop (@ OfxImageEffectPropPreMultiplication)]
            image_effect_pre_multiplication: ty_enum!(ImageEffectPropPreMultiplication),
        }
    }
    #[openfx_internal_macros::low_make_property_set_structs]
    mod __ {
        # [propset (write = host)]
        struct ActionGetFramesNeededIn {
            # [prop (@ OfxImageEffectPropThumbnailRender)]
            image_effect_thumbnail_render: ty_enum!(ImageEffectPropThumbnailRender),
            # [prop (@ OfxPropTime)]
            time: Double,
        }
    }
    #[openfx_internal_macros::low_make_property_set_structs]
    mod __ {
        # [propset (write = host)]
        struct ActionGetOutputColourspaceIn {
            # [prop (@ OfxImageClipPropPreferredColourspaces)]
            image_clip_preferred_colourspaces: [String],
        }
        # [propset (write = plugin)]
        struct ActionGetOutputColourspaceOut {
            # [prop (@ OfxImageClipPropColourspace)]
            image_clip_colourspace: String,
        }
    }
    #[openfx_internal_macros::low_make_property_set_structs]
    mod __ {
        # [propset (write = host)]
        struct ActionGetRegionOfDefinitionIn {
            # [prop (@ OfxImageEffectPropRenderScale)]
            image_effect_render_scale: [Double; 2],
            # [prop (@ OfxImageEffectPropThumbnailRender)]
            image_effect_thumbnail_render: ty_enum!(ImageEffectPropThumbnailRender),
            # [prop (@ OfxPropTime)]
            time: Double,
        }
        # [propset (write = plugin)]
        struct ActionGetRegionOfDefinitionOut {
            # [prop (@ OfxImageEffectPropRegionOfDefinition)]
            image_effect_region_of_definition: [Double; 4],
        }
    }
    #[openfx_internal_macros::low_make_property_set_structs]
    mod __ {
        # [propset (write = host)]
        struct ActionGetRegionsOfInterestIn {
            # [prop (@ OfxImageEffectPropRegionOfInterest)]
            image_effect_region_of_interest: [Double; 4],
            # [prop (@ OfxImageEffectPropRenderScale)]
            image_effect_render_scale: [Double; 2],
            # [prop (@ OfxImageEffectPropThumbnailRender)]
            image_effect_thumbnail_render: ty_enum!(ImageEffectPropThumbnailRender),
            # [prop (@ OfxPropTime)]
            time: Double,
        }
    }
    #[openfx_internal_macros::low_make_property_set_structs]
    mod __ {
        # [propset (write = plugin)]
        struct ActionGetTimeDomainOut {
            # [prop (@ OfxImageEffectPropFrameRange)]
            image_effect_frame_range: [Double; 2],
        }
    }
    #[openfx_internal_macros::low_make_property_set_structs]
    mod __ {
        # [propset (write = host)]
        struct ActionIsIdentityIn {
            # [prop (@ OfxImageEffectPropFieldToRender)]
            image_effect_field_to_render: ty_enum!(ImageEffectPropFieldToRender),
            # [prop (@ OfxImageEffectPropRenderScale)]
            image_effect_render_scale: [Double; 2],
            # [prop (@ OfxImageEffectPropRenderWindow)]
            image_effect_render_window: [Int; 4],
            # [prop (@ OfxImageEffectPropThumbnailRender)]
            image_effect_thumbnail_render: ty_enum!(ImageEffectPropThumbnailRender),
            # [prop (@ OfxPropTime)]
            time: Double,
        }
        # [propset (write = plugin)]
        struct ActionIsIdentityOut {
            # [prop (@ OfxPropName)]
            name: String,
            # [prop (@ OfxPropTime)]
            time: Double,
        }
    }
    #[openfx_internal_macros::low_make_property_set_structs]
    mod __ {
        # [propset (write = host)]
        struct ActionRenderIn {
            # [prop (@ OfxImageEffectPropBehaviourWhenUnlicensed)]
            image_effect_behaviour_when_unlicensed:
                ty_enum!(ImageEffectPropBehaviourWhenUnlicensed),
            # [prop (@ OfxImageEffectPropCudaEnabled)]
            image_effect_cuda_enabled: Bool,
            # [prop (@ OfxImageEffectPropCudaRenderSupported)]
            image_effect_cuda_render_supported: ty_enum!(ImageEffectPropCudaRenderSupported),
            # [prop (@ OfxImageEffectPropCudaStream)]
            image_effect_cuda_stream: Pointer,
            # [prop (@ OfxImageEffectPropCudaStreamSupported)]
            image_effect_cuda_stream_supported: ty_enum!(ImageEffectPropCudaStreamSupported),
            # [prop (@ OfxImageEffectPropFieldToRender)]
            image_effect_field_to_render: ty_enum!(ImageEffectPropFieldToRender),
            # [prop (@ OfxImageEffectPropInteractiveRenderStatus)]
            image_effect_interactive_render_status: Bool,
            # [prop (@ OfxImageEffectPropMetalCommandQueue)]
            image_effect_metal_command_queue: Pointer,
            # [prop (@ OfxImageEffectPropMetalEnabled)]
            image_effect_metal_enabled: Bool,
            # [prop (@ OfxImageEffectPropMetalRenderSupported)]
            image_effect_metal_render_supported: ty_enum!(ImageEffectPropMetalRenderSupported),
            # [prop (@ OfxImageEffectPropNoSpatialAwareness)]
            image_effect_no_spatial_awareness: ty_enum!(ImageEffectPropNoSpatialAwareness),
            # [prop (@ OfxImageEffectPropOpenCLCommandQueue)]
            image_effect_open_cl_command_queue: Pointer,
            # [prop (@ OfxImageEffectPropOpenCLEnabled)]
            image_effect_open_cl_enabled: Bool,
            # [prop (@ OfxImageEffectPropOpenCLImage)]
            image_effect_open_cl_image: Pointer,
            # [prop (@ OfxImageEffectPropOpenCLRenderSupported)]
            image_effect_open_cl_render_supported: ty_enum!(ImageEffectPropOpenCLRenderSupported),
            # [prop (@ OfxImageEffectPropOpenCLSupported)]
            image_effect_open_cl_supported: ty_enum!(ImageEffectPropOpenCLSupported),
            # [prop (@ OfxImageEffectPropOpenGLEnabled)]
            image_effect_open_gl_enabled: Bool,
            # [prop (@ OfxImageEffectPropOpenGLTextureIndex)]
            image_effect_open_gl_texture_index: Int,
            # [prop (@ OfxImageEffectPropOpenGLTextureTarget)]
            image_effect_open_gl_texture_target: Int,
            # [prop (@ OfxImageEffectPropRenderQualityDraft)]
            image_effect_render_quality_draft: Bool,
            # [prop (@ OfxImageEffectPropRenderScale)]
            image_effect_render_scale: [Double; 2],
            # [prop (@ OfxImageEffectPropRenderWindow)]
            image_effect_render_window: [Int; 4],
            # [prop (@ OfxImageEffectPropSequentialRenderStatus)]
            image_effect_sequential_render_status: Bool,
            # [prop (@ OfxImageEffectPropThumbnailRender)]
            image_effect_thumbnail_render: ty_enum!(ImageEffectPropThumbnailRender),
            # [prop (@ OfxPropTime)]
            time: Double,
        }
    }
}
pub mod interact {
    #[openfx_internal_macros::low_make_action_enum]
    enum InteractAction {
        # [action (core , handle = InteractInstance)]
        CreateInstance,
        # [action (core , handle = InteractDescriptor)]
        Describe,
        # [action (core , handle = InteractInstance)]
        DestroyInstance,
        # [action (handle = InteractInstance , in_args)]
        Draw,
        # [action (handle = InteractInstance , in_args)]
        GainFocus,
        # [action (handle = InteractInstance , in_args)]
        KeyDown,
        # [action (handle = InteractInstance , in_args)]
        KeyRepeat,
        # [action (handle = InteractInstance , in_args)]
        KeyUp,
        # [action (handle = InteractInstance , in_args)]
        LoseFocus,
        # [action (handle = InteractInstance , in_args)]
        PenDown,
        # [action (handle = InteractInstance , in_args)]
        PenMotion,
        # [action (handle = InteractInstance , in_args)]
        PenUp,
    }
    #[openfx_internal_macros::low_make_property_set_structs]
    mod __ {
        # [propset (write = host)]
        struct ActionDrawIn {
            # [prop (@ OfxImageEffectPropRenderScale)]
            image_effect_render_scale: [Double; 2],
            # [prop (@ OfxInteractPropBackgroundColour)]
            interact_background_colour: [Double; 3],
            # [prop (@ OfxInteractPropDrawContext)]
            interact_draw_context: Pointer,
            # [prop (@ OfxInteractPropPixelScale)]
            interact_pixel_scale: [Double; 2],
            # [prop (@ OfxPropEffectInstance)]
            effect_instance: Pointer,
            # [prop (@ OfxPropTime)]
            time: Double,
        }
    }
    #[openfx_internal_macros::low_make_property_set_structs]
    mod __ {
        # [propset (write = host)]
        struct ActionGainFocusIn {
            # [prop (@ OfxImageEffectPropRenderScale)]
            image_effect_render_scale: [Double; 2],
            # [prop (@ OfxInteractPropBackgroundColour)]
            interact_background_colour: [Double; 3],
            # [prop (@ OfxInteractPropPixelScale)]
            interact_pixel_scale: [Double; 2],
            # [prop (@ OfxPropEffectInstance)]
            effect_instance: Pointer,
            # [prop (@ OfxPropTime)]
            time: Double,
        }
    }
    #[openfx_internal_macros::low_make_property_set_structs]
    mod __ {
        # [propset (write = host)]
        struct ActionKeyDownIn {
            # [prop (@ OfxImageEffectPropRenderScale)]
            image_effect_render_scale: [Double; 2],
            # [prop (@ OfxPropEffectInstance)]
            effect_instance: Pointer,
            # [prop (@ OfxPropKeyString)]
            key_string: String,
            # [prop (@ OfxPropKeySym)]
            key_sym: Int,
            # [prop (@ OfxPropTime)]
            time: Double,
        }
    }
    #[openfx_internal_macros::low_make_property_set_structs]
    mod __ {
        # [propset (write = host)]
        struct ActionKeyRepeatIn {
            # [prop (@ OfxImageEffectPropRenderScale)]
            image_effect_render_scale: [Double; 2],
            # [prop (@ OfxPropEffectInstance)]
            effect_instance: Pointer,
            # [prop (@ OfxPropKeyString)]
            key_string: String,
            # [prop (@ OfxPropKeySym)]
            key_sym: Int,
            # [prop (@ OfxPropTime)]
            time: Double,
        }
    }
    #[openfx_internal_macros::low_make_property_set_structs]
    mod __ {
        # [propset (write = host)]
        struct ActionKeyUpIn {
            # [prop (@ OfxImageEffectPropRenderScale)]
            image_effect_render_scale: [Double; 2],
            # [prop (@ OfxPropEffectInstance)]
            effect_instance: Pointer,
            # [prop (@ OfxPropKeyString)]
            key_string: String,
            # [prop (@ OfxPropKeySym)]
            key_sym: Int,
            # [prop (@ OfxPropTime)]
            time: Double,
        }
    }
    #[openfx_internal_macros::low_make_property_set_structs]
    mod __ {
        # [propset (write = host)]
        struct ActionLoseFocusIn {
            # [prop (@ OfxImageEffectPropRenderScale)]
            image_effect_render_scale: [Double; 2],
            # [prop (@ OfxInteractPropBackgroundColour)]
            interact_background_colour: [Double; 3],
            # [prop (@ OfxInteractPropPixelScale)]
            interact_pixel_scale: [Double; 2],
            # [prop (@ OfxPropEffectInstance)]
            effect_instance: Pointer,
            # [prop (@ OfxPropTime)]
            time: Double,
        }
    }
    #[openfx_internal_macros::low_make_property_set_structs]
    mod __ {
        # [propset (write = host)]
        struct ActionPenDownIn {
            # [prop (@ OfxImageEffectPropRenderScale)]
            image_effect_render_scale: [Double; 2],
            # [prop (@ OfxInteractPropBackgroundColour)]
            interact_background_colour: [Double; 3],
            # [prop (@ OfxInteractPropPenPosition)]
            interact_pen_position: [Double; 2],
            # [prop (@ OfxInteractPropPenPressure)]
            interact_pen_pressure: Double,
            # [prop (@ OfxInteractPropPenViewportPosition)]
            interact_pen_viewport_position: [Int; 2],
            # [prop (@ OfxInteractPropPixelScale)]
            interact_pixel_scale: [Double; 2],
            # [prop (@ OfxPropEffectInstance)]
            effect_instance: Pointer,
            # [prop (@ OfxPropTime)]
            time: Double,
        }
    }
    #[openfx_internal_macros::low_make_property_set_structs]
    mod __ {
        # [propset (write = host)]
        struct ActionPenMotionIn {
            # [prop (@ OfxImageEffectPropRenderScale)]
            image_effect_render_scale: [Double; 2],
            # [prop (@ OfxInteractPropBackgroundColour)]
            interact_background_colour: [Double; 3],
            # [prop (@ OfxInteractPropPenPosition)]
            interact_pen_position: [Double; 2],
            # [prop (@ OfxInteractPropPenPressure)]
            interact_pen_pressure: Double,
            # [prop (@ OfxInteractPropPenViewportPosition)]
            interact_pen_viewport_position: [Int; 2],
            # [prop (@ OfxInteractPropPixelScale)]
            interact_pixel_scale: [Double; 2],
            # [prop (@ OfxPropEffectInstance)]
            effect_instance: Pointer,
            # [prop (@ OfxPropTime)]
            time: Double,
        }
    }
    #[openfx_internal_macros::low_make_property_set_structs]
    mod __ {
        # [propset (write = host)]
        struct ActionPenUpIn {
            # [prop (@ OfxImageEffectPropRenderScale)]
            image_effect_render_scale: [Double; 2],
            # [prop (@ OfxInteractPropBackgroundColour)]
            interact_background_colour: [Double; 3],
            # [prop (@ OfxInteractPropPenPosition)]
            interact_pen_position: [Double; 2],
            # [prop (@ OfxInteractPropPenPressure)]
            interact_pen_pressure: Double,
            # [prop (@ OfxInteractPropPenViewportPosition)]
            interact_pen_viewport_position: [Int; 2],
            # [prop (@ OfxInteractPropPixelScale)]
            interact_pixel_scale: [Double; 2],
            # [prop (@ OfxPropEffectInstance)]
            effect_instance: Pointer,
            # [prop (@ OfxPropTime)]
            time: Double,
        }
    }
}

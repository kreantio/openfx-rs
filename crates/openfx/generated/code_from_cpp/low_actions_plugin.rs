pub mod pseudo {
    pub type CustomParamInterpFunc = fn(
        handle: *const std::ffi::c_void,
        in_args: CustomParamInterpFuncIn,
        out_args: CustomParamInterpFuncOut,
    ) -> crate::generic::low::Result<()>;
    openfx_internal_macros::low_make_property_set_structs! {
        pub CustomParamInterpFuncIn {
            custom_value(OfxParamPropCustomValue): [String; 2] { read(get) };
            interpolation_amount(OfxParamPropInterpolationAmount): Double { read(get) };
            interpolation_time(OfxParamPropInterpolationTime): [Double; 2] { read(get) };
        }
        pub CustomParamInterpFuncOut {
            custom_value(OfxParamPropCustomValue): [String; 2] { write(set, reset) };
            interpolation_time(OfxParamPropInterpolationTime): [Double; 2] { write(set, reset) };
        }
    }
}
pub mod core {
    openfx_internal_macros::low_make_property_set_structs! {
        pub ActionBeginInstanceChangedIn {
            thumbnail_render(OfxImageEffectPropThumbnailRender): Enum(ImageEffectPropThumbnailRender) { read(get) };
            change_reason(OfxPropChangeReason): Enum(PropChangeReason) { read(get) };
        }
        pub ActionEndInstanceChangedIn {
            change_reason(OfxPropChangeReason): Enum(PropChangeReason) { read(get) };
        }
        pub ActionInstanceChangedIn {
            render_scale(OfxImageEffectPropRenderScale): [Double; 2] { read(get) };
            thumbnail_render(OfxImageEffectPropThumbnailRender): Enum(ImageEffectPropThumbnailRender) { read(get) };
            change_reason(OfxPropChangeReason): Enum(PropChangeReason) { read(get) };
            name(OfxPropName): String { read(get) };
            time(OfxPropTime): Double { read(get) };
            r#type(OfxPropType): String { read(get) };
        }
    }
}
pub mod image_effect {
    openfx_internal_macros::low_make_action_enum! {
        pub ImageEffectAction {
            __ ___ core::Load,
            __ ___ core::Unload,
            __ ___ core::Describe,
            __ ___ core::CreateInstance,
            __ ___ core::DestroyInstance,
            in ___ core::BeginInstanceChanged,
            in ___ core::EndInstanceChanged,
            in ___ core::InstanceChanged,
            __ ___ core::PurgeCaches,
            __ ___ core::SyncPrivateData,
            __ ___ core::BeginInstanceEdit,
            __ ___ core::EndInstanceEdit,
            in ___ BeginSequenceRender,
            in ___ DescribeInContext,
            in ___ EndSequenceRender,
            __ out GetClipPreferences,
            in out GetFramesNeeded,
            in out GetOutputColourspace,
            in out GetRegionOfDefinition,
            in ___ GetRegionsOfInterest,
            __ out GetTimeDomain,
            in ___ IsIdentity,
            in ___ Render,
        }
    }
    openfx_internal_macros::low_make_property_set_structs! {
        pub ActionBeginSequenceRenderIn {
            cuda_enabled(OfxImageEffectPropCudaEnabled): Bool { read(get) };
            cuda_render_supported(OfxImageEffectPropCudaRenderSupported): Enum(ImageEffectPropCudaRenderSupported) { read(get) };
            cuda_stream(OfxImageEffectPropCudaStream): Pointer { read(get) };
            cuda_stream_supported(OfxImageEffectPropCudaStreamSupported): Enum(ImageEffectPropCudaStreamSupported) { read(get) };
            frame_range(OfxImageEffectPropFrameRange): [Double; 2] { read(get) };
            frame_step(OfxImageEffectPropFrameStep): Double { read(get) };
            interactive_render_status(OfxImageEffectPropInteractiveRenderStatus): Bool { read(get) };
            metal_command_queue(OfxImageEffectPropMetalCommandQueue): Pointer { read(get) };
            metal_enabled(OfxImageEffectPropMetalEnabled): Bool { read(get) };
            metal_render_supported(OfxImageEffectPropMetalRenderSupported): Enum(ImageEffectPropMetalRenderSupported) { read(get) };
            no_spatial_awareness(OfxImageEffectPropNoSpatialAwareness): Enum(ImageEffectPropNoSpatialAwareness) { read(get) };
            open_cl_command_queue(OfxImageEffectPropOpenCLCommandQueue): Pointer { read(get) };
            open_cl_enabled(OfxImageEffectPropOpenCLEnabled): Bool { read(get) };
            open_cl_image(OfxImageEffectPropOpenCLImage): Pointer { read(get) };
            open_cl_render_supported(OfxImageEffectPropOpenCLRenderSupported): Enum(ImageEffectPropOpenCLRenderSupported) { read(get) };
            open_cl_supported(OfxImageEffectPropOpenCLSupported): Enum(ImageEffectPropOpenCLSupported) { read(get) };
            open_gl_enabled(OfxImageEffectPropOpenGLEnabled): Bool { read(get) };
            open_gl_texture_index(OfxImageEffectPropOpenGLTextureIndex): Int { read(get) };
            open_gl_texture_target(OfxImageEffectPropOpenGLTextureTarget): Int { read(get) };
            render_scale(OfxImageEffectPropRenderScale): [Double; 2] { read(get) };
            sequential_render_status(OfxImageEffectPropSequentialRenderStatus): Bool { read(get) };
            thumbnail_render(OfxImageEffectPropThumbnailRender): Enum(ImageEffectPropThumbnailRender) { read(get) };
            is_interactive(OfxPropIsInteractive): Bool { read(get) };
        }
        pub ActionDescribeInContextIn {
            context(OfxImageEffectPropContext): Enum(ImageEffectPropContext) { read(get) };
        }
        pub ActionEndSequenceRenderIn {
            cuda_enabled(OfxImageEffectPropCudaEnabled): Bool { read(get) };
            cuda_render_supported(OfxImageEffectPropCudaRenderSupported): Enum(ImageEffectPropCudaRenderSupported) { read(get) };
            cuda_stream(OfxImageEffectPropCudaStream): Pointer { read(get) };
            cuda_stream_supported(OfxImageEffectPropCudaStreamSupported): Enum(ImageEffectPropCudaStreamSupported) { read(get) };
            frame_range(OfxImageEffectPropFrameRange): [Double; 2] { read(get) };
            frame_step(OfxImageEffectPropFrameStep): Double { read(get) };
            interactive_render_status(OfxImageEffectPropInteractiveRenderStatus): Bool { read(get) };
            metal_command_queue(OfxImageEffectPropMetalCommandQueue): Pointer { read(get) };
            metal_enabled(OfxImageEffectPropMetalEnabled): Bool { read(get) };
            metal_render_supported(OfxImageEffectPropMetalRenderSupported): Enum(ImageEffectPropMetalRenderSupported) { read(get) };
            open_cl_command_queue(OfxImageEffectPropOpenCLCommandQueue): Pointer { read(get) };
            open_cl_enabled(OfxImageEffectPropOpenCLEnabled): Bool { read(get) };
            open_cl_image(OfxImageEffectPropOpenCLImage): Pointer { read(get) };
            open_cl_render_supported(OfxImageEffectPropOpenCLRenderSupported): Enum(ImageEffectPropOpenCLRenderSupported) { read(get) };
            open_cl_supported(OfxImageEffectPropOpenCLSupported): Enum(ImageEffectPropOpenCLSupported) { read(get) };
            open_gl_enabled(OfxImageEffectPropOpenGLEnabled): Bool { read(get) };
            open_gl_texture_index(OfxImageEffectPropOpenGLTextureIndex): Int { read(get) };
            open_gl_texture_target(OfxImageEffectPropOpenGLTextureTarget): Int { read(get) };
            render_scale(OfxImageEffectPropRenderScale): [Double; 2] { read(get) };
            sequential_render_status(OfxImageEffectPropSequentialRenderStatus): Bool { read(get) };
            is_interactive(OfxPropIsInteractive): Bool { read(get) };
        }
        pub ActionGetClipPreferencesOut {
            continuous_samples(OfxImageClipPropContinuousSamples): Bool { write(set, reset) };
            field_order(OfxImageClipPropFieldOrder): Enum(ImageClipPropFieldOrder) { write(set, reset) };
            frame_varying(OfxImageEffectFrameVarying): Bool { write(set, reset) };
            frame_rate(OfxImageEffectPropFrameRate): Double { write(set, reset) };
            pre_multiplication(OfxImageEffectPropPreMultiplication): Enum(ImageEffectPropPreMultiplication) { write(set, reset) };
        }
        pub ActionGetFramesNeededIn {
            thumbnail_render(OfxImageEffectPropThumbnailRender): Enum(ImageEffectPropThumbnailRender) { read(get) };
            time(OfxPropTime): Double { read(get) };
        }
        pub ActionGetFramesNeededOut {
            frame_range(OfxImageEffectPropFrameRange): [Double; 2] { write(set, reset) };
        }
        pub ActionGetOutputColourspaceIn {
            preferred_colourspaces(OfxImageClipPropPreferredColourspaces): [String] { read(get, len) };
        }
        pub ActionGetOutputColourspaceOut {
            colourspace(OfxImageClipPropColourspace): String { write(set, reset) };
        }
        pub ActionGetRegionOfDefinitionIn {
            render_scale(OfxImageEffectPropRenderScale): [Double; 2] { read(get) };
            thumbnail_render(OfxImageEffectPropThumbnailRender): Enum(ImageEffectPropThumbnailRender) { read(get) };
            time(OfxPropTime): Double { read(get) };
        }
        pub ActionGetRegionOfDefinitionOut {
            region_of_definition(OfxImageEffectPropRegionOfDefinition): [Double; 4] { write(set, reset) };
        }
        pub ActionGetRegionsOfInterestIn {
            region_of_interest(OfxImageEffectPropRegionOfInterest): [Double; 4] { read(get) };
            render_scale(OfxImageEffectPropRenderScale): [Double; 2] { read(get) };
            thumbnail_render(OfxImageEffectPropThumbnailRender): Enum(ImageEffectPropThumbnailRender) { read(get) };
            time(OfxPropTime): Double { read(get) };
        }
        pub ActionGetTimeDomainOut {
            frame_range(OfxImageEffectPropFrameRange): [Double; 2] { write(set, reset) };
        }
        pub ActionIsIdentityIn {
            field_to_render(OfxImageEffectPropFieldToRender): Enum(ImageEffectPropFieldToRender) { read(get) };
            render_scale(OfxImageEffectPropRenderScale): [Double; 2] { read(get) };
            render_window(OfxImageEffectPropRenderWindow): [Int; 4] { read(get) };
            thumbnail_render(OfxImageEffectPropThumbnailRender): Enum(ImageEffectPropThumbnailRender) { read(get) };
            time(OfxPropTime): Double { read(get) };
        }
        pub ActionRenderIn {
            cuda_enabled(OfxImageEffectPropCudaEnabled): Bool { read(get) };
            cuda_render_supported(OfxImageEffectPropCudaRenderSupported): Enum(ImageEffectPropCudaRenderSupported) { read(get) };
            cuda_stream(OfxImageEffectPropCudaStream): Pointer { read(get) };
            cuda_stream_supported(OfxImageEffectPropCudaStreamSupported): Enum(ImageEffectPropCudaStreamSupported) { read(get) };
            interactive_render_status(OfxImageEffectPropInteractiveRenderStatus): Bool { read(get) };
            metal_command_queue(OfxImageEffectPropMetalCommandQueue): Pointer { read(get) };
            metal_enabled(OfxImageEffectPropMetalEnabled): Bool { read(get) };
            metal_render_supported(OfxImageEffectPropMetalRenderSupported): Enum(ImageEffectPropMetalRenderSupported) { read(get) };
            no_spatial_awareness(OfxImageEffectPropNoSpatialAwareness): Enum(ImageEffectPropNoSpatialAwareness) { read(get) };
            open_cl_command_queue(OfxImageEffectPropOpenCLCommandQueue): Pointer { read(get) };
            open_cl_enabled(OfxImageEffectPropOpenCLEnabled): Bool { read(get) };
            open_cl_image(OfxImageEffectPropOpenCLImage): Pointer { read(get) };
            open_cl_render_supported(OfxImageEffectPropOpenCLRenderSupported): Enum(ImageEffectPropOpenCLRenderSupported) { read(get) };
            open_cl_supported(OfxImageEffectPropOpenCLSupported): Enum(ImageEffectPropOpenCLSupported) { read(get) };
            open_gl_enabled(OfxImageEffectPropOpenGLEnabled): Bool { read(get) };
            open_gl_texture_index(OfxImageEffectPropOpenGLTextureIndex): Int { read(get) };
            open_gl_texture_target(OfxImageEffectPropOpenGLTextureTarget): Int { read(get) };
            render_quality_draft(OfxImageEffectPropRenderQualityDraft): Bool { read(get) };
            sequential_render_status(OfxImageEffectPropSequentialRenderStatus): Bool { read(get) };
            thumbnail_render(OfxImageEffectPropThumbnailRender): Enum(ImageEffectPropThumbnailRender) { read(get) };
            time(OfxPropTime): Double { read(get) };
        }
    }
}
pub mod interact {
    openfx_internal_macros::low_make_action_enum! {
        pub InteractAction {
            __ ___ core::Describe,
            __ ___ core::CreateInstance,
            __ ___ core::DestroyInstance,
            in ___ Draw,
            in ___ GainFocus,
            in ___ KeyDown,
            in ___ KeyRepeat,
            in ___ KeyUp,
            in ___ LoseFocus,
            in ___ PenDown,
            in ___ PenMotion,
            in ___ PenUp,
        }
    }
    openfx_internal_macros::low_make_property_set_structs! {
        pub ActionDrawIn {
            render_scale(OfxImageEffectPropRenderScale): [Double; 2] { read(get) };
            background_colour(OfxInteractPropBackgroundColour): [Double; 3] { read(get) };
            draw_context(OfxInteractPropDrawContext): Pointer { read(get) };
            pixel_scale(OfxInteractPropPixelScale): [Double; 2] { read(get) };
            effect_instance(OfxPropEffectInstance): Pointer { read(get) };
            time(OfxPropTime): Double { read(get) };
        }
        pub ActionGainFocusIn {
            render_scale(OfxImageEffectPropRenderScale): [Double; 2] { read(get) };
            background_colour(OfxInteractPropBackgroundColour): [Double; 3] { read(get) };
            pixel_scale(OfxInteractPropPixelScale): [Double; 2] { read(get) };
            effect_instance(OfxPropEffectInstance): Pointer { read(get) };
            time(OfxPropTime): Double { read(get) };
        }
        pub ActionKeyDownIn {
            render_scale(OfxImageEffectPropRenderScale): [Double; 2] { read(get) };
            effect_instance(OfxPropEffectInstance): Pointer { read(get) };
            key_string(OfxPropKeyString): String { read(get) };
            key_sym(OfxPropKeySym): Int { read(get) };
            time(OfxPropTime): Double { read(get) };
        }
        pub ActionKeyRepeatIn {
            render_scale(OfxImageEffectPropRenderScale): [Double; 2] { read(get) };
            effect_instance(OfxPropEffectInstance): Pointer { read(get) };
            key_string(OfxPropKeyString): String { read(get) };
            key_sym(OfxPropKeySym): Int { read(get) };
            time(OfxPropTime): Double { read(get) };
        }
        pub ActionKeyUpIn {
            render_scale(OfxImageEffectPropRenderScale): [Double; 2] { read(get) };
            effect_instance(OfxPropEffectInstance): Pointer { read(get) };
            key_string(OfxPropKeyString): String { read(get) };
            key_sym(OfxPropKeySym): Int { read(get) };
            time(OfxPropTime): Double { read(get) };
        }
        pub ActionLoseFocusIn {
            render_scale(OfxImageEffectPropRenderScale): [Double; 2] { read(get) };
            background_colour(OfxInteractPropBackgroundColour): [Double; 3] { read(get) };
            pixel_scale(OfxInteractPropPixelScale): [Double; 2] { read(get) };
            effect_instance(OfxPropEffectInstance): Pointer { read(get) };
            time(OfxPropTime): Double { read(get) };
        }
        pub ActionPenDownIn {
            render_scale(OfxImageEffectPropRenderScale): [Double; 2] { read(get) };
            background_colour(OfxInteractPropBackgroundColour): [Double; 3] { read(get) };
            pen_position(OfxInteractPropPenPosition): [Double; 2] { read(get) };
            pen_pressure(OfxInteractPropPenPressure): Double { read(get) };
            pen_viewport_position(OfxInteractPropPenViewportPosition): [Int; 2] { read(get) };
            pixel_scale(OfxInteractPropPixelScale): [Double; 2] { read(get) };
            effect_instance(OfxPropEffectInstance): Pointer { read(get) };
            time(OfxPropTime): Double { read(get) };
        }
        pub ActionPenMotionIn {
            render_scale(OfxImageEffectPropRenderScale): [Double; 2] { read(get) };
            background_colour(OfxInteractPropBackgroundColour): [Double; 3] { read(get) };
            pen_position(OfxInteractPropPenPosition): [Double; 2] { read(get) };
            pen_pressure(OfxInteractPropPenPressure): Double { read(get) };
            pen_viewport_position(OfxInteractPropPenViewportPosition): [Int; 2] { read(get) };
            pixel_scale(OfxInteractPropPixelScale): [Double; 2] { read(get) };
            effect_instance(OfxPropEffectInstance): Pointer { read(get) };
            time(OfxPropTime): Double { read(get) };
        }
        pub ActionPenUpIn {
            render_scale(OfxImageEffectPropRenderScale): [Double; 2] { read(get) };
            background_colour(OfxInteractPropBackgroundColour): [Double; 3] { read(get) };
            pen_position(OfxInteractPropPenPosition): [Double; 2] { read(get) };
            pen_pressure(OfxInteractPropPenPressure): Double { read(get) };
            pen_viewport_position(OfxInteractPropPenViewportPosition): [Int; 2] { read(get) };
            pixel_scale(OfxInteractPropPixelScale): [Double; 2] { read(get) };
            effect_instance(OfxPropEffectInstance): Pointer { read(get) };
            time(OfxPropTime): Double { read(get) };
        }
    }
}

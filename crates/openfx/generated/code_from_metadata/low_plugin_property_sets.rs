openfx_internal_macros::low_make_property_set_structs_v2! {
    struct ImageClipDescriptorPropertySet {
        #[meta(write = plugin, @OfxImageClipPropFieldExtraction)]
        image_clip_field_extraction: ty_enum!(ImageClipPropFieldExtraction),
        #[meta(write = plugin, @OfxImageClipPropIsMask)]
        image_clip_is_mask: Bool,
        #[meta(write = plugin, @OfxImageClipPropOptional)]
        image_clip_optional: Bool,
        #[meta(write = plugin, @OfxImageEffectPropSupportedComponents)]
        image_effect_supported_components: [ty_enum!(ImageEffectPropSupportedComponents)],
        #[meta(write = plugin, @OfxImageEffectPropSupportsTiles)]
        image_effect_supports_tiles: Bool,
        #[meta(write = plugin, @OfxImageEffectPropTemporalClipAccess)]
        image_effect_temporal_clip_access: Bool,
        #[meta(write = plugin, @OfxPropLabel)]
        label: String,
        #[meta(write = plugin, @OfxPropLongLabel)]
        long_label: String,
        #[meta(write = plugin, @OfxPropName)]
        name: String,
        #[meta(write = plugin, @OfxPropShortLabel)]
        short_label: String,
        #[meta(write = plugin, @OfxPropType)]
        r#type: String,
    }
    struct ImageClipInstancePropertySet {
        #[meta(write = host, @OfxImageClipPropColourspace)]
        image_clip_colourspace: String,
        #[meta(write = host, @OfxImageClipPropConnected)]
        image_clip_connected: Bool,
        #[meta(write = host, @OfxImageClipPropContinuousSamples)]
        image_clip_continuous_samples: Bool,
        #[meta(write = host, @OfxImageClipPropFieldExtraction)]
        image_clip_field_extraction: ty_enum!(ImageClipPropFieldExtraction),
        #[meta(write = host, @OfxImageClipPropFieldOrder)]
        image_clip_field_order: ty_enum!(ImageClipPropFieldOrder),
        #[meta(write = host, @OfxImageClipPropIsMask)]
        image_clip_is_mask: Bool,
        #[meta(write = host, @OfxImageClipPropOptional)]
        image_clip_optional: Bool,
        #[meta(write = host, @OfxImageClipPropPreferredColourspaces)]
        image_clip_preferred_colourspaces: [String],
        #[meta(write = host, @OfxImageClipPropUnmappedComponents)]
        image_clip_unmapped_components: ty_enum!(ImageClipPropUnmappedComponents),
        #[meta(write = host, @OfxImageClipPropUnmappedPixelDepth)]
        image_clip_unmapped_pixel_depth: ty_enum!(ImageClipPropUnmappedPixelDepth),
        #[meta(write = host, @OfxImageEffectPropComponents)]
        image_effect_components: ty_enum!(ImageEffectPropComponents),
        #[meta(write = host, @OfxImageEffectPropFrameRange)]
        image_effect_frame_range: [Double; 2],
        #[meta(write = host, @OfxImageEffectPropFrameRate)]
        image_effect_frame_rate: Double,
        #[meta(write = host, @OfxImageEffectPropPixelDepth)]
        image_effect_pixel_depth: ty_enum!(ImageEffectPropPixelDepth),
        #[meta(write = host, @OfxImageEffectPropPreMultiplication)]
        image_effect_pre_multiplication: ty_enum!(ImageEffectPropPreMultiplication),
        #[meta(write = host, @OfxImageEffectPropSupportedComponents)]
        image_effect_supported_components: [ty_enum!(ImageEffectPropSupportedComponents)],
        #[meta(write = host, @OfxImageEffectPropSupportsTiles)]
        image_effect_supports_tiles: Bool,
        #[meta(write = host, @OfxImageEffectPropTemporalClipAccess)]
        image_effect_temporal_clip_access: Bool,
        #[meta(write = host, @OfxImageEffectPropUnmappedFrameRange)]
        image_effect_unmapped_frame_range: [Double; 2],
        #[meta(write = host, @OfxImageEffectPropUnmappedFrameRate)]
        image_effect_unmapped_frame_rate: Double,
        #[meta(write = host, @OfxImagePropPixelAspectRatio)]
        image_pixel_aspect_ratio: Double,
        #[meta(write = host, @OfxPropLabel)]
        label: String,
        #[meta(write = host, @OfxPropLongLabel)]
        long_label: String,
        #[meta(write = host, @OfxPropName)]
        name: String,
        #[meta(write = host, @OfxPropShortLabel)]
        short_label: String,
        #[meta(write = host, @OfxPropType)]
        r#type: String,
    }
    struct ImageEffectDescriptorPropertySet {
        #[meta(write = plugin, @OfxImageEffectPluginPropFieldRenderTwiceAlways)]
        image_effect_plugin_field_render_twice_always: Bool,
        #[meta(write = plugin, @OfxImageEffectPluginPropGrouping)]
        image_effect_plugin_grouping: String,
        #[meta(write = plugin, @OfxImageEffectPluginPropHostFrameThreading)]
        image_effect_plugin_host_frame_threading: Bool,
        #[meta(write = plugin, @OfxImageEffectPluginPropObsolete)]
        image_effect_plugin_obsolete: Bool,
        #[meta(write = plugin, @OfxImageEffectPluginPropOverlayInteractV1)]
        image_effect_plugin_overlay_interact_v_1: Pointer,
        #[meta(write = plugin, @OfxImageEffectPluginPropOverlayInteractV2)]
        image_effect_plugin_overlay_interact_v_2: Pointer,
        #[meta(write = plugin, @OfxImageEffectPluginPropSingleInstance)]
        image_effect_plugin_single_instance: Bool,
        #[meta(write = plugin, @OfxImageEffectPluginRenderThreadSafety)]
        image_effect_plugin_render_thread_safety: ty_enum!(
            ImageEffectPluginRenderThreadSafety
        ),
        #[meta(write = plugin, @OfxImageEffectPropCPURenderSupported)]
        image_effect_cpu_render_supported: ty_enum!(ImageEffectPropCPURenderSupported),
        #[meta(write = plugin, @OfxImageEffectPropClipPreferencesSlaveParam)]
        image_effect_clip_preferences_slave_param: [String],
        #[meta(write = plugin, @OfxImageEffectPropColourManagementAvailableConfigs)]
        image_effect_colour_management_available_configs: [String],
        #[meta(write = plugin, @OfxImageEffectPropColourManagementStyle)]
        image_effect_colour_management_style: ty_enum!(ImageEffectPropColourManagementStyle),
        #[meta(write = plugin, @OfxImageEffectPropNoSpatialAwareness)]
        image_effect_no_spatial_awareness: ty_enum!(ImageEffectPropNoSpatialAwareness),
        #[meta(write = plugin, @OfxImageEffectPropOpenCLSupported)]
        image_effect_open_cl_supported: ty_enum!(ImageEffectPropOpenCLSupported),
        #[meta(write = plugin, @OfxImageEffectPropOpenGLRenderSupported)]
        image_effect_open_gl_render_supported: ty_enum!(
            ImageEffectPropOpenGLRenderSupported
        ),
        #[meta(write = plugin, @OfxImageEffectPropSupportedContexts)]
        image_effect_supported_contexts: [ty_enum!(ImageEffectPropSupportedContexts)],
        #[meta(write = plugin, @OfxImageEffectPropSupportedPixelDepths)]
        image_effect_supported_pixel_depths: [ty_enum!(ImageEffectPropSupportedPixelDepths)],
        #[meta(write = plugin, @OfxImageEffectPropSupportsMultiResolution)]
        image_effect_supports_multi_resolution: Bool,
        #[meta(write = plugin, @OfxImageEffectPropSupportsMultipleClipDepths)]
        image_effect_supports_multiple_clip_depths: Bool,
        #[meta(write = plugin, @OfxImageEffectPropSupportsMultipleClipPARs)]
        image_effect_supports_multiple_clip_pa_rs: Bool,
        #[meta(write = plugin, @OfxImageEffectPropSupportsTiles)]
        image_effect_supports_tiles: Bool,
        #[meta(write = plugin, @OfxImageEffectPropTemporalClipAccess)]
        image_effect_temporal_clip_access: Bool,
        #[meta(write = plugin, @OfxOpenGLPropPixelDepth)]
        open_gl_pixel_depth: [ty_enum!(OpenGLPropPixelDepth)],
        #[meta(write = host, @OfxPluginPropFilePath)]
        plugin_file_path: String,
        #[meta(write = plugin, @OfxPropLabel)]
        label: String,
        #[meta(write = plugin, @OfxPropLongLabel)]
        long_label: String,
        #[meta(write = plugin, @OfxPropPluginDescription)]
        plugin_description: String,
        #[meta(write = plugin, @OfxPropShortLabel)]
        short_label: String,
        #[meta(write = plugin, @OfxPropType)]
        r#type: String,
        #[meta(write = plugin, @OfxPropVersion)]
        version: [Int],
        #[meta(write = plugin, @OfxPropVersionLabel)]
        version_label: String,
    }
    struct ImageEffectInstancePropertySet {
        #[meta(write = host, @OfxImageEffectInstancePropEffectDuration)]
        image_effect_instance_effect_duration: Double,
        #[meta(write = host, @OfxImageEffectInstancePropSequentialRender)]
        image_effect_instance_sequential_render: Bool,
        #[meta(write = host, @OfxImageEffectPropCPURenderSupported)]
        image_effect_cpu_render_supported: ty_enum!(ImageEffectPropCPURenderSupported),
        #[meta(write = host, @OfxImageEffectPropColourManagementConfig)]
        image_effect_colour_management_config: String,
        #[meta(write = host, @OfxImageEffectPropColourManagementStyle)]
        image_effect_colour_management_style: ty_enum!(ImageEffectPropColourManagementStyle),
        #[meta(write = host, @OfxImageEffectPropContext)]
        image_effect_context: ty_enum!(ImageEffectPropContext),
        #[meta(write = host, @OfxImageEffectPropDisplayColourspace)]
        image_effect_display_colourspace: String,
        #[meta(write = host, @OfxImageEffectPropFrameRate)]
        image_effect_frame_rate: Double,
        #[meta(write = host, @OfxImageEffectPropOCIOConfig)]
        image_effect_ocio_config: String,
        #[meta(write = host, @OfxImageEffectPropOCIODisplay)]
        image_effect_ocio_display: String,
        #[meta(write = host, @OfxImageEffectPropOCIOView)]
        image_effect_ocio_view: String,
        #[meta(write = host, @OfxImageEffectPropOpenGLRenderSupported)]
        image_effect_open_gl_render_supported: ty_enum!(
            ImageEffectPropOpenGLRenderSupported
        ),
        #[meta(write = host, @OfxImageEffectPropPluginHandle)]
        image_effect_plugin_handle: Pointer,
        #[meta(write = host, @OfxImageEffectPropProjectExtent)]
        image_effect_project_extent: [Double; 2],
        #[meta(write = host, @OfxImageEffectPropProjectOffset)]
        image_effect_project_offset: [Double; 2],
        #[meta(write = host, @OfxImageEffectPropProjectPixelAspectRatio)]
        image_effect_project_pixel_aspect_ratio: Double,
        #[meta(write = host, @OfxImageEffectPropProjectSize)]
        image_effect_project_size: [Double; 2],
        #[meta(write = host, @OfxImageEffectPropSupportsTiles)]
        image_effect_supports_tiles: Bool,
        #[meta(write = host, @OfxPropInstanceData)]
        instance_data: Pointer,
        #[meta(write = host, @OfxPropIsInteractive)]
        is_interactive: Bool,
        #[meta(write = host, @OfxPropType)]
        r#type: String,
    }
    struct ImageInstancePropertySet {
        #[meta(write = host, @OfxImageEffectPropComponents)]
        image_effect_components: ty_enum!(ImageEffectPropComponents),
        #[meta(write = host, @OfxImageEffectPropPixelDepth)]
        image_effect_pixel_depth: ty_enum!(ImageEffectPropPixelDepth),
        #[meta(write = host, @OfxImageEffectPropPreMultiplication)]
        image_effect_pre_multiplication: ty_enum!(ImageEffectPropPreMultiplication),
        #[meta(write = host, @OfxImageEffectPropRenderScale)]
        image_effect_render_scale: [Double; 2],
        #[meta(write = host, @OfxImagePropBounds)]
        image_bounds: [Int; 4],
        #[meta(write = host, @OfxImagePropData)]
        image_data: Pointer,
        #[meta(write = host, @OfxImagePropField)]
        image_field: ty_enum!(ImagePropField),
        #[meta(write = host, @OfxImagePropPixelAspectRatio)]
        image_pixel_aspect_ratio: Double,
        #[meta(write = host, @OfxImagePropRegionOfDefinition)]
        image_region_of_definition: [Int; 4],
        #[meta(write = host, @OfxImagePropRowBytes)]
        image_row_bytes: Int,
        #[meta(write = host, @OfxImagePropUniqueIdentifier)]
        image_unique_identifier: String,
        #[meta(write = host, @OfxPropType)]
        r#type: String,
    }
    struct ImageEffectHostDescriptorPropertySet {
        #[meta(write = host, @OfxImageEffectHostPropIsBackground)]
        image_effect_host_is_background: Bool,
        #[meta(write = host, @OfxImageEffectHostPropNativeOrigin)]
        image_effect_host_native_origin: ty_enum!(ImageEffectHostPropNativeOrigin),
        #[meta(write = host, @OfxImageEffectInstancePropSequentialRender)]
        image_effect_instance_sequential_render: Bool,
        #[meta(write = host, @OfxImageEffectPropCPURenderSupported)]
        image_effect_cpu_render_supported: ty_enum!(ImageEffectPropCPURenderSupported),
        #[meta(write = host, @OfxImageEffectPropColourManagementAvailableConfigs)]
        image_effect_colour_management_available_configs: [String],
        #[meta(write = host, @OfxImageEffectPropColourManagementStyle)]
        image_effect_colour_management_style: ty_enum!(ImageEffectPropColourManagementStyle),
        #[meta(write = host, @OfxImageEffectPropOpenCLSupported)]
        image_effect_open_cl_supported: ty_enum!(ImageEffectPropOpenCLSupported),
        #[meta(write = host, @OfxImageEffectPropOpenGLRenderSupported)]
        image_effect_open_gl_render_supported: ty_enum!(
            ImageEffectPropOpenGLRenderSupported
        ),
        #[meta(write = host, @OfxImageEffectPropRenderQualityDraft)]
        image_effect_render_quality_draft: Bool,
        #[meta(write = host, @OfxImageEffectPropSetableFielding)]
        image_effect_setable_fielding: Bool,
        #[meta(write = host, @OfxImageEffectPropSetableFrameRate)]
        image_effect_setable_frame_rate: Bool,
        #[meta(write = host, @OfxImageEffectPropSupportedComponents)]
        image_effect_supported_components: [ty_enum!(ImageEffectPropSupportedComponents)],
        #[meta(write = host, @OfxImageEffectPropSupportedContexts)]
        image_effect_supported_contexts: [ty_enum!(ImageEffectPropSupportedContexts)],
        #[meta(write = host, @OfxImageEffectPropSupportsMultiResolution)]
        image_effect_supports_multi_resolution: Bool,
        #[meta(write = host, @OfxImageEffectPropSupportsMultipleClipDepths)]
        image_effect_supports_multiple_clip_depths: Bool,
        #[meta(write = host, @OfxImageEffectPropSupportsMultipleClipPARs)]
        image_effect_supports_multiple_clip_pa_rs: Bool,
        #[meta(write = host, @OfxImageEffectPropSupportsOverlays)]
        image_effect_supports_overlays: Bool,
        #[meta(write = host, @OfxImageEffectPropSupportsTiles)]
        image_effect_supports_tiles: Bool,
        #[meta(write = host, @OfxImageEffectPropTemporalClipAccess)]
        image_effect_temporal_clip_access: Bool,
        #[meta(write = host, @OfxParamHostPropMaxPages)]
        param_host_max_pages: Int,
        #[meta(write = host, @OfxParamHostPropMaxParameters)]
        param_host_max_parameters: Int,
        #[meta(write = host, @OfxParamHostPropPageRowColumnCount)]
        param_host_page_row_column_count: [Int; 2],
        #[meta(write = host, @OfxParamHostPropSupportsBooleanAnimation)]
        param_host_supports_boolean_animation: Bool,
        #[meta(write = host, @OfxParamHostPropSupportsChoiceAnimation)]
        param_host_supports_choice_animation: Bool,
        #[meta(write = host, @OfxParamHostPropSupportsCustomAnimation)]
        param_host_supports_custom_animation: Bool,
        #[meta(write = host, @OfxParamHostPropSupportsCustomInteract)]
        param_host_supports_custom_interact: Bool,
        #[meta(write = host, @OfxParamHostPropSupportsParametricAnimation)]
        param_host_supports_parametric_animation: Bool,
        #[meta(write = host, @OfxParamHostPropSupportsStrChoice)]
        param_host_supports_str_choice: Bool,
        #[meta(write = host, @OfxParamHostPropSupportsStrChoiceAnimation)]
        param_host_supports_str_choice_animation: Bool,
        #[meta(write = host, @OfxParamHostPropSupportsStringAnimation)]
        param_host_supports_string_animation: Bool,
        #[meta(write = host, @OfxPropAPIVersion)]
        api_version: [Int],
        #[meta(write = host, @OfxPropHostOSHandle)]
        host_os_handle: Pointer,
        #[meta(write = host, @OfxPropLabel)]
        label: String,
        #[meta(write = host, @OfxPropName)]
        name: String,
        #[meta(write = host, @OfxPropType)]
        r#type: String,
        #[meta(write = host, @OfxPropVersion)]
        version: [Int],
        #[meta(write = host, @OfxPropVersionLabel)]
        version_label: String,
    }
    struct InteractDescriptorPropertySet {
        #[meta(write = host, @OfxInteractPropBitDepth)]
        interact_bit_depth: Int,
        #[meta(write = host, @OfxInteractPropHasAlpha)]
        interact_has_alpha: Bool,
    }
    struct InteractInstancePropertySet {
        #[meta(write = host, @OfxInteractPropBackgroundColour)]
        interact_background_colour: [Double; 3],
        #[meta(write = host, @OfxInteractPropBitDepth)]
        interact_bit_depth: Int,
        #[meta(write = host, @OfxInteractPropHasAlpha)]
        interact_has_alpha: Bool,
        #[meta(write = host, @OfxInteractPropPixelScale)]
        interact_pixel_scale: [Double; 2],
        #[meta(write = host, @OfxInteractPropSlaveToParam)]
        interact_slave_to_param: [String],
        #[meta(write = host, @OfxInteractPropSuggestedColour)]
        interact_suggested_colour: [Double; 3],
        #[meta(write = host, @OfxPropEffectInstance)]
        effect_instance: Pointer,
        #[meta(write = host, @OfxPropInstanceData)]
        instance_data: Pointer,
    }
    struct ParamDouble1DPropertySet {
        #[meta(write = plugin, @OfxParamPropAnimates)]
        param_animates: Bool,
        #[meta(write = plugin, @OfxParamPropCacheInvalidation)]
        param_cache_invalidation: ty_enum!(ParamPropCacheInvalidation),
        #[meta(write = plugin, @OfxParamPropCanUndo)]
        param_can_undo: Bool,
        #[meta(write = plugin, @OfxParamPropDataPtr)]
        param_data_ptr: Pointer,
        #[meta(write = plugin, @OfxParamPropDefault)]
        param_default: [ty_one_of!(Int | Double | String | Pointer)],
        #[meta(write = plugin, @OfxParamPropDigits)]
        param_digits: Int,
        #[meta(write = plugin, @OfxParamPropDisplayMax)]
        param_display_max: [ty_one_of!(Int | Double)],
        #[meta(write = plugin, @OfxParamPropDisplayMin)]
        param_display_min: [ty_one_of!(Int | Double)],
        #[meta(write = plugin, @OfxParamPropDoubleType)]
        param_double_type: ty_enum!(ParamPropDoubleType),
        #[meta(write = plugin, @OfxParamPropEnabled)]
        param_enabled: Bool,
        #[meta(write = plugin, @OfxParamPropEvaluateOnChange)]
        param_evaluate_on_change: Bool,
        #[meta(write = plugin, @OfxParamPropHasHostOverlayHandle)]
        param_has_host_overlay_handle: Bool,
        #[meta(write = plugin, @OfxParamPropHint)]
        param_hint: String,
        #[meta(write = plugin, @OfxParamPropIncrement)]
        param_increment: Double,
        #[meta(write = plugin, @OfxParamPropInteractMinimumSize)]
        param_interact_minimum_size: [Double; 2],
        #[meta(write = plugin, @OfxParamPropInteractPreferedSize)]
        param_interact_prefered_size: [Int; 2],
        #[meta(write = plugin, @OfxParamPropInteractSize)]
        param_interact_size: [Double; 2],
        #[meta(write = plugin, @OfxParamPropInteractSizeAspect)]
        param_interact_size_aspect: Double,
        #[meta(write = plugin, @OfxParamPropInteractV1)]
        param_interact_v_1: Pointer,
        #[meta(write = host, @OfxParamPropIsAnimating)]
        param_is_animating: Bool,
        #[meta(write = host, @OfxParamPropIsAutoKeying)]
        param_is_auto_keying: Bool,
        #[meta(write = plugin, @OfxParamPropMax)]
        param_max: [ty_one_of!(Int | Double)],
        #[meta(write = plugin, @OfxParamPropMin)]
        param_min: [ty_one_of!(Int | Double)],
        #[meta(write = plugin, @OfxParamPropParent)]
        param_parent: String,
        #[meta(write = plugin, @OfxParamPropPersistant)]
        param_persistant: Bool,
        #[meta(write = plugin, @OfxParamPropPluginMayWrite)]
        param_plugin_may_write: Bool,
        #[meta(write = plugin, @OfxParamPropScriptName)]
        param_script_name: String,
        #[meta(write = plugin, @OfxParamPropSecret)]
        param_secret: Bool,
        #[meta(write = plugin, @OfxParamPropShowTimeMarker)]
        param_show_time_marker: Bool,
        #[meta(write = plugin, @OfxParamPropType)]
        param_type: String,
        #[meta(write = plugin, @OfxParamPropUseHostOverlayHandle)]
        param_use_host_overlay_handle: Bool,
        #[meta(write = plugin, @OfxPropIcon)]
        icon: [String; 2],
        #[meta(write = plugin, @OfxPropLabel)]
        label: String,
        #[meta(write = plugin, @OfxPropLongLabel)]
        long_label: String,
        #[meta(write = plugin, @OfxPropName)]
        name: String,
        #[meta(write = plugin, @OfxPropShortLabel)]
        short_label: String,
        #[meta(write = plugin, @OfxPropType)]
        r#type: String,
    }
    struct ParamSetPropertySet {
        #[meta(write = plugin, @OfxPluginPropParamPageOrder)]
        plugin_param_page_order: [String],
        #[meta(write = plugin, @OfxPropParamSetNeedsSyncing)]
        param_set_needs_syncing: Bool,
    }
    struct ParamBytePropertySet {
        #[meta(write = plugin, @OfxParamPropAnimates)]
        param_animates: Bool,
        #[meta(write = plugin, @OfxParamPropCacheInvalidation)]
        param_cache_invalidation: ty_enum!(ParamPropCacheInvalidation),
        #[meta(write = plugin, @OfxParamPropCanUndo)]
        param_can_undo: Bool,
        #[meta(write = plugin, @OfxParamPropDataPtr)]
        param_data_ptr: Pointer,
        #[meta(write = plugin, @OfxParamPropDefault)]
        param_default: [ty_one_of!(Int | Double | String | Pointer)],
        #[meta(write = plugin, @OfxParamPropDisplayMax)]
        param_display_max: [ty_one_of!(Int | Double)],
        #[meta(write = plugin, @OfxParamPropDisplayMin)]
        param_display_min: [ty_one_of!(Int | Double)],
        #[meta(write = plugin, @OfxParamPropEnabled)]
        param_enabled: Bool,
        #[meta(write = plugin, @OfxParamPropEvaluateOnChange)]
        param_evaluate_on_change: Bool,
        #[meta(write = plugin, @OfxParamPropHasHostOverlayHandle)]
        param_has_host_overlay_handle: Bool,
        #[meta(write = plugin, @OfxParamPropHint)]
        param_hint: String,
        #[meta(write = plugin, @OfxParamPropInteractMinimumSize)]
        param_interact_minimum_size: [Double; 2],
        #[meta(write = plugin, @OfxParamPropInteractPreferedSize)]
        param_interact_prefered_size: [Int; 2],
        #[meta(write = plugin, @OfxParamPropInteractSize)]
        param_interact_size: [Double; 2],
        #[meta(write = plugin, @OfxParamPropInteractSizeAspect)]
        param_interact_size_aspect: Double,
        #[meta(write = plugin, @OfxParamPropInteractV1)]
        param_interact_v_1: Pointer,
        #[meta(write = host, @OfxParamPropIsAnimating)]
        param_is_animating: Bool,
        #[meta(write = host, @OfxParamPropIsAutoKeying)]
        param_is_auto_keying: Bool,
        #[meta(write = plugin, @OfxParamPropMax)]
        param_max: [ty_one_of!(Int | Double)],
        #[meta(write = plugin, @OfxParamPropMin)]
        param_min: [ty_one_of!(Int | Double)],
        #[meta(write = plugin, @OfxParamPropParent)]
        param_parent: String,
        #[meta(write = plugin, @OfxParamPropPersistant)]
        param_persistant: Bool,
        #[meta(write = plugin, @OfxParamPropPluginMayWrite)]
        param_plugin_may_write: Bool,
        #[meta(write = plugin, @OfxParamPropScriptName)]
        param_script_name: String,
        #[meta(write = plugin, @OfxParamPropSecret)]
        param_secret: Bool,
        #[meta(write = plugin, @OfxParamPropType)]
        param_type: String,
        #[meta(write = plugin, @OfxParamPropUseHostOverlayHandle)]
        param_use_host_overlay_handle: Bool,
        #[meta(write = plugin, @OfxPropIcon)]
        icon: [String; 2],
        #[meta(write = plugin, @OfxPropLabel)]
        label: String,
        #[meta(write = plugin, @OfxPropLongLabel)]
        long_label: String,
        #[meta(write = plugin, @OfxPropName)]
        name: String,
        #[meta(write = plugin, @OfxPropShortLabel)]
        short_label: String,
        #[meta(write = plugin, @OfxPropType)]
        r#type: String,
    }
    struct ParamChoicePropertySet {
        #[meta(write = plugin, @OfxParamPropAnimates)]
        param_animates: Bool,
        #[meta(write = plugin, @OfxParamPropCacheInvalidation)]
        param_cache_invalidation: ty_enum!(ParamPropCacheInvalidation),
        #[meta(write = plugin, @OfxParamPropCanUndo)]
        param_can_undo: Bool,
        #[meta(write = plugin, @OfxParamPropChoiceOption)]
        param_choice_option: [String],
        #[meta(write = plugin, @OfxParamPropChoiceOrder)]
        param_choice_order: [Int],
        #[meta(write = plugin, @OfxParamPropDataPtr)]
        param_data_ptr: Pointer,
        #[meta(write = plugin, @OfxParamPropDefault)]
        param_default: [ty_one_of!(Int | Double | String | Pointer)],
        #[meta(write = plugin, @OfxParamPropEnabled)]
        param_enabled: Bool,
        #[meta(write = plugin, @OfxParamPropEvaluateOnChange)]
        param_evaluate_on_change: Bool,
        #[meta(write = plugin, @OfxParamPropHasHostOverlayHandle)]
        param_has_host_overlay_handle: Bool,
        #[meta(write = plugin, @OfxParamPropHint)]
        param_hint: String,
        #[meta(write = plugin, @OfxParamPropInteractMinimumSize)]
        param_interact_minimum_size: [Double; 2],
        #[meta(write = plugin, @OfxParamPropInteractPreferedSize)]
        param_interact_prefered_size: [Int; 2],
        #[meta(write = plugin, @OfxParamPropInteractSize)]
        param_interact_size: [Double; 2],
        #[meta(write = plugin, @OfxParamPropInteractSizeAspect)]
        param_interact_size_aspect: Double,
        #[meta(write = plugin, @OfxParamPropInteractV1)]
        param_interact_v_1: Pointer,
        #[meta(write = host, @OfxParamPropIsAnimating)]
        param_is_animating: Bool,
        #[meta(write = host, @OfxParamPropIsAutoKeying)]
        param_is_auto_keying: Bool,
        #[meta(write = plugin, @OfxParamPropParent)]
        param_parent: String,
        #[meta(write = plugin, @OfxParamPropPersistant)]
        param_persistant: Bool,
        #[meta(write = plugin, @OfxParamPropPluginMayWrite)]
        param_plugin_may_write: Bool,
        #[meta(write = plugin, @OfxParamPropScriptName)]
        param_script_name: String,
        #[meta(write = plugin, @OfxParamPropSecret)]
        param_secret: Bool,
        #[meta(write = plugin, @OfxParamPropType)]
        param_type: String,
        #[meta(write = plugin, @OfxParamPropUseHostOverlayHandle)]
        param_use_host_overlay_handle: Bool,
        #[meta(write = plugin, @OfxPropIcon)]
        icon: [String; 2],
        #[meta(write = plugin, @OfxPropLabel)]
        label: String,
        #[meta(write = plugin, @OfxPropLongLabel)]
        long_label: String,
        #[meta(write = plugin, @OfxPropName)]
        name: String,
        #[meta(write = plugin, @OfxPropShortLabel)]
        short_label: String,
        #[meta(write = plugin, @OfxPropType)]
        r#type: String,
    }
    struct ParamCustomPropertySet {
        #[meta(write = plugin, @OfxParamPropAnimates)]
        param_animates: Bool,
        #[meta(write = plugin, @OfxParamPropCacheInvalidation)]
        param_cache_invalidation: ty_enum!(ParamPropCacheInvalidation),
        #[meta(write = plugin, @OfxParamPropCanUndo)]
        param_can_undo: Bool,
        #[meta(write = plugin, @OfxParamPropCustomInterpCallbackV1)]
        param_custom_interp_callback_v_1: Pointer,
        #[meta(write = plugin, @OfxParamPropDataPtr)]
        param_data_ptr: Pointer,
        #[meta(write = plugin, @OfxParamPropDefault)]
        param_default: [ty_one_of!(Int | Double | String | Pointer)],
        #[meta(write = plugin, @OfxParamPropEnabled)]
        param_enabled: Bool,
        #[meta(write = plugin, @OfxParamPropEvaluateOnChange)]
        param_evaluate_on_change: Bool,
        #[meta(write = plugin, @OfxParamPropHasHostOverlayHandle)]
        param_has_host_overlay_handle: Bool,
        #[meta(write = plugin, @OfxParamPropHint)]
        param_hint: String,
        #[meta(write = plugin, @OfxParamPropInteractMinimumSize)]
        param_interact_minimum_size: [Double; 2],
        #[meta(write = plugin, @OfxParamPropInteractPreferedSize)]
        param_interact_prefered_size: [Int; 2],
        #[meta(write = plugin, @OfxParamPropInteractSize)]
        param_interact_size: [Double; 2],
        #[meta(write = plugin, @OfxParamPropInteractSizeAspect)]
        param_interact_size_aspect: Double,
        #[meta(write = plugin, @OfxParamPropInteractV1)]
        param_interact_v_1: Pointer,
        #[meta(write = host, @OfxParamPropIsAnimating)]
        param_is_animating: Bool,
        #[meta(write = host, @OfxParamPropIsAutoKeying)]
        param_is_auto_keying: Bool,
        #[meta(write = plugin, @OfxParamPropParent)]
        param_parent: String,
        #[meta(write = plugin, @OfxParamPropPersistant)]
        param_persistant: Bool,
        #[meta(write = plugin, @OfxParamPropPluginMayWrite)]
        param_plugin_may_write: Bool,
        #[meta(write = plugin, @OfxParamPropScriptName)]
        param_script_name: String,
        #[meta(write = plugin, @OfxParamPropSecret)]
        param_secret: Bool,
        #[meta(write = plugin, @OfxParamPropType)]
        param_type: String,
        #[meta(write = plugin, @OfxParamPropUseHostOverlayHandle)]
        param_use_host_overlay_handle: Bool,
        #[meta(write = plugin, @OfxPropIcon)]
        icon: [String; 2],
        #[meta(write = plugin, @OfxPropLabel)]
        label: String,
        #[meta(write = plugin, @OfxPropLongLabel)]
        long_label: String,
        #[meta(write = plugin, @OfxPropName)]
        name: String,
        #[meta(write = plugin, @OfxPropShortLabel)]
        short_label: String,
        #[meta(write = plugin, @OfxPropType)]
        r#type: String,
    }
    struct ParamDouble2D3DPropertySet {
        #[meta(write = plugin, @OfxParamPropAnimates)]
        param_animates: Bool,
        #[meta(write = plugin, @OfxParamPropCacheInvalidation)]
        param_cache_invalidation: ty_enum!(ParamPropCacheInvalidation),
        #[meta(write = plugin, @OfxParamPropCanUndo)]
        param_can_undo: Bool,
        #[meta(write = plugin, @OfxParamPropDataPtr)]
        param_data_ptr: Pointer,
        #[meta(write = plugin, @OfxParamPropDefault)]
        param_default: [ty_one_of!(Int | Double | String | Pointer)],
        #[meta(write = plugin, @OfxParamPropDigits)]
        param_digits: Int,
        #[meta(write = plugin, @OfxParamPropDisplayMax)]
        param_display_max: [ty_one_of!(Int | Double)],
        #[meta(write = plugin, @OfxParamPropDisplayMin)]
        param_display_min: [ty_one_of!(Int | Double)],
        #[meta(write = plugin, @OfxParamPropDoubleType)]
        param_double_type: ty_enum!(ParamPropDoubleType),
        #[meta(write = plugin, @OfxParamPropEnabled)]
        param_enabled: Bool,
        #[meta(write = plugin, @OfxParamPropEvaluateOnChange)]
        param_evaluate_on_change: Bool,
        #[meta(write = plugin, @OfxParamPropHasHostOverlayHandle)]
        param_has_host_overlay_handle: Bool,
        #[meta(write = plugin, @OfxParamPropHint)]
        param_hint: String,
        #[meta(write = plugin, @OfxParamPropIncrement)]
        param_increment: Double,
        #[meta(write = plugin, @OfxParamPropInteractMinimumSize)]
        param_interact_minimum_size: [Double; 2],
        #[meta(write = plugin, @OfxParamPropInteractPreferedSize)]
        param_interact_prefered_size: [Int; 2],
        #[meta(write = plugin, @OfxParamPropInteractSize)]
        param_interact_size: [Double; 2],
        #[meta(write = plugin, @OfxParamPropInteractSizeAspect)]
        param_interact_size_aspect: Double,
        #[meta(write = plugin, @OfxParamPropInteractV1)]
        param_interact_v_1: Pointer,
        #[meta(write = host, @OfxParamPropIsAnimating)]
        param_is_animating: Bool,
        #[meta(write = host, @OfxParamPropIsAutoKeying)]
        param_is_auto_keying: Bool,
        #[meta(write = plugin, @OfxParamPropMax)]
        param_max: [ty_one_of!(Int | Double)],
        #[meta(write = plugin, @OfxParamPropMin)]
        param_min: [ty_one_of!(Int | Double)],
        #[meta(write = plugin, @OfxParamPropParent)]
        param_parent: String,
        #[meta(write = plugin, @OfxParamPropPersistant)]
        param_persistant: Bool,
        #[meta(write = plugin, @OfxParamPropPluginMayWrite)]
        param_plugin_may_write: Bool,
        #[meta(write = plugin, @OfxParamPropScriptName)]
        param_script_name: String,
        #[meta(write = plugin, @OfxParamPropSecret)]
        param_secret: Bool,
        #[meta(write = plugin, @OfxParamPropType)]
        param_type: String,
        #[meta(write = plugin, @OfxParamPropUseHostOverlayHandle)]
        param_use_host_overlay_handle: Bool,
        #[meta(write = plugin, @OfxPropIcon)]
        icon: [String; 2],
        #[meta(write = plugin, @OfxPropLabel)]
        label: String,
        #[meta(write = plugin, @OfxPropLongLabel)]
        long_label: String,
        #[meta(write = plugin, @OfxPropName)]
        name: String,
        #[meta(write = plugin, @OfxPropShortLabel)]
        short_label: String,
        #[meta(write = plugin, @OfxPropType)]
        r#type: String,
    }
    struct ParamGroupPropertySet {
        #[meta(write = plugin, @OfxParamPropDataPtr)]
        param_data_ptr: Pointer,
        #[meta(write = plugin, @OfxParamPropEnabled)]
        param_enabled: Bool,
        #[meta(write = plugin, @OfxParamPropGroupOpen)]
        param_group_open: Bool,
        #[meta(write = plugin, @OfxParamPropHint)]
        param_hint: String,
        #[meta(write = plugin, @OfxParamPropParent)]
        param_parent: String,
        #[meta(write = plugin, @OfxParamPropScriptName)]
        param_script_name: String,
        #[meta(write = plugin, @OfxParamPropSecret)]
        param_secret: Bool,
        #[meta(write = plugin, @OfxParamPropType)]
        param_type: String,
        #[meta(write = plugin, @OfxPropIcon)]
        icon: [String; 2],
        #[meta(write = plugin, @OfxPropLabel)]
        label: String,
        #[meta(write = plugin, @OfxPropLongLabel)]
        long_label: String,
        #[meta(write = plugin, @OfxPropName)]
        name: String,
        #[meta(write = plugin, @OfxPropShortLabel)]
        short_label: String,
        #[meta(write = plugin, @OfxPropType)]
        r#type: String,
    }
    struct ParamInteger2D3DPropertySet {
        #[meta(write = plugin, @OfxParamPropAnimates)]
        param_animates: Bool,
        #[meta(write = plugin, @OfxParamPropCacheInvalidation)]
        param_cache_invalidation: ty_enum!(ParamPropCacheInvalidation),
        #[meta(write = plugin, @OfxParamPropCanUndo)]
        param_can_undo: Bool,
        #[meta(write = plugin, @OfxParamPropDataPtr)]
        param_data_ptr: Pointer,
        #[meta(write = plugin, @OfxParamPropDefault)]
        param_default: [ty_one_of!(Int | Double | String | Pointer)],
        #[meta(write = plugin, @OfxParamPropDimensionLabel)]
        param_dimension_label: String,
        #[meta(write = plugin, @OfxParamPropDisplayMax)]
        param_display_max: [ty_one_of!(Int | Double)],
        #[meta(write = plugin, @OfxParamPropDisplayMin)]
        param_display_min: [ty_one_of!(Int | Double)],
        #[meta(write = plugin, @OfxParamPropEnabled)]
        param_enabled: Bool,
        #[meta(write = plugin, @OfxParamPropEvaluateOnChange)]
        param_evaluate_on_change: Bool,
        #[meta(write = plugin, @OfxParamPropHasHostOverlayHandle)]
        param_has_host_overlay_handle: Bool,
        #[meta(write = plugin, @OfxParamPropHint)]
        param_hint: String,
        #[meta(write = plugin, @OfxParamPropInteractMinimumSize)]
        param_interact_minimum_size: [Double; 2],
        #[meta(write = plugin, @OfxParamPropInteractPreferedSize)]
        param_interact_prefered_size: [Int; 2],
        #[meta(write = plugin, @OfxParamPropInteractSize)]
        param_interact_size: [Double; 2],
        #[meta(write = plugin, @OfxParamPropInteractSizeAspect)]
        param_interact_size_aspect: Double,
        #[meta(write = plugin, @OfxParamPropInteractV1)]
        param_interact_v_1: Pointer,
        #[meta(write = host, @OfxParamPropIsAnimating)]
        param_is_animating: Bool,
        #[meta(write = host, @OfxParamPropIsAutoKeying)]
        param_is_auto_keying: Bool,
        #[meta(write = plugin, @OfxParamPropMax)]
        param_max: [ty_one_of!(Int | Double)],
        #[meta(write = plugin, @OfxParamPropMin)]
        param_min: [ty_one_of!(Int | Double)],
        #[meta(write = plugin, @OfxParamPropParent)]
        param_parent: String,
        #[meta(write = plugin, @OfxParamPropPersistant)]
        param_persistant: Bool,
        #[meta(write = plugin, @OfxParamPropPluginMayWrite)]
        param_plugin_may_write: Bool,
        #[meta(write = plugin, @OfxParamPropScriptName)]
        param_script_name: String,
        #[meta(write = plugin, @OfxParamPropSecret)]
        param_secret: Bool,
        #[meta(write = plugin, @OfxParamPropType)]
        param_type: String,
        #[meta(write = plugin, @OfxParamPropUseHostOverlayHandle)]
        param_use_host_overlay_handle: Bool,
        #[meta(write = plugin, @OfxPropIcon)]
        icon: [String; 2],
        #[meta(write = plugin, @OfxPropLabel)]
        label: String,
        #[meta(write = plugin, @OfxPropLongLabel)]
        long_label: String,
        #[meta(write = plugin, @OfxPropName)]
        name: String,
        #[meta(write = plugin, @OfxPropShortLabel)]
        short_label: String,
        #[meta(write = plugin, @OfxPropType)]
        r#type: String,
    }
    struct ParamNormalizedSpatialPropertySet {
        #[meta(write = plugin, @OfxParamPropAnimates)]
        param_animates: Bool,
        #[meta(write = plugin, @OfxParamPropCacheInvalidation)]
        param_cache_invalidation: ty_enum!(ParamPropCacheInvalidation),
        #[meta(write = plugin, @OfxParamPropCanUndo)]
        param_can_undo: Bool,
        #[meta(write = plugin, @OfxParamPropDataPtr)]
        param_data_ptr: Pointer,
        #[meta(write = plugin, @OfxParamPropDefault)]
        param_default: [ty_one_of!(Int | Double | String | Pointer)],
        #[meta(write = plugin, @OfxParamPropDefaultCoordinateSystem)]
        param_default_coordinate_system: ty_enum!(ParamPropDefaultCoordinateSystem),
        #[meta(write = plugin, @OfxParamPropDigits)]
        param_digits: Int,
        #[meta(write = plugin, @OfxParamPropDisplayMax)]
        param_display_max: [ty_one_of!(Int | Double)],
        #[meta(write = plugin, @OfxParamPropDisplayMin)]
        param_display_min: [ty_one_of!(Int | Double)],
        #[meta(write = plugin, @OfxParamPropEnabled)]
        param_enabled: Bool,
        #[meta(write = plugin, @OfxParamPropEvaluateOnChange)]
        param_evaluate_on_change: Bool,
        #[meta(write = plugin, @OfxParamPropHasHostOverlayHandle)]
        param_has_host_overlay_handle: Bool,
        #[meta(write = plugin, @OfxParamPropHint)]
        param_hint: String,
        #[meta(write = plugin, @OfxParamPropIncrement)]
        param_increment: Double,
        #[meta(write = plugin, @OfxParamPropInteractMinimumSize)]
        param_interact_minimum_size: [Double; 2],
        #[meta(write = plugin, @OfxParamPropInteractPreferedSize)]
        param_interact_prefered_size: [Int; 2],
        #[meta(write = plugin, @OfxParamPropInteractSize)]
        param_interact_size: [Double; 2],
        #[meta(write = plugin, @OfxParamPropInteractSizeAspect)]
        param_interact_size_aspect: Double,
        #[meta(write = plugin, @OfxParamPropInteractV1)]
        param_interact_v_1: Pointer,
        #[meta(write = host, @OfxParamPropIsAnimating)]
        param_is_animating: Bool,
        #[meta(write = host, @OfxParamPropIsAutoKeying)]
        param_is_auto_keying: Bool,
        #[meta(write = plugin, @OfxParamPropMax)]
        param_max: [ty_one_of!(Int | Double)],
        #[meta(write = plugin, @OfxParamPropMin)]
        param_min: [ty_one_of!(Int | Double)],
        #[meta(write = plugin, @OfxParamPropParent)]
        param_parent: String,
        #[meta(write = plugin, @OfxParamPropPersistant)]
        param_persistant: Bool,
        #[meta(write = plugin, @OfxParamPropPluginMayWrite)]
        param_plugin_may_write: Bool,
        #[meta(write = plugin, @OfxParamPropScriptName)]
        param_script_name: String,
        #[meta(write = plugin, @OfxParamPropSecret)]
        param_secret: Bool,
        #[meta(write = plugin, @OfxParamPropType)]
        param_type: String,
        #[meta(write = plugin, @OfxParamPropUseHostOverlayHandle)]
        param_use_host_overlay_handle: Bool,
        #[meta(write = plugin, @OfxPropIcon)]
        icon: [String; 2],
        #[meta(write = plugin, @OfxPropLabel)]
        label: String,
        #[meta(write = plugin, @OfxPropLongLabel)]
        long_label: String,
        #[meta(write = plugin, @OfxPropName)]
        name: String,
        #[meta(write = plugin, @OfxPropShortLabel)]
        short_label: String,
        #[meta(write = plugin, @OfxPropType)]
        r#type: String,
    }
    struct ParamPagePropertySet {
        #[meta(write = plugin, @OfxParamPropDataPtr)]
        param_data_ptr: Pointer,
        #[meta(write = plugin, @OfxParamPropEnabled)]
        param_enabled: Bool,
        #[meta(write = plugin, @OfxParamPropHint)]
        param_hint: String,
        #[meta(write = plugin, @OfxParamPropPageChild)]
        param_page_child: [String],
        #[meta(write = plugin, @OfxParamPropParent)]
        param_parent: String,
        #[meta(write = plugin, @OfxParamPropScriptName)]
        param_script_name: String,
        #[meta(write = plugin, @OfxParamPropSecret)]
        param_secret: Bool,
        #[meta(write = plugin, @OfxParamPropType)]
        param_type: String,
        #[meta(write = plugin, @OfxPropIcon)]
        icon: [String; 2],
        #[meta(write = plugin, @OfxPropLabel)]
        label: String,
        #[meta(write = plugin, @OfxPropLongLabel)]
        long_label: String,
        #[meta(write = plugin, @OfxPropName)]
        name: String,
        #[meta(write = plugin, @OfxPropShortLabel)]
        short_label: String,
        #[meta(write = plugin, @OfxPropType)]
        r#type: String,
    }
    struct ParamParametricPropertySet {
        #[meta(write = plugin, @OfxParamPropAnimates)]
        param_animates: Bool,
        #[meta(write = plugin, @OfxParamPropCacheInvalidation)]
        param_cache_invalidation: ty_enum!(ParamPropCacheInvalidation),
        #[meta(write = plugin, @OfxParamPropCanUndo)]
        param_can_undo: Bool,
        #[meta(write = plugin, @OfxParamPropDataPtr)]
        param_data_ptr: Pointer,
        #[meta(write = plugin, @OfxParamPropDefault)]
        param_default: [ty_one_of!(Int | Double | String | Pointer)],
        #[meta(write = plugin, @OfxParamPropEnabled)]
        param_enabled: Bool,
        #[meta(write = plugin, @OfxParamPropEvaluateOnChange)]
        param_evaluate_on_change: Bool,
        #[meta(write = plugin, @OfxParamPropHasHostOverlayHandle)]
        param_has_host_overlay_handle: Bool,
        #[meta(write = plugin, @OfxParamPropHint)]
        param_hint: String,
        #[meta(write = plugin, @OfxParamPropInteractMinimumSize)]
        param_interact_minimum_size: [Double; 2],
        #[meta(write = plugin, @OfxParamPropInteractPreferedSize)]
        param_interact_prefered_size: [Int; 2],
        #[meta(write = plugin, @OfxParamPropInteractSize)]
        param_interact_size: [Double; 2],
        #[meta(write = plugin, @OfxParamPropInteractSizeAspect)]
        param_interact_size_aspect: Double,
        #[meta(write = plugin, @OfxParamPropInteractV1)]
        param_interact_v_1: Pointer,
        #[meta(write = plugin, @OfxParamPropIsAnimating)]
        param_is_animating: Bool,
        #[meta(write = plugin, @OfxParamPropIsAutoKeying)]
        param_is_auto_keying: Bool,
        #[meta(write = plugin, @OfxParamPropParametricDimension)]
        param_parametric_dimension: Int,
        #[meta(write = plugin, @OfxParamPropParametricInteractBackground)]
        param_parametric_interact_background: Pointer,
        #[meta(write = plugin, @OfxParamPropParametricRange)]
        param_parametric_range: [Double; 2],
        #[meta(write = plugin, @OfxParamPropParametricUIColour)]
        param_parametric_ui_colour: [Double],
        #[meta(write = plugin, @OfxParamPropParent)]
        param_parent: String,
        #[meta(write = plugin, @OfxParamPropPersistant)]
        param_persistant: Bool,
        #[meta(write = plugin, @OfxParamPropPluginMayWrite)]
        param_plugin_may_write: Bool,
        #[meta(write = plugin, @OfxParamPropScriptName)]
        param_script_name: String,
        #[meta(write = plugin, @OfxParamPropSecret)]
        param_secret: Bool,
        #[meta(write = plugin, @OfxParamPropType)]
        param_type: String,
        #[meta(write = plugin, @OfxParamPropUseHostOverlayHandle)]
        param_use_host_overlay_handle: Bool,
        #[meta(write = plugin, @OfxPropIcon)]
        icon: [String; 2],
        #[meta(write = plugin, @OfxPropLabel)]
        label: String,
        #[meta(write = plugin, @OfxPropLongLabel)]
        long_label: String,
        #[meta(write = plugin, @OfxPropName)]
        name: String,
        #[meta(write = plugin, @OfxPropShortLabel)]
        short_label: String,
        #[meta(write = plugin, @OfxPropType)]
        r#type: String,
    }
    struct ParamStrChoicePropertySet {
        #[meta(write = plugin, @OfxParamPropAnimates)]
        param_animates: Bool,
        #[meta(write = plugin, @OfxParamPropCacheInvalidation)]
        param_cache_invalidation: ty_enum!(ParamPropCacheInvalidation),
        #[meta(write = plugin, @OfxParamPropCanUndo)]
        param_can_undo: Bool,
        #[meta(write = plugin, @OfxParamPropChoiceEnum)]
        param_choice_enum: Bool,
        #[meta(write = plugin, @OfxParamPropChoiceOption)]
        param_choice_option: [String],
        #[meta(write = plugin, @OfxParamPropDataPtr)]
        param_data_ptr: Pointer,
        #[meta(write = plugin, @OfxParamPropDefault)]
        param_default: [ty_one_of!(Int | Double | String | Pointer)],
        #[meta(write = plugin, @OfxParamPropEnabled)]
        param_enabled: Bool,
        #[meta(write = plugin, @OfxParamPropEvaluateOnChange)]
        param_evaluate_on_change: Bool,
        #[meta(write = plugin, @OfxParamPropHasHostOverlayHandle)]
        param_has_host_overlay_handle: Bool,
        #[meta(write = plugin, @OfxParamPropHint)]
        param_hint: String,
        #[meta(write = plugin, @OfxParamPropInteractMinimumSize)]
        param_interact_minimum_size: [Double; 2],
        #[meta(write = plugin, @OfxParamPropInteractPreferedSize)]
        param_interact_prefered_size: [Int; 2],
        #[meta(write = plugin, @OfxParamPropInteractSize)]
        param_interact_size: [Double; 2],
        #[meta(write = plugin, @OfxParamPropInteractSizeAspect)]
        param_interact_size_aspect: Double,
        #[meta(write = plugin, @OfxParamPropInteractV1)]
        param_interact_v_1: Pointer,
        #[meta(write = host, @OfxParamPropIsAnimating)]
        param_is_animating: Bool,
        #[meta(write = host, @OfxParamPropIsAutoKeying)]
        param_is_auto_keying: Bool,
        #[meta(write = plugin, @OfxParamPropParent)]
        param_parent: String,
        #[meta(write = plugin, @OfxParamPropPersistant)]
        param_persistant: Bool,
        #[meta(write = plugin, @OfxParamPropPluginMayWrite)]
        param_plugin_may_write: Bool,
        #[meta(write = plugin, @OfxParamPropScriptName)]
        param_script_name: String,
        #[meta(write = plugin, @OfxParamPropSecret)]
        param_secret: Bool,
        #[meta(write = plugin, @OfxParamPropType)]
        param_type: String,
        #[meta(write = plugin, @OfxParamPropUseHostOverlayHandle)]
        param_use_host_overlay_handle: Bool,
        #[meta(write = plugin, @OfxPropIcon)]
        icon: [String; 2],
        #[meta(write = plugin, @OfxPropLabel)]
        label: String,
        #[meta(write = plugin, @OfxPropLongLabel)]
        long_label: String,
        #[meta(write = plugin, @OfxPropName)]
        name: String,
        #[meta(write = plugin, @OfxPropShortLabel)]
        short_label: String,
        #[meta(write = plugin, @OfxPropType)]
        r#type: String,
    }
    struct ParamStringPropertySet {
        #[meta(write = plugin, @OfxParamPropAnimates)]
        param_animates: Bool,
        #[meta(write = plugin, @OfxParamPropCacheInvalidation)]
        param_cache_invalidation: ty_enum!(ParamPropCacheInvalidation),
        #[meta(write = plugin, @OfxParamPropCanUndo)]
        param_can_undo: Bool,
        #[meta(write = plugin, @OfxParamPropDataPtr)]
        param_data_ptr: Pointer,
        #[meta(write = plugin, @OfxParamPropDefault)]
        param_default: [ty_one_of!(Int | Double | String | Pointer)],
        #[meta(write = plugin, @OfxParamPropDisplayMax)]
        param_display_max: [ty_one_of!(Int | Double)],
        #[meta(write = plugin, @OfxParamPropDisplayMin)]
        param_display_min: [ty_one_of!(Int | Double)],
        #[meta(write = plugin, @OfxParamPropEnabled)]
        param_enabled: Bool,
        #[meta(write = plugin, @OfxParamPropEvaluateOnChange)]
        param_evaluate_on_change: Bool,
        #[meta(write = plugin, @OfxParamPropHasHostOverlayHandle)]
        param_has_host_overlay_handle: Bool,
        #[meta(write = plugin, @OfxParamPropHint)]
        param_hint: String,
        #[meta(write = plugin, @OfxParamPropInteractMinimumSize)]
        param_interact_minimum_size: [Double; 2],
        #[meta(write = plugin, @OfxParamPropInteractPreferedSize)]
        param_interact_prefered_size: [Int; 2],
        #[meta(write = plugin, @OfxParamPropInteractSize)]
        param_interact_size: [Double; 2],
        #[meta(write = plugin, @OfxParamPropInteractSizeAspect)]
        param_interact_size_aspect: Double,
        #[meta(write = plugin, @OfxParamPropInteractV1)]
        param_interact_v_1: Pointer,
        #[meta(write = host, @OfxParamPropIsAnimating)]
        param_is_animating: Bool,
        #[meta(write = host, @OfxParamPropIsAutoKeying)]
        param_is_auto_keying: Bool,
        #[meta(write = plugin, @OfxParamPropMax)]
        param_max: [ty_one_of!(Int | Double)],
        #[meta(write = plugin, @OfxParamPropMin)]
        param_min: [ty_one_of!(Int | Double)],
        #[meta(write = plugin, @OfxParamPropParent)]
        param_parent: String,
        #[meta(write = plugin, @OfxParamPropPersistant)]
        param_persistant: Bool,
        #[meta(write = plugin, @OfxParamPropPluginMayWrite)]
        param_plugin_may_write: Bool,
        #[meta(write = plugin, @OfxParamPropScriptName)]
        param_script_name: String,
        #[meta(write = plugin, @OfxParamPropSecret)]
        param_secret: Bool,
        #[meta(write = plugin, @OfxParamPropStringFilePathExists)]
        param_string_file_path_exists: Bool,
        #[meta(write = plugin, @OfxParamPropStringMode)]
        param_string_mode: ty_enum!(ParamPropStringMode),
        #[meta(write = plugin, @OfxParamPropType)]
        param_type: String,
        #[meta(write = plugin, @OfxParamPropUseHostOverlayHandle)]
        param_use_host_overlay_handle: Bool,
        #[meta(write = plugin, @OfxPropIcon)]
        icon: [String; 2],
        #[meta(write = plugin, @OfxPropLabel)]
        label: String,
        #[meta(write = plugin, @OfxPropLongLabel)]
        long_label: String,
        #[meta(write = plugin, @OfxPropName)]
        name: String,
        #[meta(write = plugin, @OfxPropShortLabel)]
        short_label: String,
        #[meta(write = plugin, @OfxPropType)]
        r#type: String,
    }
}
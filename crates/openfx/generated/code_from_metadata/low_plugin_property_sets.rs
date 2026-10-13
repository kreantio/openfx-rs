#[openfx_internal_macros::low_make_property_set_structs]
mod __ {
    #[propset(write = plugin)]
    struct ImageClipDescriptorPropertySet {
        #[prop(@OfxImageClipPropFieldExtraction)]
        image_clip_field_extraction: ty_enum!(ImageClipPropFieldExtraction),
        #[prop(@OfxImageClipPropIsMask)]
        image_clip_is_mask: Bool,
        #[prop(@OfxImageClipPropOptional)]
        image_clip_optional: Bool,
        #[prop(@OfxImageEffectPropSupportedComponents)]
        image_effect_supported_components: [ty_enum!(
            ImageEffectPropSupportedComponents
        )],
        #[prop(@OfxImageEffectPropSupportsTiles)]
        image_effect_supports_tiles: Bool,
        #[prop(@OfxImageEffectPropTemporalClipAccess)]
        image_effect_temporal_clip_access: Bool,
        #[prop(@OfxPropLabel)]
        label: String,
        #[prop(@OfxPropLongLabel)]
        long_label: String,
        #[prop(@OfxPropName)]
        name: String,
        #[prop(@OfxPropShortLabel)]
        short_label: String,
        #[prop(@OfxPropType)]
        r#type: String,
    }
    #[propset(write = host)]
    struct ImageClipInstancePropertySet {
        #[prop(@OfxImageClipPropColourspace)]
        image_clip_colourspace: String,
        #[prop(@OfxImageClipPropConnected)]
        image_clip_connected: Bool,
        #[prop(@OfxImageClipPropContinuousSamples)]
        image_clip_continuous_samples: Bool,
        #[prop(@OfxImageClipPropFieldExtraction)]
        image_clip_field_extraction: ty_enum!(ImageClipPropFieldExtraction),
        #[prop(@OfxImageClipPropFieldOrder)]
        image_clip_field_order: ty_enum!(ImageClipPropFieldOrder),
        #[prop(@OfxImageClipPropIsMask)]
        image_clip_is_mask: Bool,
        #[prop(@OfxImageClipPropOptional)]
        image_clip_optional: Bool,
        #[prop(@OfxImageClipPropPreferredColourspaces)]
        image_clip_preferred_colourspaces: [String],
        #[prop(@OfxImageClipPropUnmappedComponents)]
        image_clip_unmapped_components: ty_enum!(ImageClipPropUnmappedComponents),
        #[prop(@OfxImageClipPropUnmappedPixelDepth)]
        image_clip_unmapped_pixel_depth: ty_enum!(ImageClipPropUnmappedPixelDepth),
        #[prop(@OfxImageEffectPropComponents)]
        image_effect_components: ty_enum!(ImageEffectPropComponents),
        #[prop(@OfxImageEffectPropFrameRange)]
        image_effect_frame_range: [Double; 2],
        #[prop(@OfxImageEffectPropFrameRate)]
        image_effect_frame_rate: Double,
        #[prop(@OfxImageEffectPropPixelDepth)]
        image_effect_pixel_depth: ty_enum!(ImageEffectPropPixelDepth),
        #[prop(@OfxImageEffectPropPreMultiplication)]
        image_effect_pre_multiplication: ty_enum!(ImageEffectPropPreMultiplication),
        #[prop(@OfxImageEffectPropSupportedComponents)]
        image_effect_supported_components: [ty_enum!(
            ImageEffectPropSupportedComponents
        )],
        #[prop(@OfxImageEffectPropSupportsTiles)]
        image_effect_supports_tiles: Bool,
        #[prop(@OfxImageEffectPropTemporalClipAccess)]
        image_effect_temporal_clip_access: Bool,
        #[prop(@OfxImageEffectPropUnmappedFrameRange)]
        image_effect_unmapped_frame_range: [Double; 2],
        #[prop(@OfxImageEffectPropUnmappedFrameRate)]
        image_effect_unmapped_frame_rate: Double,
        #[prop(@OfxImagePropPixelAspectRatio)]
        image_pixel_aspect_ratio: Double,
        #[prop(@OfxPropLabel)]
        label: String,
        #[prop(@OfxPropLongLabel)]
        long_label: String,
        #[prop(@OfxPropName)]
        name: String,
        #[prop(@OfxPropShortLabel)]
        short_label: String,
        #[prop(@OfxPropType)]
        r#type: String,
    }
    #[propset(write = plugin)]
    struct ImageEffectDescriptorPropertySet {
        #[prop(@OfxImageEffectPluginPropFieldRenderTwiceAlways)]
        image_effect_plugin_field_render_twice_always: Bool,
        #[prop(@OfxImageEffectPluginPropGrouping)]
        image_effect_plugin_grouping: String,
        #[prop(@OfxImageEffectPluginPropHostFrameThreading)]
        image_effect_plugin_host_frame_threading: Bool,
        #[prop(@OfxImageEffectPluginPropObsolete)]
        image_effect_plugin_obsolete: Bool,
        #[prop(@OfxImageEffectPluginPropOverlayInteractV1)]
        image_effect_plugin_overlay_interact_v_1: Pointer,
        #[prop(@OfxImageEffectPluginPropOverlayInteractV2)]
        image_effect_plugin_overlay_interact_v_2: Pointer,
        #[prop(@OfxImageEffectPluginPropSingleInstance)]
        image_effect_plugin_single_instance: Bool,
        #[prop(@OfxImageEffectPluginRenderThreadSafety)]
        image_effect_plugin_render_thread_safety: ty_enum!(
            ImageEffectPluginRenderThreadSafety
        ),
        #[prop(@OfxImageEffectPropCPURenderSupported)]
        image_effect_cpu_render_supported: ty_enum!(ImageEffectPropCPURenderSupported),
        #[prop(@OfxImageEffectPropClipPreferencesSlaveParam)]
        image_effect_clip_preferences_slave_param: [String],
        #[prop(@OfxImageEffectPropColourManagementAvailableConfigs)]
        image_effect_colour_management_available_configs: [String],
        #[prop(@OfxImageEffectPropColourManagementStyle)]
        image_effect_colour_management_style: ty_enum!(
            ImageEffectPropColourManagementStyle
        ),
        #[prop(@OfxImageEffectPropNoSpatialAwareness)]
        image_effect_no_spatial_awareness: ty_enum!(ImageEffectPropNoSpatialAwareness),
        #[prop(@OfxImageEffectPropOpenCLSupported)]
        image_effect_open_cl_supported: ty_enum!(ImageEffectPropOpenCLSupported),
        #[prop(@OfxImageEffectPropOpenGLRenderSupported)]
        image_effect_open_gl_render_supported: ty_enum!(
            ImageEffectPropOpenGLRenderSupported
        ),
        #[prop(@OfxImageEffectPropSupportedContexts)]
        image_effect_supported_contexts: [ty_enum!(ImageEffectPropSupportedContexts)],
        #[prop(@OfxImageEffectPropSupportedPixelDepths)]
        image_effect_supported_pixel_depths: [ty_enum!(
            ImageEffectPropSupportedPixelDepths
        )],
        #[prop(@OfxImageEffectPropSupportsMultiResolution)]
        image_effect_supports_multi_resolution: Bool,
        #[prop(@OfxImageEffectPropSupportsMultipleClipDepths)]
        image_effect_supports_multiple_clip_depths: Bool,
        #[prop(@OfxImageEffectPropSupportsMultipleClipPARs)]
        image_effect_supports_multiple_clip_pa_rs: Bool,
        #[prop(@OfxImageEffectPropSupportsTiles)]
        image_effect_supports_tiles: Bool,
        #[prop(@OfxImageEffectPropTemporalClipAccess)]
        image_effect_temporal_clip_access: Bool,
        #[prop(@OfxOpenGLPropPixelDepth)]
        open_gl_pixel_depth: [ty_enum!(OpenGLPropPixelDepth)],
        #[prop(write = host, @OfxPluginPropFilePath)]
        plugin_file_path: String,
        #[prop(@OfxPropLabel)]
        label: String,
        #[prop(@OfxPropLongLabel)]
        long_label: String,
        #[prop(@OfxPropPluginDescription)]
        plugin_description: String,
        #[prop(@OfxPropShortLabel)]
        short_label: String,
        #[prop(@OfxPropType)]
        r#type: String,
        #[prop(@OfxPropVersion)]
        version: [Int],
        #[prop(@OfxPropVersionLabel)]
        version_label: String,
    }
    #[propset(write = host)]
    struct ImageEffectInstancePropertySet {
        #[prop(@OfxImageEffectInstancePropEffectDuration)]
        image_effect_instance_effect_duration: Double,
        #[prop(@OfxImageEffectInstancePropSequentialRender)]
        image_effect_instance_sequential_render: Bool,
        #[prop(@OfxImageEffectPropCPURenderSupported)]
        image_effect_cpu_render_supported: ty_enum!(ImageEffectPropCPURenderSupported),
        #[prop(@OfxImageEffectPropColourManagementConfig)]
        image_effect_colour_management_config: String,
        #[prop(@OfxImageEffectPropColourManagementStyle)]
        image_effect_colour_management_style: ty_enum!(
            ImageEffectPropColourManagementStyle
        ),
        #[prop(@OfxImageEffectPropContext)]
        image_effect_context: ty_enum!(ImageEffectPropContext),
        #[prop(@OfxImageEffectPropDisplayColourspace)]
        image_effect_display_colourspace: String,
        #[prop(@OfxImageEffectPropFrameRate)]
        image_effect_frame_rate: Double,
        #[prop(@OfxImageEffectPropOCIOConfig)]
        image_effect_ocio_config: String,
        #[prop(@OfxImageEffectPropOCIODisplay)]
        image_effect_ocio_display: String,
        #[prop(@OfxImageEffectPropOCIOView)]
        image_effect_ocio_view: String,
        #[prop(@OfxImageEffectPropOpenGLRenderSupported)]
        image_effect_open_gl_render_supported: ty_enum!(
            ImageEffectPropOpenGLRenderSupported
        ),
        #[prop(@OfxImageEffectPropPluginHandle)]
        image_effect_plugin_handle: Pointer,
        #[prop(@OfxImageEffectPropProjectExtent)]
        image_effect_project_extent: [Double; 2],
        #[prop(@OfxImageEffectPropProjectOffset)]
        image_effect_project_offset: [Double; 2],
        #[prop(@OfxImageEffectPropProjectPixelAspectRatio)]
        image_effect_project_pixel_aspect_ratio: Double,
        #[prop(@OfxImageEffectPropProjectSize)]
        image_effect_project_size: [Double; 2],
        #[prop(@OfxImageEffectPropSupportsTiles)]
        image_effect_supports_tiles: Bool,
        #[prop(@OfxPropInstanceData)]
        instance_data: Pointer,
        #[prop(@OfxPropIsInteractive)]
        is_interactive: Bool,
        #[prop(@OfxPropType)]
        r#type: String,
    }
    #[propset(write = host)]
    struct ImageInstancePropertySet {
        #[prop(@OfxImageEffectPropComponents)]
        image_effect_components: ty_enum!(ImageEffectPropComponents),
        #[prop(@OfxImageEffectPropPixelDepth)]
        image_effect_pixel_depth: ty_enum!(ImageEffectPropPixelDepth),
        #[prop(@OfxImageEffectPropPreMultiplication)]
        image_effect_pre_multiplication: ty_enum!(ImageEffectPropPreMultiplication),
        #[prop(@OfxImageEffectPropRenderScale)]
        image_effect_render_scale: [Double; 2],
        #[prop(@OfxImagePropBounds)]
        image_bounds: [Int; 4],
        #[prop(@OfxImagePropData)]
        image_data: Pointer,
        #[prop(@OfxImagePropField)]
        image_field: ty_enum!(ImagePropField),
        #[prop(@OfxImagePropPixelAspectRatio)]
        image_pixel_aspect_ratio: Double,
        #[prop(@OfxImagePropRegionOfDefinition)]
        image_region_of_definition: [Int; 4],
        #[prop(@OfxImagePropRowBytes)]
        image_row_bytes: Int,
        #[prop(@OfxImagePropUniqueIdentifier)]
        image_unique_identifier: String,
        #[prop(@OfxPropType)]
        r#type: String,
    }
    #[propset(write = host)]
    struct ImageEffectHostDescriptorPropertySet {
        #[prop(@OfxImageEffectHostPropIsBackground)]
        image_effect_host_is_background: Bool,
        #[prop(@OfxImageEffectHostPropNativeOrigin)]
        image_effect_host_native_origin: ty_enum!(ImageEffectHostPropNativeOrigin),
        #[prop(@OfxImageEffectInstancePropSequentialRender)]
        image_effect_instance_sequential_render: Bool,
        #[prop(@OfxImageEffectPropCPURenderSupported)]
        image_effect_cpu_render_supported: ty_enum!(ImageEffectPropCPURenderSupported),
        #[prop(@OfxImageEffectPropColourManagementAvailableConfigs)]
        image_effect_colour_management_available_configs: [String],
        #[prop(@OfxImageEffectPropColourManagementStyle)]
        image_effect_colour_management_style: ty_enum!(
            ImageEffectPropColourManagementStyle
        ),
        #[prop(@OfxImageEffectPropOpenCLSupported)]
        image_effect_open_cl_supported: ty_enum!(ImageEffectPropOpenCLSupported),
        #[prop(@OfxImageEffectPropOpenGLRenderSupported)]
        image_effect_open_gl_render_supported: ty_enum!(
            ImageEffectPropOpenGLRenderSupported
        ),
        #[prop(@OfxImageEffectPropRenderQualityDraft)]
        image_effect_render_quality_draft: Bool,
        #[prop(@OfxImageEffectPropSetableFielding)]
        image_effect_setable_fielding: Bool,
        #[prop(@OfxImageEffectPropSetableFrameRate)]
        image_effect_setable_frame_rate: Bool,
        #[prop(@OfxImageEffectPropSupportedComponents)]
        image_effect_supported_components: [ty_enum!(
            ImageEffectPropSupportedComponents
        )],
        #[prop(@OfxImageEffectPropSupportedContexts)]
        image_effect_supported_contexts: [ty_enum!(ImageEffectPropSupportedContexts)],
        #[prop(@OfxImageEffectPropSupportsMultiResolution)]
        image_effect_supports_multi_resolution: Bool,
        #[prop(@OfxImageEffectPropSupportsMultipleClipDepths)]
        image_effect_supports_multiple_clip_depths: Bool,
        #[prop(@OfxImageEffectPropSupportsMultipleClipPARs)]
        image_effect_supports_multiple_clip_pa_rs: Bool,
        #[prop(@OfxImageEffectPropSupportsOverlays)]
        image_effect_supports_overlays: Bool,
        #[prop(@OfxImageEffectPropSupportsTiles)]
        image_effect_supports_tiles: Bool,
        #[prop(@OfxImageEffectPropTemporalClipAccess)]
        image_effect_temporal_clip_access: Bool,
        #[prop(@OfxParamHostPropMaxPages)]
        param_host_max_pages: Int,
        #[prop(@OfxParamHostPropMaxParameters)]
        param_host_max_parameters: Int,
        #[prop(@OfxParamHostPropPageRowColumnCount)]
        param_host_page_row_column_count: [Int; 2],
        #[prop(@OfxParamHostPropSupportsBooleanAnimation)]
        param_host_supports_boolean_animation: Bool,
        #[prop(@OfxParamHostPropSupportsChoiceAnimation)]
        param_host_supports_choice_animation: Bool,
        #[prop(@OfxParamHostPropSupportsCustomAnimation)]
        param_host_supports_custom_animation: Bool,
        #[prop(@OfxParamHostPropSupportsCustomInteract)]
        param_host_supports_custom_interact: Bool,
        #[prop(@OfxParamHostPropSupportsParametricAnimation)]
        param_host_supports_parametric_animation: Bool,
        #[prop(@OfxParamHostPropSupportsStrChoice)]
        param_host_supports_str_choice: Bool,
        #[prop(@OfxParamHostPropSupportsStrChoiceAnimation)]
        param_host_supports_str_choice_animation: Bool,
        #[prop(@OfxParamHostPropSupportsStringAnimation)]
        param_host_supports_string_animation: Bool,
        #[prop(@OfxPropAPIVersion)]
        api_version: [Int],
        #[prop(@OfxPropHostOSHandle)]
        host_os_handle: Pointer,
        #[prop(@OfxPropLabel)]
        label: String,
        #[prop(@OfxPropName)]
        name: String,
        #[prop(@OfxPropType)]
        r#type: String,
        #[prop(@OfxPropVersion)]
        version: [Int],
        #[prop(@OfxPropVersionLabel)]
        version_label: String,
    }
    #[propset(write = host)]
    struct InteractDescriptorPropertySet {
        #[prop(@OfxInteractPropBitDepth)]
        interact_bit_depth: Int,
        #[prop(@OfxInteractPropHasAlpha)]
        interact_has_alpha: Bool,
    }
    #[propset(write = host)]
    struct InteractInstancePropertySet {
        #[prop(@OfxInteractPropBackgroundColour)]
        interact_background_colour: [Double; 3],
        #[prop(@OfxInteractPropBitDepth)]
        interact_bit_depth: Int,
        #[prop(@OfxInteractPropHasAlpha)]
        interact_has_alpha: Bool,
        #[prop(@OfxInteractPropPixelScale)]
        interact_pixel_scale: [Double; 2],
        #[prop(@OfxInteractPropSlaveToParam)]
        interact_slave_to_param: [String],
        #[prop(@OfxInteractPropSuggestedColour)]
        interact_suggested_colour: [Double; 3],
        #[prop(@OfxPropEffectInstance)]
        effect_instance: Pointer,
        #[prop(@OfxPropInstanceData)]
        instance_data: Pointer,
    }
    #[propset(
        write = plugin,
        refs = ParamsAllButGroupPage+ParamsCommon+ParamsDouble+ParamsNumeric+ParamsValue
    )]
    struct ParamDouble1DPropertySet {
        #[prop(@OfxParamPropDoubleType)]
        param_double_type: ty_enum!(ParamPropDoubleType),
        #[prop(@OfxParamPropShowTimeMarker)]
        param_show_time_marker: Bool,
    }
    #[propset(write = plugin)]
    struct ParamSetPropertySet {
        #[prop(@OfxPluginPropParamPageOrder)]
        plugin_param_page_order: [String],
        #[prop(@OfxPropParamSetNeedsSyncing)]
        param_set_needs_syncing: Bool,
    }
    #[propset(
        write = plugin,
        refs = ParamsAllButGroupPage+ParamsCommon+ParamsNumeric+ParamsValue
    )]
    struct ParamBytePropertySet {}
    #[propset(write = plugin, refs = ParamsAllButGroupPage+ParamsCommon+ParamsValue)]
    struct ParamChoicePropertySet {
        #[prop(@OfxParamPropChoiceOption)]
        param_choice_option: [String],
        #[prop(@OfxParamPropChoiceOrder)]
        param_choice_order: [Int],
    }
    #[propset(write = plugin, refs = ParamsAllButGroupPage+ParamsCommon+ParamsValue)]
    struct ParamCustomPropertySet {
        #[prop(@OfxParamPropCustomInterpCallbackV1)]
        param_custom_interp_callback_v_1: Pointer,
    }
    #[propset(
        write = plugin,
        refs = ParamsAllButGroupPage+ParamsCommon+ParamsDouble+ParamsNumeric+ParamsValue
    )]
    struct ParamDouble2D3DPropertySet {
        #[prop(@OfxParamPropDoubleType)]
        param_double_type: ty_enum!(ParamPropDoubleType),
    }
    #[propset(write = plugin, refs = ParamsCommon)]
    struct ParamGroupPropertySet {
        #[prop(@OfxParamPropGroupOpen)]
        param_group_open: Bool,
    }
    #[propset(
        write = plugin,
        refs = ParamsAllButGroupPage+ParamsCommon+ParamsNumeric+ParamsValue
    )]
    struct ParamInteger2D3DPropertySet {
        #[prop(@OfxParamPropDimensionLabel)]
        param_dimension_label: String,
    }
    #[propset(
        write = plugin,
        refs = ParamsAllButGroupPage+ParamsCommon+ParamsDouble+ParamsNumeric+ParamsValue
    )]
    struct ParamNormalizedSpatialPropertySet {
        #[prop(@OfxParamPropDefaultCoordinateSystem)]
        param_default_coordinate_system: ty_enum!(ParamPropDefaultCoordinateSystem),
    }
    #[propset(write = plugin, refs = ParamsCommon)]
    struct ParamPagePropertySet {
        #[prop(@OfxParamPropPageChild)]
        param_page_child: [String],
    }
    #[propset(write = plugin, refs = ParamsAllButGroupPage+ParamsCommon+ParamsValue)]
    struct ParamParametricPropertySet {
        #[prop(@OfxParamPropAnimates)]
        param_animates: Bool,
        #[prop(@OfxParamPropCacheInvalidation)]
        param_cache_invalidation: ty_enum!(ParamPropCacheInvalidation),
        #[prop(@OfxParamPropCanUndo)]
        param_can_undo: Bool,
        #[prop(@OfxParamPropEvaluateOnChange)]
        param_evaluate_on_change: Bool,
        #[prop(@OfxParamPropIsAnimating)]
        param_is_animating: Bool,
        #[prop(@OfxParamPropIsAutoKeying)]
        param_is_auto_keying: Bool,
        #[prop(@OfxParamPropParametricDimension)]
        param_parametric_dimension: Int,
        #[prop(@OfxParamPropParametricInteractBackground)]
        param_parametric_interact_background: Pointer,
        #[prop(@OfxParamPropParametricRange)]
        param_parametric_range: [Double; 2],
        #[prop(@OfxParamPropParametricUIColour)]
        param_parametric_ui_colour: [Double],
        #[prop(@OfxParamPropPersistant)]
        param_persistant: Bool,
        #[prop(@OfxParamPropPluginMayWrite)]
        param_plugin_may_write: Bool,
    }
    #[propset(write = plugin, refs = ParamsAllButGroupPage+ParamsCommon+ParamsValue)]
    struct ParamStrChoicePropertySet {
        #[prop(@OfxParamPropChoiceEnum)]
        param_choice_enum: Bool,
        #[prop(@OfxParamPropChoiceOption)]
        param_choice_option: [String],
    }
    #[propset(
        write = plugin,
        refs = ParamsAllButGroupPage+ParamsCommon+ParamsNumeric+ParamsValue
    )]
    struct ParamStringPropertySet {
        #[prop(@OfxParamPropStringFilePathExists)]
        param_string_file_path_exists: Bool,
        #[prop(@OfxParamPropStringMode)]
        param_string_mode: ty_enum!(ParamPropStringMode),
    }
    #[propsetdefs]
    mod __ {
        #[propsetdef]
        struct ParamsAllButGroupPage {
            #[prop(@OfxParamPropHasHostOverlayHandle)]
            param_has_host_overlay_handle: Bool,
            #[prop(@OfxParamPropInteractMinimumSize)]
            param_interact_minimum_size: [Double; 2],
            #[prop(@OfxParamPropInteractPreferedSize)]
            param_interact_prefered_size: [Int; 2],
            #[prop(@OfxParamPropInteractSize)]
            param_interact_size: [Double; 2],
            #[prop(@OfxParamPropInteractSizeAspect)]
            param_interact_size_aspect: Double,
            #[prop(@OfxParamPropInteractV1)]
            param_interact_v_1: Pointer,
            #[prop(@OfxParamPropUseHostOverlayHandle)]
            param_use_host_overlay_handle: Bool,
        }
        #[propsetdef]
        struct ParamsCommon {
            #[prop(@OfxParamPropDataPtr)]
            param_data_ptr: Pointer,
            #[prop(@OfxParamPropEnabled)]
            param_enabled: Bool,
            #[prop(@OfxParamPropHint)]
            param_hint: String,
            #[prop(@OfxParamPropParent)]
            param_parent: String,
            #[prop(@OfxParamPropScriptName)]
            param_script_name: String,
            #[prop(@OfxParamPropSecret)]
            param_secret: Bool,
            #[prop(@OfxParamPropType)]
            param_type: String,
            #[prop(@OfxPropIcon)]
            icon: [String; 2],
            #[prop(@OfxPropLabel)]
            label: String,
            #[prop(@OfxPropLongLabel)]
            long_label: String,
            #[prop(@OfxPropName)]
            name: String,
            #[prop(@OfxPropShortLabel)]
            short_label: String,
            #[prop(@OfxPropType)]
            r#type: String,
        }
        #[propsetdef]
        struct ParamsDouble {
            #[prop(@OfxParamPropDigits)]
            param_digits: Int,
            #[prop(@OfxParamPropIncrement)]
            param_increment: Double,
        }
        #[propsetdef]
        struct ParamsNumeric {
            #[prop(@OfxParamPropDisplayMax)]
            param_display_max: [ty_one_of!(Int | Double)],
            #[prop(@OfxParamPropDisplayMin)]
            param_display_min: [ty_one_of!(Int | Double)],
            #[prop(@OfxParamPropMax)]
            param_max: [ty_one_of!(Int | Double)],
            #[prop(@OfxParamPropMin)]
            param_min: [ty_one_of!(Int | Double)],
        }
        #[propsetdef]
        struct ParamsValue {
            #[prop(@OfxParamPropAnimates)]
            param_animates: Bool,
            #[prop(@OfxParamPropCacheInvalidation)]
            param_cache_invalidation: ty_enum!(ParamPropCacheInvalidation),
            #[prop(@OfxParamPropCanUndo)]
            param_can_undo: Bool,
            #[prop(@OfxParamPropDefault)]
            param_default: [ty_one_of!(Int | Double | String | Pointer)],
            #[prop(@OfxParamPropEvaluateOnChange)]
            param_evaluate_on_change: Bool,
            #[prop(write = host, @OfxParamPropIsAnimating)]
            param_is_animating: Bool,
            #[prop(write = host, @OfxParamPropIsAutoKeying)]
            param_is_auto_keying: Bool,
            #[prop(@OfxParamPropPersistant)]
            param_persistant: Bool,
            #[prop(@OfxParamPropPluginMayWrite)]
            param_plugin_may_write: Bool,
        }
    }
}

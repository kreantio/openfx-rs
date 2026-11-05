use crate::low_plugin::internal_macros::make_param_define_fns;
#[openfx_internal_macros::low_make_property_set_structs]
mod __ {
    # [propset (monomorphization = { type = (Bool ; 1) , allow_list = [param_default] } , write = plugin , refs = ParamsAllButGroupPage + ParamsCommon + ParamsValue)]
    struct ParamBooleanPropertySet {}
    # [propset (monomorphization = { type = (Pointer ; 1) , allow_list = [param_default] } , write = plugin , refs = ParamsAllButGroupPage + ParamsCommon + ParamsValue)]
    struct ParamBytesPropertySet {}
    # [propset (monomorphization = { type = (Int ; 1) , allow_list = [param_default] } , write = plugin , refs = ParamsAllButGroupPage + ParamsCommon + ParamsValue)]
    struct ParamChoicePropertySet {
        # [prop (@ OfxParamPropChoiceOption)]
        param_choice_option: [String],
        # [prop (@ OfxParamPropChoiceOrder)]
        param_choice_order: [Int],
    }
    # [propset (monomorphization = { type = (String ; 1) , allow_list = [param_default] } , write = plugin , refs = ParamsAllButGroupPage + ParamsCommon + ParamsValue)]
    struct ParamCustomPropertySet {
        # [prop (@ OfxParamPropCustomInterpCallbackV1)]
        param_custom_interp_callback_v1: Pointer,
    }
    # [propset (monomorphization = { type = (Double ; 1) , allow_list = [param_default , param_display_max , param_display_min , param_max , param_min] } , write = plugin , refs = ParamsAllButGroupPage + ParamsCommon + ParamsDouble + ParamsNumeric + ParamsValue)]
    struct ParamDoublePropertySet {
        # [prop (@ OfxParamPropDefaultCoordinateSystem)]
        param_default_coordinate_system: ty_enum!(ParamPropDefaultCoordinateSystem),
        # [prop (@ OfxParamPropDoubleType)]
        param_double_type: ty_enum!(ParamPropDoubleType),
        # [prop (@ OfxParamPropShowTimeMarker)]
        param_show_time_marker: Bool,
    }
    # [propset (monomorphization = { type = (Double ; 2) , allow_list = [param_default , param_display_max , param_display_min , param_max , param_min] } , write = plugin , refs = ParamsAllButGroupPage + ParamsCommon + ParamsDouble + ParamsNumeric + ParamsValue)]
    struct ParamDouble2DPropertySet {
        # [prop (@ OfxParamPropDefaultCoordinateSystem)]
        param_default_coordinate_system: ty_enum!(ParamPropDefaultCoordinateSystem),
        # [prop (@ OfxParamPropDimensionLabel)]
        param_dimension_label: [String],
        # [prop (@ OfxParamPropDoubleType)]
        param_double_type: ty_enum!(ParamPropDoubleType),
    }
    # [propset (monomorphization = { type = (Double ; 3) , allow_list = [param_default , param_display_max , param_display_min , param_max , param_min] } , write = plugin , refs = ParamsAllButGroupPage + ParamsCommon + ParamsDouble + ParamsNumeric + ParamsValue)]
    struct ParamDouble3DPropertySet {
        # [prop (@ OfxParamPropDefaultCoordinateSystem)]
        param_default_coordinate_system: ty_enum!(ParamPropDefaultCoordinateSystem),
        # [prop (@ OfxParamPropDimensionLabel)]
        param_dimension_label: [String],
        # [prop (@ OfxParamPropDoubleType)]
        param_double_type: ty_enum!(ParamPropDoubleType),
    }
    # [propset (monomorphization = { type = None , allow_list = [] } , write = plugin , refs = ParamsCommon)]
    struct ParamGroupPropertySet {
        # [prop (@ OfxParamPropGroupOpen)]
        param_group_open: Bool,
    }
    # [propset (monomorphization = { type = (Int ; 1) , allow_list = [param_default , param_display_max , param_display_min , param_max , param_min] } , write = plugin , refs = ParamsAllButGroupPage + ParamsCommon + ParamsNumeric + ParamsValue)]
    struct ParamIntegerPropertySet {}
    # [propset (monomorphization = { type = (Int ; 2) , allow_list = [param_default , param_display_max , param_display_min , param_max , param_min] } , write = plugin , refs = ParamsAllButGroupPage + ParamsCommon + ParamsNumeric + ParamsValue)]
    struct ParamInteger2DPropertySet {
        # [prop (@ OfxParamPropDimensionLabel)]
        param_dimension_label: [String],
    }
    # [propset (monomorphization = { type = (Int ; 3) , allow_list = [param_default , param_display_max , param_display_min , param_max , param_min] } , write = plugin , refs = ParamsAllButGroupPage + ParamsCommon + ParamsNumeric + ParamsValue)]
    struct ParamInteger3DPropertySet {
        # [prop (@ OfxParamPropDimensionLabel)]
        param_dimension_label: [String],
    }
    # [propset (monomorphization = { type = None , allow_list = [] } , write = plugin , refs = ParamsCommon)]
    struct ParamPagePropertySet {
        # [prop (@ OfxParamPropPageChild)]
        param_page_child: [String],
    }
    # [propset (monomorphization = { type = None , allow_list = [] } , write = plugin , refs = ParamsAllButGroupPage + ParamsCommon)]
    struct ParamPushButtonPropertySet {}
    # [propset (monomorphization = { type = (Double ; 3) , allow_list = [param_default , param_display_max , param_display_min , param_max , param_min] } , write = plugin , refs = ParamsAllButGroupPage + ParamsCommon + ParamsNumeric + ParamsValue)]
    struct ParamRGBPropertySet {
        # [prop (@ OfxParamPropColourManagement)]
        param_colour_management: ty_enum!(ParamPropColourManagement),
    }
    # [propset (monomorphization = { type = (Double ; 4) , allow_list = [param_default , param_display_max , param_display_min , param_max , param_min] } , write = plugin , refs = ParamsAllButGroupPage + ParamsCommon + ParamsNumeric + ParamsValue)]
    struct ParamRGBAPropertySet {
        # [prop (@ OfxParamPropColourManagement)]
        param_colour_management: ty_enum!(ParamPropColourManagement),
    }
    # [propset (monomorphization = { type = (String ; 1) , allow_list = [param_default] } , write = plugin , refs = ParamsAllButGroupPage + ParamsCommon + ParamsValue)]
    struct ParamStrChoicePropertySet {
        # [prop (@ OfxParamPropChoiceEnum)]
        param_choice_enum: [String],
        # [prop (@ OfxParamPropChoiceOption)]
        param_choice_option: [String],
    }
    # [propset (monomorphization = { type = (String ; 1) , allow_list = [param_default] } , write = plugin , refs = ParamsAllButGroupPage + ParamsCommon + ParamsValue)]
    struct ParamStringPropertySet {
        # [prop (@ OfxParamPropStringFilePathExists)]
        param_string_file_path_exists: Bool,
        # [prop (@ OfxParamPropStringMode)]
        param_string_mode: ty_enum!(ParamPropStringMode),
    }
    #[propsetdefs]
    mod __ {
        #[propsetdef]
        struct ParamsAllButGroupPage {
            # [prop (write = host , @ OfxParamPropHasHostOverlayHandle)]
            param_has_host_overlay_handle: Bool,
            # [prop (@ OfxParamPropInteractMinimumSize)]
            param_interact_minimum_size: [Double; 2],
            # [prop (@ OfxParamPropInteractPreferedSize)]
            param_interact_prefered_size: [Int; 2],
            # [prop (@ OfxParamPropInteractSize)]
            param_interact_size: [Double; 2],
            # [prop (@ OfxParamPropInteractSizeAspect)]
            param_interact_size_aspect: Double,
            # [prop (@ OfxParamPropInteractV1)]
            param_interact_v1: Pointer,
            # [prop (@ OfxParamPropUseHostOverlayHandle)]
            param_use_host_overlay_handle: Bool,
        }
        #[propsetdef]
        struct ParamsCommon {
            # [prop (@ OfxParamPropDataPtr)]
            param_data_ptr: Pointer,
            # [prop (@ OfxParamPropEnabled)]
            param_enabled: Bool,
            # [prop (@ OfxParamPropHint)]
            param_hint: String,
            # [prop (@ OfxParamPropParent)]
            param_parent: String,
            # [prop (@ OfxParamPropScriptName)]
            param_script_name: String,
            # [prop (@ OfxParamPropSecret)]
            param_secret: Bool,
            # [prop (@ OfxParamPropType)]
            param_type: String,
            # [prop (@ OfxPropIcon)]
            icon: [String; 2],
            # [prop (@ OfxPropLabel)]
            label: String,
            # [prop (@ OfxPropLongLabel)]
            long_label: String,
            # [prop (@ OfxPropName)]
            name: String,
            # [prop (@ OfxPropShortLabel)]
            short_label: String,
            # [prop (@ OfxPropType)]
            r#type: String,
        }
        #[propsetdef]
        struct ParamsDouble {
            # [prop (@ OfxParamPropDigits)]
            param_digits: Int,
            # [prop (@ OfxParamPropIncrement)]
            param_increment: Double,
        }
        #[propsetdef]
        struct ParamsNumeric {
            # [prop (@ OfxParamPropDisplayMax)]
            param_display_max: [ty_one_of!(Int | Double)],
            # [prop (@ OfxParamPropDisplayMin)]
            param_display_min: [ty_one_of!(Int | Double)],
            # [prop (@ OfxParamPropMax)]
            param_max: [ty_one_of!(Int | Double)],
            # [prop (@ OfxParamPropMin)]
            param_min: [ty_one_of!(Int | Double)],
        }
        #[propsetdef]
        struct ParamsValue {
            # [prop (@ OfxParamPropAnimates)]
            param_animates: Bool,
            # [prop (@ OfxParamPropCacheInvalidation)]
            param_cache_invalidation: ty_enum!(ParamPropCacheInvalidation),
            # [prop (@ OfxParamPropCanUndo)]
            param_can_undo: Bool,
            # [prop (@ OfxParamPropDefault)]
            param_default: [ty_one_of!(Int | Double | String | Pointer)],
            # [prop (@ OfxParamPropEvaluateOnChange)]
            param_evaluate_on_change: Bool,
            # [prop (write = host , @ OfxParamPropIsAnimating)]
            param_is_animating: Bool,
            # [prop (write = host , @ OfxParamPropIsAutoKeying)]
            param_is_auto_keying: Bool,
            # [prop (@ OfxParamPropPersistant)]
            param_persistant: Bool,
            # [prop (@ OfxParamPropPluginMayWrite)]
            param_plugin_may_write: Bool,
        }
    }
}
impl crate::low_plugin::objects::ParamSetDescriptor {
    make_param_define_fns!(
        param_define_boolean,
        param_define_boolean_no_output,
        ParamBooleanPropertySet,
        kOfxParamTypeBoolean
    );
    make_param_define_fns!(
        param_define_bytes,
        param_define_bytes_no_output,
        ParamBytesPropertySet,
        kOfxParamTypeBytes
    );
    make_param_define_fns!(
        param_define_choice,
        param_define_choice_no_output,
        ParamChoicePropertySet,
        kOfxParamTypeChoice
    );
    make_param_define_fns!(
        param_define_custom,
        param_define_custom_no_output,
        ParamCustomPropertySet,
        kOfxParamTypeCustom
    );
    make_param_define_fns!(
        param_define_double,
        param_define_double_no_output,
        ParamDoublePropertySet,
        kOfxParamTypeDouble
    );
    make_param_define_fns!(
        param_define_double_2d,
        param_define_double_2d_no_output,
        ParamDouble2DPropertySet,
        kOfxParamTypeDouble2D
    );
    make_param_define_fns!(
        param_define_double_3d,
        param_define_double_3d_no_output,
        ParamDouble3DPropertySet,
        kOfxParamTypeDouble3D
    );
    make_param_define_fns!(
        param_define_group,
        param_define_group_no_output,
        ParamGroupPropertySet,
        kOfxParamTypeGroup
    );
    make_param_define_fns!(
        param_define_integer,
        param_define_integer_no_output,
        ParamIntegerPropertySet,
        kOfxParamTypeInteger
    );
    make_param_define_fns!(
        param_define_integer_2d,
        param_define_integer_2d_no_output,
        ParamInteger2DPropertySet,
        kOfxParamTypeInteger2D
    );
    make_param_define_fns!(
        param_define_integer_3d,
        param_define_integer_3d_no_output,
        ParamInteger3DPropertySet,
        kOfxParamTypeInteger3D
    );
    make_param_define_fns!(
        param_define_page,
        param_define_page_no_output,
        ParamPagePropertySet,
        kOfxParamTypePage
    );
    make_param_define_fns!(
        param_define_push_button,
        param_define_push_button_no_output,
        ParamPushButtonPropertySet,
        kOfxParamTypePushButton
    );
    make_param_define_fns!(
        param_define_rgb,
        param_define_rgb_no_output,
        ParamRGBPropertySet,
        kOfxParamTypeRGB
    );
    make_param_define_fns!(
        param_define_rgba,
        param_define_rgba_no_output,
        ParamRGBAPropertySet,
        kOfxParamTypeRGBA
    );
    make_param_define_fns!(
        param_define_str_choice,
        param_define_str_choice_no_output,
        ParamStrChoicePropertySet,
        kOfxParamTypeStrChoice
    );
    make_param_define_fns!(
        param_define_string,
        param_define_string_no_output,
        ParamStringPropertySet,
        kOfxParamTypeString
    );
}

use openfx_internal_macros::low_make_parameter_struct;
low_make_parameter_struct ! (ParamBoolean , type = [Bool ; 1]);
low_make_parameter_struct ! (ParamBytes , type = [Pointer ; 1]);
low_make_parameter_struct ! (ParamChoice , type = [Int ; 1]);
low_make_parameter_struct ! (ParamCustom , type = [String ; 1]);
low_make_parameter_struct ! (ParamDouble , type = [Double ; 1]);
low_make_parameter_struct ! (ParamDouble2D , type = [Double ; 2]);
low_make_parameter_struct ! (ParamDouble3D , type = [Double ; 3]);
low_make_parameter_struct ! (ParamGroup , type = None);
low_make_parameter_struct ! (ParamInteger , type = [Int ; 1]);
low_make_parameter_struct ! (ParamInteger2D , type = [Int ; 2]);
low_make_parameter_struct ! (ParamInteger3D , type = [Int ; 3]);
low_make_parameter_struct ! (ParamPage , type = None);
low_make_parameter_struct ! (ParamPushButton , type = None);
low_make_parameter_struct ! (ParamRGB , type = [Double ; 3]);
low_make_parameter_struct ! (ParamRGBA , type = [Double ; 4]);
low_make_parameter_struct ! (ParamStrChoice , type = [String ; 1]);
low_make_parameter_struct ! (ParamString , type = [String ; 1]);
#[openfx_internal_macros::low_make_property_set_structs]
mod __ {
    # [propset (monomorphization = { type = [Bool ; 1] , allow_list = [param_default] } , write = plugin , refs = ParamsAllButGroupPage + ParamsCommon + ParamsValue)]
    struct ParamBooleanPropertySet {}
    # [propset (monomorphization = { type = [Pointer ; 1] , allow_list = [param_default] } , write = plugin , refs = ParamsAllButGroupPage + ParamsCommon + ParamsValue)]
    struct ParamBytesPropertySet {}
    # [propset (monomorphization = { type = [Int ; 1] , allow_list = [param_default] } , write = plugin , refs = ParamsAllButGroupPage + ParamsCommon + ParamsValue)]
    struct ParamChoicePropertySet {
        # [prop (@ OfxParamPropChoiceOption)]
        param_choice_option: [String],
        # [prop (@ OfxParamPropChoiceOrder)]
        param_choice_order: [Int],
    }
    # [propset (monomorphization = { type = [String ; 1] , allow_list = [param_default] } , write = plugin , refs = ParamsAllButGroupPage + ParamsCommon + ParamsValue)]
    struct ParamCustomPropertySet {
        # [prop (@ OfxParamPropCustomInterpCallbackV1)]
        param_custom_interp_callback_v1: Pointer,
    }
    # [propset (monomorphization = { type = [Double ; 1] , allow_list = [param_default , param_display_max , param_display_min , param_max , param_min] } , write = plugin , refs = ParamsAllButGroupPage + ParamsCommon + ParamsDouble + ParamsNumeric + ParamsValue)]
    struct ParamDoublePropertySet {
        # [prop (@ OfxParamPropDefaultCoordinateSystem)]
        param_default_coordinate_system: ty_enum!(ParamPropDefaultCoordinateSystem),
        # [prop (@ OfxParamPropDoubleType)]
        param_double_type: ty_enum!(ParamPropDoubleType),
        # [prop (@ OfxParamPropShowTimeMarker)]
        param_show_time_marker: Bool,
    }
    # [propset (monomorphization = { type = [Double ; 2] , allow_list = [param_default , param_display_max , param_display_min , param_max , param_min] } , write = plugin , refs = ParamsAllButGroupPage + ParamsCommon + ParamsDouble + ParamsNumeric + ParamsValue)]
    struct ParamDouble2DPropertySet {
        # [prop (@ OfxParamPropDefaultCoordinateSystem)]
        param_default_coordinate_system: ty_enum!(ParamPropDefaultCoordinateSystem),
        # [prop (@ OfxParamPropDimensionLabel)]
        param_dimension_label: [String],
        # [prop (@ OfxParamPropDoubleType)]
        param_double_type: ty_enum!(ParamPropDoubleType),
    }
    # [propset (monomorphization = { type = [Double ; 3] , allow_list = [param_default , param_display_max , param_display_min , param_max , param_min] } , write = plugin , refs = ParamsAllButGroupPage + ParamsCommon + ParamsDouble + ParamsNumeric + ParamsValue)]
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
    # [propset (monomorphization = { type = [Int ; 1] , allow_list = [param_default , param_display_max , param_display_min , param_max , param_min] } , write = plugin , refs = ParamsAllButGroupPage + ParamsCommon + ParamsNumeric + ParamsValue)]
    struct ParamIntegerPropertySet {}
    # [propset (monomorphization = { type = [Int ; 2] , allow_list = [param_default , param_display_max , param_display_min , param_max , param_min] } , write = plugin , refs = ParamsAllButGroupPage + ParamsCommon + ParamsNumeric + ParamsValue)]
    struct ParamInteger2DPropertySet {
        # [prop (@ OfxParamPropDimensionLabel)]
        param_dimension_label: [String],
    }
    # [propset (monomorphization = { type = [Int ; 3] , allow_list = [param_default , param_display_max , param_display_min , param_max , param_min] } , write = plugin , refs = ParamsAllButGroupPage + ParamsCommon + ParamsNumeric + ParamsValue)]
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
    # [propset (monomorphization = { type = [Double ; 3] , allow_list = [param_default , param_display_max , param_display_min , param_max , param_min] } , write = plugin , refs = ParamsAllButGroupPage + ParamsCommon + ParamsNumeric + ParamsValue)]
    struct ParamRGBPropertySet {
        # [prop (@ OfxParamPropColourManagement)]
        param_colour_management: ty_enum!(ParamPropColourManagement),
    }
    # [propset (monomorphization = { type = [Double ; 4] , allow_list = [param_default , param_display_max , param_display_min , param_max , param_min] } , write = plugin , refs = ParamsAllButGroupPage + ParamsCommon + ParamsNumeric + ParamsValue)]
    struct ParamRGBAPropertySet {
        # [prop (@ OfxParamPropColourManagement)]
        param_colour_management: ty_enum!(ParamPropColourManagement),
    }
    # [propset (monomorphization = { type = [String ; 1] , allow_list = [param_default] } , write = plugin , refs = ParamsAllButGroupPage + ParamsCommon + ParamsValue)]
    struct ParamStrChoicePropertySet {
        # [prop (@ OfxParamPropChoiceEnum)]
        param_choice_enum: [String],
        # [prop (@ OfxParamPropChoiceOption)]
        param_choice_option: [String],
    }
    # [propset (monomorphization = { type = [String ; 1] , allow_list = [param_default] } , write = plugin , refs = ParamsAllButGroupPage + ParamsCommon + ParamsValue)]
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
#[openfx_internal_macros::low_impl_param_set_descriptor]
impl ParamSetDescriptor {
    #[param_define(kOfxParamTypeBoolean)]
    fn param_define_boolean() -> ParamBooleanPropertySet;
    #[param_define(kOfxParamTypeBytes)]
    fn param_define_bytes() -> ParamBytesPropertySet;
    #[param_define(kOfxParamTypeChoice)]
    fn param_define_choice() -> ParamChoicePropertySet;
    #[param_define(kOfxParamTypeCustom)]
    fn param_define_custom() -> ParamCustomPropertySet;
    #[param_define(kOfxParamTypeDouble)]
    fn param_define_double() -> ParamDoublePropertySet;
    #[param_define(kOfxParamTypeDouble2D)]
    fn param_define_double_2d() -> ParamDouble2DPropertySet;
    #[param_define(kOfxParamTypeDouble3D)]
    fn param_define_double_3d() -> ParamDouble3DPropertySet;
    #[param_define(kOfxParamTypeGroup)]
    fn param_define_group() -> ParamGroupPropertySet;
    #[param_define(kOfxParamTypeInteger)]
    fn param_define_integer() -> ParamIntegerPropertySet;
    #[param_define(kOfxParamTypeInteger2D)]
    fn param_define_integer_2d() -> ParamInteger2DPropertySet;
    #[param_define(kOfxParamTypeInteger3D)]
    fn param_define_integer_3d() -> ParamInteger3DPropertySet;
    #[param_define(kOfxParamTypePage)]
    fn param_define_page() -> ParamPagePropertySet;
    #[param_define(kOfxParamTypePushButton)]
    fn param_define_push_button() -> ParamPushButtonPropertySet;
    #[param_define(kOfxParamTypeRGB)]
    fn param_define_rgb() -> ParamRGBPropertySet;
    #[param_define(kOfxParamTypeRGBA)]
    fn param_define_rgba() -> ParamRGBAPropertySet;
    #[param_define(kOfxParamTypeStrChoice)]
    fn param_define_str_choice() -> ParamStrChoicePropertySet;
    #[param_define(kOfxParamTypeString)]
    fn param_define_string() -> ParamStringPropertySet;
}
#[openfx_internal_macros::low_impl_param_set_instance]
impl ParamSetInstance {
    #[param_get_handle(kOfxParamTypeBoolean)]
    fn param_get_handle_boolean() -> (ParamBoolean, ParamBooleanPropertySet);
    #[param_get_handle(kOfxParamTypeBytes)]
    fn param_get_handle_bytes() -> (ParamBytes, ParamBytesPropertySet);
    #[param_get_handle(kOfxParamTypeChoice)]
    fn param_get_handle_choice() -> (ParamChoice, ParamChoicePropertySet);
    #[param_get_handle(kOfxParamTypeCustom)]
    fn param_get_handle_custom() -> (ParamCustom, ParamCustomPropertySet);
    #[param_get_handle(kOfxParamTypeDouble)]
    fn param_get_handle_double() -> (ParamDouble, ParamDoublePropertySet);
    #[param_get_handle(kOfxParamTypeDouble2D)]
    fn param_get_handle_double_2d() -> (ParamDouble2D, ParamDouble2DPropertySet);
    #[param_get_handle(kOfxParamTypeDouble3D)]
    fn param_get_handle_double_3d() -> (ParamDouble3D, ParamDouble3DPropertySet);
    #[param_get_handle(kOfxParamTypeGroup)]
    fn param_get_handle_group() -> (ParamGroup, ParamGroupPropertySet);
    #[param_get_handle(kOfxParamTypeInteger)]
    fn param_get_handle_integer() -> (ParamInteger, ParamIntegerPropertySet);
    #[param_get_handle(kOfxParamTypeInteger2D)]
    fn param_get_handle_integer_2d() -> (ParamInteger2D, ParamInteger2DPropertySet);
    #[param_get_handle(kOfxParamTypeInteger3D)]
    fn param_get_handle_integer_3d() -> (ParamInteger3D, ParamInteger3DPropertySet);
    #[param_get_handle(kOfxParamTypePage)]
    fn param_get_handle_page() -> (ParamPage, ParamPagePropertySet);
    #[param_get_handle(kOfxParamTypePushButton)]
    fn param_get_handle_push_button() -> (ParamPushButton, ParamPushButtonPropertySet);
    #[param_get_handle(kOfxParamTypeRGB)]
    fn param_get_handle_rgb() -> (ParamRGB, ParamRGBPropertySet);
    #[param_get_handle(kOfxParamTypeRGBA)]
    fn param_get_handle_rgba() -> (ParamRGBA, ParamRGBAPropertySet);
    #[param_get_handle(kOfxParamTypeStrChoice)]
    fn param_get_handle_str_choice() -> (ParamStrChoice, ParamStrChoicePropertySet);
    #[param_get_handle(kOfxParamTypeString)]
    fn param_get_handle_string() -> (ParamString, ParamStringPropertySet);
}

use crate::{low::InboundResult, low_plugin::suites::ParameterSuiteV1, sys_umbrella as su};
use std::ffi::CStr;
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
    #[doc = "## Safety\n\n`self` and `parameter_suite` must be valid."]
    pub unsafe fn param_define_boolean(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<ParamBooleanPropertySet> {
        unsafe {
            self.param_define(parameter_suite, su::kOfxParamTypeBoolean, name)
                .map_ok(ParamBooleanPropertySet::from)
        }
    }
    #[doc = "## Safety\n\n`self` and `parameter_suite` must be valid."]
    pub unsafe fn param_define_boolean_no_output(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<()> {
        unsafe { self.param_define_no_output(parameter_suite, su::kOfxParamTypeBoolean, name) }
    }
    #[doc = "## Safety\n\n`self` and `parameter_suite` must be valid."]
    pub unsafe fn param_define_bytes(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<ParamBytesPropertySet> {
        unsafe {
            self.param_define(parameter_suite, su::kOfxParamTypeBytes, name)
                .map_ok(ParamBytesPropertySet::from)
        }
    }
    #[doc = "## Safety\n\n`self` and `parameter_suite` must be valid."]
    pub unsafe fn param_define_bytes_no_output(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<()> {
        unsafe { self.param_define_no_output(parameter_suite, su::kOfxParamTypeBytes, name) }
    }
    #[doc = "## Safety\n\n`self` and `parameter_suite` must be valid."]
    pub unsafe fn param_define_choice(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<ParamChoicePropertySet> {
        unsafe {
            self.param_define(parameter_suite, su::kOfxParamTypeChoice, name)
                .map_ok(ParamChoicePropertySet::from)
        }
    }
    #[doc = "## Safety\n\n`self` and `parameter_suite` must be valid."]
    pub unsafe fn param_define_choice_no_output(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<()> {
        unsafe { self.param_define_no_output(parameter_suite, su::kOfxParamTypeChoice, name) }
    }
    #[doc = "## Safety\n\n`self` and `parameter_suite` must be valid."]
    pub unsafe fn param_define_custom(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<ParamCustomPropertySet> {
        unsafe {
            self.param_define(parameter_suite, su::kOfxParamTypeCustom, name)
                .map_ok(ParamCustomPropertySet::from)
        }
    }
    #[doc = "## Safety\n\n`self` and `parameter_suite` must be valid."]
    pub unsafe fn param_define_custom_no_output(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<()> {
        unsafe { self.param_define_no_output(parameter_suite, su::kOfxParamTypeCustom, name) }
    }
    #[doc = "## Safety\n\n`self` and `parameter_suite` must be valid."]
    pub unsafe fn param_define_double(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<ParamDoublePropertySet> {
        unsafe {
            self.param_define(parameter_suite, su::kOfxParamTypeDouble, name)
                .map_ok(ParamDoublePropertySet::from)
        }
    }
    #[doc = "## Safety\n\n`self` and `parameter_suite` must be valid."]
    pub unsafe fn param_define_double_no_output(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<()> {
        unsafe { self.param_define_no_output(parameter_suite, su::kOfxParamTypeDouble, name) }
    }
    #[doc = "## Safety\n\n`self` and `parameter_suite` must be valid."]
    pub unsafe fn param_define_double_2d(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<ParamDouble2DPropertySet> {
        unsafe {
            self.param_define(parameter_suite, su::kOfxParamTypeDouble2D, name)
                .map_ok(ParamDouble2DPropertySet::from)
        }
    }
    #[doc = "## Safety\n\n`self` and `parameter_suite` must be valid."]
    pub unsafe fn param_define_double_2d_no_output(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<()> {
        unsafe { self.param_define_no_output(parameter_suite, su::kOfxParamTypeDouble2D, name) }
    }
    #[doc = "## Safety\n\n`self` and `parameter_suite` must be valid."]
    pub unsafe fn param_define_double_3d(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<ParamDouble3DPropertySet> {
        unsafe {
            self.param_define(parameter_suite, su::kOfxParamTypeDouble3D, name)
                .map_ok(ParamDouble3DPropertySet::from)
        }
    }
    #[doc = "## Safety\n\n`self` and `parameter_suite` must be valid."]
    pub unsafe fn param_define_double_3d_no_output(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<()> {
        unsafe { self.param_define_no_output(parameter_suite, su::kOfxParamTypeDouble3D, name) }
    }
    #[doc = "## Safety\n\n`self` and `parameter_suite` must be valid."]
    pub unsafe fn param_define_group(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<ParamGroupPropertySet> {
        unsafe {
            self.param_define(parameter_suite, su::kOfxParamTypeGroup, name)
                .map_ok(ParamGroupPropertySet::from)
        }
    }
    #[doc = "## Safety\n\n`self` and `parameter_suite` must be valid."]
    pub unsafe fn param_define_group_no_output(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<()> {
        unsafe { self.param_define_no_output(parameter_suite, su::kOfxParamTypeGroup, name) }
    }
    #[doc = "## Safety\n\n`self` and `parameter_suite` must be valid."]
    pub unsafe fn param_define_integer(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<ParamIntegerPropertySet> {
        unsafe {
            self.param_define(parameter_suite, su::kOfxParamTypeInteger, name)
                .map_ok(ParamIntegerPropertySet::from)
        }
    }
    #[doc = "## Safety\n\n`self` and `parameter_suite` must be valid."]
    pub unsafe fn param_define_integer_no_output(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<()> {
        unsafe { self.param_define_no_output(parameter_suite, su::kOfxParamTypeInteger, name) }
    }
    #[doc = "## Safety\n\n`self` and `parameter_suite` must be valid."]
    pub unsafe fn param_define_integer_2d(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<ParamInteger2DPropertySet> {
        unsafe {
            self.param_define(parameter_suite, su::kOfxParamTypeInteger2D, name)
                .map_ok(ParamInteger2DPropertySet::from)
        }
    }
    #[doc = "## Safety\n\n`self` and `parameter_suite` must be valid."]
    pub unsafe fn param_define_integer_2d_no_output(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<()> {
        unsafe { self.param_define_no_output(parameter_suite, su::kOfxParamTypeInteger2D, name) }
    }
    #[doc = "## Safety\n\n`self` and `parameter_suite` must be valid."]
    pub unsafe fn param_define_integer_3d(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<ParamInteger3DPropertySet> {
        unsafe {
            self.param_define(parameter_suite, su::kOfxParamTypeInteger3D, name)
                .map_ok(ParamInteger3DPropertySet::from)
        }
    }
    #[doc = "## Safety\n\n`self` and `parameter_suite` must be valid."]
    pub unsafe fn param_define_integer_3d_no_output(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<()> {
        unsafe { self.param_define_no_output(parameter_suite, su::kOfxParamTypeInteger3D, name) }
    }
    #[doc = "## Safety\n\n`self` and `parameter_suite` must be valid."]
    pub unsafe fn param_define_page(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<ParamPagePropertySet> {
        unsafe {
            self.param_define(parameter_suite, su::kOfxParamTypePage, name)
                .map_ok(ParamPagePropertySet::from)
        }
    }
    #[doc = "## Safety\n\n`self` and `parameter_suite` must be valid."]
    pub unsafe fn param_define_page_no_output(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<()> {
        unsafe { self.param_define_no_output(parameter_suite, su::kOfxParamTypePage, name) }
    }
    #[doc = "## Safety\n\n`self` and `parameter_suite` must be valid."]
    pub unsafe fn param_define_push_button(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<ParamPushButtonPropertySet> {
        unsafe {
            self.param_define(parameter_suite, su::kOfxParamTypePushButton, name)
                .map_ok(ParamPushButtonPropertySet::from)
        }
    }
    #[doc = "## Safety\n\n`self` and `parameter_suite` must be valid."]
    pub unsafe fn param_define_push_button_no_output(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<()> {
        unsafe { self.param_define_no_output(parameter_suite, su::kOfxParamTypePushButton, name) }
    }
    #[doc = "## Safety\n\n`self` and `parameter_suite` must be valid."]
    pub unsafe fn param_define_rgb(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<ParamRGBPropertySet> {
        unsafe {
            self.param_define(parameter_suite, su::kOfxParamTypeRGB, name)
                .map_ok(ParamRGBPropertySet::from)
        }
    }
    #[doc = "## Safety\n\n`self` and `parameter_suite` must be valid."]
    pub unsafe fn param_define_rgb_no_output(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<()> {
        unsafe { self.param_define_no_output(parameter_suite, su::kOfxParamTypeRGB, name) }
    }
    #[doc = "## Safety\n\n`self` and `parameter_suite` must be valid."]
    pub unsafe fn param_define_rgba(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<ParamRGBAPropertySet> {
        unsafe {
            self.param_define(parameter_suite, su::kOfxParamTypeRGBA, name)
                .map_ok(ParamRGBAPropertySet::from)
        }
    }
    #[doc = "## Safety\n\n`self` and `parameter_suite` must be valid."]
    pub unsafe fn param_define_rgba_no_output(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<()> {
        unsafe { self.param_define_no_output(parameter_suite, su::kOfxParamTypeRGBA, name) }
    }
    #[doc = "## Safety\n\n`self` and `parameter_suite` must be valid."]
    pub unsafe fn param_define_str_choice(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<ParamStrChoicePropertySet> {
        unsafe {
            self.param_define(parameter_suite, su::kOfxParamTypeStrChoice, name)
                .map_ok(ParamStrChoicePropertySet::from)
        }
    }
    #[doc = "## Safety\n\n`self` and `parameter_suite` must be valid."]
    pub unsafe fn param_define_str_choice_no_output(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<()> {
        unsafe { self.param_define_no_output(parameter_suite, su::kOfxParamTypeStrChoice, name) }
    }
    #[doc = "## Safety\n\n`self` and `parameter_suite` must be valid."]
    pub unsafe fn param_define_string(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<ParamStringPropertySet> {
        unsafe {
            self.param_define(parameter_suite, su::kOfxParamTypeString, name)
                .map_ok(ParamStringPropertySet::from)
        }
    }
    #[doc = "## Safety\n\n`self` and `parameter_suite` must be valid."]
    pub unsafe fn param_define_string_no_output(
        &self,
        parameter_suite: &ParameterSuiteV1,
        name: &CStr,
    ) -> InboundResult<()> {
        unsafe { self.param_define_no_output(parameter_suite, su::kOfxParamTypeString, name) }
    }
}

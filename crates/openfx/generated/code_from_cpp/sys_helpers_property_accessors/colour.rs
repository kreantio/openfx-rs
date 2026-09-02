openfx_internal_macros::sys_helpers_make_property_accessors! {
    OfxImageClipPropColourspace: String { set get reset };
    OfxImageClipPropPreferredColourspaces: [String] { set get reset get_dimensions };
    OfxImageEffectPropColourManagementAvailableConfigs: [String] { set get reset get_dimensions };
    OfxImageEffectPropColourManagementConfig: String { set get reset };
    OfxImageEffectPropColourManagementStyle: String { set get reset };
    OfxImageEffectPropDisplayColourspace: String { set get reset };
    OfxImageEffectPropOCIOConfig: String { set get reset };
    OfxImageEffectPropOCIODisplay: String { set get reset };
    OfxImageEffectPropOCIOView: String { set get reset };
}

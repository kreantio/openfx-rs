openfx_internal_macros::sys_helpers_make_property_accessors! {
    OfxPluginPropFilePath: String { set get reset };
    OfxPropAPIVersion: [Int] { set get reset get_dimensions };
    OfxPropChangeReason: String { set get reset };
    OfxPropEffectInstance: Pointer { set get reset };
    OfxPropHostOSHandle: Pointer { set get reset };
    OfxPropIcon: [String; 2] { set get reset };
    OfxPropInstanceData: Pointer { set get reset };
    OfxPropIsInteractive: Int { set get reset };
    OfxPropLabel: String { set get reset };
    OfxPropLongLabel: String { set get reset };
    OfxPropName: String { set get reset };
    OfxPropPluginDescription: String { set get reset };
    OfxPropShortLabel: String { set get reset };
    OfxPropTime: Double { set get reset };
    OfxPropType: String { set get reset };
    OfxPropVersion: [Int] { set get reset get_dimensions };
    OfxPropVersionLabel: String { set get reset };
}

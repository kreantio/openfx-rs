openfx_internal_macros::sys_helpers_make_property_accessors! {
    OfxInteractPropBackgroundColour: [Double; 3] { set get reset };
    OfxInteractPropBitDepth: Int { set get reset };
    OfxInteractPropHasAlpha: Int { set get reset };
    OfxInteractPropPenPosition: [Double; 2] { set get reset };
    OfxInteractPropPenPressure: Double { set get reset };
    OfxInteractPropPenViewportPosition: [Int; 2] { set get reset };
    OfxInteractPropPixelScale: [Double; 2] { set get reset };
    OfxInteractPropSlaveToParam: [String] { set get reset get_dimensions };
    OfxInteractPropSuggestedColour: [Double; 3] { set get reset };
}

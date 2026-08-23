openfx_internal_macros::sys_helpers_make_property_accessors! {
    OfxParamHostPropSupportsParametricAnimation: Int { set get reset };
    OfxParamPropParametricDimension: Int { set get reset };
    OfxParamPropParametricInteractBackground: Pointer { set get reset };
    OfxParamPropParametricRange: [Double; 2] { set get reset };
    OfxParamPropParametricUIColour: [Double] { set get reset get_dimensions };
}

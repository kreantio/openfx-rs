openfx_internal_macros::sys_helpers_make_property_accessors! {
    OfxImageEffectPropCPURenderSupported: String { set get reset };
    OfxImageEffectPropCudaEnabled: Int { set get reset };
    OfxImageEffectPropCudaRenderSupported: String { set get reset };
    OfxImageEffectPropCudaStream: Pointer { set get reset };
    OfxImageEffectPropCudaStreamSupported: String { set get reset };
    OfxImageEffectPropMetalCommandQueue: Pointer { set get reset };
    OfxImageEffectPropMetalEnabled: Int { set get reset };
    OfxImageEffectPropMetalRenderSupported: String { set get reset };
    OfxImageEffectPropOpenCLCommandQueue: Pointer { set get reset };
    OfxImageEffectPropOpenCLEnabled: Int { set get reset };
    OfxImageEffectPropOpenCLImage: Pointer { set get reset };
    OfxImageEffectPropOpenCLRenderSupported: String { set get reset };
    OfxImageEffectPropOpenCLSupported: String { set get reset };
    OfxImageEffectPropOpenGLEnabled: Int { set get reset };
    OfxImageEffectPropOpenGLRenderSupported: String { set get reset };
    OfxImageEffectPropOpenGLTextureIndex: Int { set get reset };
    OfxImageEffectPropOpenGLTextureTarget: Int { set get reset };
    OfxOpenGLPropPixelDepth: [String] { set get reset get_dimensions };
}

openfx_internal_macros::low_make_property_enums! {
    enum ImageClipPropFieldExtraction {
        #[sys(kOfxImageFieldBoth)]
        Both,
        #[sys(kOfxImageFieldDoubled)]
        Doubled,
        #[sys(kOfxImageFieldLower)]
        Lower,
        #[sys(kOfxImageFieldNone)]
        None,
        #[sys(kOfxImageFieldSingle)]
        Single,
        #[sys(kOfxImageFieldUpper)]
        Upper,
    }
    enum ImageClipPropFieldOrder {
        #[sys(kOfxImageFieldLower)]
        Lower,
        #[sys(kOfxImageFieldNone)]
        None,
        #[sys(kOfxImageFieldUpper)]
        Upper,
    }
    enum ImageClipPropUnmappedComponents {
        #[sys(kOfxImageComponentAlpha)]
        Alpha,
        #[sys(kOfxImageComponentNone)]
        None,
        #[sys(kOfxImageComponentRGB)]
        RGB,
        #[sys(kOfxImageComponentRGBA)]
        RGBA,
    }
    enum ImageClipPropUnmappedPixelDepth {
        #[sys(kOfxBitDepthByte)]
        Byte,
        #[sys(kOfxBitDepthFloat)]
        Float,
        #[sys(kOfxBitDepthHalf)]
        Half,
        #[sys(kOfxBitDepthNone)]
        None,
        #[sys(kOfxBitDepthShort)]
        Short,
    }
    enum ImageEffectHostPropNativeOrigin {
        #[sys(kOfxHostNativeOriginBottomLeft)]
        BottomLeft,
        #[sys(kOfxHostNativeOriginCenter)]
        Center,
        #[sys(kOfxHostNativeOriginTopLeft)]
        TopLeft,
    }
    enum ImageEffectPluginRenderThreadSafety {
        #[sys(kOfxImageEffectRenderFullySafe)]
        FullySafe,
        #[sys(kOfxImageEffectRenderInstanceSafe)]
        InstanceSafe,
        #[sys(kOfxImageEffectRenderUnsafe)]
        Unsafe,
    }
    enum ImageEffectPropCPURenderSupported {
        #[sys_literal(c"false")]
        False,
        #[sys_literal(c"true")]
        True,
    }
    enum ImageEffectPropColourManagementStyle {
        #[sys(kOfxImageEffectColourManagementBasic)]
        Basic,
        #[sys(kOfxImageEffectColourManagementCore)]
        Core,
        #[sys(kOfxImageEffectColourManagementFull)]
        Full,
        #[sys(kOfxImageEffectColourManagementNone)]
        None,
        #[sys(kOfxImageEffectColourManagementOCIO)]
        OCIO,
    }
    enum ImageEffectPropComponents {
        #[sys(kOfxImageComponentAlpha)]
        Alpha,
        #[sys(kOfxImageComponentNone)]
        None,
        #[sys(kOfxImageComponentRGB)]
        RGB,
        #[sys(kOfxImageComponentRGBA)]
        RGBA,
    }
    enum ImageEffectPropContext {
        #[sys(kOfxImageEffectContextFilter)]
        Filter,
        #[sys(kOfxImageEffectContextGeneral)]
        General,
        #[sys(kOfxImageEffectContextGenerator)]
        Generator,
        #[sys(kOfxImageEffectContextPaint)]
        Paint,
        #[sys(kOfxImageEffectContextRetimer)]
        Retimer,
        #[sys(kOfxImageEffectContextTransition)]
        Transition,
    }
    enum ImageEffectPropCudaRenderSupported {
        #[sys_literal(c"false")]
        False,
        #[sys_literal(c"needed")]
        Needed,
        #[sys_literal(c"true")]
        True,
    }
    enum ImageEffectPropCudaStreamSupported {
        #[sys_literal(c"false")]
        False,
        #[sys_literal(c"needed")]
        Needed,
        #[sys_literal(c"true")]
        True,
    }
    enum ImageEffectPropFieldToRender {
        #[sys(kOfxImageFieldBoth)]
        Both,
        #[sys(kOfxImageFieldLower)]
        Lower,
        #[sys(kOfxImageFieldNone)]
        None,
        #[sys(kOfxImageFieldUpper)]
        Upper,
    }
    enum ImageEffectPropMetalRenderSupported {
        #[sys_literal(c"false")]
        False,
        #[sys_literal(c"needed")]
        Needed,
        #[sys_literal(c"true")]
        True,
    }
    enum ImageEffectPropNoSpatialAwareness {
        #[sys_literal(c"false")]
        False,
        #[sys_literal(c"true")]
        True,
    }
    enum ImageEffectPropOpenCLRenderSupported {
        #[sys_literal(c"false")]
        False,
        #[sys_literal(c"needed")]
        Needed,
        #[sys_literal(c"true")]
        True,
    }
    enum ImageEffectPropOpenCLSupported {
        #[sys_literal(c"false")]
        False,
        #[sys_literal(c"true")]
        True,
    }
    enum ImageEffectPropOpenGLRenderSupported {
        #[sys_literal(c"false")]
        False,
        #[sys_literal(c"needed")]
        Needed,
        #[sys_literal(c"true")]
        True,
    }
    enum ImageEffectPropPixelDepth {
        #[sys(kOfxBitDepthByte)]
        Byte,
        #[sys(kOfxBitDepthFloat)]
        Float,
        #[sys(kOfxBitDepthHalf)]
        Half,
        #[sys(kOfxBitDepthNone)]
        None,
        #[sys(kOfxBitDepthShort)]
        Short,
    }
    enum ImageEffectPropPreMultiplication {
        #[sys(kOfxImagePreMultiplied)]
        PreMultiplied,
        #[sys(kOfxImageUnPreMultiplied)]
        UnPreMultiplied,
        #[sys(kOfxImageOpaque)]
        Opaque,
    }
    enum ImageEffectPropSupportedComponents {
        #[sys(kOfxImageComponentAlpha)]
        Alpha,
        #[sys(kOfxImageComponentNone)]
        None,
        #[sys(kOfxImageComponentRGB)]
        RGB,
        #[sys(kOfxImageComponentRGBA)]
        RGBA,
    }
    enum ImageEffectPropSupportedContexts {
        #[sys(kOfxImageEffectContextFilter)]
        Filter,
        #[sys(kOfxImageEffectContextGeneral)]
        General,
        #[sys(kOfxImageEffectContextGenerator)]
        Generator,
        #[sys(kOfxImageEffectContextPaint)]
        Paint,
        #[sys(kOfxImageEffectContextRetimer)]
        Retimer,
        #[sys(kOfxImageEffectContextTransition)]
        Transition,
    }
    enum ImageEffectPropSupportedPixelDepths {
        #[sys(kOfxBitDepthByte)]
        Byte,
        #[sys(kOfxBitDepthFloat)]
        Float,
        #[sys(kOfxBitDepthHalf)]
        Half,
        #[sys(kOfxBitDepthNone)]
        None,
        #[sys(kOfxBitDepthShort)]
        Short,
    }
    enum ImageEffectPropThumbnailRender {
        #[sys_literal(c"false")]
        False,
        #[sys_literal(c"true")]
        True,
    }
    enum ImagePropField {
        #[sys(kOfxImageFieldBoth)]
        Both,
        #[sys(kOfxImageFieldLower)]
        Lower,
        #[sys(kOfxImageFieldNone)]
        None,
        #[sys(kOfxImageFieldUpper)]
        Upper,
    }
    enum OpenGLPropPixelDepth {
        #[sys(kOfxBitDepthByte)]
        Byte,
        #[sys(kOfxBitDepthFloat)]
        Float,
        #[sys(kOfxBitDepthHalf)]
        Half,
        #[sys(kOfxBitDepthNone)]
        None,
        #[sys(kOfxBitDepthShort)]
        Short,
    }
    enum ParamPropCacheInvalidation {
        #[sys(kOfxParamInvalidateAll)]
        All,
        #[sys(kOfxParamInvalidateValueChange)]
        ValueChange,
        #[sys(kOfxParamInvalidateValueChangeToEnd)]
        ValueChangeToEnd,
    }
    enum ParamPropDefaultCoordinateSystem {
        #[sys(kOfxParamCoordinatesCanonical)]
        Canonical,
        #[sys(kOfxParamCoordinatesNormalised)]
        Normalised,
    }
    enum ParamPropDoubleType {
        #[sys(kOfxParamDoubleTypeAbsoluteTime)]
        AbsoluteTime,
        #[sys(kOfxParamDoubleTypeAngle)]
        Angle,
        #[sys(kOfxParamDoubleTypePlain)]
        Plain,
        #[sys(kOfxParamDoubleTypeScale)]
        Scale,
        #[sys(kOfxParamDoubleTypeTime)]
        Time,
        #[sys(kOfxParamDoubleTypeX)]
        X,
        #[sys(kOfxParamDoubleTypeXAbsolute)]
        XAbsolute,
        #[sys(kOfxParamDoubleTypeXY)]
        XY,
        #[sys(kOfxParamDoubleTypeXYAbsolute)]
        XYAbsolute,
        #[sys(kOfxParamDoubleTypeY)]
        Y,
        #[sys(kOfxParamDoubleTypeYAbsolute)]
        YAbsolute,
    }
    enum ParamPropStringMode {
        #[sys(kOfxParamStringIsDirectoryPath)]
        DirectoryPath,
        #[sys(kOfxParamStringIsFilePath)]
        FilePath,
        #[sys(kOfxParamStringIsLabel)]
        Label,
        #[sys(kOfxParamStringIsMultiLine)]
        MultiLine,
        #[sys(kOfxParamStringIsRichTextFormat)]
        RichTextFormat,
        #[sys(kOfxParamStringIsSingleLine)]
        SingleLine,
    }
    enum PropChangeReason {
        #[sys(kOfxChangePluginEdited)]
        PluginEdited,
        #[sys(kOfxChangeTime)]
        Time,
        #[sys(kOfxChangeUserEdited)]
        UserEdited,
    }
}

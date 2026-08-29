openfx_internal_macros::sys_helpers_make_property_enums! {
    ImageClipPropFieldExtraction {
        Both => crate::sys_umbrella::kOfxImageFieldBoth,
        Doubled => crate::sys_umbrella::kOfxImageFieldDoubled,
        Lower => crate::sys_umbrella::kOfxImageFieldLower,
        None => crate::sys_umbrella::kOfxImageFieldNone,
        Single => crate::sys_umbrella::kOfxImageFieldSingle,
        Upper => crate::sys_umbrella::kOfxImageFieldUpper,
    }
    ImageClipPropFieldOrder {
        Lower => crate::sys_umbrella::kOfxImageFieldLower,
        None => crate::sys_umbrella::kOfxImageFieldNone,
        Upper => crate::sys_umbrella::kOfxImageFieldUpper,
    }
    ImageClipPropUnmappedComponents {
        Alpha => crate::sys_umbrella::kOfxImageComponentAlpha,
        None => crate::sys_umbrella::kOfxImageComponentNone,
        RGB => crate::sys_umbrella::kOfxImageComponentRGB,
        RGBA => crate::sys_umbrella::kOfxImageComponentRGBA,
    }
    ImageClipPropUnmappedPixelDepth {
        Byte => crate::sys_umbrella::kOfxBitDepthByte,
        Float => crate::sys_umbrella::kOfxBitDepthFloat,
        Half => crate::sys_umbrella::kOfxBitDepthHalf,
        None => crate::sys_umbrella::kOfxBitDepthNone,
        Short => crate::sys_umbrella::kOfxBitDepthShort,
    }
    ImageEffectHostPropNativeOrigin {
        BottomLeft => crate::sys_umbrella::kOfxHostNativeOriginBottomLeft,
        Center => crate::sys_umbrella::kOfxHostNativeOriginCenter,
        TopLeft => crate::sys_umbrella::kOfxHostNativeOriginTopLeft,
    }
    ImageEffectPluginRenderThreadSafety {
        FullySafe => crate::sys_umbrella::kOfxImageEffectRenderFullySafe,
        InstanceSafe => crate::sys_umbrella::kOfxImageEffectRenderInstanceSafe,
        Unsafe => crate::sys_umbrella::kOfxImageEffectRenderUnsafe,
    }
    ImageEffectPropColourManagementStyle {
        Basic => crate::sys_umbrella::kOfxImageEffectColourManagementBasic,
        Core => crate::sys_umbrella::kOfxImageEffectColourManagementCore,
        Full => crate::sys_umbrella::kOfxImageEffectColourManagementFull,
        None => crate::sys_umbrella::kOfxImageEffectColourManagementNone,
        OCIO => crate::sys_umbrella::kOfxImageEffectColourManagementOCIO,
    }
    ImageEffectPropComponents {
        Alpha => crate::sys_umbrella::kOfxImageComponentAlpha,
        None => crate::sys_umbrella::kOfxImageComponentNone,
        RGB => crate::sys_umbrella::kOfxImageComponentRGB,
        RGBA => crate::sys_umbrella::kOfxImageComponentRGBA,
    }
    ImageEffectPropContext {
        Filter => crate::sys_umbrella::kOfxImageEffectContextFilter,
        General => crate::sys_umbrella::kOfxImageEffectContextGeneral,
        Generator => crate::sys_umbrella::kOfxImageEffectContextGenerator,
        Paint => crate::sys_umbrella::kOfxImageEffectContextPaint,
        Retimer => crate::sys_umbrella::kOfxImageEffectContextRetimer,
        Transition => crate::sys_umbrella::kOfxImageEffectContextTransition,
    }
    ImageEffectPropCPURenderSupported {
        False : c"false",
        True : c"true",
    }
    ImageEffectPropCudaRenderSupported {
        False : c"false",
        Needed : c"needed",
        True : c"true",
    }
    ImageEffectPropCudaStreamSupported {
        False : c"false",
        Needed : c"needed",
        True : c"true",
    }
    ImageEffectPropFieldToRender {
        Both => crate::sys_umbrella::kOfxImageFieldBoth,
        Lower => crate::sys_umbrella::kOfxImageFieldLower,
        None => crate::sys_umbrella::kOfxImageFieldNone,
        Upper => crate::sys_umbrella::kOfxImageFieldUpper,
    }
    ImageEffectPropMetalRenderSupported {
        False : c"false",
        Needed : c"needed",
        True : c"true",
    }
    ImageEffectPropNoSpatialAwareness {
        False : c"false",
        True : c"true",
    }
    ImageEffectPropOpenCLRenderSupported {
        False : c"false",
        Needed : c"needed",
        True : c"true",
    }
    ImageEffectPropOpenCLSupported {
        False : c"false",
        True : c"true",
    }
    ImageEffectPropOpenGLRenderSupported {
        False : c"false",
        Needed : c"needed",
        True : c"true",
    }
    ImageEffectPropPixelDepth {
        Byte => crate::sys_umbrella::kOfxBitDepthByte,
        Float => crate::sys_umbrella::kOfxBitDepthFloat,
        Half => crate::sys_umbrella::kOfxBitDepthHalf,
        None => crate::sys_umbrella::kOfxBitDepthNone,
        Short => crate::sys_umbrella::kOfxBitDepthShort,
    }
    ImageEffectPropPreMultiplication {
        PreMultiplied => crate::sys_umbrella::kOfxImagePreMultiplied,
        UnPreMultiplied => crate::sys_umbrella::kOfxImageUnPreMultiplied,
        Opaque => crate::sys_umbrella::kOfxImageOpaque,
    }
    ImageEffectPropSupportedComponents {
        Alpha => crate::sys_umbrella::kOfxImageComponentAlpha,
        None => crate::sys_umbrella::kOfxImageComponentNone,
        RGB => crate::sys_umbrella::kOfxImageComponentRGB,
        RGBA => crate::sys_umbrella::kOfxImageComponentRGBA,
    }
    ImageEffectPropSupportedContexts {
        Filter => crate::sys_umbrella::kOfxImageEffectContextFilter,
        General => crate::sys_umbrella::kOfxImageEffectContextGeneral,
        Generator => crate::sys_umbrella::kOfxImageEffectContextGenerator,
        Paint => crate::sys_umbrella::kOfxImageEffectContextPaint,
        Retimer => crate::sys_umbrella::kOfxImageEffectContextRetimer,
        Transition => crate::sys_umbrella::kOfxImageEffectContextTransition,
    }
    ImageEffectPropSupportedPixelDepths {
        Byte => crate::sys_umbrella::kOfxBitDepthByte,
        Float => crate::sys_umbrella::kOfxBitDepthFloat,
        Half => crate::sys_umbrella::kOfxBitDepthHalf,
        None => crate::sys_umbrella::kOfxBitDepthNone,
        Short => crate::sys_umbrella::kOfxBitDepthShort,
    }
    ImageEffectPropThumbnailRender {
        False : c"false",
        True : c"true",
    }
    ImagePropField {
        Both => crate::sys_umbrella::kOfxImageFieldBoth,
        Lower => crate::sys_umbrella::kOfxImageFieldLower,
        None => crate::sys_umbrella::kOfxImageFieldNone,
        Upper => crate::sys_umbrella::kOfxImageFieldUpper,
    }
    OpenGLPropPixelDepth {
        Byte => crate::sys_umbrella::kOfxBitDepthByte,
        Float => crate::sys_umbrella::kOfxBitDepthFloat,
        Half => crate::sys_umbrella::kOfxBitDepthHalf,
        None => crate::sys_umbrella::kOfxBitDepthNone,
        Short => crate::sys_umbrella::kOfxBitDepthShort,
    }
    ParamPropCacheInvalidation {
        All => crate::sys_umbrella::kOfxParamInvalidateAll,
        ValueChange => crate::sys_umbrella::kOfxParamInvalidateValueChange,
        ValueChangeToEnd => crate::sys_umbrella::kOfxParamInvalidateValueChangeToEnd,
    }
    ParamPropDefaultCoordinateSystem {
        Canonical => crate::sys_umbrella::kOfxParamCoordinatesCanonical,
        Normalised => crate::sys_umbrella::kOfxParamCoordinatesNormalised,
    }
    ParamPropDoubleType {
        AbsoluteTime => crate::sys_umbrella::kOfxParamDoubleTypeAbsoluteTime,
        Angle => crate::sys_umbrella::kOfxParamDoubleTypeAngle,
        Plain => crate::sys_umbrella::kOfxParamDoubleTypePlain,
        Scale => crate::sys_umbrella::kOfxParamDoubleTypeScale,
        Time => crate::sys_umbrella::kOfxParamDoubleTypeTime,
        X => crate::sys_umbrella::kOfxParamDoubleTypeX,
        XAbsolute => crate::sys_umbrella::kOfxParamDoubleTypeXAbsolute,
        XY => crate::sys_umbrella::kOfxParamDoubleTypeXY,
        XYAbsolute => crate::sys_umbrella::kOfxParamDoubleTypeXYAbsolute,
        Y => crate::sys_umbrella::kOfxParamDoubleTypeY,
        YAbsolute => crate::sys_umbrella::kOfxParamDoubleTypeYAbsolute,
    }
    ParamPropStringMode {
        DirectoryPath => crate::sys_umbrella::kOfxParamStringIsDirectoryPath,
        FilePath => crate::sys_umbrella::kOfxParamStringIsFilePath,
        Label => crate::sys_umbrella::kOfxParamStringIsLabel,
        MultiLine => crate::sys_umbrella::kOfxParamStringIsMultiLine,
        RichTextFormat => crate::sys_umbrella::kOfxParamStringIsRichTextFormat,
        SingleLine => crate::sys_umbrella::kOfxParamStringIsSingleLine,
    }
    PropChangeReason {
        PluginEdited => crate::sys_umbrella::kOfxChangePluginEdited,
        Time => crate::sys_umbrella::kOfxChangeTime,
        UserEdited => crate::sys_umbrella::kOfxChangeUserEdited,
    }
}

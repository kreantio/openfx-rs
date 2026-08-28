crate::internal::low_macros::make_enum_from_paths!(ImageClipPropFieldExtraction,
    /// See: [`crate::sys_umbrella::kOfxImageFieldBoth`].
    Both => crate::sys_umbrella::kOfxImageFieldBoth,
    /// See: [`crate::sys_umbrella::kOfxImageFieldDoubled`].
    Doubled => crate::sys_umbrella::kOfxImageFieldDoubled,
    /// See: [`crate::sys_umbrella::kOfxImageFieldLower`].
    Lower => crate::sys_umbrella::kOfxImageFieldLower,
    /// See: [`crate::sys_umbrella::kOfxImageFieldNone`].
    None => crate::sys_umbrella::kOfxImageFieldNone,
    /// See: [`crate::sys_umbrella::kOfxImageFieldSingle`].
    Single => crate::sys_umbrella::kOfxImageFieldSingle,
    /// See: [`crate::sys_umbrella::kOfxImageFieldUpper`].
    Upper => crate::sys_umbrella::kOfxImageFieldUpper,
);
crate::internal::low_macros::make_enum_from_paths!(ImageClipPropFieldOrder,
    /// See: [`crate::sys_umbrella::kOfxImageFieldLower`].
    Lower => crate::sys_umbrella::kOfxImageFieldLower,
    /// See: [`crate::sys_umbrella::kOfxImageFieldNone`].
    None => crate::sys_umbrella::kOfxImageFieldNone,
    /// See: [`crate::sys_umbrella::kOfxImageFieldUpper`].
    Upper => crate::sys_umbrella::kOfxImageFieldUpper,
);
crate::internal::low_macros::make_enum_from_paths!(ImageClipPropUnmappedComponents,
    /// See: [`crate::sys_umbrella::kOfxImageComponentAlpha`].
    Alpha => crate::sys_umbrella::kOfxImageComponentAlpha,
    /// See: [`crate::sys_umbrella::kOfxImageComponentNone`].
    None => crate::sys_umbrella::kOfxImageComponentNone,
    /// See: [`crate::sys_umbrella::kOfxImageComponentRGB`].
    RGB => crate::sys_umbrella::kOfxImageComponentRGB,
    /// See: [`crate::sys_umbrella::kOfxImageComponentRGBA`].
    RGBA => crate::sys_umbrella::kOfxImageComponentRGBA,
);
crate::internal::low_macros::make_enum_from_paths!(ImageClipPropUnmappedPixelDepth,
    /// See: [`crate::sys_umbrella::kOfxBitDepthByte`].
    Byte => crate::sys_umbrella::kOfxBitDepthByte,
    /// See: [`crate::sys_umbrella::kOfxBitDepthFloat`].
    Float => crate::sys_umbrella::kOfxBitDepthFloat,
    /// See: [`crate::sys_umbrella::kOfxBitDepthHalf`].
    Half => crate::sys_umbrella::kOfxBitDepthHalf,
    /// See: [`crate::sys_umbrella::kOfxBitDepthNone`].
    None => crate::sys_umbrella::kOfxBitDepthNone,
    /// See: [`crate::sys_umbrella::kOfxBitDepthShort`].
    Short => crate::sys_umbrella::kOfxBitDepthShort,
);
crate::internal::low_macros::make_enum_from_paths!(ImageEffectHostPropNativeOrigin,
    /// See: [`crate::sys_umbrella::kOfxHostNativeOriginBottomLeft`].
    BottomLeft => crate::sys_umbrella::kOfxHostNativeOriginBottomLeft,
    /// See: [`crate::sys_umbrella::kOfxHostNativeOriginCenter`].
    Center => crate::sys_umbrella::kOfxHostNativeOriginCenter,
    /// See: [`crate::sys_umbrella::kOfxHostNativeOriginTopLeft`].
    TopLeft => crate::sys_umbrella::kOfxHostNativeOriginTopLeft,
);
crate::internal::low_macros::make_enum_from_paths!(ImageEffectPluginRenderThreadSafety,
    /// See: [`crate::sys_umbrella::kOfxImageEffectRenderFullySafe`].
    FullySafe => crate::sys_umbrella::kOfxImageEffectRenderFullySafe,
    /// See: [`crate::sys_umbrella::kOfxImageEffectRenderInstanceSafe`].
    InstanceSafe => crate::sys_umbrella::kOfxImageEffectRenderInstanceSafe,
    /// See: [`crate::sys_umbrella::kOfxImageEffectRenderUnsafe`].
    Unsafe => crate::sys_umbrella::kOfxImageEffectRenderUnsafe,
);
crate::internal::low_macros::make_enum_from_paths!(ImageEffectPropColourManagementStyle,
    /// See: [`crate::sys_umbrella::kOfxImageEffectColourManagementBasic`].
    Basic => crate::sys_umbrella::kOfxImageEffectColourManagementBasic,
    /// See: [`crate::sys_umbrella::kOfxImageEffectColourManagementCore`].
    Core => crate::sys_umbrella::kOfxImageEffectColourManagementCore,
    /// See: [`crate::sys_umbrella::kOfxImageEffectColourManagementFull`].
    Full => crate::sys_umbrella::kOfxImageEffectColourManagementFull,
    /// See: [`crate::sys_umbrella::kOfxImageEffectColourManagementNone`].
    None => crate::sys_umbrella::kOfxImageEffectColourManagementNone,
    /// See: [`crate::sys_umbrella::kOfxImageEffectColourManagementOCIO`].
    OCIO => crate::sys_umbrella::kOfxImageEffectColourManagementOCIO,
);
crate::internal::low_macros::make_enum_from_paths!(ImageEffectPropComponents,
    /// See: [`crate::sys_umbrella::kOfxImageComponentAlpha`].
    Alpha => crate::sys_umbrella::kOfxImageComponentAlpha,
    /// See: [`crate::sys_umbrella::kOfxImageComponentNone`].
    None => crate::sys_umbrella::kOfxImageComponentNone,
    /// See: [`crate::sys_umbrella::kOfxImageComponentRGB`].
    RGB => crate::sys_umbrella::kOfxImageComponentRGB,
    /// See: [`crate::sys_umbrella::kOfxImageComponentRGBA`].
    RGBA => crate::sys_umbrella::kOfxImageComponentRGBA,
);
crate::internal::low_macros::make_enum_from_paths!(ImageEffectPropContext,
    /// See: [`crate::sys_umbrella::kOfxImageEffectContextFilter`].
    Filter => crate::sys_umbrella::kOfxImageEffectContextFilter,
    /// See: [`crate::sys_umbrella::kOfxImageEffectContextGeneral`].
    General => crate::sys_umbrella::kOfxImageEffectContextGeneral,
    /// See: [`crate::sys_umbrella::kOfxImageEffectContextGenerator`].
    Generator => crate::sys_umbrella::kOfxImageEffectContextGenerator,
    /// See: [`crate::sys_umbrella::kOfxImageEffectContextPaint`].
    Paint => crate::sys_umbrella::kOfxImageEffectContextPaint,
    /// See: [`crate::sys_umbrella::kOfxImageEffectContextRetimer`].
    Retimer => crate::sys_umbrella::kOfxImageEffectContextRetimer,
    /// See: [`crate::sys_umbrella::kOfxImageEffectContextTransition`].
    Transition => crate::sys_umbrella::kOfxImageEffectContextTransition,
);
crate::internal::low_macros::make_enum_from_idents!(ImageEffectPropCPURenderSupported,
    False : c"false",
    True : c"true",
);
crate::internal::low_macros::make_enum_from_idents!(ImageEffectPropCudaRenderSupported,
    False : c"false",
    Needed : c"needed",
    True : c"true",
);
crate::internal::low_macros::make_enum_from_idents!(ImageEffectPropCudaStreamSupported,
    False : c"false",
    Needed : c"needed",
    True : c"true",
);
crate::internal::low_macros::make_enum_from_paths!(ImageEffectPropFieldToRender,
    /// See: [`crate::sys_umbrella::kOfxImageFieldBoth`].
    Both => crate::sys_umbrella::kOfxImageFieldBoth,
    /// See: [`crate::sys_umbrella::kOfxImageFieldLower`].
    Lower => crate::sys_umbrella::kOfxImageFieldLower,
    /// See: [`crate::sys_umbrella::kOfxImageFieldNone`].
    None => crate::sys_umbrella::kOfxImageFieldNone,
    /// See: [`crate::sys_umbrella::kOfxImageFieldUpper`].
    Upper => crate::sys_umbrella::kOfxImageFieldUpper,
);
crate::internal::low_macros::make_enum_from_idents!(ImageEffectPropMetalRenderSupported,
    False : c"false",
    Needed : c"needed",
    True : c"true",
);
crate::internal::low_macros::make_enum_from_idents!(ImageEffectPropNoSpatialAwareness,
    False : c"false",
    True : c"true",
);
crate::internal::low_macros::make_enum_from_idents!(ImageEffectPropOpenCLRenderSupported,
    False : c"false",
    Needed : c"needed",
    True : c"true",
);
crate::internal::low_macros::make_enum_from_idents!(ImageEffectPropOpenCLSupported,
    False : c"false",
    True : c"true",
);
crate::internal::low_macros::make_enum_from_idents!(ImageEffectPropOpenGLRenderSupported,
    False : c"false",
    Needed : c"needed",
    True : c"true",
);
crate::internal::low_macros::make_enum_from_paths!(ImageEffectPropPixelDepth,
    /// See: [`crate::sys_umbrella::kOfxBitDepthByte`].
    Byte => crate::sys_umbrella::kOfxBitDepthByte,
    /// See: [`crate::sys_umbrella::kOfxBitDepthFloat`].
    Float => crate::sys_umbrella::kOfxBitDepthFloat,
    /// See: [`crate::sys_umbrella::kOfxBitDepthHalf`].
    Half => crate::sys_umbrella::kOfxBitDepthHalf,
    /// See: [`crate::sys_umbrella::kOfxBitDepthNone`].
    None => crate::sys_umbrella::kOfxBitDepthNone,
    /// See: [`crate::sys_umbrella::kOfxBitDepthShort`].
    Short => crate::sys_umbrella::kOfxBitDepthShort,
);
crate::internal::low_macros::make_enum_from_paths!(ImageEffectPropPreMultiplication,
    /// See: [`crate::sys_umbrella::kOfxImageOpaque`].
    Opaque => crate::sys_umbrella::kOfxImageOpaque,
    /// See: [`crate::sys_umbrella::kOfxImagePreMultiplied`].
    PreMultiplied => crate::sys_umbrella::kOfxImagePreMultiplied,
    /// See: [`crate::sys_umbrella::kOfxImageUnPreMultiplied`].
    UnPreMultiplied => crate::sys_umbrella::kOfxImageUnPreMultiplied,
);
crate::internal::low_macros::make_enum_from_paths!(ImageEffectPropSupportedComponents,
    /// See: [`crate::sys_umbrella::kOfxImageComponentAlpha`].
    Alpha => crate::sys_umbrella::kOfxImageComponentAlpha,
    /// See: [`crate::sys_umbrella::kOfxImageComponentNone`].
    None => crate::sys_umbrella::kOfxImageComponentNone,
    /// See: [`crate::sys_umbrella::kOfxImageComponentRGB`].
    RGB => crate::sys_umbrella::kOfxImageComponentRGB,
    /// See: [`crate::sys_umbrella::kOfxImageComponentRGBA`].
    RGBA => crate::sys_umbrella::kOfxImageComponentRGBA,
);
crate::internal::low_macros::make_enum_from_paths!(ImageEffectPropSupportedContexts,
    /// See: [`crate::sys_umbrella::kOfxImageEffectContextFilter`].
    Filter => crate::sys_umbrella::kOfxImageEffectContextFilter,
    /// See: [`crate::sys_umbrella::kOfxImageEffectContextGeneral`].
    General => crate::sys_umbrella::kOfxImageEffectContextGeneral,
    /// See: [`crate::sys_umbrella::kOfxImageEffectContextGenerator`].
    Generator => crate::sys_umbrella::kOfxImageEffectContextGenerator,
    /// See: [`crate::sys_umbrella::kOfxImageEffectContextPaint`].
    Paint => crate::sys_umbrella::kOfxImageEffectContextPaint,
    /// See: [`crate::sys_umbrella::kOfxImageEffectContextRetimer`].
    Retimer => crate::sys_umbrella::kOfxImageEffectContextRetimer,
    /// See: [`crate::sys_umbrella::kOfxImageEffectContextTransition`].
    Transition => crate::sys_umbrella::kOfxImageEffectContextTransition,
);
crate::internal::low_macros::make_enum_from_paths!(ImageEffectPropSupportedPixelDepths,
    /// See: [`crate::sys_umbrella::kOfxBitDepthByte`].
    Byte => crate::sys_umbrella::kOfxBitDepthByte,
    /// See: [`crate::sys_umbrella::kOfxBitDepthFloat`].
    Float => crate::sys_umbrella::kOfxBitDepthFloat,
    /// See: [`crate::sys_umbrella::kOfxBitDepthHalf`].
    Half => crate::sys_umbrella::kOfxBitDepthHalf,
    /// See: [`crate::sys_umbrella::kOfxBitDepthNone`].
    None => crate::sys_umbrella::kOfxBitDepthNone,
    /// See: [`crate::sys_umbrella::kOfxBitDepthShort`].
    Short => crate::sys_umbrella::kOfxBitDepthShort,
);
crate::internal::low_macros::make_enum_from_idents!(ImageEffectPropThumbnailRender,
    False : c"false",
    True : c"true",
);
crate::internal::low_macros::make_enum_from_paths!(ImagePropField,
    /// See: [`crate::sys_umbrella::kOfxImageFieldBoth`].
    Both => crate::sys_umbrella::kOfxImageFieldBoth,
    /// See: [`crate::sys_umbrella::kOfxImageFieldLower`].
    Lower => crate::sys_umbrella::kOfxImageFieldLower,
    /// See: [`crate::sys_umbrella::kOfxImageFieldNone`].
    None => crate::sys_umbrella::kOfxImageFieldNone,
    /// See: [`crate::sys_umbrella::kOfxImageFieldUpper`].
    Upper => crate::sys_umbrella::kOfxImageFieldUpper,
);
crate::internal::low_macros::make_enum_from_paths!(OpenGLPropPixelDepth,
    /// See: [`crate::sys_umbrella::kOfxBitDepthByte`].
    Byte => crate::sys_umbrella::kOfxBitDepthByte,
    /// See: [`crate::sys_umbrella::kOfxBitDepthFloat`].
    Float => crate::sys_umbrella::kOfxBitDepthFloat,
    /// See: [`crate::sys_umbrella::kOfxBitDepthHalf`].
    Half => crate::sys_umbrella::kOfxBitDepthHalf,
    /// See: [`crate::sys_umbrella::kOfxBitDepthNone`].
    None => crate::sys_umbrella::kOfxBitDepthNone,
    /// See: [`crate::sys_umbrella::kOfxBitDepthShort`].
    Short => crate::sys_umbrella::kOfxBitDepthShort,
);
crate::internal::low_macros::make_enum_from_paths!(ParamPropCacheInvalidation,
    /// See: [`crate::sys_umbrella::kOfxParamInvalidateAll`].
    All => crate::sys_umbrella::kOfxParamInvalidateAll,
    /// See: [`crate::sys_umbrella::kOfxParamInvalidateValueChange`].
    ValueChange => crate::sys_umbrella::kOfxParamInvalidateValueChange,
    /// See: [`crate::sys_umbrella::kOfxParamInvalidateValueChangeToEnd`].
    ValueChangeToEnd => crate::sys_umbrella::kOfxParamInvalidateValueChangeToEnd,
);
crate::internal::low_macros::make_enum_from_paths!(ParamPropDefaultCoordinateSystem,
    /// See: [`crate::sys_umbrella::kOfxParamCoordinatesCanonical`].
    Canonical => crate::sys_umbrella::kOfxParamCoordinatesCanonical,
    /// See: [`crate::sys_umbrella::kOfxParamCoordinatesNormalised`].
    Normalised => crate::sys_umbrella::kOfxParamCoordinatesNormalised,
);
crate::internal::low_macros::make_enum_from_paths!(ParamPropDoubleType,
    /// See: [`crate::sys_umbrella::kOfxParamDoubleTypeAbsoluteTime`].
    AbsoluteTime => crate::sys_umbrella::kOfxParamDoubleTypeAbsoluteTime,
    /// See: [`crate::sys_umbrella::kOfxParamDoubleTypeAngle`].
    Angle => crate::sys_umbrella::kOfxParamDoubleTypeAngle,
    /// See: [`crate::sys_umbrella::kOfxParamDoubleTypePlain`].
    Plain => crate::sys_umbrella::kOfxParamDoubleTypePlain,
    /// See: [`crate::sys_umbrella::kOfxParamDoubleTypeScale`].
    Scale => crate::sys_umbrella::kOfxParamDoubleTypeScale,
    /// See: [`crate::sys_umbrella::kOfxParamDoubleTypeTime`].
    Time => crate::sys_umbrella::kOfxParamDoubleTypeTime,
    /// See: [`crate::sys_umbrella::kOfxParamDoubleTypeX`].
    X => crate::sys_umbrella::kOfxParamDoubleTypeX,
    /// See: [`crate::sys_umbrella::kOfxParamDoubleTypeXAbsolute`].
    XAbsolute => crate::sys_umbrella::kOfxParamDoubleTypeXAbsolute,
    /// See: [`crate::sys_umbrella::kOfxParamDoubleTypeXY`].
    XY => crate::sys_umbrella::kOfxParamDoubleTypeXY,
    /// See: [`crate::sys_umbrella::kOfxParamDoubleTypeXYAbsolute`].
    XYAbsolute => crate::sys_umbrella::kOfxParamDoubleTypeXYAbsolute,
    /// See: [`crate::sys_umbrella::kOfxParamDoubleTypeY`].
    Y => crate::sys_umbrella::kOfxParamDoubleTypeY,
    /// See: [`crate::sys_umbrella::kOfxParamDoubleTypeYAbsolute`].
    YAbsolute => crate::sys_umbrella::kOfxParamDoubleTypeYAbsolute,
);
crate::internal::low_macros::make_enum_from_paths!(ParamPropStringMode,
    /// See: [`crate::sys_umbrella::kOfxParamStringIsDirectoryPath`].
    DirectoryPath => crate::sys_umbrella::kOfxParamStringIsDirectoryPath,
    /// See: [`crate::sys_umbrella::kOfxParamStringIsFilePath`].
    FilePath => crate::sys_umbrella::kOfxParamStringIsFilePath,
    /// See: [`crate::sys_umbrella::kOfxParamStringIsLabel`].
    Label => crate::sys_umbrella::kOfxParamStringIsLabel,
    /// See: [`crate::sys_umbrella::kOfxParamStringIsMultiLine`].
    MultiLine => crate::sys_umbrella::kOfxParamStringIsMultiLine,
    /// See: [`crate::sys_umbrella::kOfxParamStringIsRichTextFormat`].
    RichTextFormat => crate::sys_umbrella::kOfxParamStringIsRichTextFormat,
    /// See: [`crate::sys_umbrella::kOfxParamStringIsSingleLine`].
    SingleLine => crate::sys_umbrella::kOfxParamStringIsSingleLine,
);
crate::internal::low_macros::make_enum_from_paths!(PropChangeReason,
    /// See: [`crate::sys_umbrella::kOfxChangePluginEdited`].
    PluginEdited => crate::sys_umbrella::kOfxChangePluginEdited,
    /// See: [`crate::sys_umbrella::kOfxChangeTime`].
    Time => crate::sys_umbrella::kOfxChangeTime,
    /// See: [`crate::sys_umbrella::kOfxChangeUserEdited`].
    UserEdited => crate::sys_umbrella::kOfxChangeUserEdited,
);

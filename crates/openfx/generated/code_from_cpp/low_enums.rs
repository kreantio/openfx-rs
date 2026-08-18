macro make_enum_from_idents($name:ident, $($var:ident : $var_cstr:literal),*) {
  #[derive(Debug, Clone)]
  pub enum $name {
    $($var,)*
    Other(*const std::os::raw::c_char),
  }
  impl $name {
    /// ## SAFETY
    ///
    /// - The pointer must be valid and point to a null-terminated C string.
    /// - The pointer must live at least as long as the returned [`Self`] value.
    pub unsafe fn from_ptr(ptr: *const std::os::raw::c_char) -> Self {
      let cstr = unsafe { std::ffi::CStr::from_ptr(ptr) };
      $(
        if cstr == $var_cstr {
          return Self::$var;
        }
      )*
      Self::Other(cstr.as_ptr())
    }

    /// ## SAFETY
    ///
    /// The returned pointer is valid as long as the [`std::ffi::CString`] inside [`Self::Other`] is not dropped.
    pub fn as_ptr(&self) -> *const std::os::raw::c_char {
      if let Self::Other(ptr) = self {
        return *ptr;
      }
      $(
        if matches!(self, Self::$var) {
          return $var_cstr.as_ptr() as *const std::os::raw::c_char;
        }
      )*
      unreachable!()
    }
  }
}

macro make_enum_from_paths($name:ident, $(#[$meta:meta] $var:ident => $var_path:path),*) {
  #[derive(Debug, Clone)]
  pub enum $name {
    $(#[$meta] $var,)*
    Other(*const std::os::raw::c_char),
  }
  impl $name {
    /// ## SAFETY
    ///
    /// - The pointer must be valid and point to a null-terminated C string.
    /// - The pointer must live at least as long as the returned [`Self`] value.
    pub unsafe fn from_ptr(ptr: *const std::os::raw::c_char) -> Self {
      let cstr = unsafe { std::ffi::CStr::from_ptr(ptr) };
      $(
        if cstr == $var_path {
          return Self::$var;
        }
      )*
      Self::Other(cstr.as_ptr())
    }

    /// ## SAFETY
    ///
    /// The returned pointer is valid as long as the [`std::ffi::CString`] inside [`Self::Other`] is not dropped.
    pub fn as_ptr(&self) -> *const std::os::raw::c_char {
      if let Self::Other(ptr) = self {
        return *ptr;
      }
      $(
        if matches!(self, Self::$var) {
          return $var_path.as_ptr() as *const std::os::raw::c_char;
        }
      )*
      unreachable!()
    }
  }
}

make_enum_from_paths!(OfxImageClipPropFieldExtraction, #[doc = "See: [`crate::sys_umbrella::kOfxImageFieldBoth`]."] OfxImageFieldBoth => crate::sys_umbrella::kOfxImageFieldBoth, #[doc = "See: [`crate::sys_umbrella::kOfxImageFieldDoubled`]."] OfxImageFieldDoubled => crate::sys_umbrella::kOfxImageFieldDoubled, #[doc = "See: [`crate::sys_umbrella::kOfxImageFieldLower`]."] OfxImageFieldLower => crate::sys_umbrella::kOfxImageFieldLower, #[doc = "See: [`crate::sys_umbrella::kOfxImageFieldNone`]."] OfxImageFieldNone => crate::sys_umbrella::kOfxImageFieldNone, #[doc = "See: [`crate::sys_umbrella::kOfxImageFieldSingle`]."] OfxImageFieldSingle => crate::sys_umbrella::kOfxImageFieldSingle, #[doc = "See: [`crate::sys_umbrella::kOfxImageFieldUpper`]."] OfxImageFieldUpper => crate::sys_umbrella::kOfxImageFieldUpper);
make_enum_from_paths!(OfxImageClipPropFieldOrder, #[doc = "See: [`crate::sys_umbrella::kOfxImageFieldLower`]."] OfxImageFieldLower => crate::sys_umbrella::kOfxImageFieldLower, #[doc = "See: [`crate::sys_umbrella::kOfxImageFieldNone`]."] OfxImageFieldNone => crate::sys_umbrella::kOfxImageFieldNone, #[doc = "See: [`crate::sys_umbrella::kOfxImageFieldUpper`]."] OfxImageFieldUpper => crate::sys_umbrella::kOfxImageFieldUpper);
make_enum_from_paths!(OfxImageClipPropUnmappedComponents, #[doc = "See: [`crate::sys_umbrella::kOfxImageComponentAlpha`]."] OfxImageComponentAlpha => crate::sys_umbrella::kOfxImageComponentAlpha, #[doc = "See: [`crate::sys_umbrella::kOfxImageComponentNone`]."] OfxImageComponentNone => crate::sys_umbrella::kOfxImageComponentNone, #[doc = "See: [`crate::sys_umbrella::kOfxImageComponentRGB`]."] OfxImageComponentRGB => crate::sys_umbrella::kOfxImageComponentRGB, #[doc = "See: [`crate::sys_umbrella::kOfxImageComponentRGBA`]."] OfxImageComponentRGBA => crate::sys_umbrella::kOfxImageComponentRGBA);
make_enum_from_paths!(OfxImageClipPropUnmappedPixelDepth, #[doc = "See: [`crate::sys_umbrella::kOfxBitDepthByte`]."] OfxBitDepthByte => crate::sys_umbrella::kOfxBitDepthByte, #[doc = "See: [`crate::sys_umbrella::kOfxBitDepthFloat`]."] OfxBitDepthFloat => crate::sys_umbrella::kOfxBitDepthFloat, #[doc = "See: [`crate::sys_umbrella::kOfxBitDepthHalf`]."] OfxBitDepthHalf => crate::sys_umbrella::kOfxBitDepthHalf, #[doc = "See: [`crate::sys_umbrella::kOfxBitDepthNone`]."] OfxBitDepthNone => crate::sys_umbrella::kOfxBitDepthNone, #[doc = "See: [`crate::sys_umbrella::kOfxBitDepthShort`]."] OfxBitDepthShort => crate::sys_umbrella::kOfxBitDepthShort);
make_enum_from_paths!(OfxImageEffectHostPropNativeOrigin, #[doc = "See: [`crate::sys_umbrella::kOfxHostNativeOriginBottomLeft`]."] OfxHostNativeOriginBottomLeft => crate::sys_umbrella::kOfxHostNativeOriginBottomLeft, #[doc = "See: [`crate::sys_umbrella::kOfxHostNativeOriginCenter`]."] OfxHostNativeOriginCenter => crate::sys_umbrella::kOfxHostNativeOriginCenter, #[doc = "See: [`crate::sys_umbrella::kOfxHostNativeOriginTopLeft`]."] OfxHostNativeOriginTopLeft => crate::sys_umbrella::kOfxHostNativeOriginTopLeft);
make_enum_from_paths!(OfxImageEffectPluginRenderThreadSafety, #[doc = "See: [`crate::sys_umbrella::kOfxImageEffectRenderFullySafe`]."] OfxImageEffectRenderFullySafe => crate::sys_umbrella::kOfxImageEffectRenderFullySafe, #[doc = "See: [`crate::sys_umbrella::kOfxImageEffectRenderInstanceSafe`]."] OfxImageEffectRenderInstanceSafe => crate::sys_umbrella::kOfxImageEffectRenderInstanceSafe, #[doc = "See: [`crate::sys_umbrella::kOfxImageEffectRenderUnsafe`]."] OfxImageEffectRenderUnsafe => crate::sys_umbrella::kOfxImageEffectRenderUnsafe);
make_enum_from_paths!(OfxImageEffectPropColourManagementStyle, #[doc = "See: [`crate::sys_umbrella::kOfxImageEffectColourManagementBasic`]."] OfxImageEffectColourManagementBasic => crate::sys_umbrella::kOfxImageEffectColourManagementBasic, #[doc = "See: [`crate::sys_umbrella::kOfxImageEffectColourManagementCore`]."] OfxImageEffectColourManagementCore => crate::sys_umbrella::kOfxImageEffectColourManagementCore, #[doc = "See: [`crate::sys_umbrella::kOfxImageEffectColourManagementFull`]."] OfxImageEffectColourManagementFull => crate::sys_umbrella::kOfxImageEffectColourManagementFull, #[doc = "See: [`crate::sys_umbrella::kOfxImageEffectColourManagementNone`]."] OfxImageEffectColourManagementNone => crate::sys_umbrella::kOfxImageEffectColourManagementNone, #[doc = "See: [`crate::sys_umbrella::kOfxImageEffectColourManagementOCIO`]."] OfxImageEffectColourManagementOCIO => crate::sys_umbrella::kOfxImageEffectColourManagementOCIO);
make_enum_from_paths!(OfxImageEffectPropComponents, #[doc = "See: [`crate::sys_umbrella::kOfxImageComponentAlpha`]."] OfxImageComponentAlpha => crate::sys_umbrella::kOfxImageComponentAlpha, #[doc = "See: [`crate::sys_umbrella::kOfxImageComponentNone`]."] OfxImageComponentNone => crate::sys_umbrella::kOfxImageComponentNone, #[doc = "See: [`crate::sys_umbrella::kOfxImageComponentRGB`]."] OfxImageComponentRGB => crate::sys_umbrella::kOfxImageComponentRGB, #[doc = "See: [`crate::sys_umbrella::kOfxImageComponentRGBA`]."] OfxImageComponentRGBA => crate::sys_umbrella::kOfxImageComponentRGBA);
make_enum_from_paths!(OfxImageEffectPropContext, #[doc = "See: [`crate::sys_umbrella::kOfxImageEffectContextFilter`]."] OfxImageEffectContextFilter => crate::sys_umbrella::kOfxImageEffectContextFilter, #[doc = "See: [`crate::sys_umbrella::kOfxImageEffectContextGeneral`]."] OfxImageEffectContextGeneral => crate::sys_umbrella::kOfxImageEffectContextGeneral, #[doc = "See: [`crate::sys_umbrella::kOfxImageEffectContextGenerator`]."] OfxImageEffectContextGenerator => crate::sys_umbrella::kOfxImageEffectContextGenerator, #[doc = "See: [`crate::sys_umbrella::kOfxImageEffectContextPaint`]."] OfxImageEffectContextPaint => crate::sys_umbrella::kOfxImageEffectContextPaint, #[doc = "See: [`crate::sys_umbrella::kOfxImageEffectContextRetimer`]."] OfxImageEffectContextRetimer => crate::sys_umbrella::kOfxImageEffectContextRetimer, #[doc = "See: [`crate::sys_umbrella::kOfxImageEffectContextTransition`]."] OfxImageEffectContextTransition => crate::sys_umbrella::kOfxImageEffectContextTransition);
make_enum_from_idents!(OfxImageEffectPropCPURenderSupported, r#false : c"false", r#true : c"true");
make_enum_from_idents!(OfxImageEffectPropCudaRenderSupported, r#false : c"false", r#needed : c"needed", r#true : c"true");
make_enum_from_idents!(OfxImageEffectPropCudaStreamSupported, r#false : c"false", r#needed : c"needed", r#true : c"true");
make_enum_from_paths!(OfxImageEffectPropFieldToRender, #[doc = "See: [`crate::sys_umbrella::kOfxImageFieldBoth`]."] OfxImageFieldBoth => crate::sys_umbrella::kOfxImageFieldBoth, #[doc = "See: [`crate::sys_umbrella::kOfxImageFieldLower`]."] OfxImageFieldLower => crate::sys_umbrella::kOfxImageFieldLower, #[doc = "See: [`crate::sys_umbrella::kOfxImageFieldNone`]."] OfxImageFieldNone => crate::sys_umbrella::kOfxImageFieldNone, #[doc = "See: [`crate::sys_umbrella::kOfxImageFieldUpper`]."] OfxImageFieldUpper => crate::sys_umbrella::kOfxImageFieldUpper);
make_enum_from_idents!(OfxImageEffectPropMetalRenderSupported, r#false : c"false", r#needed : c"needed", r#true : c"true");
make_enum_from_idents!(OfxImageEffectPropNoSpatialAwareness, r#false : c"false", r#true : c"true");
make_enum_from_idents!(OfxImageEffectPropOpenCLRenderSupported, r#false : c"false", r#needed : c"needed", r#true : c"true");
make_enum_from_idents!(OfxImageEffectPropOpenCLSupported, r#false : c"false", r#true : c"true");
make_enum_from_idents!(OfxImageEffectPropOpenGLRenderSupported, r#false : c"false", r#needed : c"needed", r#true : c"true");
make_enum_from_paths!(OfxImageEffectPropPixelDepth, #[doc = "See: [`crate::sys_umbrella::kOfxBitDepthByte`]."] OfxBitDepthByte => crate::sys_umbrella::kOfxBitDepthByte, #[doc = "See: [`crate::sys_umbrella::kOfxBitDepthFloat`]."] OfxBitDepthFloat => crate::sys_umbrella::kOfxBitDepthFloat, #[doc = "See: [`crate::sys_umbrella::kOfxBitDepthHalf`]."] OfxBitDepthHalf => crate::sys_umbrella::kOfxBitDepthHalf, #[doc = "See: [`crate::sys_umbrella::kOfxBitDepthNone`]."] OfxBitDepthNone => crate::sys_umbrella::kOfxBitDepthNone, #[doc = "See: [`crate::sys_umbrella::kOfxBitDepthShort`]."] OfxBitDepthShort => crate::sys_umbrella::kOfxBitDepthShort);
make_enum_from_paths!(OfxImageEffectPropPreMultiplication, #[doc = "See: [`crate::sys_umbrella::kOfxImageOpaque`]."] OfxImageOpaque => crate::sys_umbrella::kOfxImageOpaque, #[doc = "See: [`crate::sys_umbrella::kOfxImagePreMultiplied`]."] OfxImagePreMultiplied => crate::sys_umbrella::kOfxImagePreMultiplied, #[doc = "See: [`crate::sys_umbrella::kOfxImageUnPreMultiplied`]."] OfxImageUnPreMultiplied => crate::sys_umbrella::kOfxImageUnPreMultiplied);
make_enum_from_paths!(OfxImageEffectPropSupportedComponents, #[doc = "See: [`crate::sys_umbrella::kOfxImageComponentAlpha`]."] OfxImageComponentAlpha => crate::sys_umbrella::kOfxImageComponentAlpha, #[doc = "See: [`crate::sys_umbrella::kOfxImageComponentNone`]."] OfxImageComponentNone => crate::sys_umbrella::kOfxImageComponentNone, #[doc = "See: [`crate::sys_umbrella::kOfxImageComponentRGB`]."] OfxImageComponentRGB => crate::sys_umbrella::kOfxImageComponentRGB, #[doc = "See: [`crate::sys_umbrella::kOfxImageComponentRGBA`]."] OfxImageComponentRGBA => crate::sys_umbrella::kOfxImageComponentRGBA);
make_enum_from_paths!(OfxImageEffectPropSupportedContexts, #[doc = "See: [`crate::sys_umbrella::kOfxImageEffectContextFilter`]."] OfxImageEffectContextFilter => crate::sys_umbrella::kOfxImageEffectContextFilter, #[doc = "See: [`crate::sys_umbrella::kOfxImageEffectContextGeneral`]."] OfxImageEffectContextGeneral => crate::sys_umbrella::kOfxImageEffectContextGeneral, #[doc = "See: [`crate::sys_umbrella::kOfxImageEffectContextGenerator`]."] OfxImageEffectContextGenerator => crate::sys_umbrella::kOfxImageEffectContextGenerator, #[doc = "See: [`crate::sys_umbrella::kOfxImageEffectContextPaint`]."] OfxImageEffectContextPaint => crate::sys_umbrella::kOfxImageEffectContextPaint, #[doc = "See: [`crate::sys_umbrella::kOfxImageEffectContextRetimer`]."] OfxImageEffectContextRetimer => crate::sys_umbrella::kOfxImageEffectContextRetimer, #[doc = "See: [`crate::sys_umbrella::kOfxImageEffectContextTransition`]."] OfxImageEffectContextTransition => crate::sys_umbrella::kOfxImageEffectContextTransition);
make_enum_from_paths!(OfxImageEffectPropSupportedPixelDepths, #[doc = "See: [`crate::sys_umbrella::kOfxBitDepthByte`]."] OfxBitDepthByte => crate::sys_umbrella::kOfxBitDepthByte, #[doc = "See: [`crate::sys_umbrella::kOfxBitDepthFloat`]."] OfxBitDepthFloat => crate::sys_umbrella::kOfxBitDepthFloat, #[doc = "See: [`crate::sys_umbrella::kOfxBitDepthHalf`]."] OfxBitDepthHalf => crate::sys_umbrella::kOfxBitDepthHalf, #[doc = "See: [`crate::sys_umbrella::kOfxBitDepthNone`]."] OfxBitDepthNone => crate::sys_umbrella::kOfxBitDepthNone, #[doc = "See: [`crate::sys_umbrella::kOfxBitDepthShort`]."] OfxBitDepthShort => crate::sys_umbrella::kOfxBitDepthShort);
make_enum_from_idents!(OfxImageEffectPropThumbnailRender, r#false : c"false", r#true : c"true");
make_enum_from_paths!(OfxImagePropField, #[doc = "See: [`crate::sys_umbrella::kOfxImageFieldBoth`]."] OfxImageFieldBoth => crate::sys_umbrella::kOfxImageFieldBoth, #[doc = "See: [`crate::sys_umbrella::kOfxImageFieldLower`]."] OfxImageFieldLower => crate::sys_umbrella::kOfxImageFieldLower, #[doc = "See: [`crate::sys_umbrella::kOfxImageFieldNone`]."] OfxImageFieldNone => crate::sys_umbrella::kOfxImageFieldNone, #[doc = "See: [`crate::sys_umbrella::kOfxImageFieldUpper`]."] OfxImageFieldUpper => crate::sys_umbrella::kOfxImageFieldUpper);
make_enum_from_paths!(OfxOpenGLPropPixelDepth, #[doc = "See: [`crate::sys_umbrella::kOfxBitDepthByte`]."] OfxBitDepthByte => crate::sys_umbrella::kOfxBitDepthByte, #[doc = "See: [`crate::sys_umbrella::kOfxBitDepthFloat`]."] OfxBitDepthFloat => crate::sys_umbrella::kOfxBitDepthFloat, #[doc = "See: [`crate::sys_umbrella::kOfxBitDepthHalf`]."] OfxBitDepthHalf => crate::sys_umbrella::kOfxBitDepthHalf, #[doc = "See: [`crate::sys_umbrella::kOfxBitDepthNone`]."] OfxBitDepthNone => crate::sys_umbrella::kOfxBitDepthNone, #[doc = "See: [`crate::sys_umbrella::kOfxBitDepthShort`]."] OfxBitDepthShort => crate::sys_umbrella::kOfxBitDepthShort);
make_enum_from_paths!(OfxParamPropCacheInvalidation, #[doc = "See: [`crate::sys_umbrella::kOfxParamInvalidateAll`]."] OfxParamInvalidateAll => crate::sys_umbrella::kOfxParamInvalidateAll, #[doc = "See: [`crate::sys_umbrella::kOfxParamInvalidateValueChange`]."] OfxParamInvalidateValueChange => crate::sys_umbrella::kOfxParamInvalidateValueChange, #[doc = "See: [`crate::sys_umbrella::kOfxParamInvalidateValueChangeToEnd`]."] OfxParamInvalidateValueChangeToEnd => crate::sys_umbrella::kOfxParamInvalidateValueChangeToEnd);
make_enum_from_paths!(OfxParamPropDefaultCoordinateSystem, #[doc = "See: [`crate::sys_umbrella::kOfxParamCoordinatesCanonical`]."] OfxParamCoordinatesCanonical => crate::sys_umbrella::kOfxParamCoordinatesCanonical, #[doc = "See: [`crate::sys_umbrella::kOfxParamCoordinatesNormalised`]."] OfxParamCoordinatesNormalised => crate::sys_umbrella::kOfxParamCoordinatesNormalised);
make_enum_from_paths!(OfxParamPropDoubleType, #[doc = "See: [`crate::sys_umbrella::kOfxParamDoubleTypeAbsoluteTime`]."] OfxParamDoubleTypeAbsoluteTime => crate::sys_umbrella::kOfxParamDoubleTypeAbsoluteTime, #[doc = "See: [`crate::sys_umbrella::kOfxParamDoubleTypeAngle`]."] OfxParamDoubleTypeAngle => crate::sys_umbrella::kOfxParamDoubleTypeAngle, #[doc = "See: [`crate::sys_umbrella::kOfxParamDoubleTypePlain`]."] OfxParamDoubleTypePlain => crate::sys_umbrella::kOfxParamDoubleTypePlain, #[doc = "See: [`crate::sys_umbrella::kOfxParamDoubleTypeScale`]."] OfxParamDoubleTypeScale => crate::sys_umbrella::kOfxParamDoubleTypeScale, #[doc = "See: [`crate::sys_umbrella::kOfxParamDoubleTypeTime`]."] OfxParamDoubleTypeTime => crate::sys_umbrella::kOfxParamDoubleTypeTime, #[doc = "See: [`crate::sys_umbrella::kOfxParamDoubleTypeX`]."] OfxParamDoubleTypeX => crate::sys_umbrella::kOfxParamDoubleTypeX, #[doc = "See: [`crate::sys_umbrella::kOfxParamDoubleTypeXAbsolute`]."] OfxParamDoubleTypeXAbsolute => crate::sys_umbrella::kOfxParamDoubleTypeXAbsolute, #[doc = "See: [`crate::sys_umbrella::kOfxParamDoubleTypeXY`]."] OfxParamDoubleTypeXY => crate::sys_umbrella::kOfxParamDoubleTypeXY, #[doc = "See: [`crate::sys_umbrella::kOfxParamDoubleTypeXYAbsolute`]."] OfxParamDoubleTypeXYAbsolute => crate::sys_umbrella::kOfxParamDoubleTypeXYAbsolute, #[doc = "See: [`crate::sys_umbrella::kOfxParamDoubleTypeY`]."] OfxParamDoubleTypeY => crate::sys_umbrella::kOfxParamDoubleTypeY, #[doc = "See: [`crate::sys_umbrella::kOfxParamDoubleTypeYAbsolute`]."] OfxParamDoubleTypeYAbsolute => crate::sys_umbrella::kOfxParamDoubleTypeYAbsolute);
make_enum_from_paths!(OfxParamPropStringMode, #[doc = "See: [`crate::sys_umbrella::kOfxParamStringIsDirectoryPath`]."] OfxParamStringIsDirectoryPath => crate::sys_umbrella::kOfxParamStringIsDirectoryPath, #[doc = "See: [`crate::sys_umbrella::kOfxParamStringIsFilePath`]."] OfxParamStringIsFilePath => crate::sys_umbrella::kOfxParamStringIsFilePath, #[doc = "See: [`crate::sys_umbrella::kOfxParamStringIsLabel`]."] OfxParamStringIsLabel => crate::sys_umbrella::kOfxParamStringIsLabel, #[doc = "See: [`crate::sys_umbrella::kOfxParamStringIsMultiLine`]."] OfxParamStringIsMultiLine => crate::sys_umbrella::kOfxParamStringIsMultiLine, #[doc = "See: [`crate::sys_umbrella::kOfxParamStringIsRichTextFormat`]."] OfxParamStringIsRichTextFormat => crate::sys_umbrella::kOfxParamStringIsRichTextFormat, #[doc = "See: [`crate::sys_umbrella::kOfxParamStringIsSingleLine`]."] OfxParamStringIsSingleLine => crate::sys_umbrella::kOfxParamStringIsSingleLine);
make_enum_from_paths!(OfxPropChangeReason, #[doc = "See: [`crate::sys_umbrella::kOfxChangePluginEdited`]."] OfxChangePluginEdited => crate::sys_umbrella::kOfxChangePluginEdited, #[doc = "See: [`crate::sys_umbrella::kOfxChangeTime`]."] OfxChangeTime => crate::sys_umbrella::kOfxChangeTime, #[doc = "See: [`crate::sys_umbrella::kOfxChangeUserEdited`]."] OfxChangeUserEdited => crate::sys_umbrella::kOfxChangeUserEdited);

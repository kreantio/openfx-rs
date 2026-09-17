/// ```doxygen
/// @brief String to label images with YUVA components
/// --ofxImageEffects.h
/// @deprecated - removed in v1.4. Note, this has been deprecated in v1.3
/// ```
pub const kOfxImageComponentYUVA: &::std::ffi::CStr = c"OfxImageComponentYUVA";
/// ```doxygen
/// @brief Indicates whether an effect is performing an analysis pass.
/// --ofxImageEffects.h
/// @deprecated - This feature has been deprecated - officially commented out v1.4.
///
///     @propdef
///     type: bool
///     dimension: 1
///     deprecated: "1.4"
/// ```
pub const kOfxImageEffectPropInAnalysis: &::std::ffi::CStr = c"OfxImageEffectPropInAnalysis";
/// ```doxygen
/// @brief The size of an interact's openGL viewport
/// -- ofxInteract.h
/// @deprecated - V1.3: This property is the redundant and its use will be deprecated in future releases.
/// V1.4: Removed
///
///     @propdef
///     type: int
///     dimension: 2
///     cname: kOfxInteractPropViewportSize
///     deprecated: "1.3"
/// ```
pub const kOfxInteractPropViewportSize: &::std::ffi::CStr = c"OfxInteractPropViewport";
/// ```doxygen
/// @brief value for the ::kOfxParamPropDoubleType property, indicating a size normalised to the X dimension. See \ref ::kOfxParamPropDoubleType.
/// -- ofxParam.h
/// @deprecated - V1.3: Deprecated in favour of ::OfxParamDoubleTypeX
/// V1.4: Removed
/// ```
pub const kOfxParamDoubleTypeNormalisedX: &::std::ffi::CStr = c"OfxParamDoubleTypeNormalisedX";
/// ```doxygen
/// @brief value for the ::kOfxParamPropDoubleType property, indicating a size normalised to the Y dimension. See \ref ::kOfxParamPropDoubleType.
/// -- ofxParam.h
/// @deprecated - V1.3: Deprecated in favour of ::OfxParamDoubleTypeY
/// V1.4: Removed
/// ```
pub const kOfxParamDoubleTypeNormalisedY: &::std::ffi::CStr = c"OfxParamDoubleTypeNormalisedY";
/// ```doxygen
/// @brief value for the ::kOfxParamPropDoubleType property, indicating an absolute position normalised to the X dimension. See \ref ::kOfxParamPropDoubleType.
/// -- ofxParam.h
/// @deprecated - V1.3: Deprecated in favour of ::OfxParamDoubleTypeXAbsolute
/// V1.4: Removed
/// ```
pub const kOfxParamDoubleTypeNormalisedXAbsolute: &::std::ffi::CStr = c"OfxParamDoubleTypeNormalisedXAbsolute";
/// ```doxygen
/// @brief value for the ::kOfxParamPropDoubleType property, indicating an absolute position  normalised to the Y dimension. See \ref ::kOfxParamPropDoubleType.
/// -- ofxParam.h
/// @deprecated - V1.3: Deprecated in favour of ::OfxParamDoubleTypeYAbsolute
/// V1.4: Removed
/// ```
pub const kOfxParamDoubleTypeNormalisedYAbsolute: &::std::ffi::CStr = c"OfxParamDoubleTypeNormalisedYAbsolute";
/// ```doxygen
/// @brief value for the ::kOfxParamPropDoubleType property, indicating normalisation to the X and Y dimension for 2D params. See \ref ::kOfxParamPropDoubleType.
/// -- ofxParam.h
/// @deprecated - V1.3: Deprecated in favour of ::OfxParamDoubleTypeXY
/// V1.4: Removed
/// ```
pub const kOfxParamDoubleTypeNormalisedXY: &::std::ffi::CStr = c"OfxParamDoubleTypeNormalisedXY";
/// ```doxygen
/// @brief value for the ::kOfxParamPropDoubleType property, indicating normalisation to the X and Y dimension for a 2D param that can be interpreted as an absolute spatial position. See \ref ::kOfxParamPropDoubleType.
/// -- ofxParam.h
/// @deprecated - V1.3: Deprecated in favour of ::kOfxParamDoubleTypeXYAbsolute
/// V1.4: Removed
/// ```
pub const kOfxParamDoubleTypeNormalisedXYAbsolute: &::std::ffi::CStr = c"OfxParamDoubleTypeNormalisedXYAbsolute";
/// ```doxygen
/// @brief Defines an 8 bit per component YUVA pixel
/// -- ofxPixels.h
/// Deprecated in 1.3, removed in 1.4
/// ```
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxYUVAColourB {
    pub y: ::std::os::raw::c_uchar,
    pub u: ::std::os::raw::c_uchar,
    pub v: ::std::os::raw::c_uchar,
    pub a: ::std::os::raw::c_uchar,
}
/// ```doxygen
/// @brief Defines an 16 bit per component YUVA pixel
/// -- ofxPixels.h
/// @deprecated -  Deprecated in 1.3, removed in 1.4
/// ```
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxYUVAColourS {
    pub y: ::std::os::raw::c_ushort,
    pub u: ::std::os::raw::c_ushort,
    pub v: ::std::os::raw::c_ushort,
    pub a: ::std::os::raw::c_ushort,
}
/// ```doxygen
/// @brief Defines an floating point component YUVA pixel
/// -- ofxPixels.h
/// @deprecated -  Deprecated in 1.3, removed in 1.4
/// ```
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxYUVAColourF {
    pub y: f32,
    pub u: f32,
    pub v: f32,
    pub a: f32,
}

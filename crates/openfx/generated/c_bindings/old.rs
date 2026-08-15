pub const kOfxImageComponentYUVA: &::std::ffi::CStr = c"OfxImageComponentYUVA";
pub const kOfxImageEffectPropInAnalysis: &::std::ffi::CStr = c"OfxImageEffectPropInAnalysis";
pub const kOfxInteractPropViewportSize: &::std::ffi::CStr = c"OfxInteractPropViewport";
pub const kOfxParamDoubleTypeNormalisedX: &::std::ffi::CStr = c"OfxParamDoubleTypeNormalisedX";
pub const kOfxParamDoubleTypeNormalisedY: &::std::ffi::CStr = c"OfxParamDoubleTypeNormalisedY";
pub const kOfxParamDoubleTypeNormalisedXAbsolute: &::std::ffi::CStr = c"OfxParamDoubleTypeNormalisedXAbsolute";
pub const kOfxParamDoubleTypeNormalisedYAbsolute: &::std::ffi::CStr = c"OfxParamDoubleTypeNormalisedYAbsolute";
pub const kOfxParamDoubleTypeNormalisedXY: &::std::ffi::CStr = c"OfxParamDoubleTypeNormalisedXY";
pub const kOfxParamDoubleTypeNormalisedXYAbsolute: &::std::ffi::CStr = c"OfxParamDoubleTypeNormalisedXYAbsolute";
/** @brief Defines an 8 bit per component YUVA pixel
-- ofxPixels.h
Deprecated in 1.3, removed in 1.4*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxYUVAColourB {
    pub y: ::std::os::raw::c_uchar,
    pub u: ::std::os::raw::c_uchar,
    pub v: ::std::os::raw::c_uchar,
    pub a: ::std::os::raw::c_uchar,
}
/** @brief Defines an 16 bit per component YUVA pixel
-- ofxPixels.h
@deprecated -  Deprecated in 1.3, removed in 1.4*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxYUVAColourS {
    pub y: ::std::os::raw::c_ushort,
    pub u: ::std::os::raw::c_ushort,
    pub v: ::std::os::raw::c_ushort,
    pub a: ::std::os::raw::c_ushort,
}
/** @brief Defines an floating point component YUVA pixel
-- ofxPixels.h
@deprecated -  Deprecated in 1.3, removed in 1.4*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxYUVAColourF {
    pub y: f32,
    pub u: f32,
    pub v: f32,
    pub a: f32,
}

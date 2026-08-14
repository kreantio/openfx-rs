use super::core::{
    OfxHost, OfxPlugin, OfxPluginEntryPoint, OfxPointD, OfxPointI, OfxPropertySetHandle,
    OfxPropertySetStruct, OfxRangeD, OfxRangeI, OfxRectD, OfxRectI, OfxStatus, OfxTime,
    kOfxActionBeginInstanceChanged, kOfxActionBeginInstanceEdit,
    kOfxActionCreateInstance, kOfxActionDescribe, kOfxActionDestroyInstance,
    kOfxActionEndInstanceChanged, kOfxActionEndInstanceEdit, kOfxActionInstanceChanged,
    kOfxActionLoad, kOfxActionPurgeCaches, kOfxActionSyncPrivateData, kOfxActionUnload,
    kOfxBitDepthByte, kOfxBitDepthFloat, kOfxBitDepthHalf, kOfxBitDepthNone,
    kOfxBitDepthShort, kOfxChangePluginEdited, kOfxChangeTime, kOfxChangeUserEdited,
    kOfxFlagInfiniteMax, kOfxFlagInfiniteMin, kOfxPluginPropFilePath, kOfxPropAPIVersion,
    kOfxPropChangeReason, kOfxPropEffectInstance, kOfxPropHostOSHandle, kOfxPropIcon,
    kOfxPropInstanceData, kOfxPropIsInteractive, kOfxPropLabel, kOfxPropLongLabel,
    kOfxPropName, kOfxPropPluginDescription, kOfxPropShortLabel, kOfxPropTime,
    kOfxPropType, kOfxPropVersion, kOfxPropVersionLabel,
};
pub const kOfxImageComponentYUVA: &::std::ffi::CStr = c"OfxImageComponentYUVA";
pub const kOfxImageEffectPropInAnalysis: &::std::ffi::CStr = c"OfxImageEffectPropInAnalysis";
pub const kOfxInteractPropViewportSize: &::std::ffi::CStr = c"OfxInteractPropViewport";
pub const kOfxParamDoubleTypeNormalisedX: &::std::ffi::CStr = c"OfxParamDoubleTypeNormalisedX";
pub const kOfxParamDoubleTypeNormalisedY: &::std::ffi::CStr = c"OfxParamDoubleTypeNormalisedY";
pub const kOfxParamDoubleTypeNormalisedXAbsolute: &::std::ffi::CStr = c"OfxParamDoubleTypeNormalisedXAbsolute";
pub const kOfxParamDoubleTypeNormalisedYAbsolute: &::std::ffi::CStr = c"OfxParamDoubleTypeNormalisedYAbsolute";
pub const kOfxParamDoubleTypeNormalisedXY: &::std::ffi::CStr = c"OfxParamDoubleTypeNormalisedXY";
pub const kOfxParamDoubleTypeNormalisedXYAbsolute: &::std::ffi::CStr = c"OfxParamDoubleTypeNormalisedXYAbsolute";
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxHost"][::std::mem::size_of::<OfxHost>() - 16usize];
    ["Alignment of OfxHost"][::std::mem::align_of::<OfxHost>() - 8usize];
    ["Offset of field: OfxHost::host"][::std::mem::offset_of!(OfxHost, host) - 0usize];
    [
        "Offset of field: OfxHost::fetchSuite",
    ][::std::mem::offset_of!(OfxHost, fetchSuite) - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxPlugin"][::std::mem::size_of::<OfxPlugin>() - 48usize];
    ["Alignment of OfxPlugin"][::std::mem::align_of::<OfxPlugin>() - 8usize];
    [
        "Offset of field: OfxPlugin::pluginApi",
    ][::std::mem::offset_of!(OfxPlugin, pluginApi) - 0usize];
    [
        "Offset of field: OfxPlugin::apiVersion",
    ][::std::mem::offset_of!(OfxPlugin, apiVersion) - 8usize];
    [
        "Offset of field: OfxPlugin::pluginIdentifier",
    ][::std::mem::offset_of!(OfxPlugin, pluginIdentifier) - 16usize];
    [
        "Offset of field: OfxPlugin::pluginVersionMajor",
    ][::std::mem::offset_of!(OfxPlugin, pluginVersionMajor) - 24usize];
    [
        "Offset of field: OfxPlugin::pluginVersionMinor",
    ][::std::mem::offset_of!(OfxPlugin, pluginVersionMinor) - 28usize];
    [
        "Offset of field: OfxPlugin::setHost",
    ][::std::mem::offset_of!(OfxPlugin, setHost) - 32usize];
    [
        "Offset of field: OfxPlugin::mainEntry",
    ][::std::mem::offset_of!(OfxPlugin, mainEntry) - 40usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxRangeI"][::std::mem::size_of::<OfxRangeI>() - 8usize];
    ["Alignment of OfxRangeI"][::std::mem::align_of::<OfxRangeI>() - 4usize];
    ["Offset of field: OfxRangeI::min"][::std::mem::offset_of!(OfxRangeI, min) - 0usize];
    ["Offset of field: OfxRangeI::max"][::std::mem::offset_of!(OfxRangeI, max) - 4usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxRangeD"][::std::mem::size_of::<OfxRangeD>() - 16usize];
    ["Alignment of OfxRangeD"][::std::mem::align_of::<OfxRangeD>() - 8usize];
    ["Offset of field: OfxRangeD::min"][::std::mem::offset_of!(OfxRangeD, min) - 0usize];
    ["Offset of field: OfxRangeD::max"][::std::mem::offset_of!(OfxRangeD, max) - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxPointI"][::std::mem::size_of::<OfxPointI>() - 8usize];
    ["Alignment of OfxPointI"][::std::mem::align_of::<OfxPointI>() - 4usize];
    ["Offset of field: OfxPointI::x"][::std::mem::offset_of!(OfxPointI, x) - 0usize];
    ["Offset of field: OfxPointI::y"][::std::mem::offset_of!(OfxPointI, y) - 4usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxPointD"][::std::mem::size_of::<OfxPointD>() - 16usize];
    ["Alignment of OfxPointD"][::std::mem::align_of::<OfxPointD>() - 8usize];
    ["Offset of field: OfxPointD::x"][::std::mem::offset_of!(OfxPointD, x) - 0usize];
    ["Offset of field: OfxPointD::y"][::std::mem::offset_of!(OfxPointD, y) - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxRectI"][::std::mem::size_of::<OfxRectI>() - 16usize];
    ["Alignment of OfxRectI"][::std::mem::align_of::<OfxRectI>() - 4usize];
    ["Offset of field: OfxRectI::x1"][::std::mem::offset_of!(OfxRectI, x1) - 0usize];
    ["Offset of field: OfxRectI::y1"][::std::mem::offset_of!(OfxRectI, y1) - 4usize];
    ["Offset of field: OfxRectI::x2"][::std::mem::offset_of!(OfxRectI, x2) - 8usize];
    ["Offset of field: OfxRectI::y2"][::std::mem::offset_of!(OfxRectI, y2) - 12usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxRectD"][::std::mem::size_of::<OfxRectD>() - 32usize];
    ["Alignment of OfxRectD"][::std::mem::align_of::<OfxRectD>() - 8usize];
    ["Offset of field: OfxRectD::x1"][::std::mem::offset_of!(OfxRectD, x1) - 0usize];
    ["Offset of field: OfxRectD::y1"][::std::mem::offset_of!(OfxRectD, y1) - 8usize];
    ["Offset of field: OfxRectD::x2"][::std::mem::offset_of!(OfxRectD, x2) - 16usize];
    ["Offset of field: OfxRectD::y2"][::std::mem::offset_of!(OfxRectD, y2) - 24usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxYUVAColourB"][::std::mem::size_of::<OfxYUVAColourB>() - 4usize];
    ["Alignment of OfxYUVAColourB"][::std::mem::align_of::<OfxYUVAColourB>() - 1usize];
    [
        "Offset of field: OfxYUVAColourB::y",
    ][::std::mem::offset_of!(OfxYUVAColourB, y) - 0usize];
    [
        "Offset of field: OfxYUVAColourB::u",
    ][::std::mem::offset_of!(OfxYUVAColourB, u) - 1usize];
    [
        "Offset of field: OfxYUVAColourB::v",
    ][::std::mem::offset_of!(OfxYUVAColourB, v) - 2usize];
    [
        "Offset of field: OfxYUVAColourB::a",
    ][::std::mem::offset_of!(OfxYUVAColourB, a) - 3usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxYUVAColourS"][::std::mem::size_of::<OfxYUVAColourS>() - 8usize];
    ["Alignment of OfxYUVAColourS"][::std::mem::align_of::<OfxYUVAColourS>() - 2usize];
    [
        "Offset of field: OfxYUVAColourS::y",
    ][::std::mem::offset_of!(OfxYUVAColourS, y) - 0usize];
    [
        "Offset of field: OfxYUVAColourS::u",
    ][::std::mem::offset_of!(OfxYUVAColourS, u) - 2usize];
    [
        "Offset of field: OfxYUVAColourS::v",
    ][::std::mem::offset_of!(OfxYUVAColourS, v) - 4usize];
    [
        "Offset of field: OfxYUVAColourS::a",
    ][::std::mem::offset_of!(OfxYUVAColourS, a) - 6usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxYUVAColourF"][::std::mem::size_of::<OfxYUVAColourF>() - 16usize];
    ["Alignment of OfxYUVAColourF"][::std::mem::align_of::<OfxYUVAColourF>() - 4usize];
    [
        "Offset of field: OfxYUVAColourF::y",
    ][::std::mem::offset_of!(OfxYUVAColourF, y) - 0usize];
    [
        "Offset of field: OfxYUVAColourF::u",
    ][::std::mem::offset_of!(OfxYUVAColourF, u) - 4usize];
    [
        "Offset of field: OfxYUVAColourF::v",
    ][::std::mem::offset_of!(OfxYUVAColourF, v) - 8usize];
    [
        "Offset of field: OfxYUVAColourF::a",
    ][::std::mem::offset_of!(OfxYUVAColourF, a) - 12usize];
};

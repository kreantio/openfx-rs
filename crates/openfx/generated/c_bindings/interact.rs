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
pub const kOfxInteractSuite: &::std::ffi::CStr = c"OfxInteractSuite";
pub const kOfxInteractPropSlaveToParam: &::std::ffi::CStr = c"OfxInteractPropSlaveToParam";
pub const kOfxInteractPropPixelScale: &::std::ffi::CStr = c"OfxInteractPropPixelScale";
pub const kOfxInteractPropBackgroundColour: &::std::ffi::CStr = c"OfxInteractPropBackgroundColour";
pub const kOfxInteractPropSuggestedColour: &::std::ffi::CStr = c"OfxInteractPropSuggestedColour";
pub const kOfxInteractPropPenPosition: &::std::ffi::CStr = c"OfxInteractPropPenPosition";
pub const kOfxInteractPropPenViewportPosition: &::std::ffi::CStr = c"OfxInteractPropPenViewportPosition";
pub const kOfxInteractPropPenPressure: &::std::ffi::CStr = c"OfxInteractPropPenPressure";
pub const kOfxInteractPropBitDepth: &::std::ffi::CStr = c"OfxInteractPropBitDepth";
pub const kOfxInteractPropHasAlpha: &::std::ffi::CStr = c"OfxInteractPropHasAlpha";
pub const kOfxActionDescribeInteract: &::std::ffi::CStr = c"OfxActionDescribe";
pub const kOfxActionCreateInstanceInteract: &::std::ffi::CStr = c"OfxActionCreateInstance";
pub const kOfxActionDestroyInstanceInteract: &::std::ffi::CStr = c"OfxActionDestroyInstance";
pub const kOfxInteractActionDraw: &::std::ffi::CStr = c"OfxInteractActionDraw";
pub const kOfxInteractActionPenMotion: &::std::ffi::CStr = c"OfxInteractActionPenMotion";
pub const kOfxInteractActionPenDown: &::std::ffi::CStr = c"OfxInteractActionPenDown";
pub const kOfxInteractActionPenUp: &::std::ffi::CStr = c"OfxInteractActionPenUp";
pub const kOfxInteractActionKeyDown: &::std::ffi::CStr = c"OfxInteractActionKeyDown";
pub const kOfxInteractActionKeyUp: &::std::ffi::CStr = c"OfxInteractActionKeyUp";
pub const kOfxInteractActionKeyRepeat: &::std::ffi::CStr = c"OfxInteractActionKeyRepeat";
pub const kOfxInteractActionGainFocus: &::std::ffi::CStr = c"OfxInteractActionGainFocus";
pub const kOfxInteractActionLoseFocus: &::std::ffi::CStr = c"OfxInteractActionLoseFocus";
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
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxInteract {
    _unused: [u8; 0],
}
/// @brief Blind declaration of an OFX interactive gui
pub type OfxInteractHandle = *mut OfxInteract;
/// @brief OFX suite that allows an effect to interact with an openGL window so as to provide custom interfaces.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxInteractSuiteV1 {
    /// @brief Requests an openGL buffer swap on the interact instance
    pub interactSwapBuffers: ::std::option::Option<
        unsafe extern "C" fn(interactInstance: OfxInteractHandle) -> OfxStatus,
    >,
    /// @brief Requests a redraw of the interact instance
    pub interactRedraw: ::std::option::Option<
        unsafe extern "C" fn(interactInstance: OfxInteractHandle) -> OfxStatus,
    >,
    /// @brief Gets the property set handle for this interact handle
    pub interactGetPropertySet: ::std::option::Option<
        unsafe extern "C" fn(
            interactInstance: OfxInteractHandle,
            property: *mut OfxPropertySetHandle,
        ) -> OfxStatus,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of OfxInteractSuiteV1",
    ][::std::mem::size_of::<OfxInteractSuiteV1>() - 24usize];
    [
        "Alignment of OfxInteractSuiteV1",
    ][::std::mem::align_of::<OfxInteractSuiteV1>() - 8usize];
    [
        "Offset of field: OfxInteractSuiteV1::interactSwapBuffers",
    ][::std::mem::offset_of!(OfxInteractSuiteV1, interactSwapBuffers) - 0usize];
    [
        "Offset of field: OfxInteractSuiteV1::interactRedraw",
    ][::std::mem::offset_of!(OfxInteractSuiteV1, interactRedraw) - 8usize];
    [
        "Offset of field: OfxInteractSuiteV1::interactGetPropertySet",
    ][::std::mem::offset_of!(OfxInteractSuiteV1, interactGetPropertySet) - 16usize];
};

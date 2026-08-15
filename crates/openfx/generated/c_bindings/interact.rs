use super::core::{OfxPropertySetHandle, OfxStatus};
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

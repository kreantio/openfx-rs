// Copyright OpenFX and contributors to the OpenFX project.
// SPDX-License-Identifier: BSD-3-Clause
use super::core::OfxStatus;
/// ```doxygen
/// @brief The name of the Dialog suite, used to fetch from a host via
///     OfxHost::fetchSuite
/// ```
pub const kOfxDialogSuite: &::std::ffi::CStr = c"OfxDialogSuite";
/// ```doxygen
/// @brief Action called after a dialog has requested a 'Dialog'
///          The arguments to the action are:
///           \arg \c user_data Pointer which was provided when the plugin requested the Dialog
///
/// 	   When the plugin receives this action it is safe to popup a dialog.
/// 	   It runs in the host's UI thread, which may differ from the main OFX processing thread.
/// 	   Plugin should return from this action when all Dialog interactions are done.
/// 	   At that point the host will continue again.
/// 	   The host will not send any other messages asynchronous to this one.
/// ```
pub const kOfxActionDialog: &::std::ffi::CStr = c"OfxActionDialog";
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxDialogSuiteV1 {
    /// ```doxygen
    /// @brief Request the host to send a kOfxActionDialog to the plugin from its UI thread.
    ///   \pre
    ///     - user_data: A pointer to any user data
    ///   \post
    ///   @returns
    ///     - ::kOfxStatOK - The host has queued the request and will send an 'OfxActionDialog'
    ///     - ::kOfxStatFailed - The host has no provisio for this or can not deal with it currently.
    /// ```
    pub RequestDialog: ::std::option::Option<
        unsafe extern "C" fn(user_data: *mut ::std::os::raw::c_void) -> OfxStatus,
    >,
    /// ```doxygen
    /// @brief Inform the host of redraw event so it can redraw itself
    ///       If the host runs fullscreen in OpenGL, it would otherwise not receive
    /// redraw event when a dialog in front would catch all events.
    ///   \pre
    ///   \post
    ///   @returns
    ///     - ::kOfxStatReplyDefault
    /// ```
    pub NotifyRedrawPending: ::std::option::Option<unsafe extern "C" fn() -> OfxStatus>,
}

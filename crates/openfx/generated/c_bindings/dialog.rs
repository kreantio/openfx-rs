use super::core::OfxStatus;
pub const kOfxDialogSuite: &::std::ffi::CStr = c"OfxDialogSuite";
pub const kOfxActionDialog: &::std::ffi::CStr = c"OfxActionDialog";
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxDialogSuiteV1 {
    /** @brief Request the host to send a kOfxActionDialog to the plugin from its UI thread.
\pre
- user_data: A pointer to any user data
\post
@returns
- ::kOfxStatOK - The host has queued the request and will send an 'OfxActionDialog'
- ::kOfxStatFailed - The host has no provisio for this or can not deal with it currently.*/
    pub RequestDialog: ::std::option::Option<
        unsafe extern "C" fn(user_data: *mut ::std::os::raw::c_void) -> OfxStatus,
    >,
    /** @brief Inform the host of redraw event so it can redraw itself
If the host runs fullscreen in OpenGL, it would otherwise not receive
redraw event when a dialog in front would catch all events.
\pre
\post
@returns
- ::kOfxStatReplyDefault*/
    pub NotifyRedrawPending: ::std::option::Option<unsafe extern "C" fn() -> OfxStatus>,
}

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
pub const kOfxMessageSuite: &::std::ffi::CStr = c"OfxMessageSuite";
pub const kOfxMessageFatal: &::std::ffi::CStr = c"OfxMessageFatal";
pub const kOfxMessageError: &::std::ffi::CStr = c"OfxMessageError";
pub const kOfxMessageWarning: &::std::ffi::CStr = c"OfxMessageWarning";
pub const kOfxMessageMessage: &::std::ffi::CStr = c"OfxMessageMessage";
pub const kOfxMessageLog: &::std::ffi::CStr = c"OfxMessageLog";
pub const kOfxMessageQuestion: &::std::ffi::CStr = c"OfxMessageQuestion";
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
/** @brief The OFX suite that allows a plug-in to pass messages back to a user. The V2 suite extends on this
in a backwards compatible manner.*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxMessageSuiteV1 {
    /** @brief Post a message on the host, using printf style varargs

\arg \c handle     effect handle (descriptor or instance) the message should be associated with, may be NULL
\arg \c messageType string describing the kind of message to post, one of the kOfxMessageType* constants
\arg \c messageId plugin specified id to associate with this message. If overriding the message in XML resource, the message is identified with this, this may be NULL, or "", in which case no override will occur,
\arg \c format    printf style format string
\arg \c ...       printf style varargs list to print

\returns
- ::kOfxStatOK - if the message was successfully posted
- ::kOfxStatReplyYes - if the message was of type  kOfxMessageQuestion and the user reply yes
- ::kOfxStatReplyNo - if the message was of type kOfxMessageQuestion and the user reply no
- ::kOfxStatFailed - if the message could not be posted for some reason
*/
    pub message: ::std::option::Option<
        unsafe extern "C" fn(
            handle: *mut ::std::os::raw::c_void,
            messageType: *const ::std::os::raw::c_char,
            messageId: *const ::std::os::raw::c_char,
            format: *const ::std::os::raw::c_char,
            ...
        ) -> OfxStatus,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxMessageSuiteV1"][::std::mem::size_of::<OfxMessageSuiteV1>() - 8usize];
    [
        "Alignment of OfxMessageSuiteV1",
    ][::std::mem::align_of::<OfxMessageSuiteV1>() - 8usize];
    [
        "Offset of field: OfxMessageSuiteV1::message",
    ][::std::mem::offset_of!(OfxMessageSuiteV1, message) - 0usize];
};
/** @brief The OFX suite that allows a plug-in to pass messages back to a user.

This extends OfxMessageSuiteV1, and should be considered a replacement to version 1.

Note that this suite has been extended in backwards compatible manner, so that a host can return this struct
for both V1 and V2.*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxMessageSuiteV2 {
    /** @brief Post a transient message on the host, using printf style varargs. Same as the V1 message suite call.

\arg \c handle     effect handle (descriptor or instance) the message should be associated with, may be null
\arg \c messageType string describing the kind of message to post, one of the kOfxMessageType* constants
\arg \c messageId plugin specified id to associate with this message. If overriding the message in XML resource, the message is identified with this, this may be NULL, or "", in which case no override will occur,
\arg \c format    printf style format string
\arg \c ...       printf style varargs list to print

\returns
- ::kOfxStatOK - if the message was successfully posted
- ::kOfxStatReplyYes - if the message was of type  kOfxMessageQuestion and the user reply yes
- ::kOfxStatReplyNo - if the message was of type kOfxMessageQuestion and the user reply no
- ::kOfxStatFailed - if the message could not be posted for some reason*/
    pub message: ::std::option::Option<
        unsafe extern "C" fn(
            handle: *mut ::std::os::raw::c_void,
            messageType: *const ::std::os::raw::c_char,
            messageId: *const ::std::os::raw::c_char,
            format: *const ::std::os::raw::c_char,
            ...
        ) -> OfxStatus,
    >,
    /** @brief Post a persistent message on an effect, using printf style varargs, and set error states. New for V2 message suite.

\arg \c handle     effect instance handle the message should be associated with, may NOT be null,
\arg \c messageType string describing the kind of message to post, should be one of...
- kOfxMessageError
- kOfxMessageWarning
- kOfxMessageMessage
\arg \c messageId plugin specified id to associate with this message. If overriding the message in XML resource, the message is identified with this, this may be NULL, or "", in which case no override will occur,
\arg \c format    printf style format string
\arg \c ...       printf style varargs list to print

\returns
- ::kOfxStatOK - if the message was successfully posted
- ::kOfxStatErrBadHandle - the handle was rubbish
- ::kOfxStatFailed - if the message could not be posted for some reason

Persistent messages are associated with an effect handle until explicitly cleared by an effect. So if an error message is posted the error state, and associated message will persist and be displayed on the effect appropriately. (eg: draw a node in red on a node based compostor and display the message when clicked on).

If \e messageType is error or warning, associated error states should be flagged on host applications. Posting an error message implies that the host cannot proceed, a warning allows the host to proceed, whilst a simple message should have no stop anything.*/
    pub setPersistentMessage: ::std::option::Option<
        unsafe extern "C" fn(
            handle: *mut ::std::os::raw::c_void,
            messageType: *const ::std::os::raw::c_char,
            messageId: *const ::std::os::raw::c_char,
            format: *const ::std::os::raw::c_char,
            ...
        ) -> OfxStatus,
    >,
    /** @brief Clears any persistent message on an effect handle that was set by OfxMessageSuiteV2::setPersistentMessage. New for V2 message suite.

\arg \c handle     effect instance handle messages should be cleared from.
\arg \c handle     effect handle (descriptor or instance)

\returns
- ::kOfxStatOK - if the message was successfully cleared
- ::kOfxStatErrBadHandle - the handle was rubbish
- ::kOfxStatFailed - if the message could not be cleared for some reason

Clearing a message will clear any associated error state.*/
    pub clearPersistentMessage: ::std::option::Option<
        unsafe extern "C" fn(handle: *mut ::std::os::raw::c_void) -> OfxStatus,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxMessageSuiteV2"][::std::mem::size_of::<OfxMessageSuiteV2>() - 24usize];
    [
        "Alignment of OfxMessageSuiteV2",
    ][::std::mem::align_of::<OfxMessageSuiteV2>() - 8usize];
    [
        "Offset of field: OfxMessageSuiteV2::message",
    ][::std::mem::offset_of!(OfxMessageSuiteV2, message) - 0usize];
    [
        "Offset of field: OfxMessageSuiteV2::setPersistentMessage",
    ][::std::mem::offset_of!(OfxMessageSuiteV2, setPersistentMessage) - 8usize];
    [
        "Offset of field: OfxMessageSuiteV2::clearPersistentMessage",
    ][::std::mem::offset_of!(OfxMessageSuiteV2, clearPersistentMessage) - 16usize];
};

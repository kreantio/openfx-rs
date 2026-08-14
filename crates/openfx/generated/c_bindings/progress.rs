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
pub const kOfxProgressSuite: &::std::ffi::CStr = c"OfxProgressSuite";
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
/** @brief A suite that provides progress feedback from a plugin to an application

A plugin instance can initiate, update and close a progress indicator with
this suite.

This is an optional suite in the Image Effect API.

API V1.4: Amends the documentation of progress suite V1 so that it is
expected that it can be raised in a modal manner and have a "cancel"
button when invoked in instanceChanged. Plugins that perform analysis
post an appropriate message, raise the progress monitor in a modal manner
and should poll to see if processing has been aborted. Any cancellation
should be handled gracefully by the plugin (eg: reset analysis parameters
to default values), clear allocated memory...

Many hosts already operate as described above. kOfxStatReplyNo should be
returned to the plugin during progressUpdate when the user presses
cancel.

Suite V2: Adds an ID that can be looked up for internationalisation and
so on. When a new version is introduced, because plug-ins need to support
old versions, and plug-in's new releases are not necessary in synch with
hosts (or users don't immediately update), best practice is to support
the 2 suite versions. That is, the plugin should check if V2 exists; if
not then check if V1 exists. This way a graceful transition is
guaranteed.  So plugin should fetchSuite passing 2,
(OfxProgressSuiteV2*) fetchSuite(mHost->mHost->host, kOfxProgressSuite,2);
and if no success pass (OfxProgressSuiteV1*)
fetchSuite(mHost->mHost->host, kOfxProgressSuite,1);*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxProgressSuiteV1 {
    /** @brief Initiate a progress bar display.

Call this to initiate the display of a progress bar.

\arg \c effectInstance the instance of the plugin this progress bar is
associated with. It cannot be NULL.
\arg \c label          a text label to display in any message portion of the
progress object's user interface. A UTF8 string.

\pre                   - There is no currently ongoing progress display for this instance.

\returns
- ::kOfxStatOK - the handle is now valid for use
- ::kOfxStatFailed - the progress object failed for some reason
- ::kOfxStatErrBadHandle - effectInstance was invalid*/
    pub progressStart: ::std::option::Option<
        unsafe extern "C" fn(
            effectInstance: *mut ::std::os::raw::c_void,
            label: *const ::std::os::raw::c_char,
        ) -> OfxStatus,
    >,
    /** @brief Indicate how much of the processing task has been completed and reports on any abort status.

\arg \c effectInstance the instance of the plugin this progress bar is
associated with. It cannot be NULL.
\arg \c progress a number between 0.0 and 1.0 indicating what proportion of the current task has been processed.

\returns
- ::kOfxStatOK - the progress object was successfully updated and the task should continue
- ::kOfxStatReplyNo - the progress object was successfully updated and the task should abort
- ::kOfxStatErrBadHandle - the progress handle was invalid,*/
    pub progressUpdate: ::std::option::Option<
        unsafe extern "C" fn(
            effectInstance: *mut ::std::os::raw::c_void,
            progress: f64,
        ) -> OfxStatus,
    >,
    /** @brief Signal that we are finished with the progress meter.

Call this when you are done with the progress meter and no
longer need it displayed.

\arg \c effectInstance the instance of the plugin this progress bar is
associated with. It cannot be NULL.

\post - you can no longer call progressUpdate on the instance

\returns
- ::kOfxStatOK - the progress object was successfully closed
- ::kOfxStatErrBadHandle - the progress handle was invalid,*/
    pub progressEnd: ::std::option::Option<
        unsafe extern "C" fn(effectInstance: *mut ::std::os::raw::c_void) -> OfxStatus,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of OfxProgressSuiteV1",
    ][::std::mem::size_of::<OfxProgressSuiteV1>() - 24usize];
    [
        "Alignment of OfxProgressSuiteV1",
    ][::std::mem::align_of::<OfxProgressSuiteV1>() - 8usize];
    [
        "Offset of field: OfxProgressSuiteV1::progressStart",
    ][::std::mem::offset_of!(OfxProgressSuiteV1, progressStart) - 0usize];
    [
        "Offset of field: OfxProgressSuiteV1::progressUpdate",
    ][::std::mem::offset_of!(OfxProgressSuiteV1, progressUpdate) - 8usize];
    [
        "Offset of field: OfxProgressSuiteV1::progressEnd",
    ][::std::mem::offset_of!(OfxProgressSuiteV1, progressEnd) - 16usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxProgressSuiteV2 {
    /** @brief Initiate a progress bar display.

Call this to initiate the display of a progress bar.

\arg \c effectInstance the instance of the plugin this progress bar is
associated with. It cannot be NULL.
\arg \c message        a text label to display in any message portion of the
progress object's user interface. A UTF8 string.
\arg \c messageId      plugin-specified id to associate with this message.
If overriding the message in an XML resource, the message
is identified with this, this may be NULL, or "", in
which case no override will occur.
New in V2 of this suite.

\pre                   - There is no currently ongoing progress display for this instance.

\returns
- ::kOfxStatOK - the handle is now valid for use
- ::kOfxStatFailed - the progress object failed for some reason
- ::kOfxStatErrBadHandle - effectInstance was invalid*/
    pub progressStart: ::std::option::Option<
        unsafe extern "C" fn(
            effectInstance: *mut ::std::os::raw::c_void,
            message: *const ::std::os::raw::c_char,
            messageid: *const ::std::os::raw::c_char,
        ) -> OfxStatus,
    >,
    /** @brief Indicate how much of the processing task has been completed and reports on any abort status.

\arg \c effectInstance the instance of the plugin this progress bar is
associated with. It cannot be NULL.
\arg \c progress a number between 0.0 and 1.0 indicating what proportion of the current task has been processed.

\returns
- ::kOfxStatOK - the progress object was successfully updated and the task should continue
- ::kOfxStatReplyNo - the progress object was successfully updated and the task should abort
- ::kOfxStatErrBadHandle - the progress handle was invalid,*/
    pub progressUpdate: ::std::option::Option<
        unsafe extern "C" fn(
            effectInstance: *mut ::std::os::raw::c_void,
            progress: f64,
        ) -> OfxStatus,
    >,
    /** @brief Signal that we are finished with the progress meter.

Call this when you are done with the progress meter and no
longer need it displayed.

\arg \c effectInstance the instance of the plugin this progress bar is
associated with. It cannot be NULL.

\post - you can no longer call progressUpdate on the instance

\returns
- ::kOfxStatOK - the progress object was successfully closed
- ::kOfxStatErrBadHandle - the progress handle was invalid,*/
    pub progressEnd: ::std::option::Option<
        unsafe extern "C" fn(effectInstance: *mut ::std::os::raw::c_void) -> OfxStatus,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of OfxProgressSuiteV2",
    ][::std::mem::size_of::<OfxProgressSuiteV2>() - 24usize];
    [
        "Alignment of OfxProgressSuiteV2",
    ][::std::mem::align_of::<OfxProgressSuiteV2>() - 8usize];
    [
        "Offset of field: OfxProgressSuiteV2::progressStart",
    ][::std::mem::offset_of!(OfxProgressSuiteV2, progressStart) - 0usize];
    [
        "Offset of field: OfxProgressSuiteV2::progressUpdate",
    ][::std::mem::offset_of!(OfxProgressSuiteV2, progressUpdate) - 8usize];
    [
        "Offset of field: OfxProgressSuiteV2::progressEnd",
    ][::std::mem::offset_of!(OfxProgressSuiteV2, progressEnd) - 16usize];
};

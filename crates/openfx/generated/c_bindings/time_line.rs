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
pub const kOfxTimeLineSuite: &::std::ffi::CStr = c"OfxTimeLineSuite";
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
/** @brief Suite to control timelines

This suite is used to enquire and control a timeline associated with a plug-in
instance.

This is an optional suite in the Image Effect API.*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxTimeLineSuiteV1 {
    /** @brief Get the time value of the timeline that is controlling to the indicated effect.

\arg \c instance is the instance of the effect changing the timeline, cast to a void *
\arg \c time pointer through which the timeline value should be returned

This function returns the current time value of the timeline associated with the effect instance.

@returns
- ::kOfxStatOK - the time enquiry was successful
- ::kOfxStatFailed - the enquiry failed for some host specific reason
- ::kOfxStatErrBadHandle - the effect handle was invalid*/
    pub getTime: ::std::option::Option<
        unsafe extern "C" fn(
            instance: *mut ::std::os::raw::c_void,
            time: *mut f64,
        ) -> OfxStatus,
    >,
    /** @brief Move the timeline control to the indicated time.

\arg \c instance is the instance of the effect changing the timeline, cast to a void *
\arg \c time is the time to change the timeline to. This is in the temporal coordinate system of the effect.

This function moves the timeline to the indicated frame and returns. Any side effects of the timeline
change are also triggered and completed before this returns (for example instance changed actions and renders
if the output of the effect is being viewed).

@returns
- ::kOfxStatOK - the time was changed successfully, will all side effects if the change completed
- ::kOfxStatFailed - the change failed for some host specific reason
- ::kOfxStatErrBadHandle - the effect handle was invalid
- ::kOfxStatErrValue - the time was an illegal value*/
    pub gotoTime: ::std::option::Option<
        unsafe extern "C" fn(
            instance: *mut ::std::os::raw::c_void,
            time: f64,
        ) -> OfxStatus,
    >,
    /** @brief Get the current bounds on a timeline

\arg \c instance is the instance of the effect changing the timeline, cast to a void *
\arg \c firstTime is the first time on the timeline. This is in the temporal coordinate system of the effect.
\arg \c lastTime is last time on the timeline. This is in the temporal coordinate system of the effect.

This function

@returns
- ::kOfxStatOK - the time enquiry was successful
- ::kOfxStatFailed - the enquiry failed for some host specific reason
- ::kOfxStatErrBadHandle - the effect handle was invalid*/
    pub getTimeBounds: ::std::option::Option<
        unsafe extern "C" fn(
            instance: *mut ::std::os::raw::c_void,
            firstTime: *mut f64,
            lastTime: *mut f64,
        ) -> OfxStatus,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of OfxTimeLineSuiteV1",
    ][::std::mem::size_of::<OfxTimeLineSuiteV1>() - 24usize];
    [
        "Alignment of OfxTimeLineSuiteV1",
    ][::std::mem::align_of::<OfxTimeLineSuiteV1>() - 8usize];
    [
        "Offset of field: OfxTimeLineSuiteV1::getTime",
    ][::std::mem::offset_of!(OfxTimeLineSuiteV1, getTime) - 0usize];
    [
        "Offset of field: OfxTimeLineSuiteV1::gotoTime",
    ][::std::mem::offset_of!(OfxTimeLineSuiteV1, gotoTime) - 8usize];
    [
        "Offset of field: OfxTimeLineSuiteV1::getTimeBounds",
    ][::std::mem::offset_of!(OfxTimeLineSuiteV1, getTimeBounds) - 16usize];
};

pub const kOfxActionLoad: &::std::ffi::CStr = c"OfxActionLoad";
pub const kOfxActionDescribe: &::std::ffi::CStr = c"OfxActionDescribe";
pub const kOfxActionUnload: &::std::ffi::CStr = c"OfxActionUnload";
pub const kOfxActionPurgeCaches: &::std::ffi::CStr = c"OfxActionPurgeCaches";
pub const kOfxActionSyncPrivateData: &::std::ffi::CStr = c"OfxActionSyncPrivateData";
pub const kOfxActionCreateInstance: &::std::ffi::CStr = c"OfxActionCreateInstance";
pub const kOfxActionDestroyInstance: &::std::ffi::CStr = c"OfxActionDestroyInstance";
pub const kOfxActionInstanceChanged: &::std::ffi::CStr = c"OfxActionInstanceChanged";
pub const kOfxActionBeginInstanceChanged: &::std::ffi::CStr = c"OfxActionBeginInstanceChanged";
pub const kOfxActionEndInstanceChanged: &::std::ffi::CStr = c"OfxActionEndInstanceChanged";
pub const kOfxActionBeginInstanceEdit: &::std::ffi::CStr = c"OfxActionBeginInstanceEdit";
pub const kOfxActionEndInstanceEdit: &::std::ffi::CStr = c"OfxActionEndInstanceEdit";
pub const kOfxPropAPIVersion: &::std::ffi::CStr = c"OfxPropAPIVersion";
pub const kOfxPropTime: &::std::ffi::CStr = c"OfxPropTime";
pub const kOfxPropIsInteractive: &::std::ffi::CStr = c"OfxPropIsInteractive";
pub const kOfxPluginPropFilePath: &::std::ffi::CStr = c"OfxPluginPropFilePath";
pub const kOfxPropInstanceData: &::std::ffi::CStr = c"OfxPropInstanceData";
pub const kOfxPropType: &::std::ffi::CStr = c"OfxPropType";
pub const kOfxPropName: &::std::ffi::CStr = c"OfxPropName";
pub const kOfxPropVersion: &::std::ffi::CStr = c"OfxPropVersion";
pub const kOfxPropVersionLabel: &::std::ffi::CStr = c"OfxPropVersionLabel";
pub const kOfxPropPluginDescription: &::std::ffi::CStr = c"OfxPropPluginDescription";
pub const kOfxPropLabel: &::std::ffi::CStr = c"OfxPropLabel";
pub const kOfxPropIcon: &::std::ffi::CStr = c"OfxPropIcon";
pub const kOfxPropShortLabel: &::std::ffi::CStr = c"OfxPropShortLabel";
pub const kOfxPropLongLabel: &::std::ffi::CStr = c"OfxPropLongLabel";
pub const kOfxPropChangeReason: &::std::ffi::CStr = c"OfxPropChangeReason";
pub const kOfxPropEffectInstance: &::std::ffi::CStr = c"OfxPropEffectInstance";
pub const kOfxPropHostOSHandle: &::std::ffi::CStr = c"OfxPropHostOSHandle";
pub const kOfxChangeUserEdited: &::std::ffi::CStr = c"OfxChangeUserEdited";
pub const kOfxChangePluginEdited: &::std::ffi::CStr = c"OfxChangePluginEdited";
pub const kOfxChangeTime: &::std::ffi::CStr = c"OfxChangeTime";
pub const kOfxFlagInfiniteMax: u32 = 2147483647;
pub const kOfxFlagInfiniteMin: i32 = -2147483648;
pub const kOfxBitDepthNone: &::std::ffi::CStr = c"OfxBitDepthNone";
pub const kOfxBitDepthByte: &::std::ffi::CStr = c"OfxBitDepthByte";
pub const kOfxBitDepthShort: &::std::ffi::CStr = c"OfxBitDepthShort";
pub const kOfxBitDepthHalf: &::std::ffi::CStr = c"OfxBitDepthHalf";
pub const kOfxBitDepthFloat: &::std::ffi::CStr = c"OfxBitDepthFloat";
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxPropertySetStruct {
    _unused: [u8; 0],
}
/// @brief Blind data structure to manipulate sets of properties through
pub type OfxPropertySetHandle = *mut OfxPropertySetStruct;
/// @brief OFX status return type
pub type OfxStatus = ::std::os::raw::c_int;
/** @brief Generic host structure passed to OfxPlugin::setHost function

This structure contains what is needed by a plug-in to bootstrap its connection
to the host.*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxHost {
    /** @brief Global handle to the host. Extract relevant host properties from this.
This pointer will be valid while the binary containing the plug-in is loaded.*/
    pub host: OfxPropertySetHandle,
    /** @brief The function which the plug-in uses to fetch suites from the host.

\arg \c host          the host the suite is being fetched from this \em must be the \e host member of the OfxHost struct containing fetchSuite.
\arg \c suiteName     ASCII string labelling the host supplied API
\arg \c suiteVersion  version of that suite to fetch

Any API fetched will be valid while the binary containing the plug-in is loaded.

Repeated calls to fetchSuite with the same parameters will return the same pointer.

It is recommended that hosts should return the same host and suite pointers to all plugins
in the same shared lib or bundle.

returns
- NULL if the API is unknown (either the api or the version requested),
- pointer to the relevant API if it was found*/
    pub fetchSuite: ::std::option::Option<
        unsafe extern "C" fn(
            host: OfxPropertySetHandle,
            suiteName: *const ::std::os::raw::c_char,
            suiteVersion: ::std::os::raw::c_int,
        ) -> *const ::std::os::raw::c_void,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxHost"][::std::mem::size_of::<OfxHost>() - 16usize];
    ["Alignment of OfxHost"][::std::mem::align_of::<OfxHost>() - 8usize];
    ["Offset of field: OfxHost::host"][::std::mem::offset_of!(OfxHost, host) - 0usize];
    [
        "Offset of field: OfxHost::fetchSuite",
    ][::std::mem::offset_of!(OfxHost, fetchSuite) - 8usize];
};
/** @brief Entry point for plug-ins

\arg \c action   ASCII c string indicating which action to take
\arg \c instance object to which action should be applied, this will need to be cast to the appropriate blind data type depending on the \e action
\arg \c inData   handle that contains action specific properties
\arg \c outData  handle where the plug-in should set various action specific properties

This is how the host generally communicates with a plug-in. Entry points are used to pass messages
to various objects used within OFX. The main use is within the OfxPlugin struct.

The exact set of actions is determined by the plug-in API that is being implemented, however all plug-ins
can perform several actions. For the list of actions consult \ref ActionsAll.*/
pub type OfxPluginEntryPoint = ::std::option::Option<
    unsafe extern "C" fn(
        action: *const ::std::os::raw::c_char,
        handle: *const ::std::os::raw::c_void,
        inArgs: OfxPropertySetHandle,
        outArgs: OfxPropertySetHandle,
    ) -> OfxStatus,
>;
/** @brief The structure that defines a plug-in to a host.

 This structure is the first element in any plug-in structure
 using the OFX plug-in architecture. By examining its members
 a host can determine the API that the plug-in implements,
 the version of that API, its name and version.

 For details see \ref Architecture.
*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxPlugin {
    /** Defines the type of the plug-in, this will tell the host what the plug-in does. e.g.: an image
effects plug-in would be a "OfxImageEffectPlugin"*/
    pub pluginApi: *const ::std::os::raw::c_char,
    /// Defines the version of the pluginApi that this plug-in implements
    pub apiVersion: ::std::os::raw::c_int,
    /** String that uniquely labels the plug-in among all plug-ins that implement an API.
It need not necessarily be human sensible, however the preference is to use reverse
internet domain name of the developer, followed by a '.' then by a name that represents
the plug-in.. It must be a legal ASCII string and have no whitespace in the
name and no non printing chars.
For example "uk.co.somesoftwarehouse.myPlugin"*/
    pub pluginIdentifier: *const ::std::os::raw::c_char,
    /// Major version of this plug-in, this gets incremented when backwards compatibility is broken.
    pub pluginVersionMajor: ::std::os::raw::c_uint,
    /**  Major version of this plug-in, this gets incremented when software is changed,
but does not break backwards compatibility.*/
    pub pluginVersionMinor: ::std::os::raw::c_uint,
    /** @brief Function the host uses to connect the plug-in to the host's api fetcher

\arg \c fetchApi pointer to host's API fetcher

Mandatory function.

The very first function called in a plug-in. The plug-in \em must \em not call any OFX functions within this, it must only set its local copy of the host pointer.

\pre
- nothing else has been called

\post
- the pointer suite is valid until the plug-in is unloaded

It is recommended that hosts should return the same host and suite pointers to all plugins
in the same shared lib or bundle.*/
    pub setHost: ::std::option::Option<unsafe extern "C" fn(host: *mut OfxHost)>,
    /** @brief Main entry point for plug-ins

Mandatory function.

The exact set of actions is determined by the plug-in API that is being implemented, however all plug-ins
can perform several actions. For the list of actions consult \ref ActionsAll.

Preconditions
- setHost has been called*/
    pub mainEntry: OfxPluginEntryPoint,
}
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
/// @brief How time is specified within the OFX API
pub type OfxTime = f64;
/// @brief Defines one dimensional integer bounds
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxRangeI {
    pub min: ::std::os::raw::c_int,
    pub max: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxRangeI"][::std::mem::size_of::<OfxRangeI>() - 8usize];
    ["Alignment of OfxRangeI"][::std::mem::align_of::<OfxRangeI>() - 4usize];
    ["Offset of field: OfxRangeI::min"][::std::mem::offset_of!(OfxRangeI, min) - 0usize];
    ["Offset of field: OfxRangeI::max"][::std::mem::offset_of!(OfxRangeI, max) - 4usize];
};
/// @brief Defines one dimensional double bounds
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxRangeD {
    pub min: f64,
    pub max: f64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxRangeD"][::std::mem::size_of::<OfxRangeD>() - 16usize];
    ["Alignment of OfxRangeD"][::std::mem::align_of::<OfxRangeD>() - 8usize];
    ["Offset of field: OfxRangeD::min"][::std::mem::offset_of!(OfxRangeD, min) - 0usize];
    ["Offset of field: OfxRangeD::max"][::std::mem::offset_of!(OfxRangeD, max) - 8usize];
};
/// @brief Defines two dimensional integer point
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxPointI {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxPointI"][::std::mem::size_of::<OfxPointI>() - 8usize];
    ["Alignment of OfxPointI"][::std::mem::align_of::<OfxPointI>() - 4usize];
    ["Offset of field: OfxPointI::x"][::std::mem::offset_of!(OfxPointI, x) - 0usize];
    ["Offset of field: OfxPointI::y"][::std::mem::offset_of!(OfxPointI, y) - 4usize];
};
/// @brief Defines two dimensional double point
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxPointD {
    pub x: f64,
    pub y: f64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxPointD"][::std::mem::size_of::<OfxPointD>() - 16usize];
    ["Alignment of OfxPointD"][::std::mem::align_of::<OfxPointD>() - 8usize];
    ["Offset of field: OfxPointD::x"][::std::mem::offset_of!(OfxPointD, x) - 0usize];
    ["Offset of field: OfxPointD::y"][::std::mem::offset_of!(OfxPointD, y) - 8usize];
};
/** @brief Defines two dimensional integer region

Regions are x1 <= x < x2

Infinite regions are flagged by setting
- x1 = \ref kOfxFlagInfiniteMin
- y1 = \ref kOfxFlagInfiniteMin
- x2 = \ref kOfxFlagInfiniteMax
- y2 = \ref kOfxFlagInfiniteMax
*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxRectI {
    pub x1: ::std::os::raw::c_int,
    pub y1: ::std::os::raw::c_int,
    pub x2: ::std::os::raw::c_int,
    pub y2: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxRectI"][::std::mem::size_of::<OfxRectI>() - 16usize];
    ["Alignment of OfxRectI"][::std::mem::align_of::<OfxRectI>() - 4usize];
    ["Offset of field: OfxRectI::x1"][::std::mem::offset_of!(OfxRectI, x1) - 0usize];
    ["Offset of field: OfxRectI::y1"][::std::mem::offset_of!(OfxRectI, y1) - 4usize];
    ["Offset of field: OfxRectI::x2"][::std::mem::offset_of!(OfxRectI, x2) - 8usize];
    ["Offset of field: OfxRectI::y2"][::std::mem::offset_of!(OfxRectI, y2) - 12usize];
};
/** @brief Defines two dimensional double region

Regions are x1 <= x < x2

Infinite regions are flagged by setting
- x1 = \ref kOfxFlagInfiniteMin
- y1 = \ref kOfxFlagInfiniteMin
- x2 = \ref kOfxFlagInfiniteMax
- y2 = \ref kOfxFlagInfiniteMax
*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxRectD {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxRectD"][::std::mem::size_of::<OfxRectD>() - 32usize];
    ["Alignment of OfxRectD"][::std::mem::align_of::<OfxRectD>() - 8usize];
    ["Offset of field: OfxRectD::x1"][::std::mem::offset_of!(OfxRectD, x1) - 0usize];
    ["Offset of field: OfxRectD::y1"][::std::mem::offset_of!(OfxRectD, y1) - 8usize];
    ["Offset of field: OfxRectD::x2"][::std::mem::offset_of!(OfxRectD, x2) - 16usize];
    ["Offset of field: OfxRectD::y2"][::std::mem::offset_of!(OfxRectD, y2) - 24usize];
};

pub const kOfxStatOK: OfxStatus = 0;
pub const kOfxStatFailed: OfxStatus = 1;
pub const kOfxStatErrFatal: OfxStatus = 2;
pub const kOfxStatErrUnknown: OfxStatus = 3;
pub const kOfxStatErrMissingHostFeature: OfxStatus = 4;
pub const kOfxStatErrUnsupported: OfxStatus = 5;
pub const kOfxStatErrExists: OfxStatus = 6;
pub const kOfxStatErrFormat: OfxStatus = 7;
pub const kOfxStatErrMemory: OfxStatus = 8;
pub const kOfxStatErrBadHandle: OfxStatus = 9;
pub const kOfxStatErrBadIndex: OfxStatus = 10;
pub const kOfxStatErrValue: OfxStatus = 11;
pub const kOfxStatReplyYes: OfxStatus = 12;
pub const kOfxStatReplyNo: OfxStatus = 13;
pub const kOfxStatReplyDefault: OfxStatus = 14;
pub const kOfxStatUnlicensed: OfxStatus = 15;
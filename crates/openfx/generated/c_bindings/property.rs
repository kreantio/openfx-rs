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
pub const kOfxPropertySuite: &::std::ffi::CStr = c"OfxPropertySuite";
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
/// @brief The OFX suite used to access properties on OFX objects.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxPropertySuiteV1 {
    /** @brief Set a single value in a pointer property

\arg \c properties handle of the thing holding the property
\arg \c property string labelling the property
\arg \c index for multidimenstional properties and is dimension of the one we are setting
\arg \c value value of the property we are setting

@returns
- ::kOfxStatOK
- ::kOfxStatErrBadHandle
- ::kOfxStatErrUnknown
- ::kOfxStatErrBadIndex
- ::kOfxStatErrValue*/
    pub propSetPointer: ::std::option::Option<
        unsafe extern "C" fn(
            properties: OfxPropertySetHandle,
            property: *const ::std::os::raw::c_char,
            index: ::std::os::raw::c_int,
            value: *mut ::std::os::raw::c_void,
        ) -> OfxStatus,
    >,
    /** @brief Set a single value in a string property

\arg \c properties handle of the thing holding the property
\arg \c property string labelling the property
\arg \c index for multidimenstional properties and is dimension of the one we are setting
\arg \c value value of the property we are setting

@returns
- ::kOfxStatOK
- ::kOfxStatErrBadHandle
- ::kOfxStatErrUnknown
- ::kOfxStatErrBadIndex
- ::kOfxStatErrValue*/
    pub propSetString: ::std::option::Option<
        unsafe extern "C" fn(
            properties: OfxPropertySetHandle,
            property: *const ::std::os::raw::c_char,
            index: ::std::os::raw::c_int,
            value: *const ::std::os::raw::c_char,
        ) -> OfxStatus,
    >,
    /** @brief Set a single value in a double property

\arg \c properties handle of the thing holding the property
\arg \c property string labelling the property
\arg \c index for multidimenstional properties and is dimension of the one we are setting
\arg \c value value of the property we are setting

@returns
- ::kOfxStatOK
- ::kOfxStatErrBadHandle
- ::kOfxStatErrUnknown
- ::kOfxStatErrBadIndex
- ::kOfxStatErrValue*/
    pub propSetDouble: ::std::option::Option<
        unsafe extern "C" fn(
            properties: OfxPropertySetHandle,
            property: *const ::std::os::raw::c_char,
            index: ::std::os::raw::c_int,
            value: f64,
        ) -> OfxStatus,
    >,
    /** @brief Set a single value in  an int property

\arg \c properties handle of the thing holding the property
\arg \c property string labelling the property
\arg \c index for multidimenstional properties and is dimension of the one we are setting
\arg \c value value of the property we are setting

@returns
- ::kOfxStatOK
- ::kOfxStatErrBadHandle
- ::kOfxStatErrUnknown
- ::kOfxStatErrBadIndex
- ::kOfxStatErrValue*/
    pub propSetInt: ::std::option::Option<
        unsafe extern "C" fn(
            properties: OfxPropertySetHandle,
            property: *const ::std::os::raw::c_char,
            index: ::std::os::raw::c_int,
            value: ::std::os::raw::c_int,
        ) -> OfxStatus,
    >,
    /** @brief Set multiple values of the pointer property

\arg \c properties handle of the thing holding the property
\arg \c property string labelling the property
\arg \c count number of values we are setting in that property (ie: indices 0..count-1)
\arg \c value pointer to an array of property values

@returns
- ::kOfxStatOK
- ::kOfxStatErrBadHandle
- ::kOfxStatErrUnknown
- ::kOfxStatErrBadIndex
- ::kOfxStatErrValue*/
    pub propSetPointerN: ::std::option::Option<
        unsafe extern "C" fn(
            properties: OfxPropertySetHandle,
            property: *const ::std::os::raw::c_char,
            count: ::std::os::raw::c_int,
            value: *const *mut ::std::os::raw::c_void,
        ) -> OfxStatus,
    >,
    /** @brief Set multiple values of a string property

\arg \c properties handle of the thing holding the property
\arg \c property string labelling the property
\arg \c count number of values we are setting in that property (ie: indices 0..count-1)
\arg \c value pointer to an array of property values

@returns
- ::kOfxStatOK
- ::kOfxStatErrBadHandle
- ::kOfxStatErrUnknown
- ::kOfxStatErrBadIndex
- ::kOfxStatErrValue*/
    pub propSetStringN: ::std::option::Option<
        unsafe extern "C" fn(
            properties: OfxPropertySetHandle,
            property: *const ::std::os::raw::c_char,
            count: ::std::os::raw::c_int,
            value: *const *const ::std::os::raw::c_char,
        ) -> OfxStatus,
    >,
    /** @brief Set multiple values of  a double property

\arg \c properties handle of the thing holding the property
\arg \c property string labelling the property
\arg \c count number of values we are setting in that property (ie: indices 0..count-1)
\arg \c value pointer to an array of property values

@returns
- ::kOfxStatOK
- ::kOfxStatErrBadHandle
- ::kOfxStatErrUnknown
- ::kOfxStatErrBadIndex
- ::kOfxStatErrValue
*/
    pub propSetDoubleN: ::std::option::Option<
        unsafe extern "C" fn(
            properties: OfxPropertySetHandle,
            property: *const ::std::os::raw::c_char,
            count: ::std::os::raw::c_int,
            value: *const f64,
        ) -> OfxStatus,
    >,
    /** @brief Set multiple values of an int property

\arg \c properties handle of the thing holding the property
\arg \c property string labelling the property
\arg \c count number of values we are setting in that property (ie: indices 0..count-1)
\arg \c value pointer to an array of property values

@returns
- ::kOfxStatOK
- ::kOfxStatErrBadHandle
- ::kOfxStatErrUnknown
- ::kOfxStatErrBadIndex
- ::kOfxStatErrValue
*/
    pub propSetIntN: ::std::option::Option<
        unsafe extern "C" fn(
            properties: OfxPropertySetHandle,
            property: *const ::std::os::raw::c_char,
            count: ::std::os::raw::c_int,
            value: *const ::std::os::raw::c_int,
        ) -> OfxStatus,
    >,
    /** @brief Get a single value from a pointer property

\arg \c properties handle of the thing holding the property
\arg \c property string labelling the property
\arg \c index refers to the index of a multi-dimensional property
\arg \c value pointer the return location

@returns
- ::kOfxStatOK
- ::kOfxStatErrBadHandle
- ::kOfxStatErrUnknown
- ::kOfxStatErrBadIndex*/
    pub propGetPointer: ::std::option::Option<
        unsafe extern "C" fn(
            properties: OfxPropertySetHandle,
            property: *const ::std::os::raw::c_char,
            index: ::std::os::raw::c_int,
            value: *mut *mut ::std::os::raw::c_void,
        ) -> OfxStatus,
    >,
    /** @brief Get a single value of a string property

\arg \c properties handle of the thing holding the property
\arg \c property string labelling the property
\arg \c index refers to the index of a multi-dimensional property
\arg \c value pointer the return location

@returns
- ::kOfxStatOK
- ::kOfxStatErrBadHandle
- ::kOfxStatErrUnknown
- ::kOfxStatErrBadIndex*/
    pub propGetString: ::std::option::Option<
        unsafe extern "C" fn(
            properties: OfxPropertySetHandle,
            property: *const ::std::os::raw::c_char,
            index: ::std::os::raw::c_int,
            value: *mut *mut ::std::os::raw::c_char,
        ) -> OfxStatus,
    >,
    /** @brief Get a single value of a double property

\arg \c properties handle of the thing holding the property
\arg \c property string labelling the property
\arg \c index refers to the index of a multi-dimensional property
\arg \c value pointer the return location

See the note \ref ArchitectureStrings for how to deal with strings.

@returns
- ::kOfxStatOK
- ::kOfxStatErrBadHandle
- ::kOfxStatErrUnknown
- ::kOfxStatErrBadIndex*/
    pub propGetDouble: ::std::option::Option<
        unsafe extern "C" fn(
            properties: OfxPropertySetHandle,
            property: *const ::std::os::raw::c_char,
            index: ::std::os::raw::c_int,
            value: *mut f64,
        ) -> OfxStatus,
    >,
    /** @brief Get a single value of an int property

\arg \c properties handle of the thing holding the property
\arg \c property string labelling the property
\arg \c index refers to the index of a multi-dimensional property
\arg \c value pointer the return location

@returns
- ::kOfxStatOK
- ::kOfxStatErrBadHandle
- ::kOfxStatErrUnknown
- ::kOfxStatErrBadIndex*/
    pub propGetInt: ::std::option::Option<
        unsafe extern "C" fn(
            properties: OfxPropertySetHandle,
            property: *const ::std::os::raw::c_char,
            index: ::std::os::raw::c_int,
            value: *mut ::std::os::raw::c_int,
        ) -> OfxStatus,
    >,
    /** @brief Get multiple values of a pointer property

\arg \c properties handle of the thing holding the property
\arg \c property string labelling the property
\arg \c count number of values we are getting of that property (ie: indices 0..count-1)
\arg \c value pointer to an array of where we will return the property values

@returns
- ::kOfxStatOK
- ::kOfxStatErrBadHandle
- ::kOfxStatErrUnknown
- ::kOfxStatErrBadIndex*/
    pub propGetPointerN: ::std::option::Option<
        unsafe extern "C" fn(
            properties: OfxPropertySetHandle,
            property: *const ::std::os::raw::c_char,
            count: ::std::os::raw::c_int,
            value: *mut *mut ::std::os::raw::c_void,
        ) -> OfxStatus,
    >,
    /** @brief Get multiple values of a string property

\arg \c properties handle of the thing holding the property
\arg \c property string labelling the property
\arg \c count number of values we are getting of that property (ie: indices 0..count-1)
\arg \c value pointer to an array of where we will return the property values

See the note \ref ArchitectureStrings for how to deal with strings.

@returns
- ::kOfxStatOK
- ::kOfxStatErrBadHandle
- ::kOfxStatErrUnknown
- ::kOfxStatErrBadIndex*/
    pub propGetStringN: ::std::option::Option<
        unsafe extern "C" fn(
            properties: OfxPropertySetHandle,
            property: *const ::std::os::raw::c_char,
            count: ::std::os::raw::c_int,
            value: *mut *mut ::std::os::raw::c_char,
        ) -> OfxStatus,
    >,
    /** @brief Get multiple values of a double property

\arg \c properties handle of the thing holding the property
\arg \c property string labelling the property
\arg \c count number of values we are getting of that property (ie: indices 0..count-1)
\arg \c value pointer to an array of where we will return the property values

@returns
- ::kOfxStatOK
- ::kOfxStatErrBadHandle
- ::kOfxStatErrUnknown
- ::kOfxStatErrBadIndex*/
    pub propGetDoubleN: ::std::option::Option<
        unsafe extern "C" fn(
            properties: OfxPropertySetHandle,
            property: *const ::std::os::raw::c_char,
            count: ::std::os::raw::c_int,
            value: *mut f64,
        ) -> OfxStatus,
    >,
    /** @brief Get multiple values of an int property

\arg \c properties handle of the thing holding the property
\arg \c property string labelling the property
\arg \c count number of values we are getting of that property (ie: indices 0..count-1)
\arg \c value pointer to an array of where we will return the property values

@returns
- ::kOfxStatOK
- ::kOfxStatErrBadHandle
- ::kOfxStatErrUnknown
- ::kOfxStatErrBadIndex*/
    pub propGetIntN: ::std::option::Option<
        unsafe extern "C" fn(
            properties: OfxPropertySetHandle,
            property: *const ::std::os::raw::c_char,
            count: ::std::os::raw::c_int,
            value: *mut ::std::os::raw::c_int,
        ) -> OfxStatus,
    >,
    /** @brief Resets all dimensions of a property to its default value

\arg \c properties handle of the thing holding the property
\arg \c property string labelling the property we are resetting

@returns
- ::kOfxStatOK
- ::kOfxStatErrBadHandle
- ::kOfxStatErrUnknown*/
    pub propReset: ::std::option::Option<
        unsafe extern "C" fn(
            properties: OfxPropertySetHandle,
            property: *const ::std::os::raw::c_char,
        ) -> OfxStatus,
    >,
    /** @brief Gets the dimension of the property

\arg \c properties handle of the thing holding the property
\arg \c property string labelling the property we are resetting
\arg \c count pointer to an integer where the value is returned

@returns
- ::kOfxStatOK
- ::kOfxStatErrBadHandle
- ::kOfxStatErrUnknown*/
    pub propGetDimension: ::std::option::Option<
        unsafe extern "C" fn(
            properties: OfxPropertySetHandle,
            property: *const ::std::os::raw::c_char,
            count: *mut ::std::os::raw::c_int,
        ) -> OfxStatus,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of OfxPropertySuiteV1",
    ][::std::mem::size_of::<OfxPropertySuiteV1>() - 144usize];
    [
        "Alignment of OfxPropertySuiteV1",
    ][::std::mem::align_of::<OfxPropertySuiteV1>() - 8usize];
    [
        "Offset of field: OfxPropertySuiteV1::propSetPointer",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propSetPointer) - 0usize];
    [
        "Offset of field: OfxPropertySuiteV1::propSetString",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propSetString) - 8usize];
    [
        "Offset of field: OfxPropertySuiteV1::propSetDouble",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propSetDouble) - 16usize];
    [
        "Offset of field: OfxPropertySuiteV1::propSetInt",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propSetInt) - 24usize];
    [
        "Offset of field: OfxPropertySuiteV1::propSetPointerN",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propSetPointerN) - 32usize];
    [
        "Offset of field: OfxPropertySuiteV1::propSetStringN",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propSetStringN) - 40usize];
    [
        "Offset of field: OfxPropertySuiteV1::propSetDoubleN",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propSetDoubleN) - 48usize];
    [
        "Offset of field: OfxPropertySuiteV1::propSetIntN",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propSetIntN) - 56usize];
    [
        "Offset of field: OfxPropertySuiteV1::propGetPointer",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propGetPointer) - 64usize];
    [
        "Offset of field: OfxPropertySuiteV1::propGetString",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propGetString) - 72usize];
    [
        "Offset of field: OfxPropertySuiteV1::propGetDouble",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propGetDouble) - 80usize];
    [
        "Offset of field: OfxPropertySuiteV1::propGetInt",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propGetInt) - 88usize];
    [
        "Offset of field: OfxPropertySuiteV1::propGetPointerN",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propGetPointerN) - 96usize];
    [
        "Offset of field: OfxPropertySuiteV1::propGetStringN",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propGetStringN) - 104usize];
    [
        "Offset of field: OfxPropertySuiteV1::propGetDoubleN",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propGetDoubleN) - 112usize];
    [
        "Offset of field: OfxPropertySuiteV1::propGetIntN",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propGetIntN) - 120usize];
    [
        "Offset of field: OfxPropertySuiteV1::propReset",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propReset) - 128usize];
    [
        "Offset of field: OfxPropertySuiteV1::propGetDimension",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propGetDimension) - 136usize];
};

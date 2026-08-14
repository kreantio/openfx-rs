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
use super::param::{
    OfxBytes, OfxCustomParamInterpFuncV1, OfxParamHandle, OfxParamSetHandle,
    OfxParamSetStruct, OfxParamStruct, OfxParameterSuiteV1,
    kOfxParamCoordinatesCanonical, kOfxParamCoordinatesNormalised,
    kOfxParamDoubleTypeAbsoluteTime, kOfxParamDoubleTypeAngle, kOfxParamDoubleTypePlain,
    kOfxParamDoubleTypeScale, kOfxParamDoubleTypeTime, kOfxParamDoubleTypeX,
    kOfxParamDoubleTypeXAbsolute, kOfxParamDoubleTypeXY, kOfxParamDoubleTypeXYAbsolute,
    kOfxParamDoubleTypeY, kOfxParamDoubleTypeYAbsolute, kOfxParamHostPropMaxPages,
    kOfxParamHostPropMaxParameters, kOfxParamHostPropPageRowColumnCount,
    kOfxParamHostPropSupportsBooleanAnimation, kOfxParamHostPropSupportsChoiceAnimation,
    kOfxParamHostPropSupportsCustomAnimation, kOfxParamHostPropSupportsCustomInteract,
    kOfxParamHostPropSupportsStrChoice, kOfxParamHostPropSupportsStrChoiceAnimation,
    kOfxParamHostPropSupportsStringAnimation, kOfxParamInvalidateAll,
    kOfxParamInvalidateValueChange, kOfxParamInvalidateValueChangeToEnd,
    kOfxParamPageSkipColumn, kOfxParamPageSkipRow, kOfxParamPropAnimates,
    kOfxParamPropCacheInvalidation, kOfxParamPropCanUndo, kOfxParamPropChoiceEnum,
    kOfxParamPropChoiceOption, kOfxParamPropChoiceOrder,
    kOfxParamPropCustomInterpCallbackV1, kOfxParamPropCustomValue, kOfxParamPropDataPtr,
    kOfxParamPropDefault, kOfxParamPropDefaultCoordinateSystem, kOfxParamPropDigits,
    kOfxParamPropDimensionLabel, kOfxParamPropDisplayMax, kOfxParamPropDisplayMin,
    kOfxParamPropDoubleType, kOfxParamPropEnabled, kOfxParamPropEvaluateOnChange,
    kOfxParamPropGroupOpen, kOfxParamPropHasHostOverlayHandle, kOfxParamPropHint,
    kOfxParamPropIncrement, kOfxParamPropInteractMinimumSize,
    kOfxParamPropInteractPreferedSize, kOfxParamPropInteractSize,
    kOfxParamPropInteractSizeAspect, kOfxParamPropInteractV1,
    kOfxParamPropInterpolationAmount, kOfxParamPropInterpolationTime,
    kOfxParamPropIsAnimating, kOfxParamPropIsAutoKeying, kOfxParamPropMax,
    kOfxParamPropMin, kOfxParamPropPageChild, kOfxParamPropParent,
    kOfxParamPropPersistant, kOfxParamPropPluginMayWrite, kOfxParamPropScriptName,
    kOfxParamPropSecret, kOfxParamPropShowTimeMarker, kOfxParamPropStringFilePathExists,
    kOfxParamPropStringMode, kOfxParamPropType, kOfxParamPropUseHostOverlayHandle,
    kOfxParamStringIsDirectoryPath, kOfxParamStringIsFilePath, kOfxParamStringIsLabel,
    kOfxParamStringIsMultiLine, kOfxParamStringIsRichTextFormat,
    kOfxParamStringIsSingleLine, kOfxParamTypeBoolean, kOfxParamTypeBytes,
    kOfxParamTypeChoice, kOfxParamTypeCustom, kOfxParamTypeDouble, kOfxParamTypeDouble2D,
    kOfxParamTypeDouble3D, kOfxParamTypeGroup, kOfxParamTypeInteger,
    kOfxParamTypeInteger2D, kOfxParamTypeInteger3D, kOfxParamTypePage,
    kOfxParamTypePushButton, kOfxParamTypeRGB, kOfxParamTypeRGBA, kOfxParamTypeStrChoice,
    kOfxParamTypeString, kOfxParameterSuite, kOfxPluginPropParamPageOrder,
    kOfxPropParamSetNeedsSyncing, kOfxTypeParameter, kOfxTypeParameterInstance,
};
use super::property::{OfxPropertySuiteV1, kOfxPropertySuite};
pub const kOfxParametricParameterSuite: &::std::ffi::CStr = c"OfxParametricParameterSuite";
pub const kOfxParamTypeParametric: &::std::ffi::CStr = c"OfxParamTypeParametric";
pub const kOfxParamPropParametricDimension: &::std::ffi::CStr = c"OfxParamPropParametricDimension";
pub const kOfxParamPropParametricUIColour: &::std::ffi::CStr = c"OfxParamPropParametricUIColour";
pub const kOfxParamPropParametricInteractBackground: &::std::ffi::CStr = c"OfxParamPropParametricInteractBackground";
pub const kOfxParamHostPropSupportsParametricAnimation: &::std::ffi::CStr = c"OfxParamHostPropSupportsParametricAnimation";
pub const kOfxParamPropParametricRange: &::std::ffi::CStr = c"OfxParamPropParametricRange";
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxBytes"][::std::mem::size_of::<OfxBytes>() - 16usize];
    ["Alignment of OfxBytes"][::std::mem::align_of::<OfxBytes>() - 8usize];
    ["Offset of field: OfxBytes::data"][::std::mem::offset_of!(OfxBytes, data) - 0usize];
    [
        "Offset of field: OfxBytes::length",
    ][::std::mem::offset_of!(OfxBytes, length) - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of OfxParameterSuiteV1",
    ][::std::mem::size_of::<OfxParameterSuiteV1>() - 144usize];
    [
        "Alignment of OfxParameterSuiteV1",
    ][::std::mem::align_of::<OfxParameterSuiteV1>() - 8usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramDefine",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramDefine) - 0usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramGetHandle",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramGetHandle) - 8usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramSetGetPropertySet",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramSetGetPropertySet) - 16usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramGetPropertySet",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramGetPropertySet) - 24usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramGetValue",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramGetValue) - 32usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramGetValueAtTime",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramGetValueAtTime) - 40usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramGetDerivative",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramGetDerivative) - 48usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramGetIntegral",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramGetIntegral) - 56usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramSetValue",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramSetValue) - 64usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramSetValueAtTime",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramSetValueAtTime) - 72usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramGetNumKeys",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramGetNumKeys) - 80usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramGetKeyTime",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramGetKeyTime) - 88usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramGetKeyIndex",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramGetKeyIndex) - 96usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramDeleteKey",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramDeleteKey) - 104usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramDeleteAllKeys",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramDeleteAllKeys) - 112usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramCopy",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramCopy) - 120usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramEditBegin",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramEditBegin) - 128usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramEditEnd",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramEditEnd) - 136usize];
};
/** @brief The OFX suite used to define and manipulate 'parametric' parameters.

This is an optional suite.

Parametric parameters are in effect 'functions' a plug-in can ask a host to arbitrarily
evaluate for some value 'x'. A classic use case would be for constructing look-up tables,
a plug-in would ask the host to evaluate one at multiple values from 0 to 1 and use that
to fill an array.

A host would probably represent this to a user as a cubic curve in a standard curve editor
interface, or possibly through scripting. The user would then use this to define the 'shape'
of the parameter.

The evaluation of such params is not the same as animation, they are returning values based
on some arbitrary argument orthogonal to time, so to evaluate such a param, you need to pass
a parametric position and time.

Often, you would want such a parametric parameter to be multi-dimensional, for example, a
colour look-up table might want three values, one for red, green and blue. Rather than
declare three separate parametric parameters, it would be better to have one such parameter
with multiple values in it.

The major complication with these parameters is how to allow a plug-in to set values, and
defaults. The default default value of a parametric curve is to be an identity lookup. If
a plugin wishes to set a different default value for a curve, it can use the suite to set
key/value pairs on the \em descriptor of the param. When a new instance is made, it will
have these curve values as a default.*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxParametricParameterSuiteV1 {
    /** @brief Evaluates a parametric parameter

\arg \c param                 handle to the parametric parameter
\arg \c curveIndex            which dimension to evaluate
\arg \c time                  the time to evaluate to the parametric param at
\arg \c parametricPosition    the position to evaluate the parametric param at
\arg \c returnValue           pointer to a double where a value is returned

@returns
- ::kOfxStatOK            - all was fine
- ::kOfxStatErrBadHandle  - if the parameter handle was invalid
- ::kOfxStatErrBadIndex   - the curve index was invalid*/
    pub parametricParamGetValue: ::std::option::Option<
        unsafe extern "C" fn(
            param: OfxParamHandle,
            curveIndex: ::std::os::raw::c_int,
            time: OfxTime,
            parametricPosition: f64,
            returnValue: *mut f64,
        ) -> OfxStatus,
    >,
    /** @brief Returns the number of control points in the parametric param.

\arg \c param                 handle to the parametric parameter
\arg \c curveIndex            which dimension to check
\arg \c time                  the time to check
\arg \c returnValue           pointer to an integer where the value is returned.

@returns
- ::kOfxStatOK            - all was fine
- ::kOfxStatErrBadHandle  - if the parameter handle was invalid
- ::kOfxStatErrBadIndex   - the curve index was invalid*/
    pub parametricParamGetNControlPoints: ::std::option::Option<
        unsafe extern "C" fn(
            param: OfxParamHandle,
            curveIndex: ::std::os::raw::c_int,
            time: f64,
            returnValue: *mut ::std::os::raw::c_int,
        ) -> OfxStatus,
    >,
    /** @brief Returns the key/value pair of the nth control point.

\arg \c param                 handle to the parametric parameter
\arg \c curveIndex            which dimension to check
\arg \c time                  the time to check
\arg \c nthCtl                the nth control point to get the value of
\arg \c key                   pointer to a double where the key will be returned
\arg \c value                 pointer to a double where the value will be returned

@returns
- ::kOfxStatOK            - all was fine
- ::kOfxStatErrBadHandle  - if the parameter handle was invalid
- ::kOfxStatErrUnknown    - if the type is unknown*/
    pub parametricParamGetNthControlPoint: ::std::option::Option<
        unsafe extern "C" fn(
            param: OfxParamHandle,
            curveIndex: ::std::os::raw::c_int,
            time: f64,
            nthCtl: ::std::os::raw::c_int,
            key: *mut f64,
            value: *mut f64,
        ) -> OfxStatus,
    >,
    /** @brief Modifies an existing control point on a curve

\arg \c param                 handle to the parametric parameter
\arg \c curveIndex            which dimension to set
\arg \c time                  the time to set the value at
\arg \c nthCtl                the control point to modify
\arg \c key                   key of the control point
\arg \c value                 value of the control point
\arg \c addAnimationKey       if the param is an animatable, setting this to true will
force an animation keyframe to be set as well as a curve key,
otherwise if false, a key will only be added if the curve is already
animating.

@returns
- ::kOfxStatOK            - all was fine
- ::kOfxStatErrBadHandle  - if the parameter handle was invalid
- ::kOfxStatErrUnknown    - if the type is unknown

This modifies an existing control point. Note that by changing key, the order of the
control point may be modified (as you may move it before or after anther point). So be
careful when iterating over a curves control points and you change a key.*/
    pub parametricParamSetNthControlPoint: ::std::option::Option<
        unsafe extern "C" fn(
            param: OfxParamHandle,
            curveIndex: ::std::os::raw::c_int,
            time: f64,
            nthCtl: ::std::os::raw::c_int,
            key: f64,
            value: f64,
            addAnimationKey: bool,
        ) -> OfxStatus,
    >,
    /** @brief Adds a control point to the curve.

\arg \c param                 handle to the parametric parameter
\arg \c curveIndex            which dimension to set
\arg \c time                  the time to set the value at
\arg \c key                   key of the control point
\arg \c value                 value of the control point
\arg \c addAnimationKey       if the param is an animatable, setting this to true will
force an animation keyframe to be set as well as a curve key,
otherwise if false, a key will only be added if the curve is already
animating.

@returns
- ::kOfxStatOK            - all was fine
- ::kOfxStatErrBadHandle  - if the parameter handle was invalid
- ::kOfxStatErrUnknown    - if the type is unknown

This will add a new control point to the given dimension of a parametric parameter. If a key exists
sufficiently close to 'key', then it will be set to the indicated control point.*/
    pub parametricParamAddControlPoint: ::std::option::Option<
        unsafe extern "C" fn(
            param: OfxParamHandle,
            curveIndex: ::std::os::raw::c_int,
            time: f64,
            key: f64,
            value: f64,
            addAnimationKey: bool,
        ) -> OfxStatus,
    >,
    /** @brief Deletes the nth control point from a parametric param.

\arg \c param                 handle to the parametric parameter
\arg \c curveIndex            which dimension to delete
\arg \c nthCtl                the control point to delete*/
    pub parametricParamDeleteControlPoint: ::std::option::Option<
        unsafe extern "C" fn(
            param: OfxParamHandle,
            curveIndex: ::std::os::raw::c_int,
            nthCtl: ::std::os::raw::c_int,
        ) -> OfxStatus,
    >,
    /** @brief Delete all curve control points on the given param.

\arg \c param                 handle to the parametric parameter
\arg \c curveIndex            which dimension to clear*/
    pub parametricParamDeleteAllControlPoints: ::std::option::Option<
        unsafe extern "C" fn(
            param: OfxParamHandle,
            curveIndex: ::std::os::raw::c_int,
        ) -> OfxStatus,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of OfxParametricParameterSuiteV1",
    ][::std::mem::size_of::<OfxParametricParameterSuiteV1>() - 56usize];
    [
        "Alignment of OfxParametricParameterSuiteV1",
    ][::std::mem::align_of::<OfxParametricParameterSuiteV1>() - 8usize];
    [
        "Offset of field: OfxParametricParameterSuiteV1::parametricParamGetValue",
    ][::std::mem::offset_of!(OfxParametricParameterSuiteV1, parametricParamGetValue)
        - 0usize];
    [
        "Offset of field: OfxParametricParameterSuiteV1::parametricParamGetNControlPoints",
    ][::std::mem::offset_of!(
        OfxParametricParameterSuiteV1, parametricParamGetNControlPoints
    ) - 8usize];
    [
        "Offset of field: OfxParametricParameterSuiteV1::parametricParamGetNthControlPoint",
    ][::std::mem::offset_of!(
        OfxParametricParameterSuiteV1, parametricParamGetNthControlPoint
    ) - 16usize];
    [
        "Offset of field: OfxParametricParameterSuiteV1::parametricParamSetNthControlPoint",
    ][::std::mem::offset_of!(
        OfxParametricParameterSuiteV1, parametricParamSetNthControlPoint
    ) - 24usize];
    [
        "Offset of field: OfxParametricParameterSuiteV1::parametricParamAddControlPoint",
    ][::std::mem::offset_of!(
        OfxParametricParameterSuiteV1, parametricParamAddControlPoint
    ) - 32usize];
    [
        "Offset of field: OfxParametricParameterSuiteV1::parametricParamDeleteControlPoint",
    ][::std::mem::offset_of!(
        OfxParametricParameterSuiteV1, parametricParamDeleteControlPoint
    ) - 40usize];
    [
        "Offset of field: OfxParametricParameterSuiteV1::parametricParamDeleteAllControlPoints",
    ][::std::mem::offset_of!(
        OfxParametricParameterSuiteV1, parametricParamDeleteAllControlPoints
    ) - 48usize];
};

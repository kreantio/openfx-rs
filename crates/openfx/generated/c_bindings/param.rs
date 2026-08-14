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
use super::property::{OfxPropertySuiteV1, kOfxPropertySuite};
pub const kOfxParameterSuite: &::std::ffi::CStr = c"OfxParameterSuite";
pub const kOfxTypeParameter: &::std::ffi::CStr = c"OfxTypeParameter";
pub const kOfxTypeParameterInstance: &::std::ffi::CStr = c"OfxTypeParameterInstance";
pub const kOfxParamTypeInteger: &::std::ffi::CStr = c"OfxParamTypeInteger";
pub const kOfxParamTypeDouble: &::std::ffi::CStr = c"OfxParamTypeDouble";
pub const kOfxParamTypeBoolean: &::std::ffi::CStr = c"OfxParamTypeBoolean";
pub const kOfxParamTypeChoice: &::std::ffi::CStr = c"OfxParamTypeChoice";
pub const kOfxParamTypeStrChoice: &::std::ffi::CStr = c"OfxParamTypeStrChoice";
pub const kOfxParamTypeRGBA: &::std::ffi::CStr = c"OfxParamTypeRGBA";
pub const kOfxParamTypeRGB: &::std::ffi::CStr = c"OfxParamTypeRGB";
pub const kOfxParamTypeDouble2D: &::std::ffi::CStr = c"OfxParamTypeDouble2D";
pub const kOfxParamTypeInteger2D: &::std::ffi::CStr = c"OfxParamTypeInteger2D";
pub const kOfxParamTypeDouble3D: &::std::ffi::CStr = c"OfxParamTypeDouble3D";
pub const kOfxParamTypeInteger3D: &::std::ffi::CStr = c"OfxParamTypeInteger3D";
pub const kOfxParamTypeString: &::std::ffi::CStr = c"OfxParamTypeString";
pub const kOfxParamTypeCustom: &::std::ffi::CStr = c"OfxParamTypeCustom";
pub const kOfxParamTypeBytes: &::std::ffi::CStr = c"OfxParamTypeBytes";
pub const kOfxParamTypeGroup: &::std::ffi::CStr = c"OfxParamTypeGroup";
pub const kOfxParamTypePage: &::std::ffi::CStr = c"OfxParamTypePage";
pub const kOfxParamTypePushButton: &::std::ffi::CStr = c"OfxParamTypePushButton";
pub const kOfxParamHostPropSupportsCustomAnimation: &::std::ffi::CStr = c"OfxParamHostPropSupportsCustomAnimation";
pub const kOfxParamHostPropSupportsStringAnimation: &::std::ffi::CStr = c"OfxParamHostPropSupportsStringAnimation";
pub const kOfxParamHostPropSupportsBooleanAnimation: &::std::ffi::CStr = c"OfxParamHostPropSupportsBooleanAnimation";
pub const kOfxParamHostPropSupportsChoiceAnimation: &::std::ffi::CStr = c"OfxParamHostPropSupportsChoiceAnimation";
pub const kOfxParamHostPropSupportsCustomInteract: &::std::ffi::CStr = c"OfxParamHostPropSupportsCustomInteract";
pub const kOfxParamHostPropMaxParameters: &::std::ffi::CStr = c"OfxParamHostPropMaxParameters";
pub const kOfxParamHostPropMaxPages: &::std::ffi::CStr = c"OfxParamHostPropMaxPages";
pub const kOfxParamHostPropPageRowColumnCount: &::std::ffi::CStr = c"OfxParamHostPropPageRowColumnCount";
pub const kOfxParamPageSkipRow: &::std::ffi::CStr = c"OfxParamPageSkipRow";
pub const kOfxParamPageSkipColumn: &::std::ffi::CStr = c"OfxParamPageSkipColumn";
pub const kOfxParamPropInteractV1: &::std::ffi::CStr = c"OfxParamPropInteractV1";
pub const kOfxParamPropInteractSize: &::std::ffi::CStr = c"OfxParamPropInteractSize";
pub const kOfxParamPropInteractSizeAspect: &::std::ffi::CStr = c"OfxParamPropInteractSizeAspect";
pub const kOfxParamPropInteractMinimumSize: &::std::ffi::CStr = c"OfxParamPropInteractMinimumSize";
pub const kOfxParamPropInteractPreferedSize: &::std::ffi::CStr = c"OfxParamPropInteractPreferedSize";
pub const kOfxParamPropType: &::std::ffi::CStr = c"OfxParamPropType";
pub const kOfxParamPropAnimates: &::std::ffi::CStr = c"OfxParamPropAnimates";
pub const kOfxParamPropCanUndo: &::std::ffi::CStr = c"OfxParamPropCanUndo";
pub const kOfxPropParamSetNeedsSyncing: &::std::ffi::CStr = c"OfxPropParamSetNeedsSyncing";
pub const kOfxParamPropIsAnimating: &::std::ffi::CStr = c"OfxParamPropIsAnimating";
pub const kOfxParamPropPluginMayWrite: &::std::ffi::CStr = c"OfxParamPropPluginMayWrite";
pub const kOfxParamPropPersistant: &::std::ffi::CStr = c"OfxParamPropPersistant";
pub const kOfxParamPropEvaluateOnChange: &::std::ffi::CStr = c"OfxParamPropEvaluateOnChange";
pub const kOfxParamPropSecret: &::std::ffi::CStr = c"OfxParamPropSecret";
pub const kOfxParamPropScriptName: &::std::ffi::CStr = c"OfxParamPropScriptName";
pub const kOfxParamPropCacheInvalidation: &::std::ffi::CStr = c"OfxParamPropCacheInvalidation";
pub const kOfxParamInvalidateValueChange: &::std::ffi::CStr = c"OfxParamInvalidateValueChange";
pub const kOfxParamInvalidateValueChangeToEnd: &::std::ffi::CStr = c"OfxParamInvalidateValueChangeToEnd";
pub const kOfxParamInvalidateAll: &::std::ffi::CStr = c"OfxParamInvalidateAll";
pub const kOfxParamPropHint: &::std::ffi::CStr = c"OfxParamPropHint";
pub const kOfxParamPropDefault: &::std::ffi::CStr = c"OfxParamPropDefault";
pub const kOfxParamPropDoubleType: &::std::ffi::CStr = c"OfxParamPropDoubleType";
pub const kOfxParamDoubleTypePlain: &::std::ffi::CStr = c"OfxParamDoubleTypePlain";
pub const kOfxParamDoubleTypeScale: &::std::ffi::CStr = c"OfxParamDoubleTypeScale";
pub const kOfxParamDoubleTypeAngle: &::std::ffi::CStr = c"OfxParamDoubleTypeAngle";
pub const kOfxParamDoubleTypeTime: &::std::ffi::CStr = c"OfxParamDoubleTypeTime";
pub const kOfxParamDoubleTypeAbsoluteTime: &::std::ffi::CStr = c"OfxParamDoubleTypeAbsoluteTime";
pub const kOfxParamDoubleTypeX: &::std::ffi::CStr = c"OfxParamDoubleTypeX";
pub const kOfxParamDoubleTypeY: &::std::ffi::CStr = c"OfxParamDoubleTypeY";
pub const kOfxParamDoubleTypeXAbsolute: &::std::ffi::CStr = c"OfxParamDoubleTypeXAbsolute";
pub const kOfxParamDoubleTypeYAbsolute: &::std::ffi::CStr = c"OfxParamDoubleTypeYAbsolute";
pub const kOfxParamDoubleTypeXY: &::std::ffi::CStr = c"OfxParamDoubleTypeXY";
pub const kOfxParamDoubleTypeXYAbsolute: &::std::ffi::CStr = c"OfxParamDoubleTypeXYAbsolute";
pub const kOfxParamPropDefaultCoordinateSystem: &::std::ffi::CStr = c"OfxParamPropDefaultCoordinateSystem";
pub const kOfxParamCoordinatesCanonical: &::std::ffi::CStr = c"OfxParamCoordinatesCanonical";
pub const kOfxParamCoordinatesNormalised: &::std::ffi::CStr = c"OfxParamCoordinatesNormalised";
pub const kOfxParamPropHasHostOverlayHandle: &::std::ffi::CStr = c"OfxParamPropHasHostOverlayHandle";
pub const kOfxParamPropUseHostOverlayHandle: &::std::ffi::CStr = c"kOfxParamPropUseHostOverlayHandle";
pub const kOfxParamInterpTypeConstantStep: &::std::ffi::CStr = c"OfxParamInterpTypeConstantStep";
pub const kOfxParamInterpTypeLinear: &::std::ffi::CStr = c"OfxParamInterpTypeLinear";
pub const kOfxParamInterpTypeSmooth: &::std::ffi::CStr = c"OfxParamInterpTypeSmooth";
pub const kOfxParamInterpType: &::std::ffi::CStr = c"OfxParamInterpType";
pub const kOfxParamPropShowTimeMarker: &::std::ffi::CStr = c"OfxParamPropShowTimeMarker";
pub const kOfxPluginPropParamPageOrder: &::std::ffi::CStr = c"OfxPluginPropParamPageOrder";
pub const kOfxParamPropPageChild: &::std::ffi::CStr = c"OfxParamPropPageChild";
pub const kOfxParamPropParent: &::std::ffi::CStr = c"OfxParamPropParent";
pub const kOfxParamPropGroupOpen: &::std::ffi::CStr = c"OfxParamPropGroupOpen";
pub const kOfxParamPropEnabled: &::std::ffi::CStr = c"OfxParamPropEnabled";
pub const kOfxParamPropDataPtr: &::std::ffi::CStr = c"OfxParamPropDataPtr";
pub const kOfxParamPropChoiceOption: &::std::ffi::CStr = c"OfxParamPropChoiceOption";
pub const kOfxParamPropChoiceOrder: &::std::ffi::CStr = c"OfxParamPropChoiceOrder";
pub const kOfxParamPropChoiceEnum: &::std::ffi::CStr = c"OfxParamPropChoiceEnum";
pub const kOfxParamHostPropSupportsStrChoiceAnimation: &::std::ffi::CStr = c"OfxParamHostPropSupportsStrChoiceAnimation";
pub const kOfxParamHostPropSupportsStrChoice: &::std::ffi::CStr = c"OfxParamHostPropSupportsStrChoice";
pub const kOfxParamPropMin: &::std::ffi::CStr = c"OfxParamPropMin";
pub const kOfxParamPropMax: &::std::ffi::CStr = c"OfxParamPropMax";
pub const kOfxParamPropDisplayMin: &::std::ffi::CStr = c"OfxParamPropDisplayMin";
pub const kOfxParamPropDisplayMax: &::std::ffi::CStr = c"OfxParamPropDisplayMax";
pub const kOfxParamPropIncrement: &::std::ffi::CStr = c"OfxParamPropIncrement";
pub const kOfxParamPropDigits: &::std::ffi::CStr = c"OfxParamPropDigits";
pub const kOfxParamPropDimensionLabel: &::std::ffi::CStr = c"OfxParamPropDimensionLabel";
pub const kOfxParamPropIsAutoKeying: &::std::ffi::CStr = c"OfxParamPropIsAutoKeying";
pub const kOfxParamPropCustomInterpCallbackV1: &::std::ffi::CStr = c"OfxParamPropCustomCallbackV1";
pub const kOfxParamPropStringMode: &::std::ffi::CStr = c"OfxParamPropStringMode";
pub const kOfxParamPropStringFilePathExists: &::std::ffi::CStr = c"OfxParamPropStringFilePathExists";
pub const kOfxParamStringIsSingleLine: &::std::ffi::CStr = c"OfxParamStringIsSingleLine";
pub const kOfxParamStringIsMultiLine: &::std::ffi::CStr = c"OfxParamStringIsMultiLine";
pub const kOfxParamStringIsFilePath: &::std::ffi::CStr = c"OfxParamStringIsFilePath";
pub const kOfxParamStringIsDirectoryPath: &::std::ffi::CStr = c"OfxParamStringIsDirectoryPath";
pub const kOfxParamStringIsLabel: &::std::ffi::CStr = c"OfxParamStringIsLabel";
pub const kOfxParamStringIsRichTextFormat: &::std::ffi::CStr = c"OfxParamStringIsRichTextFormat";
pub const kOfxParamPropCustomValue: &::std::ffi::CStr = c"OfxParamPropCustomValue";
pub const kOfxParamPropInterpolationTime: &::std::ffi::CStr = c"OfxParamPropInterpolationTime";
pub const kOfxParamPropInterpolationAmount: &::std::ffi::CStr = c"OfxParamPropInterpolationAmount";
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
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxParamStruct {
    _unused: [u8; 0],
}
/// @brief Blind declaration of an OFX param
pub type OfxParamHandle = *mut OfxParamStruct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxParamSetStruct {
    _unused: [u8; 0],
}
/** @brief Blind declaration of an OFX parameter set
*/
pub type OfxParamSetHandle = *mut OfxParamSetStruct;
/// @brief Provides information for a parameter of type kOfxParamTypeBytes
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxBytes {
    /// @brief a pointer to the data buffer
    pub data: *const ::std::os::raw::c_uchar,
    /// @brief the length of the data buffer, in bytes
    pub length: usize,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxBytes"][::std::mem::size_of::<OfxBytes>() - 16usize];
    ["Alignment of OfxBytes"][::std::mem::align_of::<OfxBytes>() - 8usize];
    ["Offset of field: OfxBytes::data"][::std::mem::offset_of!(OfxBytes, data) - 0usize];
    [
        "Offset of field: OfxBytes::length",
    ][::std::mem::offset_of!(OfxBytes, length) - 8usize];
};
/** @brief Function prototype for custom parameter interpolation callback functions

\arg \c instance    the plugin instance that this parameter occurs in
\arg \c inArgs      handle holding the following properties...
- kOfxPropName - the name of the custom parameter to interpolate
- kOfxPropTime - absolute time the interpolation is occurring at
- kOfxParamPropCustomValue - string property that gives the value of the two keyframes to interpolate, in this case 2D
- kOfxParamPropInterpolationTime - 2D double property that gives the time of the two keyframes we are interpolating
- kOfxParamPropInterpolationAmount - 1D double property indicating how much to interpolate between the two keyframes

\arg \c outArgs     handle holding the following properties to be set
- kOfxParamPropCustomValue - the value of the interpolated custom parameter, in this case 1D

This function allows custom parameters to animate by performing interpolation between keys.

The plugin needs to parse the two strings encoding keyframes on either side of the time
we need a value for. It should then interpolate a new value for it, encode it into a string and set
the ::kOfxParamPropCustomValue property with this on the outArgs handle.

The interp value is a linear interpolation amount, however his may be derived from a cubic (or other) curve.

@actiondef
inArgs:
- OfxParamPropCustomValue
- OfxParamPropInterpolationTime
- OfxParamPropInterpolationAmount
outArgs:
- OfxParamPropCustomValue
- OfxParamPropInterpolationTime*/
pub type OfxCustomParamInterpFuncV1 = ::std::option::Option<
    unsafe extern "C" fn(
        instance: OfxParamSetHandle,
        inArgs: OfxPropertySetHandle,
        outArgs: OfxPropertySetHandle,
    ) -> OfxStatus,
>;
/// @brief The OFX suite used to define and manipulate user visible parameters
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxParameterSuiteV1 {
    /** @brief Defines a new parameter of the given type in a describe action

\arg \c paramSet    handle to the parameter set descriptor that will hold this parameter
\arg \c paramType   type of the parameter to create, one of the kOfxParamType* #defines
\arg \c name        unique name of the parameter
\arg \c propertySet if not null, a pointer to the parameter descriptor's property set will be placed here.

This function defines a parameter in a parameter set and returns a property set which is used to describe that parameter.

This function does not actually create a parameter, it only says that one should exist in any subsequent instances. To fetch an
parameter instance paramGetHandle must be called on an instance.

This function can always be called in one of a plug-in's "describe" functions which defines the parameter sets common to all instances of a plugin.

@returns
- ::kOfxStatOK             - the parameter was created correctly
- ::kOfxStatErrBadHandle   - if the plugin handle was invalid
- ::kOfxStatErrExists      - if a parameter of that name exists already in this plugin
- ::kOfxStatErrUnknown     - if the type is unknown
- ::kOfxStatErrUnsupported - if the type is known but unsupported*/
    pub paramDefine: ::std::option::Option<
        unsafe extern "C" fn(
            paramSet: OfxParamSetHandle,
            paramType: *const ::std::os::raw::c_char,
            name: *const ::std::os::raw::c_char,
            propertySet: *mut OfxPropertySetHandle,
        ) -> OfxStatus,
    >,
    /** @brief Retrieves the handle for a parameter in a given parameter set

\arg \c paramSet    instance of the plug-in to fetch the property handle from
\arg \c name        parameter to ask about
\arg \c param       pointer to a param handle, the value is returned here
\arg \c propertySet if not null, a pointer to the parameter's property set will be placed here.

Parameter handles retrieved from an instance are always distinct in each instance. The parameter handle is valid for the life-time of the instance. Parameter handles in instances are distinct from parameter handles in plugins. You cannot call this in a plugin's describe function, as it needs an instance to work on.

@returns
- ::kOfxStatOK       - the parameter was found and returned
- ::kOfxStatErrBadHandle  - if the plugin handle was invalid
- ::kOfxStatErrUnknown    - if the type is unknown*/
    pub paramGetHandle: ::std::option::Option<
        unsafe extern "C" fn(
            paramSet: OfxParamSetHandle,
            name: *const ::std::os::raw::c_char,
            param: *mut OfxParamHandle,
            propertySet: *mut OfxPropertySetHandle,
        ) -> OfxStatus,
    >,
    /** @brief Retrieves the property set handle for the given parameter set

\arg \c paramSet    parameter set to get the property set for
\arg \c propHandle  pointer to a the property set handle, value is returedn her

\note The property handle belonging to a parameter set is the same as the property handle belonging to the plugin instance.

@returns
- ::kOfxStatOK       - the property set was found and returned
- ::kOfxStatErrBadHandle  - if the parameter handle was invalid
- ::kOfxStatErrUnknown    - if the type is unknown*/
    pub paramSetGetPropertySet: ::std::option::Option<
        unsafe extern "C" fn(
            paramSet: OfxParamSetHandle,
            propHandle: *mut OfxPropertySetHandle,
        ) -> OfxStatus,
    >,
    /** @brief Retrieves the property set handle for the given parameter

\arg \c param       parameter to get the property set for
\arg \c propHandle  pointer to a the property set handle, value is returedn her

The property handle is valid for the lifetime of the parameter, which is the lifetime of the instance that owns the parameter

@returns
- ::kOfxStatOK       - the property set was found and returned
- ::kOfxStatErrBadHandle  - if the parameter handle was invalid
- ::kOfxStatErrUnknown    - if the type is unknown*/
    pub paramGetPropertySet: ::std::option::Option<
        unsafe extern "C" fn(
            param: OfxParamHandle,
            propHandle: *mut OfxPropertySetHandle,
        ) -> OfxStatus,
    >,
    /** @brief Gets the current value of a parameter,

\arg \c paramHandle parameter handle to fetch value from
\arg \c ...         one or more pointers to variables of the relevant type to hold the parameter's value

This gets the current value of a parameter. The varargs ... argument needs to be pointer to C variables
of the relevant type for this parameter. Note that params with multiple values (eg Colour) take
multiple args here. For example...

@verbatim
OfxParamHandle myDoubleParam, *myColourParam;
ofxHost->paramGetHandle(instance, "myDoubleParam", &myDoubleParam);
double myDoubleValue;
ofxHost->paramGetValue(myDoubleParam, &myDoubleValue);
ofxHost->paramGetHandle(instance, "myColourParam", &myColourParam);
double myR, myG, myB;
ofxHost->paramGetValue(myColourParam, &myR, &myG, &myB);
@endverbatim

\note \c paramGetValue should only be called from within a ::kOfxActionInstanceChanged or interact action and never from the render actions (which should always use paramGetValueAtTime).

@returns
- ::kOfxStatOK       - all was OK
- ::kOfxStatErrBadHandle  - if the parameter handle was invalid*/
    pub paramGetValue: ::std::option::Option<
        unsafe extern "C" fn(paramHandle: OfxParamHandle, ...) -> OfxStatus,
    >,
    /** @brief Gets the value of a parameter at a specific time.

\arg \c paramHandle parameter handle to fetch value from
\arg \c time        at what point in time to look up the parameter
\arg \c ...         one or more pointers to variables of the relevant type to hold the parameter's value

This gets the current value of a parameter. The varargs needs to be pointer to C variables
of the relevant type for this parameter. See OfxParameterSuiteV1::paramGetValue for notes on
the varags list

@returns
- ::kOfxStatOK       - all was OK
- ::kOfxStatErrBadHandle  - if the parameter handle was invalid*/
    pub paramGetValueAtTime: ::std::option::Option<
        unsafe extern "C" fn(
            paramHandle: OfxParamHandle,
            time: OfxTime,
            ...
        ) -> OfxStatus,
    >,
    /** @brief Gets the derivative of a parameter at a specific time.

\arg \c paramHandle parameter handle to fetch value from
\arg \c time        at what point in time to look up the parameter
\arg \c ...         one or more pointers to variables of the relevant type to hold the parameter's derivative

This gets the derivative of the parameter at the indicated time.

The varargs needs to be pointer to C variables
of the relevant type for this parameter. See OfxParameterSuiteV1::paramGetValue for notes on
the varags list.

Only double and colour params can have their derivatives found.

@returns
- ::kOfxStatOK       - all was OK
- ::kOfxStatErrBadHandle  - if the parameter handle was invalid*/
    pub paramGetDerivative: ::std::option::Option<
        unsafe extern "C" fn(
            paramHandle: OfxParamHandle,
            time: OfxTime,
            ...
        ) -> OfxStatus,
    >,
    /** @brief Gets the integral of a parameter over a specific time range,

\arg \c paramHandle parameter handle to fetch integral from
\arg \c time1       where to start evaluating the integral
\arg \c time2       where to stop evaluating the integral
\arg \c ...         one or more pointers to variables of the relevant type to hold the parameter's integral

This gets the integral of the parameter over the specified time range.

The varargs needs to be pointer to C variables
of the relevant type for this parameter. See OfxParameterSuiteV1::paramGetValue for notes on
the varags list.

Only double and colour params can be integrated.

@returns
- ::kOfxStatOK       - all was OK
- ::kOfxStatErrBadHandle  - if the parameter handle was invalid*/
    pub paramGetIntegral: ::std::option::Option<
        unsafe extern "C" fn(
            paramHandle: OfxParamHandle,
            time1: OfxTime,
            time2: OfxTime,
            ...
        ) -> OfxStatus,
    >,
    /** @brief Sets the current value of a parameter

\arg \c paramHandle parameter handle to set value in
\arg \c ...         one or more variables of the relevant type to hold the parameter's value

This sets the current value of a parameter. The varargs ... argument needs to be values
of the relevant type for this parameter. Note that params with multiple values (eg Colour) take
multiple args here. For example...
@verbatim
ofxHost->paramSetValue(instance, "myDoubleParam", double(10));
ofxHost->paramSetValue(instance, "myColourParam", double(pix.r), double(pix.g), double(pix.b));
@endverbatim

\note \c paramSetValue should only be called from within a ::kOfxActionInstanceChanged or interact action.

@returns
- ::kOfxStatOK       - all was OK
- ::kOfxStatErrBadHandle  - if the parameter handle was invalid*/
    pub paramSetValue: ::std::option::Option<
        unsafe extern "C" fn(paramHandle: OfxParamHandle, ...) -> OfxStatus,
    >,
    /** @brief Keyframes the value of a parameter at a specific time.

\arg \c paramHandle parameter handle to set value in
\arg \c time        at what point in time to set the keyframe
\arg \c ...         one or more variables of the relevant type to hold the parameter's value

This sets a keyframe in the parameter at the indicated time to have the indicated value.
The varargs ... argument needs to be values of the relevant type for this parameter. See the note on
OfxParameterSuiteV1::paramSetValue for more detail

\note \c paramSetValueAtTime should only be called from within a ::kOfxActionInstanceChanged or interact action.

V1.3: This function can be called the ::kOfxActionInstanceChanged action and during image effect analysis render passes.
V1.4: This function can be called the ::kOfxActionInstanceChanged action
@returns
- ::kOfxStatOK       - all was OK
- ::kOfxStatErrBadHandle  - if the parameter handle was invalid*/
    pub paramSetValueAtTime: ::std::option::Option<
        unsafe extern "C" fn(
            paramHandle: OfxParamHandle,
            time: OfxTime,
            ...
        ) -> OfxStatus,
    >,
    /** @brief Returns the number of keyframes in the parameter

\arg \c paramHandle parameter handle to interrogate
\arg \c numberOfKeys pointer to integer where the return value is placed

V1.3: This function can be called the ::kOfxActionInstanceChanged action and during image effect analysis render passes.
V1.4: This function can be called the ::kOfxActionInstanceChanged action

Returns the number of keyframes in the parameter.

@returns
- ::kOfxStatOK       - all was OK
- ::kOfxStatErrBadHandle  - if the parameter handle was invalid*/
    pub paramGetNumKeys: ::std::option::Option<
        unsafe extern "C" fn(
            paramHandle: OfxParamHandle,
            numberOfKeys: *mut ::std::os::raw::c_uint,
        ) -> OfxStatus,
    >,
    /** @brief Returns the time of the nth key

\arg \c paramHandle parameter handle to interrogate
\arg \c nthKey      which key to ask about (0 to paramGetNumKeys -1), ordered by time
\arg \c time	   pointer to OfxTime where the return value is placed

@returns
- ::kOfxStatOK       - all was OK
- ::kOfxStatErrBadHandle  - if the parameter handle was invalid
- ::kOfxStatErrBadIndex   - the nthKey does not exist*/
    pub paramGetKeyTime: ::std::option::Option<
        unsafe extern "C" fn(
            paramHandle: OfxParamHandle,
            nthKey: ::std::os::raw::c_uint,
            time: *mut OfxTime,
        ) -> OfxStatus,
    >,
    /** @brief Finds the index of a keyframe at/before/after a specified time.

\arg \c paramHandle parameter handle to search
\arg \c time        what time to search from
\arg \c direction
- == 0 indicates search for a key at the indicated time (some small delta)
- > 0 indicates search for the next key after the indicated time
- < 0 indicates search for the previous key before the indicated time
\arg \c index	   pointer to an integer which in which the index is returned set to -1 if no key was found

@returns
- ::kOfxStatOK            - all was OK
- ::kOfxStatFailed        - if the search failed to find a key
- ::kOfxStatErrBadHandle  - if the parameter handle was invalid*/
    pub paramGetKeyIndex: ::std::option::Option<
        unsafe extern "C" fn(
            paramHandle: OfxParamHandle,
            time: OfxTime,
            direction: ::std::os::raw::c_int,
            index: *mut ::std::os::raw::c_int,
        ) -> OfxStatus,
    >,
    /** @brief Deletes a keyframe if one exists at the given time.

\arg \c paramHandle parameter handle to delete the key from
\arg \c time        time at which a keyframe is

@returns
- ::kOfxStatOK       - all was OK
- ::kOfxStatErrBadHandle  - if the parameter handle was invalid
- ::kOfxStatErrBadIndex   - no key at the given time*/
    pub paramDeleteKey: ::std::option::Option<
        unsafe extern "C" fn(paramHandle: OfxParamHandle, time: OfxTime) -> OfxStatus,
    >,
    /** @brief Deletes all keyframes from a parameter.

\arg \c paramHandle parameter handle to delete the keys from
\arg \c name        parameter to delete the keyframes from is

V1.3: This function can be called the ::kOfxActionInstanceChanged action and during image effect analysis render passes.
V1.4: This function can be called the ::kOfxActionInstanceChanged action

@returns
- ::kOfxStatOK       - all was OK
- ::kOfxStatErrBadHandle  - if the parameter handle was invalid*/
    pub paramDeleteAllKeys: ::std::option::Option<
        unsafe extern "C" fn(paramHandle: OfxParamHandle) -> OfxStatus,
    >,
    /** @brief Copies one parameter to another, including any animation etc...

\arg \c paramTo     parameter to set
\arg \c paramFrom   parameter to copy from
\arg \c dstOffset   temporal offset to apply to keys when writing to the paramTo
\arg \c frameRange  if paramFrom has animation, and frameRange is not null, only this range of keys will be copied

This copies the value of \e paramFrom to \e paramTo, including any animation it may have. All the previous values in \e paramTo will be lost.

To choose all animation in \e paramFrom set \e frameRange to [0, 0]

V1.3: This function can be called the ::kOfxActionInstanceChanged action and during image effect analysis render passes.
V1.4: This function can be called the ::kOfxActionInstanceChanged action

\pre
- Both parameters must be of the same type.

\return
- ::kOfxStatOK       - all was OK
- ::kOfxStatErrBadHandle  - if the parameter handle was invalid*/
    pub paramCopy: ::std::option::Option<
        unsafe extern "C" fn(
            paramTo: OfxParamHandle,
            paramFrom: OfxParamHandle,
            dstOffset: OfxTime,
            frameRange: *const OfxRangeD,
        ) -> OfxStatus,
    >,
    /** @brief Used to group any parameter changes for undo/redo purposes

\arg \c paramSet    the parameter set in which this is happening
\arg \c name        label to attach to any undo/redo string UTF8

If a plugin calls paramSetValue/paramSetValueAtTime on one or more parameters, either from custom GUI interaction
or some analysis of imagery etc.. this is used to indicate the start of a set of a parameter
changes that should be considered part of a single undo/redo block.

\note \c paramEditBegin should only be called from within a ::kOfxActionInstanceChanged or interact action.

See also OfxParameterSuiteV1::paramEditEnd

\return
- ::kOfxStatOK       - all was OK
- ::kOfxStatErrBadHandle  - if the instance handle was invalid
*/
    pub paramEditBegin: ::std::option::Option<
        unsafe extern "C" fn(
            paramSet: OfxParamSetHandle,
            name: *const ::std::os::raw::c_char,
        ) -> OfxStatus,
    >,
    /** @brief Used to group any parameter changes for undo/redo purposes

\arg \c paramSet    parameter set in which this is happening

If a plugin calls paramSetValue/paramSetValueAtTime on one or more parameters, either from custom GUI interaction
or some analysis of imagery etc.. this is used to indicate the end of a set of parameter
changes that should be considerred part of a single undo/redo block

\note \c paramEditEnd should only be called from within a ::kOfxActionInstanceChanged or interact action.

See also OfxParameterSuiteV1::paramEditBegin

@returns
- ::kOfxStatOK       - all was OK
- ::kOfxStatErrBadHandle  - if the instance handle was invalid
*/
    pub paramEditEnd: ::std::option::Option<
        unsafe extern "C" fn(paramSet: OfxParamSetHandle) -> OfxStatus,
    >,
}
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

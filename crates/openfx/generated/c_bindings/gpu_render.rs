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
use super::image_effect::{
    OfxImageClipHandle, OfxImageClipStruct, OfxImageEffectHandle, OfxImageEffectStruct,
    OfxImageEffectSuiteV1, OfxImageMemoryHandle, OfxImageMemoryStruct,
    kOfxHostNativeOriginBottomLeft, kOfxHostNativeOriginCenter,
    kOfxHostNativeOriginTopLeft, kOfxImageClipPropConnected,
    kOfxImageClipPropContinuousSamples, kOfxImageClipPropFieldExtraction,
    kOfxImageClipPropFieldOrder, kOfxImageClipPropIsMask, kOfxImageClipPropOptional,
    kOfxImageClipPropUnmappedComponents, kOfxImageClipPropUnmappedPixelDepth,
    kOfxImageComponentAlpha, kOfxImageComponentNone, kOfxImageComponentRGB,
    kOfxImageComponentRGBA, kOfxImageEffectActionBeginSequenceRender,
    kOfxImageEffectActionDescribeInContext, kOfxImageEffectActionEndSequenceRender,
    kOfxImageEffectActionGetClipPreferences, kOfxImageEffectActionGetFramesNeeded,
    kOfxImageEffectActionGetRegionOfDefinition,
    kOfxImageEffectActionGetRegionsOfInterest, kOfxImageEffectActionGetTimeDomain,
    kOfxImageEffectActionIsIdentity, kOfxImageEffectActionRender,
    kOfxImageEffectContextFilter, kOfxImageEffectContextGeneral,
    kOfxImageEffectContextGenerator, kOfxImageEffectContextPaint,
    kOfxImageEffectContextRetimer, kOfxImageEffectContextTransition,
    kOfxImageEffectFrameVarying, kOfxImageEffectHostPropIsBackground,
    kOfxImageEffectHostPropNativeOrigin, kOfxImageEffectInstancePropEffectDuration,
    kOfxImageEffectInstancePropSequentialRender, kOfxImageEffectOutputClipName,
    kOfxImageEffectPluginApi, kOfxImageEffectPluginApiVersion,
    kOfxImageEffectPluginPropFieldRenderTwiceAlways, kOfxImageEffectPluginPropGrouping,
    kOfxImageEffectPluginPropHostFrameThreading, kOfxImageEffectPluginPropObsolete,
    kOfxImageEffectPluginPropOverlayInteractV1,
    kOfxImageEffectPluginPropOverlayInteractV2, kOfxImageEffectPluginPropSingleInstance,
    kOfxImageEffectPluginRenderThreadSafety, kOfxImageEffectPropBehaviourWhenUnlicensed,
    kOfxImageEffectPropClipPreferencesSlaveParam, kOfxImageEffectPropComponents,
    kOfxImageEffectPropContext, kOfxImageEffectPropFieldToRender,
    kOfxImageEffectPropFrameRange, kOfxImageEffectPropFrameRate,
    kOfxImageEffectPropFrameStep, kOfxImageEffectPropInteractiveRenderStatus,
    kOfxImageEffectPropNoSpatialAwareness, kOfxImageEffectPropPixelDepth,
    kOfxImageEffectPropPluginHandle, kOfxImageEffectPropPreMultiplication,
    kOfxImageEffectPropProjectExtent, kOfxImageEffectPropProjectOffset,
    kOfxImageEffectPropProjectPixelAspectRatio, kOfxImageEffectPropProjectSize,
    kOfxImageEffectPropRegionOfDefinition, kOfxImageEffectPropRegionOfInterest,
    kOfxImageEffectPropRenderQualityDraft, kOfxImageEffectPropRenderScale,
    kOfxImageEffectPropRenderWindow, kOfxImageEffectPropSequentialRenderStatus,
    kOfxImageEffectPropSetableFielding, kOfxImageEffectPropSetableFrameRate,
    kOfxImageEffectPropSupportedComponents, kOfxImageEffectPropSupportedContexts,
    kOfxImageEffectPropSupportedPixelDepths, kOfxImageEffectPropSupportsMultiResolution,
    kOfxImageEffectPropSupportsMultipleClipDepths,
    kOfxImageEffectPropSupportsMultipleClipPARs, kOfxImageEffectPropSupportsOverlays,
    kOfxImageEffectPropSupportsTiles, kOfxImageEffectPropTemporalClipAccess,
    kOfxImageEffectPropThumbnailRender, kOfxImageEffectPropUnmappedFrameRange,
    kOfxImageEffectPropUnmappedFrameRate, kOfxImageEffectRenderFullySafe,
    kOfxImageEffectRenderInstanceSafe, kOfxImageEffectRenderUnsafe,
    kOfxImageEffectRetimerParamName, kOfxImageEffectSimpleSourceClipName,
    kOfxImageEffectSuite, kOfxImageEffectTransitionParamName,
    kOfxImageEffectTransitionSourceFromClipName,
    kOfxImageEffectTransitionSourceToClipName, kOfxImageFieldBoth, kOfxImageFieldDoubled,
    kOfxImageFieldLower, kOfxImageFieldNone, kOfxImageFieldSingle, kOfxImageFieldUpper,
    kOfxImageOpaque, kOfxImagePreMultiplied, kOfxImagePropBounds, kOfxImagePropData,
    kOfxImagePropField, kOfxImagePropPixelAspectRatio, kOfxImagePropRegionOfDefinition,
    kOfxImagePropRowBytes, kOfxImagePropUniqueIdentifier, kOfxImageUnPreMultiplied,
    kOfxTypeClip, kOfxTypeImage, kOfxTypeImageEffect, kOfxTypeImageEffectHost,
    kOfxTypeImageEffectInstance, kOfxUnlicensedContinue, kOfxUnlicensedFail,
};
use super::interact::{
    OfxInteract, OfxInteractHandle, OfxInteractSuiteV1, kOfxActionCreateInstanceInteract,
    kOfxActionDescribeInteract, kOfxActionDestroyInstanceInteract,
    kOfxInteractActionDraw, kOfxInteractActionGainFocus, kOfxInteractActionKeyDown,
    kOfxInteractActionKeyRepeat, kOfxInteractActionKeyUp, kOfxInteractActionLoseFocus,
    kOfxInteractActionPenDown, kOfxInteractActionPenMotion, kOfxInteractActionPenUp,
    kOfxInteractPropBackgroundColour, kOfxInteractPropBitDepth, kOfxInteractPropHasAlpha,
    kOfxInteractPropPenPosition, kOfxInteractPropPenPressure,
    kOfxInteractPropPenViewportPosition, kOfxInteractPropPixelScale,
    kOfxInteractPropSlaveToParam, kOfxInteractPropSuggestedColour, kOfxInteractSuite,
};
use super::memory::{OfxMemorySuiteV1, kOfxMemorySuite};
use super::message::{
    OfxMessageSuiteV1, OfxMessageSuiteV2, kOfxMessageError, kOfxMessageFatal,
    kOfxMessageLog, kOfxMessageMessage, kOfxMessageQuestion, kOfxMessageSuite,
    kOfxMessageWarning,
};
use super::multi_thread::{
    OfxMultiThreadSuiteV1, OfxMutex, OfxMutexHandle, OfxThreadFunctionV1,
    kOfxMultiThreadSuite,
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
    kOfxParamHostPropSupportsStringAnimation, kOfxParamInterpType,
    kOfxParamInterpTypeConstantStep, kOfxParamInterpTypeLinear,
    kOfxParamInterpTypeSmooth, kOfxParamInvalidateAll, kOfxParamInvalidateValueChange,
    kOfxParamInvalidateValueChangeToEnd, kOfxParamPageSkipColumn, kOfxParamPageSkipRow,
    kOfxParamPropAnimates, kOfxParamPropCacheInvalidation, kOfxParamPropCanUndo,
    kOfxParamPropChoiceEnum, kOfxParamPropChoiceOption, kOfxParamPropChoiceOrder,
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
pub const kOfxOpenGLRenderSuite: &::std::ffi::CStr = c"OfxImageEffectOpenGLRenderSuite";
pub const kOfxImageEffectPropOpenGLRenderSupported: &::std::ffi::CStr = c"OfxImageEffectPropOpenGLRenderSupported";
pub const kOfxOpenGLPropPixelDepth: &::std::ffi::CStr = c"OfxOpenGLPropPixelDepth";
pub const kOfxImageEffectPropOpenGLEnabled: &::std::ffi::CStr = c"OfxImageEffectPropOpenGLEnabled";
pub const kOfxImageEffectPropOpenGLTextureIndex: &::std::ffi::CStr = c"OfxImageEffectPropOpenGLTextureIndex";
pub const kOfxImageEffectPropOpenGLTextureTarget: &::std::ffi::CStr = c"OfxImageEffectPropOpenGLTextureTarget";
pub const kOfxImageEffectPropCPURenderSupported: &::std::ffi::CStr = c"OfxImageEffectPropCPURenderSupported";
pub const kOfxActionOpenGLContextAttached: &::std::ffi::CStr = c"OfxActionOpenGLContextAttached";
pub const kOfxActionOpenGLContextDetached: &::std::ffi::CStr = c"kOfxActionOpenGLContextDetached";
pub const kOfxImageEffectPropCudaRenderSupported: &::std::ffi::CStr = c"OfxImageEffectPropCudaRenderSupported";
pub const kOfxImageEffectPropCudaEnabled: &::std::ffi::CStr = c"OfxImageEffectPropCudaEnabled";
pub const kOfxImageEffectPropCudaStreamSupported: &::std::ffi::CStr = c"OfxImageEffectPropCudaStreamSupported";
pub const kOfxImageEffectPropCudaStream: &::std::ffi::CStr = c"OfxImageEffectPropCudaStream";
pub const kOfxImageEffectPropMetalRenderSupported: &::std::ffi::CStr = c"OfxImageEffectPropMetalRenderSupported";
pub const kOfxImageEffectPropMetalEnabled: &::std::ffi::CStr = c"OfxImageEffectPropMetalEnabled";
pub const kOfxImageEffectPropMetalCommandQueue: &::std::ffi::CStr = c"OfxImageEffectPropMetalCommandQueue";
pub const kOfxImageEffectPropOpenCLRenderSupported: &::std::ffi::CStr = c"OfxImageEffectPropOpenCLRenderSupported";
pub const kOfxImageEffectPropOpenCLSupported: &::std::ffi::CStr = c"OfxImageEffectPropOpenCLSupported";
pub const kOfxImageEffectPropOpenCLEnabled: &::std::ffi::CStr = c"OfxImageEffectPropOpenCLEnabled";
pub const kOfxImageEffectPropOpenCLCommandQueue: &::std::ffi::CStr = c"OfxImageEffectPropOpenCLCommandQueue";
pub const kOfxImageEffectPropOpenCLImage: &::std::ffi::CStr = c"OfxImageEffectPropOpenCLImage";
pub const kOfxOpenCLProgramSuite: &::std::ffi::CStr = c"OfxOpenCLProgramSuite";
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of OfxInteractSuiteV1",
    ][::std::mem::size_of::<OfxInteractSuiteV1>() - 24usize];
    [
        "Alignment of OfxInteractSuiteV1",
    ][::std::mem::align_of::<OfxInteractSuiteV1>() - 8usize];
    [
        "Offset of field: OfxInteractSuiteV1::interactSwapBuffers",
    ][::std::mem::offset_of!(OfxInteractSuiteV1, interactSwapBuffers) - 0usize];
    [
        "Offset of field: OfxInteractSuiteV1::interactRedraw",
    ][::std::mem::offset_of!(OfxInteractSuiteV1, interactRedraw) - 8usize];
    [
        "Offset of field: OfxInteractSuiteV1::interactGetPropertySet",
    ][::std::mem::offset_of!(OfxInteractSuiteV1, interactGetPropertySet) - 16usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxMemorySuiteV1"][::std::mem::size_of::<OfxMemorySuiteV1>() - 16usize];
    [
        "Alignment of OfxMemorySuiteV1",
    ][::std::mem::align_of::<OfxMemorySuiteV1>() - 8usize];
    [
        "Offset of field: OfxMemorySuiteV1::memoryAlloc",
    ][::std::mem::offset_of!(OfxMemorySuiteV1, memoryAlloc) - 0usize];
    [
        "Offset of field: OfxMemorySuiteV1::memoryFree",
    ][::std::mem::offset_of!(OfxMemorySuiteV1, memoryFree) - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of OfxMultiThreadSuiteV1",
    ][::std::mem::size_of::<OfxMultiThreadSuiteV1>() - 72usize];
    [
        "Alignment of OfxMultiThreadSuiteV1",
    ][::std::mem::align_of::<OfxMultiThreadSuiteV1>() - 8usize];
    [
        "Offset of field: OfxMultiThreadSuiteV1::multiThread",
    ][::std::mem::offset_of!(OfxMultiThreadSuiteV1, multiThread) - 0usize];
    [
        "Offset of field: OfxMultiThreadSuiteV1::multiThreadNumCPUs",
    ][::std::mem::offset_of!(OfxMultiThreadSuiteV1, multiThreadNumCPUs) - 8usize];
    [
        "Offset of field: OfxMultiThreadSuiteV1::multiThreadIndex",
    ][::std::mem::offset_of!(OfxMultiThreadSuiteV1, multiThreadIndex) - 16usize];
    [
        "Offset of field: OfxMultiThreadSuiteV1::multiThreadIsSpawnedThread",
    ][::std::mem::offset_of!(OfxMultiThreadSuiteV1, multiThreadIsSpawnedThread)
        - 24usize];
    [
        "Offset of field: OfxMultiThreadSuiteV1::mutexCreate",
    ][::std::mem::offset_of!(OfxMultiThreadSuiteV1, mutexCreate) - 32usize];
    [
        "Offset of field: OfxMultiThreadSuiteV1::mutexDestroy",
    ][::std::mem::offset_of!(OfxMultiThreadSuiteV1, mutexDestroy) - 40usize];
    [
        "Offset of field: OfxMultiThreadSuiteV1::mutexLock",
    ][::std::mem::offset_of!(OfxMultiThreadSuiteV1, mutexLock) - 48usize];
    [
        "Offset of field: OfxMultiThreadSuiteV1::mutexUnLock",
    ][::std::mem::offset_of!(OfxMultiThreadSuiteV1, mutexUnLock) - 56usize];
    [
        "Offset of field: OfxMultiThreadSuiteV1::mutexTryLock",
    ][::std::mem::offset_of!(OfxMultiThreadSuiteV1, mutexTryLock) - 64usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of OfxImageEffectSuiteV1",
    ][::std::mem::size_of::<OfxImageEffectSuiteV1>() - 104usize];
    [
        "Alignment of OfxImageEffectSuiteV1",
    ][::std::mem::align_of::<OfxImageEffectSuiteV1>() - 8usize];
    [
        "Offset of field: OfxImageEffectSuiteV1::getPropertySet",
    ][::std::mem::offset_of!(OfxImageEffectSuiteV1, getPropertySet) - 0usize];
    [
        "Offset of field: OfxImageEffectSuiteV1::getParamSet",
    ][::std::mem::offset_of!(OfxImageEffectSuiteV1, getParamSet) - 8usize];
    [
        "Offset of field: OfxImageEffectSuiteV1::clipDefine",
    ][::std::mem::offset_of!(OfxImageEffectSuiteV1, clipDefine) - 16usize];
    [
        "Offset of field: OfxImageEffectSuiteV1::clipGetHandle",
    ][::std::mem::offset_of!(OfxImageEffectSuiteV1, clipGetHandle) - 24usize];
    [
        "Offset of field: OfxImageEffectSuiteV1::clipGetPropertySet",
    ][::std::mem::offset_of!(OfxImageEffectSuiteV1, clipGetPropertySet) - 32usize];
    [
        "Offset of field: OfxImageEffectSuiteV1::clipGetImage",
    ][::std::mem::offset_of!(OfxImageEffectSuiteV1, clipGetImage) - 40usize];
    [
        "Offset of field: OfxImageEffectSuiteV1::clipReleaseImage",
    ][::std::mem::offset_of!(OfxImageEffectSuiteV1, clipReleaseImage) - 48usize];
    [
        "Offset of field: OfxImageEffectSuiteV1::clipGetRegionOfDefinition",
    ][::std::mem::offset_of!(OfxImageEffectSuiteV1, clipGetRegionOfDefinition)
        - 56usize];
    [
        "Offset of field: OfxImageEffectSuiteV1::abort",
    ][::std::mem::offset_of!(OfxImageEffectSuiteV1, abort) - 64usize];
    [
        "Offset of field: OfxImageEffectSuiteV1::imageMemoryAlloc",
    ][::std::mem::offset_of!(OfxImageEffectSuiteV1, imageMemoryAlloc) - 72usize];
    [
        "Offset of field: OfxImageEffectSuiteV1::imageMemoryFree",
    ][::std::mem::offset_of!(OfxImageEffectSuiteV1, imageMemoryFree) - 80usize];
    [
        "Offset of field: OfxImageEffectSuiteV1::imageMemoryLock",
    ][::std::mem::offset_of!(OfxImageEffectSuiteV1, imageMemoryLock) - 88usize];
    [
        "Offset of field: OfxImageEffectSuiteV1::imageMemoryUnlock",
    ][::std::mem::offset_of!(OfxImageEffectSuiteV1, imageMemoryUnlock) - 96usize];
};
/** @brief OFX suite that provides image to texture conversion for OpenGL
processing*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxImageEffectOpenGLRenderSuiteV1 {
    /** @brief loads an image from an OFX clip as a texture into OpenGL

\arg \c clip   clip to load the image from
\arg \c time   effect time to load the image from
\arg \c format requested texture format (As in
none,byte,word,half,float, etc..)
When set to NULL, the host decides the format based on the
plug-in's ::kOfxOpenGLPropPixelDepth setting.
\arg \c region region of the image to load (optional, set to NULL to
get a 'default' region)
this is in the \ref CanonicalCoordinates.
\arg \c textureHandle property set containing information about the
texture

An image is fetched from a clip at the indicated time for the given region
and loaded into an OpenGL texture. When a specific format is requested, the
host ensures it gives the requested format.
When the clip specified is the "Output" clip, the format is ignored and
the host must bind the resulting texture as the current color buffer
(render target). This may also be done prior to calling the
::kOfxImageEffectActionRender action.
If the \em region parameter is set to non-NULL, then it will be clipped to
the clip's Region of Definition for the given time.
The returned image will be \em at \em least as big as this region.
If the region parameter is not set or is NULL, then the region fetched will be at
least the Region of Interest the effect has previously specified, clipped to
the clip's Region of Definition.
Information about the texture, including the texture index, is returned in
the \em textureHandle argument.
The properties on this handle will be...
- ::kOfxImageEffectPropOpenGLTextureIndex
- ::kOfxImageEffectPropOpenGLTextureTarget
- ::kOfxImageEffectPropPixelDepth
- ::kOfxImageEffectPropComponents
- ::kOfxImageEffectPropPreMultiplication
- ::kOfxImageEffectPropRenderScale
- ::kOfxImagePropPixelAspectRatio
- ::kOfxImagePropBounds
- ::kOfxImagePropRegionOfDefinition
- ::kOfxImagePropRowBytes
- ::kOfxImagePropField
- ::kOfxImagePropUniqueIdentifier

With the exception of the OpenGL specifics, these properties are the same
as the properties in an image handle returned by clipGetImage in the image
effect suite.
\pre
- clip was returned by clipGetHandle
- Format property in the texture handle

\post
- texture handle to be disposed of by clipFreeTexture before the action
returns
- when the clip specified is the "Output" clip, the format is ignored and
the host must bind the resulting texture as the current color buffer
(render target).
This may also be done prior to calling the render action.

@returns
- ::kOfxStatOK           - the image was successfully fetched and returned
in the handle,
- ::kOfxStatFailed       - the image could not be fetched because it does
not exist in the clip at the indicated
time and/or region, the plug-in should continue
operation, but assume the image was black and
transparent.
- ::kOfxStatErrBadHandle - the clip handle was invalid,
- ::kOfxStatErrMemory    - not enough OpenGL memory was available for the
effect to load the texture.
The plug-in should abort the GL render and
return ::kOfxStatErrMemory, after which the host can
decide to retry the operation with CPU based processing.

\note
- this is the OpenGL equivalent of clipGetImage from OfxImageEffectSuiteV1
*/
    pub clipLoadTexture: ::std::option::Option<
        unsafe extern "C" fn(
            clip: OfxImageClipHandle,
            time: OfxTime,
            format: *const ::std::os::raw::c_char,
            region: *const OfxRectD,
            textureHandle: *mut OfxPropertySetHandle,
        ) -> OfxStatus,
    >,
    /** @brief Releases the texture handle previously returned by
clipLoadTexture

For input clips, this also deletes the texture from OpenGL.
This should also be called on the output clip; for the Output
clip, it just releases the handle but does not delete the
texture (since the host will need to read it).

\pre
- textureHandle was returned by clipGetImage

\post
- all operations on textureHandle will be invalid, and the OpenGL texture
it referred to has been deleted (for source clips)

@returns
- ::kOfxStatOK - the image was successfully fetched and returned in the
handle,
- ::kOfxStatFailed - general failure for some reason,
- ::kOfxStatErrBadHandle - the image handle was invalid,*/
    pub clipFreeTexture: ::std::option::Option<
        unsafe extern "C" fn(textureHandle: OfxPropertySetHandle) -> OfxStatus,
    >,
    /** @brief Request the host to minimize its GPU resource load

When a plug-in fails to allocate GPU resources, it can call this function to
request the host to flush its GPU resources if it holds any.
After the function the plug-in can try again to allocate resources which then
might succeed if the host actually has released anything.

\pre
\post
- No changes to the plug-in GL state should have been made.

@returns
- ::kOfxStatOK           - the host has actually released some
resources,
- ::kOfxStatReplyDefault - nothing the host could do..*/
    pub flushResources: ::std::option::Option<unsafe extern "C" fn() -> OfxStatus>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of OfxImageEffectOpenGLRenderSuiteV1",
    ][::std::mem::size_of::<OfxImageEffectOpenGLRenderSuiteV1>() - 24usize];
    [
        "Alignment of OfxImageEffectOpenGLRenderSuiteV1",
    ][::std::mem::align_of::<OfxImageEffectOpenGLRenderSuiteV1>() - 8usize];
    [
        "Offset of field: OfxImageEffectOpenGLRenderSuiteV1::clipLoadTexture",
    ][::std::mem::offset_of!(OfxImageEffectOpenGLRenderSuiteV1, clipLoadTexture)
        - 0usize];
    [
        "Offset of field: OfxImageEffectOpenGLRenderSuiteV1::clipFreeTexture",
    ][::std::mem::offset_of!(OfxImageEffectOpenGLRenderSuiteV1, clipFreeTexture)
        - 8usize];
    [
        "Offset of field: OfxImageEffectOpenGLRenderSuiteV1::flushResources",
    ][::std::mem::offset_of!(OfxImageEffectOpenGLRenderSuiteV1, flushResources)
        - 16usize];
};
/** @brief OFX suite that allows a plug-in to get OpenCL programs compiled

This is an optional suite the host can provide for building OpenCL programs for the plug-in,
as an alternative to calling clCreateProgramWithSource / clBuildProgram. There are two advantages to
doing this: The host can add flags (such as -cl-denorms-are-zero) to the build call, and may also
cache program binaries for performance (however, if the source of the program or the OpenCL
environment changes, the host must recompile so some mechanism such as hashing must be used).*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxOpenCLProgramSuiteV1 {
    /// @brief Compiles the OpenCL program
    pub compileProgram: ::std::option::Option<
        unsafe extern "C" fn(
            pszProgramSource: *const ::std::os::raw::c_char,
            fOptional: ::std::os::raw::c_int,
            pResult: *mut ::std::os::raw::c_void,
        ) -> OfxStatus,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of OfxOpenCLProgramSuiteV1",
    ][::std::mem::size_of::<OfxOpenCLProgramSuiteV1>() - 8usize];
    [
        "Alignment of OfxOpenCLProgramSuiteV1",
    ][::std::mem::align_of::<OfxOpenCLProgramSuiteV1>() - 8usize];
    [
        "Offset of field: OfxOpenCLProgramSuiteV1::compileProgram",
    ][::std::mem::offset_of!(OfxOpenCLProgramSuiteV1, compileProgram) - 0usize];
};

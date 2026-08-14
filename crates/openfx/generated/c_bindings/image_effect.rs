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
pub const kOfxImageEffectPluginApi: &::std::ffi::CStr = c"OfxImageEffectPluginAPI";
pub const kOfxImageEffectPluginApiVersion: u32 = 1;
pub const kOfxImageComponentNone: &::std::ffi::CStr = c"OfxImageComponentNone";
pub const kOfxImageComponentRGBA: &::std::ffi::CStr = c"OfxImageComponentRGBA";
pub const kOfxImageComponentRGB: &::std::ffi::CStr = c"OfxImageComponentRGB";
pub const kOfxImageComponentAlpha: &::std::ffi::CStr = c"OfxImageComponentAlpha";
pub const kOfxImageEffectContextGenerator: &::std::ffi::CStr = c"OfxImageEffectContextGenerator";
pub const kOfxImageEffectContextFilter: &::std::ffi::CStr = c"OfxImageEffectContextFilter";
pub const kOfxImageEffectContextTransition: &::std::ffi::CStr = c"OfxImageEffectContextTransition";
pub const kOfxImageEffectContextPaint: &::std::ffi::CStr = c"OfxImageEffectContextPaint";
pub const kOfxImageEffectContextGeneral: &::std::ffi::CStr = c"OfxImageEffectContextGeneral";
pub const kOfxImageEffectContextRetimer: &::std::ffi::CStr = c"OfxImageEffectContextRetimer";
pub const kOfxTypeImageEffectHost: &::std::ffi::CStr = c"OfxTypeImageEffectHost";
pub const kOfxTypeImageEffect: &::std::ffi::CStr = c"OfxTypeImageEffect";
pub const kOfxTypeImageEffectInstance: &::std::ffi::CStr = c"OfxTypeImageEffectInstance";
pub const kOfxTypeClip: &::std::ffi::CStr = c"OfxTypeClip";
pub const kOfxTypeImage: &::std::ffi::CStr = c"OfxTypeImage";
pub const kOfxImageEffectActionGetRegionOfDefinition: &::std::ffi::CStr = c"OfxImageEffectActionGetRegionOfDefinition";
pub const kOfxImageEffectActionGetRegionsOfInterest: &::std::ffi::CStr = c"OfxImageEffectActionGetRegionsOfInterest";
pub const kOfxImageEffectActionGetTimeDomain: &::std::ffi::CStr = c"OfxImageEffectActionGetTimeDomain";
pub const kOfxImageEffectActionGetFramesNeeded: &::std::ffi::CStr = c"OfxImageEffectActionGetFramesNeeded";
pub const kOfxImageEffectActionGetClipPreferences: &::std::ffi::CStr = c"OfxImageEffectActionGetClipPreferences";
pub const kOfxImageEffectActionIsIdentity: &::std::ffi::CStr = c"OfxImageEffectActionIsIdentity";
pub const kOfxImageEffectActionRender: &::std::ffi::CStr = c"OfxImageEffectActionRender";
pub const kOfxImageEffectActionBeginSequenceRender: &::std::ffi::CStr = c"OfxImageEffectActionBeginSequenceRender";
pub const kOfxImageEffectActionEndSequenceRender: &::std::ffi::CStr = c"OfxImageEffectActionEndSequenceRender";
pub const kOfxImageEffectActionDescribeInContext: &::std::ffi::CStr = c"OfxImageEffectActionDescribeInContext";
pub const kOfxImageEffectPropSupportedContexts: &::std::ffi::CStr = c"OfxImageEffectPropSupportedContexts";
pub const kOfxImageEffectPropPluginHandle: &::std::ffi::CStr = c"OfxImageEffectPropPluginHandle";
pub const kOfxImageEffectHostPropIsBackground: &::std::ffi::CStr = c"OfxImageEffectHostPropIsBackground";
pub const kOfxImageEffectPluginPropSingleInstance: &::std::ffi::CStr = c"OfxImageEffectPluginPropSingleInstance";
pub const kOfxImageEffectPluginRenderThreadSafety: &::std::ffi::CStr = c"OfxImageEffectPluginRenderThreadSafety";
pub const kOfxImageEffectRenderUnsafe: &::std::ffi::CStr = c"OfxImageEffectRenderUnsafe";
pub const kOfxImageEffectRenderInstanceSafe: &::std::ffi::CStr = c"OfxImageEffectRenderInstanceSafe";
pub const kOfxImageEffectRenderFullySafe: &::std::ffi::CStr = c"OfxImageEffectRenderFullySafe";
pub const kOfxImageEffectPluginPropHostFrameThreading: &::std::ffi::CStr = c"OfxImageEffectPluginPropHostFrameThreading";
pub const kOfxImageEffectPropSupportsMultipleClipDepths: &::std::ffi::CStr = c"OfxImageEffectPropMultipleClipDepths";
pub const kOfxImageEffectPropSupportsMultipleClipPARs: &::std::ffi::CStr = c"OfxImageEffectPropSupportsMultipleClipPARs";
pub const kOfxImageEffectPropClipPreferencesSlaveParam: &::std::ffi::CStr = c"OfxImageEffectPropClipPreferencesSlaveParam";
pub const kOfxImageEffectPropSetableFrameRate: &::std::ffi::CStr = c"OfxImageEffectPropSetableFrameRate";
pub const kOfxImageEffectPropSetableFielding: &::std::ffi::CStr = c"OfxImageEffectPropSetableFielding";
pub const kOfxImageEffectInstancePropSequentialRender: &::std::ffi::CStr = c"OfxImageEffectInstancePropSequentialRender";
pub const kOfxImageEffectPropSequentialRenderStatus: &::std::ffi::CStr = c"OfxImageEffectPropSequentialRenderStatus";
pub const kOfxHostNativeOriginBottomLeft: &::std::ffi::CStr = c"kOfxImageEffectHostPropNativeOriginBottomLeft";
pub const kOfxHostNativeOriginTopLeft: &::std::ffi::CStr = c"kOfxImageEffectHostPropNativeOriginTopLeft";
pub const kOfxHostNativeOriginCenter: &::std::ffi::CStr = c"kOfxImageEffectHostPropNativeOriginCenter";
pub const kOfxImageEffectHostPropNativeOrigin: &::std::ffi::CStr = c"OfxImageEffectHostPropNativeOrigin";
pub const kOfxImageEffectPropInteractiveRenderStatus: &::std::ffi::CStr = c"OfxImageEffectPropInteractiveRenderStatus";
pub const kOfxImageEffectPluginPropGrouping: &::std::ffi::CStr = c"OfxImageEffectPluginPropGrouping";
pub const kOfxImageEffectPluginPropObsolete: &::std::ffi::CStr = c"OfxImageEffectPluginPropObsolete";
pub const kOfxImageEffectPropSupportsOverlays: &::std::ffi::CStr = c"OfxImageEffectPropSupportsOverlays";
pub const kOfxImageEffectPluginPropOverlayInteractV1: &::std::ffi::CStr = c"OfxImageEffectPluginPropOverlayInteractV1";
pub const kOfxImageEffectPluginPropOverlayInteractV2: &::std::ffi::CStr = c"OfxImageEffectPluginPropOverlayInteractV2";
pub const kOfxImageEffectPropSupportsMultiResolution: &::std::ffi::CStr = c"OfxImageEffectPropSupportsMultiResolution";
pub const kOfxImageEffectPropSupportsTiles: &::std::ffi::CStr = c"OfxImageEffectPropSupportsTiles";
pub const kOfxImageEffectPropTemporalClipAccess: &::std::ffi::CStr = c"OfxImageEffectPropTemporalClipAccess";
pub const kOfxImageEffectPropContext: &::std::ffi::CStr = c"OfxImageEffectPropContext";
pub const kOfxImageEffectPropPixelDepth: &::std::ffi::CStr = c"OfxImageEffectPropPixelDepth";
pub const kOfxImageEffectPropComponents: &::std::ffi::CStr = c"OfxImageEffectPropComponents";
pub const kOfxImagePropUniqueIdentifier: &::std::ffi::CStr = c"OfxImagePropUniqueIdentifier";
pub const kOfxImageClipPropContinuousSamples: &::std::ffi::CStr = c"OfxImageClipPropContinuousSamples";
pub const kOfxImageClipPropUnmappedPixelDepth: &::std::ffi::CStr = c"OfxImageClipPropUnmappedPixelDepth";
pub const kOfxImageClipPropUnmappedComponents: &::std::ffi::CStr = c"OfxImageClipPropUnmappedComponents";
pub const kOfxImageEffectPropPreMultiplication: &::std::ffi::CStr = c"OfxImageEffectPropPreMultiplication";
pub const kOfxImageOpaque: &::std::ffi::CStr = c"OfxImageOpaque";
pub const kOfxImagePreMultiplied: &::std::ffi::CStr = c"OfxImageAlphaPremultiplied";
pub const kOfxImageUnPreMultiplied: &::std::ffi::CStr = c"OfxImageAlphaUnPremultiplied";
pub const kOfxImageEffectPropSupportedPixelDepths: &::std::ffi::CStr = c"OfxImageEffectPropSupportedPixelDepths";
pub const kOfxImageEffectPropSupportedComponents: &::std::ffi::CStr = c"OfxImageEffectPropSupportedComponents";
pub const kOfxImageClipPropOptional: &::std::ffi::CStr = c"OfxImageClipPropOptional";
pub const kOfxImageClipPropIsMask: &::std::ffi::CStr = c"OfxImageClipPropIsMask";
pub const kOfxImagePropPixelAspectRatio: &::std::ffi::CStr = c"OfxImagePropPixelAspectRatio";
pub const kOfxImageEffectPropFrameRate: &::std::ffi::CStr = c"OfxImageEffectPropFrameRate";
pub const kOfxImageEffectPropUnmappedFrameRate: &::std::ffi::CStr = c"OfxImageEffectPropUnmappedFrameRate";
pub const kOfxImageEffectPropFrameStep: &::std::ffi::CStr = c"OfxImageEffectPropFrameStep";
pub const kOfxImageEffectPropFrameRange: &::std::ffi::CStr = c"OfxImageEffectPropFrameRange";
pub const kOfxImageEffectPropUnmappedFrameRange: &::std::ffi::CStr = c"OfxImageEffectPropUnmappedFrameRange";
pub const kOfxImageClipPropConnected: &::std::ffi::CStr = c"OfxImageClipPropConnected";
pub const kOfxImageEffectFrameVarying: &::std::ffi::CStr = c"OfxImageEffectFrameVarying";
pub const kOfxImageEffectPropRenderScale: &::std::ffi::CStr = c"OfxImageEffectPropRenderScale";
pub const kOfxImageEffectPropRenderQualityDraft: &::std::ffi::CStr = c"OfxImageEffectPropRenderQualityDraft";
pub const kOfxImageEffectPropNoSpatialAwareness: &::std::ffi::CStr = c"OfxImageEffectPropNoSpatialAwareness";
pub const kOfxImageEffectPropThumbnailRender: &::std::ffi::CStr = c"OfxImageEffectPropThumbnailRender";
pub const kOfxImageEffectPropProjectExtent: &::std::ffi::CStr = c"OfxImageEffectPropProjectExtent";
pub const kOfxImageEffectPropProjectSize: &::std::ffi::CStr = c"OfxImageEffectPropProjectSize";
pub const kOfxImageEffectPropProjectOffset: &::std::ffi::CStr = c"OfxImageEffectPropProjectOffset";
pub const kOfxImageEffectPropProjectPixelAspectRatio: &::std::ffi::CStr = c"OfxImageEffectPropPixelAspectRatio";
pub const kOfxImageEffectInstancePropEffectDuration: &::std::ffi::CStr = c"OfxImageEffectInstancePropEffectDuration";
pub const kOfxImageClipPropFieldOrder: &::std::ffi::CStr = c"OfxImageClipPropFieldOrder";
pub const kOfxImagePropData: &::std::ffi::CStr = c"OfxImagePropData";
pub const kOfxImagePropBounds: &::std::ffi::CStr = c"OfxImagePropBounds";
pub const kOfxImagePropRegionOfDefinition: &::std::ffi::CStr = c"OfxImagePropRegionOfDefinition";
pub const kOfxImagePropRowBytes: &::std::ffi::CStr = c"OfxImagePropRowBytes";
pub const kOfxImagePropField: &::std::ffi::CStr = c"OfxImagePropField";
pub const kOfxImageEffectPluginPropFieldRenderTwiceAlways: &::std::ffi::CStr = c"OfxImageEffectPluginPropFieldRenderTwiceAlways";
pub const kOfxImageClipPropFieldExtraction: &::std::ffi::CStr = c"OfxImageClipPropFieldExtraction";
pub const kOfxImageEffectPropFieldToRender: &::std::ffi::CStr = c"OfxImageEffectPropFieldToRender";
pub const kOfxImageEffectPropRegionOfDefinition: &::std::ffi::CStr = c"OfxImageEffectPropRegionOfDefinition";
pub const kOfxImageEffectPropRegionOfInterest: &::std::ffi::CStr = c"OfxImageEffectPropRegionOfInterest";
pub const kOfxImageEffectPropRenderWindow: &::std::ffi::CStr = c"OfxImageEffectPropRenderWindow";
pub const kOfxImageFieldNone: &::std::ffi::CStr = c"OfxFieldNone";
pub const kOfxImageFieldLower: &::std::ffi::CStr = c"OfxFieldLower";
pub const kOfxImageFieldUpper: &::std::ffi::CStr = c"OfxFieldUpper";
pub const kOfxImageFieldBoth: &::std::ffi::CStr = c"OfxFieldBoth";
pub const kOfxImageFieldSingle: &::std::ffi::CStr = c"OfxFieldSingle";
pub const kOfxImageFieldDoubled: &::std::ffi::CStr = c"OfxFieldDoubled";
pub const kOfxImageEffectPropBehaviourWhenUnlicensed: &::std::ffi::CStr = c"OfxImageEffectPropBehaviourWhenUnlicensed";
pub const kOfxUnlicensedContinue: &::std::ffi::CStr = c"OfxUnlicensedContinue";
pub const kOfxUnlicensedFail: &::std::ffi::CStr = c"OfxUnlicensedFail";
pub const kOfxImageEffectOutputClipName: &::std::ffi::CStr = c"Output";
pub const kOfxImageEffectSimpleSourceClipName: &::std::ffi::CStr = c"Source";
pub const kOfxImageEffectTransitionSourceFromClipName: &::std::ffi::CStr = c"SourceFrom";
pub const kOfxImageEffectTransitionSourceToClipName: &::std::ffi::CStr = c"SourceTo";
pub const kOfxImageEffectTransitionParamName: &::std::ffi::CStr = c"Transition";
pub const kOfxImageEffectRetimerParamName: &::std::ffi::CStr = c"SourceTime";
pub const kOfxImageEffectSuite: &::std::ffi::CStr = c"OfxImageEffectSuite";
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
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxImageEffectStruct {
    _unused: [u8; 0],
}
/// @brief Blind declaration of an OFX image effect
pub type OfxImageEffectHandle = *mut OfxImageEffectStruct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxImageClipStruct {
    _unused: [u8; 0],
}
/// @brief Blind declaration of an OFX image effect
pub type OfxImageClipHandle = *mut OfxImageClipStruct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxImageMemoryStruct {
    _unused: [u8; 0],
}
/// @brief Blind declaration for an handle to image memory returned by the image memory management routines
pub type OfxImageMemoryHandle = *mut OfxImageMemoryStruct;
/** @brief The OFX suite for image effects

This suite provides the functions needed by a plugin to defined and use an image effect plugin.*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxImageEffectSuiteV1 {
    /** @brief Retrieves the property set for the given image effect

\arg \c imageEffect   image effect to get the property set for
\arg \c propHandle    pointer to a the property set pointer, value is returned here

The property handle is for the duration of the image effect handle.

@returns
- ::kOfxStatOK       - the property set was found and returned
- ::kOfxStatErrBadHandle  - if the parameter handle was invalid
- ::kOfxStatErrUnknown    - if the type is unknown*/
    pub getPropertySet: ::std::option::Option<
        unsafe extern "C" fn(
            imageEffect: OfxImageEffectHandle,
            propHandle: *mut OfxPropertySetHandle,
        ) -> OfxStatus,
    >,
    /** @brief Retrieves the parameter set for the given image effect

\arg \c imageEffect   image effect to get the property set for
\arg \c paramSet     pointer to a the parameter set, value is returned here

The param set handle is valid for the lifetime of the image effect handle.

@returns
- ::kOfxStatOK       - the property set was found and returned
- ::kOfxStatErrBadHandle  - if the parameter handle was invalid
- ::kOfxStatErrUnknown    - if the type is unknown*/
    pub getParamSet: ::std::option::Option<
        unsafe extern "C" fn(
            imageEffect: OfxImageEffectHandle,
            paramSet: *mut OfxParamSetHandle,
        ) -> OfxStatus,
    >,
    /** @brief Define a clip to the effect.

\arg \c pluginHandle handle passed into 'describeInContext' action
\arg \c name unique name of the clip to define
\arg \c propertySet property handle for the clip descriptor will be returned here

This function defines a clip to a host, the returned property set is used to describe
various aspects of the clip to the host. Note that this does not create a clip instance.

\pre
- we are inside the describe in context action.

@returns*/
    pub clipDefine: ::std::option::Option<
        unsafe extern "C" fn(
            imageEffect: OfxImageEffectHandle,
            name: *const ::std::os::raw::c_char,
            propertySet: *mut OfxPropertySetHandle,
        ) -> OfxStatus,
    >,
    /** @brief Get the property handle of the named input clip in the given instance

\arg \c imageEffect an instance handle to the plugin
\arg \c name        name of the clip, previously used in a clip define call
\arg \c clip        where to return the clip
\arg \c propertySet  if not NULL, the descriptor handle for a parameter's property set will be placed here.

The propertySet will have the same value as would be returned by OfxImageEffectSuiteV1::clipGetPropertySet

This return a clip handle for the given instance, note that this will \em not be the same as the
clip handle returned by clipDefine and will be distanct to clip handles in any other instance
of the plugin.

Not a valid call in any of the describe actions.

\pre
- create instance action called,
- \e name passed to clipDefine for this context,
- not inside describe or describe in context actions.

\post
- handle will be valid for the life time of the instance.
*/
    pub clipGetHandle: ::std::option::Option<
        unsafe extern "C" fn(
            imageEffect: OfxImageEffectHandle,
            name: *const ::std::os::raw::c_char,
            clip: *mut OfxImageClipHandle,
            propertySet: *mut OfxPropertySetHandle,
        ) -> OfxStatus,
    >,
    /** @brief Retrieves the property set for a given clip

\arg \c clip         clip effect to get the property set for
\arg \c propHandle   pointer to a the property set handle, value is returedn her

The property handle is valid for the lifetime of the clip, which is generally the lifetime of the instance.

@returns
- ::kOfxStatOK       - the property set was found and returned
- ::kOfxStatErrBadHandle  - if the parameter handle was invalid
- ::kOfxStatErrUnknown    - if the type is unknown*/
    pub clipGetPropertySet: ::std::option::Option<
        unsafe extern "C" fn(
            clip: OfxImageClipHandle,
            propHandle: *mut OfxPropertySetHandle,
        ) -> OfxStatus,
    >,
    /** @brief Get a handle for an image in a clip at the indicated time and indicated region

\arg \c clip  clip to extract the image from
\arg \c time        time to fetch the image at
\arg \c region      region to fetch the image from (optional, set to NULL to get a 'default' region)
this is in the \ref CanonicalCoordinates.
\arg \c imageHandle property set containing the image's data

An image is fetched from a clip at the indicated time for the given region and returned in the imageHandle.

If the \e region parameter is not set to NULL, then it will be clipped to the clip's Region of Definition for the given time. The returned image will be \em at \em least as big as this region. If the region parameter is not set, then the region fetched will be at least the Region of Interest the effect has previously specified, clipped the clip's Region of Definition.

If clipGetImage is called twice with the same parameters, then two separate image handles will be returned, each of which must be release. The underlying implementation could share image data pointers and use reference counting to maintain them.

\pre
- clip was returned by clipGetHandle

\post
- image handle is only valid for the duration of the action clipGetImage is called in
- image handle to be disposed of by clipReleaseImage before the action returns

@returns
- ::kOfxStatOK - the image was successfully fetched and returned in the handle,
- ::kOfxStatFailed - the image could not be fetched because it does not exist in the clip at the indicated time and/or region, the plugin
should continue operation, but assume the image was black and transparent.
- ::kOfxStatErrBadHandle - the clip handle was invalid,
- ::kOfxStatErrMemory - the host had not enough memory to complete the operation, plugin should abort whatever it was doing.
*/
    pub clipGetImage: ::std::option::Option<
        unsafe extern "C" fn(
            clip: OfxImageClipHandle,
            time: OfxTime,
            region: *const OfxRectD,
            imageHandle: *mut OfxPropertySetHandle,
        ) -> OfxStatus,
    >,
    /** @brief Releases the image handle previously returned by clipGetImage


\pre
- imageHandle was returned by clipGetImage

\post
- all operations on imageHandle will be invalid

@returns
- ::kOfxStatOK - the image was successfully fetched and returned in the handle,
- ::kOfxStatErrBadHandle - the image handle was invalid,*/
    pub clipReleaseImage: ::std::option::Option<
        unsafe extern "C" fn(imageHandle: OfxPropertySetHandle) -> OfxStatus,
    >,
    /** @brief Returns the spatial region of definition of the clip at the given time

\arg \c clipHandle  return this clip's region of definition
\arg \c time        time to use when determining clip's region of definition
\arg \c bounds      (out) bounds are returned here -- in \ref CanonicalCoordinates

\pre
- clipHandle was returned by clipGetHandle

\post
- bounds will be filled the RoD of the clip at the indicated time

@returns
- ::kOfxStatOK - the region was successfully found and returned in the handle,
- ::kOfxStatFailed - the region could not be determined,
- ::kOfxStatErrBadHandle - the clip handle was invalid,
- ::kOfxStatErrMemory - the host had not enough memory to complete the operation, plugin should abort whatever it was doing.*/
    pub clipGetRegionOfDefinition: ::std::option::Option<
        unsafe extern "C" fn(
            clip: OfxImageClipHandle,
            time: OfxTime,
            bounds: *mut OfxRectD,
        ) -> OfxStatus,
    >,
    /** @brief Returns whether to abort processing or not.

\arg \c imageEffect  instance of the image effect

A host may want to signal to a plugin that it should stop whatever rendering it is doing and start again.
Generally this is done in interactive threads in response to users tweaking some parameter.

This function indicates whether a plugin should stop whatever processing it is doing.

@returns
- 0 if the effect should continue whatever processing it is doing
- 1 if the effect should abort whatever processing it is doing*/
    pub abort: ::std::option::Option<
        unsafe extern "C" fn(imageEffect: OfxImageEffectHandle) -> ::std::os::raw::c_int,
    >,
    /** @brief Allocate memory from the host's image memory pool

\arg \c instanceHandle  effect instance to associate with this memory allocation, may be NULL.
\arg \c nBytes          number of bytes to allocate
\arg \c memoryHandle    pointer to the memory handle where a return value is placed

Memory handles allocated by this should be freed by OfxImageEffectSuiteV1::imageMemoryFree.
To access the memory behind the handle you need to call  OfxImageEffectSuiteV1::imageMemoryLock.

See \ref ImageEffectsMemoryAllocation.

@returns
- kOfxStatOK if all went well, a valid memory handle is placed in \e memoryHandle
- kOfxStatErrBadHandle if instanceHandle is not valid, memoryHandle is set to NULL
- kOfxStatErrMemory if there was not enough memory to satisfy the call, memoryHandle is set to NULL*/
    pub imageMemoryAlloc: ::std::option::Option<
        unsafe extern "C" fn(
            instanceHandle: OfxImageEffectHandle,
            nBytes: usize,
            memoryHandle: *mut OfxImageMemoryHandle,
        ) -> OfxStatus,
    >,
    /** @brief Frees a memory handle and associated memory.

\arg \c memoryHandle memory handle returned by imageMemoryAlloc

This function frees a memory handle and associated memory that was previously allocated via OfxImageEffectSuiteV1::imageMemoryAlloc

If there are outstanding locks, these are ignored and the handle and memory are freed anyway.

See \ref ImageEffectsMemoryAllocation.

@returns
- kOfxStatOK if the memory was cleanly deleted
- kOfxStatErrBadHandle if the value of \e memoryHandle was not a valid pointer returned by OfxImageEffectSuiteV1::imageMemoryAlloc*/
    pub imageMemoryFree: ::std::option::Option<
        unsafe extern "C" fn(memoryHandle: OfxImageMemoryHandle) -> OfxStatus,
    >,
    /** @brief Lock the memory associated with a memory handle and make it available for use.

\arg \c memoryHandle memory handle returned by imageMemoryAlloc
\arg \c returnedPtr where to the pointer to the locked memory

This function locks them memory associated with a memory handle and returns a pointer to it. The memory will be 16 byte aligned, to allow use of vector operations.

Note that memory locks and unlocks nest.

After the first lock call, the contents of the memory pointer to by \e returnedPtr is undefined. All subsequent calls to lock will return memory with the same contents as  the previous call.

Also, if unlocked, then relocked, the memory associated with a memory handle may be at a different address.

See also OfxImageEffectSuiteV1::imageMemoryUnlock and \ref ImageEffectsMemoryAllocation.

@returns
- kOfxStatOK if the memory was locked, a pointer is placed in \e returnedPtr
- kOfxStatErrBadHandle if the value of \e memoryHandle was not a valid pointer returned by OfxImageEffectSuiteV1::imageMemoryAlloc, null is placed in \e *returnedPtr
- kOfxStatErrMemory if there was not enough memory to satisfy the call, \e *returnedPtr is set to NULL*/
    pub imageMemoryLock: ::std::option::Option<
        unsafe extern "C" fn(
            memoryHandle: OfxImageMemoryHandle,
            returnedPtr: *mut *mut ::std::os::raw::c_void,
        ) -> OfxStatus,
    >,
    /** @brief Unlock allocated image data

\arg \c allocatedData pointer to memory previously returned by OfxImageEffectSuiteV1::imageAlloc

This function unlocks a previously locked memory handle. Once completely unlocked, memory associated with a memoryHandle is no longer available for use. Attempting to use it results in undefined behaviour.

Note that locks and unlocks nest, and to fully unlock memory you need to match the count of locks placed upon it.

Also note, if you unlock a completely unlocked handle, it has no effect (ie: the lock count can't be negative).

If unlocked, then relocked, the memory associated with a memory handle may be at a different address, however the contents will remain the same.

See also OfxImageEffectSuiteV1::imageMemoryLock and \ref ImageEffectsMemoryAllocation.

@returns
- kOfxStatOK if the memory was unlocked cleanly,
- kOfxStatErrBadHandle if the value of \e memoryHandle was not a valid pointer returned by OfxImageEffectSuiteV1::imageMemoryAlloc, null is placed in \e *returnedPtr*/
    pub imageMemoryUnlock: ::std::option::Option<
        unsafe extern "C" fn(memoryHandle: OfxImageMemoryHandle) -> OfxStatus,
    >,
}
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

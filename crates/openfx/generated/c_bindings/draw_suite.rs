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
use super::pixels::{
    OfxRGBAColourB, OfxRGBAColourD, OfxRGBAColourF, OfxRGBAColourS, OfxRGBColourB,
    OfxRGBColourD, OfxRGBColourF, OfxRGBColourS,
};
pub const kOfxDrawSuite: &::std::ffi::CStr = c"OfxDrawSuite";
pub const kOfxInteractPropDrawContext: &::std::ffi::CStr = c"OfxInteractPropDrawContext";
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
    ["Size of OfxRGBAColourB"][::std::mem::size_of::<OfxRGBAColourB>() - 4usize];
    ["Alignment of OfxRGBAColourB"][::std::mem::align_of::<OfxRGBAColourB>() - 1usize];
    [
        "Offset of field: OfxRGBAColourB::r",
    ][::std::mem::offset_of!(OfxRGBAColourB, r) - 0usize];
    [
        "Offset of field: OfxRGBAColourB::g",
    ][::std::mem::offset_of!(OfxRGBAColourB, g) - 1usize];
    [
        "Offset of field: OfxRGBAColourB::b",
    ][::std::mem::offset_of!(OfxRGBAColourB, b) - 2usize];
    [
        "Offset of field: OfxRGBAColourB::a",
    ][::std::mem::offset_of!(OfxRGBAColourB, a) - 3usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxRGBAColourS"][::std::mem::size_of::<OfxRGBAColourS>() - 8usize];
    ["Alignment of OfxRGBAColourS"][::std::mem::align_of::<OfxRGBAColourS>() - 2usize];
    [
        "Offset of field: OfxRGBAColourS::r",
    ][::std::mem::offset_of!(OfxRGBAColourS, r) - 0usize];
    [
        "Offset of field: OfxRGBAColourS::g",
    ][::std::mem::offset_of!(OfxRGBAColourS, g) - 2usize];
    [
        "Offset of field: OfxRGBAColourS::b",
    ][::std::mem::offset_of!(OfxRGBAColourS, b) - 4usize];
    [
        "Offset of field: OfxRGBAColourS::a",
    ][::std::mem::offset_of!(OfxRGBAColourS, a) - 6usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxRGBAColourF"][::std::mem::size_of::<OfxRGBAColourF>() - 16usize];
    ["Alignment of OfxRGBAColourF"][::std::mem::align_of::<OfxRGBAColourF>() - 4usize];
    [
        "Offset of field: OfxRGBAColourF::r",
    ][::std::mem::offset_of!(OfxRGBAColourF, r) - 0usize];
    [
        "Offset of field: OfxRGBAColourF::g",
    ][::std::mem::offset_of!(OfxRGBAColourF, g) - 4usize];
    [
        "Offset of field: OfxRGBAColourF::b",
    ][::std::mem::offset_of!(OfxRGBAColourF, b) - 8usize];
    [
        "Offset of field: OfxRGBAColourF::a",
    ][::std::mem::offset_of!(OfxRGBAColourF, a) - 12usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxRGBAColourD"][::std::mem::size_of::<OfxRGBAColourD>() - 32usize];
    ["Alignment of OfxRGBAColourD"][::std::mem::align_of::<OfxRGBAColourD>() - 8usize];
    [
        "Offset of field: OfxRGBAColourD::r",
    ][::std::mem::offset_of!(OfxRGBAColourD, r) - 0usize];
    [
        "Offset of field: OfxRGBAColourD::g",
    ][::std::mem::offset_of!(OfxRGBAColourD, g) - 8usize];
    [
        "Offset of field: OfxRGBAColourD::b",
    ][::std::mem::offset_of!(OfxRGBAColourD, b) - 16usize];
    [
        "Offset of field: OfxRGBAColourD::a",
    ][::std::mem::offset_of!(OfxRGBAColourD, a) - 24usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxRGBColourB"][::std::mem::size_of::<OfxRGBColourB>() - 3usize];
    ["Alignment of OfxRGBColourB"][::std::mem::align_of::<OfxRGBColourB>() - 1usize];
    [
        "Offset of field: OfxRGBColourB::r",
    ][::std::mem::offset_of!(OfxRGBColourB, r) - 0usize];
    [
        "Offset of field: OfxRGBColourB::g",
    ][::std::mem::offset_of!(OfxRGBColourB, g) - 1usize];
    [
        "Offset of field: OfxRGBColourB::b",
    ][::std::mem::offset_of!(OfxRGBColourB, b) - 2usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxRGBColourS"][::std::mem::size_of::<OfxRGBColourS>() - 6usize];
    ["Alignment of OfxRGBColourS"][::std::mem::align_of::<OfxRGBColourS>() - 2usize];
    [
        "Offset of field: OfxRGBColourS::r",
    ][::std::mem::offset_of!(OfxRGBColourS, r) - 0usize];
    [
        "Offset of field: OfxRGBColourS::g",
    ][::std::mem::offset_of!(OfxRGBColourS, g) - 2usize];
    [
        "Offset of field: OfxRGBColourS::b",
    ][::std::mem::offset_of!(OfxRGBColourS, b) - 4usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxRGBColourF"][::std::mem::size_of::<OfxRGBColourF>() - 12usize];
    ["Alignment of OfxRGBColourF"][::std::mem::align_of::<OfxRGBColourF>() - 4usize];
    [
        "Offset of field: OfxRGBColourF::r",
    ][::std::mem::offset_of!(OfxRGBColourF, r) - 0usize];
    [
        "Offset of field: OfxRGBColourF::g",
    ][::std::mem::offset_of!(OfxRGBColourF, g) - 4usize];
    [
        "Offset of field: OfxRGBColourF::b",
    ][::std::mem::offset_of!(OfxRGBColourF, b) - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxRGBColourD"][::std::mem::size_of::<OfxRGBColourD>() - 24usize];
    ["Alignment of OfxRGBColourD"][::std::mem::align_of::<OfxRGBColourD>() - 8usize];
    [
        "Offset of field: OfxRGBColourD::r",
    ][::std::mem::offset_of!(OfxRGBColourD, r) - 0usize];
    [
        "Offset of field: OfxRGBColourD::g",
    ][::std::mem::offset_of!(OfxRGBColourD, g) - 8usize];
    [
        "Offset of field: OfxRGBColourD::b",
    ][::std::mem::offset_of!(OfxRGBColourD, b) - 16usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxDrawContext {
    _unused: [u8; 0],
}
/// @brief Blind declaration of an OFX drawing context
pub type OfxDrawContextHandle = *mut OfxDrawContext;
pub const OfxStandardColour_kOfxStandardColourOverlayBackground: OfxStandardColour = 0;
pub const OfxStandardColour_kOfxStandardColourOverlayActive: OfxStandardColour = 1;
pub const OfxStandardColour_kOfxStandardColourOverlaySelected: OfxStandardColour = 2;
pub const OfxStandardColour_kOfxStandardColourOverlayDeselected: OfxStandardColour = 3;
pub const OfxStandardColour_kOfxStandardColourOverlayMarqueeFG: OfxStandardColour = 4;
pub const OfxStandardColour_kOfxStandardColourOverlayMarqueeBG: OfxStandardColour = 5;
pub const OfxStandardColour_kOfxStandardColourOverlayText: OfxStandardColour = 6;
/// @brief Defines valid values for OfxDrawSuiteV1::getColour
pub type OfxStandardColour = ::std::os::raw::c_uint;
pub const OfxDrawLineStipplePattern_kOfxDrawLineStipplePatternSolid: OfxDrawLineStipplePattern = 0;
pub const OfxDrawLineStipplePattern_kOfxDrawLineStipplePatternDot: OfxDrawLineStipplePattern = 1;
pub const OfxDrawLineStipplePattern_kOfxDrawLineStipplePatternDash: OfxDrawLineStipplePattern = 2;
pub const OfxDrawLineStipplePattern_kOfxDrawLineStipplePatternAltDash: OfxDrawLineStipplePattern = 3;
pub const OfxDrawLineStipplePattern_kOfxDrawLineStipplePatternDotDash: OfxDrawLineStipplePattern = 4;
/// @brief Defines valid values for OfxDrawSuiteV1::setLineStipple
pub type OfxDrawLineStipplePattern = ::std::os::raw::c_uint;
pub const OfxDrawPrimitive_kOfxDrawPrimitiveLines: OfxDrawPrimitive = 0;
pub const OfxDrawPrimitive_kOfxDrawPrimitiveLineStrip: OfxDrawPrimitive = 1;
pub const OfxDrawPrimitive_kOfxDrawPrimitiveLineLoop: OfxDrawPrimitive = 2;
pub const OfxDrawPrimitive_kOfxDrawPrimitiveRectangle: OfxDrawPrimitive = 3;
pub const OfxDrawPrimitive_kOfxDrawPrimitivePolygon: OfxDrawPrimitive = 4;
pub const OfxDrawPrimitive_kOfxDrawPrimitiveEllipse: OfxDrawPrimitive = 5;
/// @brief Defines valid values for OfxDrawSuiteV1::draw
pub type OfxDrawPrimitive = ::std::os::raw::c_uint;
pub const OfxDrawTextAlignment_kOfxDrawTextAlignmentLeft: OfxDrawTextAlignment = 1;
pub const OfxDrawTextAlignment_kOfxDrawTextAlignmentRight: OfxDrawTextAlignment = 2;
pub const OfxDrawTextAlignment_kOfxDrawTextAlignmentTop: OfxDrawTextAlignment = 4;
pub const OfxDrawTextAlignment_kOfxDrawTextAlignmentBottom: OfxDrawTextAlignment = 8;
pub const OfxDrawTextAlignment_kOfxDrawTextAlignmentBaseline: OfxDrawTextAlignment = 16;
pub const OfxDrawTextAlignment_kOfxDrawTextAlignmentCenterH: OfxDrawTextAlignment = 3;
pub const OfxDrawTextAlignment_kOfxDrawTextAlignmentCenterV: OfxDrawTextAlignment = 20;
/// @brief Defines text alignment values for OfxDrawSuiteV1::drawText
pub type OfxDrawTextAlignment = ::std::os::raw::c_uint;
/** @brief OFX suite that allows an effect to draw to a host-defined display context.
To use this, the plugin must use kOfxImageEffectPluginPropOverlayInteractV2.*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxDrawSuiteV1 {
    /** @brief Retrieves the host's desired draw colour for

\arg \c context  draw context
\arg \c std_colour desired colour type
\arg \c colour      returned RGBA colour

@returns
- ::kOfxStatOK - the colour was returned
- ::kOfxStatErrValue - std_colour was invalid
- ::kOfxStatFailed - failure, e.g. if function is called outside kOfxInteractActionDraw*/
    pub getColour: ::std::option::Option<
        unsafe extern "C" fn(
            context: OfxDrawContextHandle,
            std_colour: OfxStandardColour,
            colour: *mut OfxRGBAColourF,
        ) -> OfxStatus,
    >,
    /** @brief Sets the colour for future drawing operations (lines, filled shapes and text)

\arg \c context  draw context
\arg \c colour      RGBA colour

The host should use "over" compositing when using a non-opaque colour.

@returns
- ::kOfxStatOK - the colour was changed
- ::kOfxStatFailed - failure, e.g. if function is called outside kOfxInteractActionDraw*/
    pub setColour: ::std::option::Option<
        unsafe extern "C" fn(
            context: OfxDrawContextHandle,
            colour: *const OfxRGBAColourF,
        ) -> OfxStatus,
    >,
    /** @brief Sets the line width for future line drawing operations

\arg \c context  draw context
\arg \c width     line width

Use width 0 for a single pixel line or non-zero for a smooth line of the desired width

The host should adjust for screen density.

@returns
- ::kOfxStatOK - the width was changed
- ::kOfxStatFailed - failure, e.g. if function is called outside kOfxInteractActionDraw*/
    pub setLineWidth: ::std::option::Option<
        unsafe extern "C" fn(context: OfxDrawContextHandle, width: f32) -> OfxStatus,
    >,
    /** @brief Sets the stipple pattern for future line drawing operations

\arg \c context  draw context
\arg \c pattern  desired stipple pattern

@returns
- ::kOfxStatOK - the pattern was changed
- ::kOfxStatErrValue - pattern was not valid
- ::kOfxStatFailed - failure, e.g. if function is called outside kOfxInteractActionDraw*/
    pub setLineStipple: ::std::option::Option<
        unsafe extern "C" fn(
            context: OfxDrawContextHandle,
            pattern: OfxDrawLineStipplePattern,
        ) -> OfxStatus,
    >,
    /** @brief Draws a primitive of the desired type

\arg \c context  draw context
\arg \c primitive  desired primitive
\arg \c points  array of points in the primitive
\arg \c point_count  number of points in the array

kOfxDrawPrimitiveLines - like GL_LINES, n points draws n/2 separated lines
kOfxDrawPrimitiveLineStrip - like GL_LINE_STRIP, n points draws n-1 connected lines
kOfxDrawPrimitiveLineLoop - like GL_LINE_LOOP, n points draws n connected lines
kOfxDrawPrimitiveRectangle - draws an axis-aligned filled rectangle defined by 2 opposite corner points
kOfxDrawPrimitivePolygon - like GL_POLYGON, draws a filled n-sided polygon
kOfxDrawPrimitiveEllipse - draws a axis-aligned elliptical line (not filled) within the rectangle defined by 2 opposite corner points

@returns
- ::kOfxStatOK - the draw was completed
- ::kOfxStatErrValue - invalid primitive, or point_count not valid for primitive
- ::kOfxStatFailed - failure, e.g. if function is called outside kOfxInteractActionDraw*/
    pub draw: ::std::option::Option<
        unsafe extern "C" fn(
            context: OfxDrawContextHandle,
            primitive: OfxDrawPrimitive,
            points: *const OfxPointD,
            point_count: ::std::os::raw::c_int,
        ) -> OfxStatus,
    >,
    /** @brief Draws text at the specified position

\arg \c context  draw context
\arg \c text  text to draw (UTF-8 encoded)
\arg \c pos  position at which to align the text
\arg \c alignment  text alignment flags (see kOfxDrawTextAlignment*)

The text font face and size are determined by the host.

@returns
- ::kOfxStatOK - the text was drawn
- ::kOfxStatErrValue - text or pos were not defined
- ::kOfxStatFailed - failure, e.g. if function is called outside kOfxInteractActionDraw*/
    pub drawText: ::std::option::Option<
        unsafe extern "C" fn(
            context: OfxDrawContextHandle,
            text: *const ::std::os::raw::c_char,
            pos: *const OfxPointD,
            alignment: ::std::os::raw::c_int,
        ) -> OfxStatus,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxDrawSuiteV1"][::std::mem::size_of::<OfxDrawSuiteV1>() - 48usize];
    ["Alignment of OfxDrawSuiteV1"][::std::mem::align_of::<OfxDrawSuiteV1>() - 8usize];
    [
        "Offset of field: OfxDrawSuiteV1::getColour",
    ][::std::mem::offset_of!(OfxDrawSuiteV1, getColour) - 0usize];
    [
        "Offset of field: OfxDrawSuiteV1::setColour",
    ][::std::mem::offset_of!(OfxDrawSuiteV1, setColour) - 8usize];
    [
        "Offset of field: OfxDrawSuiteV1::setLineWidth",
    ][::std::mem::offset_of!(OfxDrawSuiteV1, setLineWidth) - 16usize];
    [
        "Offset of field: OfxDrawSuiteV1::setLineStipple",
    ][::std::mem::offset_of!(OfxDrawSuiteV1, setLineStipple) - 24usize];
    [
        "Offset of field: OfxDrawSuiteV1::draw",
    ][::std::mem::offset_of!(OfxDrawSuiteV1, draw) - 32usize];
    [
        "Offset of field: OfxDrawSuiteV1::drawText",
    ][::std::mem::offset_of!(OfxDrawSuiteV1, drawText) - 40usize];
};

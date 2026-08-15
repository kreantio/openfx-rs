use super::core::{OfxPointD, OfxStatus};
use super::pixels::OfxRGBAColourF;
pub const kOfxDrawSuite: &::std::ffi::CStr = c"OfxDrawSuite";
pub const kOfxInteractPropDrawContext: &::std::ffi::CStr = c"OfxInteractPropDrawContext";
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

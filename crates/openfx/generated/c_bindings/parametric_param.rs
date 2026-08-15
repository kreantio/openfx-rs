use super::core::{OfxStatus, OfxTime};
use super::param::OfxParamHandle;
pub const kOfxParametricParameterSuite: &::std::ffi::CStr = c"OfxParametricParameterSuite";
pub const kOfxParamTypeParametric: &::std::ffi::CStr = c"OfxParamTypeParametric";
pub const kOfxParamPropParametricDimension: &::std::ffi::CStr = c"OfxParamPropParametricDimension";
pub const kOfxParamPropParametricUIColour: &::std::ffi::CStr = c"OfxParamPropParametricUIColour";
pub const kOfxParamPropParametricInteractBackground: &::std::ffi::CStr = c"OfxParamPropParametricInteractBackground";
pub const kOfxParamHostPropSupportsParametricAnimation: &::std::ffi::CStr = c"OfxParamHostPropSupportsParametricAnimation";
pub const kOfxParamPropParametricRange: &::std::ffi::CStr = c"OfxParamPropParametricRange";
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

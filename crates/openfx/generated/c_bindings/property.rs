use super::core::{OfxPropertySetHandle, OfxStatus};
pub const kOfxPropertySuite: &::std::ffi::CStr = c"OfxPropertySuite";
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

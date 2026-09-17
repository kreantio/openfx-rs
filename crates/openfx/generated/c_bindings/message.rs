use super::core::OfxStatus;
pub const kOfxMessageSuite: &::std::ffi::CStr = c"OfxMessageSuite";
/// ```doxygen
/// @brief String used to type fatal error messages
///
///     Fatal error messages should only be posted by a plugin when it can no longer continue operation.
/// ```
pub const kOfxMessageFatal: &::std::ffi::CStr = c"OfxMessageFatal";
/// ```doxygen
/// @brief String used to type error messages
///
///     Ordinary error messages should be posted when there is an error in operation that is recoverable by
///     user intervention.
/// ```
pub const kOfxMessageError: &::std::ffi::CStr = c"OfxMessageError";
/// ```doxygen
/// @brief String used to type warning messages
///
///     Warnings indicate states that allow for operations to proceed, but are not necessarily optimal.
/// ```
pub const kOfxMessageWarning: &::std::ffi::CStr = c"OfxMessageWarning";
/// ```doxygen
/// @brief String used to type simple ordinary messages
///
///     Ordinary messages simply convey information from the plugin directly to the user.
/// ```
pub const kOfxMessageMessage: &::std::ffi::CStr = c"OfxMessageMessage";
/// ```doxygen
/// @brief String used to type log messages
///
///     Log messages are written out to a log and not to the end user.
/// ```
pub const kOfxMessageLog: &::std::ffi::CStr = c"OfxMessageLog";
/// ```doxygen
/// @brief String used to type yes/no messages
///
///     The host is to enter a modal state which waits for the user to respond yes or no.
/// The OfxMessageSuiteV1::message function which posted the message will only return after
/// the user responds. When asking a question, the OfxStatus code returned by the message function will be,
///     - kOfxStatReplyYes - if the user replied 'yes' to the question
///     - kOfxStatReplyNo - if the user replied 'no' to the question
///     - some error code if an error was encounterred
///
///     It is an error to post a question message if the plugin is not in an interactive session.
/// ```
pub const kOfxMessageQuestion: &::std::ffi::CStr = c"OfxMessageQuestion";
/// ```doxygen
/// @brief The OFX suite that allows a plug-in to pass messages back to a user. The V2 suite extends on this
/// in a backwards compatible manner.
/// ```
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxMessageSuiteV1 {
    /// ```doxygen
    /// @brief Post a message on the host, using printf style varargs
    ///
    /// \arg \c handle     effect handle (descriptor or instance) the message should be associated with, may be NULL
    /// \arg \c messageType string describing the kind of message to post, one of the kOfxMessageType* constants
    /// \arg \c messageId plugin specified id to associate with this message. If overriding the message in XML resource, the message is identified with this, this may be NULL, or "", in which case no override will occur,
    /// \arg \c format    printf style format string
    /// \arg \c ...       printf style varargs list to print
    ///
    /// \returns
    /// - ::kOfxStatOK - if the message was successfully posted
    /// - ::kOfxStatReplyYes - if the message was of type  kOfxMessageQuestion and the user reply yes
    /// - ::kOfxStatReplyNo - if the message was of type kOfxMessageQuestion and the user reply no
    /// - ::kOfxStatFailed - if the message could not be posted for some reason
    /// ```
    pub message: ::std::option::Option<
        unsafe extern "C" fn(
            handle: *mut ::std::os::raw::c_void,
            messageType: *const ::std::os::raw::c_char,
            messageId: *const ::std::os::raw::c_char,
            format: *const ::std::os::raw::c_char,
            ...
        ) -> OfxStatus,
    >,
}
/// ```doxygen
/// @brief The OFX suite that allows a plug-in to pass messages back to a user.
///
/// This extends OfxMessageSuiteV1, and should be considered a replacement to version 1.
///
/// Note that this suite has been extended in backwards compatible manner, so that a host can return this struct
/// for both V1 and V2.
/// ```
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxMessageSuiteV2 {
    /// ```doxygen
    /// @brief Post a transient message on the host, using printf style varargs. Same as the V1 message suite call.
    ///
    /// \arg \c handle     effect handle (descriptor or instance) the message should be associated with, may be null
    /// \arg \c messageType string describing the kind of message to post, one of the kOfxMessageType* constants
    /// \arg \c messageId plugin specified id to associate with this message. If overriding the message in XML resource, the message is identified with this, this may be NULL, or "", in which case no override will occur,
    /// \arg \c format    printf style format string
    /// \arg \c ...       printf style varargs list to print
    ///
    /// \returns
    /// - ::kOfxStatOK - if the message was successfully posted
    /// - ::kOfxStatReplyYes - if the message was of type  kOfxMessageQuestion and the user reply yes
    /// - ::kOfxStatReplyNo - if the message was of type kOfxMessageQuestion and the user reply no
    /// - ::kOfxStatFailed - if the message could not be posted for some reason
    /// ```
    pub message: ::std::option::Option<
        unsafe extern "C" fn(
            handle: *mut ::std::os::raw::c_void,
            messageType: *const ::std::os::raw::c_char,
            messageId: *const ::std::os::raw::c_char,
            format: *const ::std::os::raw::c_char,
            ...
        ) -> OfxStatus,
    >,
    /// ```doxygen
    /// @brief Post a persistent message on an effect, using printf style varargs, and set error states. New for V2 message suite.
    ///
    /// \arg \c handle     effect instance handle the message should be associated with, may NOT be null,
    /// \arg \c messageType string describing the kind of message to post, should be one of...
    /// - kOfxMessageError
    /// - kOfxMessageWarning
    /// - kOfxMessageMessage
    /// \arg \c messageId plugin specified id to associate with this message. If overriding the message in XML resource, the message is identified with this, this may be NULL, or "", in which case no override will occur,
    /// \arg \c format    printf style format string
    /// \arg \c ...       printf style varargs list to print
    ///
    /// \returns
    /// - ::kOfxStatOK - if the message was successfully posted
    /// - ::kOfxStatErrBadHandle - the handle was rubbish
    /// - ::kOfxStatFailed - if the message could not be posted for some reason
    ///
    /// Persistent messages are associated with an effect handle until explicitly cleared by an effect. So if an error message is posted the error state, and associated message will persist and be displayed on the effect appropriately. (eg: draw a node in red on a node based compostor and display the message when clicked on).
    ///
    /// If \e messageType is error or warning, associated error states should be flagged on host applications. Posting an error message implies that the host cannot proceed, a warning allows the host to proceed, whilst a simple message should have no stop anything.
    /// ```
    pub setPersistentMessage: ::std::option::Option<
        unsafe extern "C" fn(
            handle: *mut ::std::os::raw::c_void,
            messageType: *const ::std::os::raw::c_char,
            messageId: *const ::std::os::raw::c_char,
            format: *const ::std::os::raw::c_char,
            ...
        ) -> OfxStatus,
    >,
    /// ```doxygen
    /// @brief Clears any persistent message on an effect handle that was set by OfxMessageSuiteV2::setPersistentMessage. New for V2 message suite.
    ///
    /// \arg \c handle     effect instance handle messages should be cleared from.
    /// \arg \c handle     effect handle (descriptor or instance)
    ///
    /// \returns
    /// - ::kOfxStatOK - if the message was successfully cleared
    /// - ::kOfxStatErrBadHandle - the handle was rubbish
    /// - ::kOfxStatFailed - if the message could not be cleared for some reason
    ///
    /// Clearing a message will clear any associated error state.
    /// ```
    pub clearPersistentMessage: ::std::option::Option<
        unsafe extern "C" fn(handle: *mut ::std::os::raw::c_void) -> OfxStatus,
    >,
}

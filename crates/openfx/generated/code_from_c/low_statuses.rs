pub enum Status {
    ErrBadHandle,
    ErrBadIndex,
    ErrExists,
    ErrFatal,
    ErrFormat,
    ErrMemory,
    ErrMissingHostFeature,
    ErrUnknown,
    ErrUnsupported,
    ErrValue,
    Failed,
    OK,
    ReplyDefault,
    ReplyNo,
    ReplyYes,
    Unlicensed,
    Unknown(crate::sys::generic::core::OfxStatus),
}
impl From<crate::sys::generic::core::OfxStatus> for Status {
    fn from(status: crate::sys::generic::core::OfxStatus) -> Self {
        match status {
            crate::sys::generic::core::kOfxStatErrBadHandle => Self::ErrBadHandle,
            crate::sys::generic::core::kOfxStatErrBadIndex => Self::ErrBadIndex,
            crate::sys::generic::core::kOfxStatErrExists => Self::ErrExists,
            crate::sys::generic::core::kOfxStatErrFatal => Self::ErrFatal,
            crate::sys::generic::core::kOfxStatErrFormat => Self::ErrFormat,
            crate::sys::generic::core::kOfxStatErrMemory => Self::ErrMemory,
            crate::sys::generic::core::kOfxStatErrMissingHostFeature => {
                Self::ErrMissingHostFeature
            }
            crate::sys::generic::core::kOfxStatErrUnknown => Self::ErrUnknown,
            crate::sys::generic::core::kOfxStatErrUnsupported => Self::ErrUnsupported,
            crate::sys::generic::core::kOfxStatErrValue => Self::ErrValue,
            crate::sys::generic::core::kOfxStatFailed => Self::Failed,
            crate::sys::generic::core::kOfxStatOK => Self::OK,
            crate::sys::generic::core::kOfxStatReplyDefault => Self::ReplyDefault,
            crate::sys::generic::core::kOfxStatReplyNo => Self::ReplyNo,
            crate::sys::generic::core::kOfxStatReplyYes => Self::ReplyYes,
            crate::sys::generic::core::kOfxStatUnlicensed => Self::Unlicensed,
            _ => Self::Unknown(status),
        }
    }
}
impl From<Status> for crate::sys::generic::core::OfxStatus {
    fn from(status: Status) -> Self {
        match status {
            Status::ErrBadHandle => crate::sys::generic::core::kOfxStatErrBadHandle,
            Status::ErrBadIndex => crate::sys::generic::core::kOfxStatErrBadIndex,
            Status::ErrExists => crate::sys::generic::core::kOfxStatErrExists,
            Status::ErrFatal => crate::sys::generic::core::kOfxStatErrFatal,
            Status::ErrFormat => crate::sys::generic::core::kOfxStatErrFormat,
            Status::ErrMemory => crate::sys::generic::core::kOfxStatErrMemory,
            Status::ErrMissingHostFeature => {
                crate::sys::generic::core::kOfxStatErrMissingHostFeature
            }
            Status::ErrUnknown => crate::sys::generic::core::kOfxStatErrUnknown,
            Status::ErrUnsupported => crate::sys::generic::core::kOfxStatErrUnsupported,
            Status::ErrValue => crate::sys::generic::core::kOfxStatErrValue,
            Status::Failed => crate::sys::generic::core::kOfxStatFailed,
            Status::OK => crate::sys::generic::core::kOfxStatOK,
            Status::ReplyDefault => crate::sys::generic::core::kOfxStatReplyDefault,
            Status::ReplyNo => crate::sys::generic::core::kOfxStatReplyNo,
            Status::ReplyYes => crate::sys::generic::core::kOfxStatReplyYes,
            Status::Unlicensed => crate::sys::generic::core::kOfxStatUnlicensed,
            Status::Unknown(status) => status,
        }
    }
}

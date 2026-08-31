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
    Unknown(crate::generic::sys::core::OfxStatus),
}
impl From<crate::generic::sys::core::OfxStatus> for Status {
    fn from(status: crate::generic::sys::core::OfxStatus) -> Self {
        match status {
            crate::generic::sys::core::kOfxStatErrBadHandle => Self::ErrBadHandle,
            crate::generic::sys::core::kOfxStatErrBadIndex => Self::ErrBadIndex,
            crate::generic::sys::core::kOfxStatErrExists => Self::ErrExists,
            crate::generic::sys::core::kOfxStatErrFatal => Self::ErrFatal,
            crate::generic::sys::core::kOfxStatErrFormat => Self::ErrFormat,
            crate::generic::sys::core::kOfxStatErrMemory => Self::ErrMemory,
            crate::generic::sys::core::kOfxStatErrMissingHostFeature => {
                Self::ErrMissingHostFeature
            }
            crate::generic::sys::core::kOfxStatErrUnknown => Self::ErrUnknown,
            crate::generic::sys::core::kOfxStatErrUnsupported => Self::ErrUnsupported,
            crate::generic::sys::core::kOfxStatErrValue => Self::ErrValue,
            crate::generic::sys::core::kOfxStatFailed => Self::Failed,
            crate::generic::sys::core::kOfxStatOK => Self::OK,
            crate::generic::sys::core::kOfxStatReplyDefault => Self::ReplyDefault,
            crate::generic::sys::core::kOfxStatReplyNo => Self::ReplyNo,
            crate::generic::sys::core::kOfxStatReplyYes => Self::ReplyYes,
            crate::generic::sys::core::kOfxStatUnlicensed => Self::Unlicensed,
            _ => Self::Unknown(status),
        }
    }
}
impl From<Status> for crate::generic::sys::core::OfxStatus {
    fn from(status: Status) -> Self {
        match status {
            Status::ErrBadHandle => crate::generic::sys::core::kOfxStatErrBadHandle,
            Status::ErrBadIndex => crate::generic::sys::core::kOfxStatErrBadIndex,
            Status::ErrExists => crate::generic::sys::core::kOfxStatErrExists,
            Status::ErrFatal => crate::generic::sys::core::kOfxStatErrFatal,
            Status::ErrFormat => crate::generic::sys::core::kOfxStatErrFormat,
            Status::ErrMemory => crate::generic::sys::core::kOfxStatErrMemory,
            Status::ErrMissingHostFeature => {
                crate::generic::sys::core::kOfxStatErrMissingHostFeature
            }
            Status::ErrUnknown => crate::generic::sys::core::kOfxStatErrUnknown,
            Status::ErrUnsupported => crate::generic::sys::core::kOfxStatErrUnsupported,
            Status::ErrValue => crate::generic::sys::core::kOfxStatErrValue,
            Status::Failed => crate::generic::sys::core::kOfxStatFailed,
            Status::OK => crate::generic::sys::core::kOfxStatOK,
            Status::ReplyDefault => crate::generic::sys::core::kOfxStatReplyDefault,
            Status::ReplyNo => crate::generic::sys::core::kOfxStatReplyNo,
            Status::ReplyYes => crate::generic::sys::core::kOfxStatReplyYes,
            Status::Unlicensed => crate::generic::sys::core::kOfxStatUnlicensed,
            Status::Unknown(status) => status,
        }
    }
}

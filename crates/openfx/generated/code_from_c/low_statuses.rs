pub enum OfxStatus {
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
impl From<crate::generic::sys::core::OfxStatus> for OfxStatus {
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
impl From<OfxStatus> for crate::generic::sys::core::OfxStatus {
    fn from(status: OfxStatus) -> Self {
        match status {
            OfxStatus::ErrBadHandle => crate::generic::sys::core::kOfxStatErrBadHandle,
            OfxStatus::ErrBadIndex => crate::generic::sys::core::kOfxStatErrBadIndex,
            OfxStatus::ErrExists => crate::generic::sys::core::kOfxStatErrExists,
            OfxStatus::ErrFatal => crate::generic::sys::core::kOfxStatErrFatal,
            OfxStatus::ErrFormat => crate::generic::sys::core::kOfxStatErrFormat,
            OfxStatus::ErrMemory => crate::generic::sys::core::kOfxStatErrMemory,
            OfxStatus::ErrMissingHostFeature => {
                crate::generic::sys::core::kOfxStatErrMissingHostFeature
            }
            OfxStatus::ErrUnknown => crate::generic::sys::core::kOfxStatErrUnknown,
            OfxStatus::ErrUnsupported => {
                crate::generic::sys::core::kOfxStatErrUnsupported
            }
            OfxStatus::ErrValue => crate::generic::sys::core::kOfxStatErrValue,
            OfxStatus::Failed => crate::generic::sys::core::kOfxStatFailed,
            OfxStatus::OK => crate::generic::sys::core::kOfxStatOK,
            OfxStatus::ReplyDefault => crate::generic::sys::core::kOfxStatReplyDefault,
            OfxStatus::ReplyNo => crate::generic::sys::core::kOfxStatReplyNo,
            OfxStatus::ReplyYes => crate::generic::sys::core::kOfxStatReplyYes,
            OfxStatus::Unlicensed => crate::generic::sys::core::kOfxStatUnlicensed,
            OfxStatus::Unknown(status) => status,
        }
    }
}

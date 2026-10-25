const _: () = assert!(
    crate::sys_umbrella::kOfxStatGPUOutOfMemory == crate::sys_umbrella::kOfxStatGLOutOfMemory
);
const _: () = assert!(
    crate::sys_umbrella::kOfxStatGPURenderFailed == crate::sys_umbrella::kOfxStatGLRenderFailed
);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[must_use]
pub enum StatusError {
    ErrBadHandle,
    ErrBadIndex,
    ErrExists,
    ErrFatal,
    ErrFormat,
    ErrImageFormat,
    ErrMemory,
    ErrMissingHostFeature,
    ErrUnknown,
    ErrUnsupported,
    ErrValue,
    Failed,
    GPUOutOfMemory,
    GPURenderFailed,
    Unlicensed,
}
impl StatusError {
    pub fn try_from_sys(sys_status: crate::sys_umbrella::OfxStatus) -> Option<Self> {
        match sys_status {
            crate::sys_umbrella::kOfxStatErrBadHandle => Some(Self::ErrBadHandle),
            crate::sys_umbrella::kOfxStatErrBadIndex => Some(Self::ErrBadIndex),
            crate::sys_umbrella::kOfxStatErrExists => Some(Self::ErrExists),
            crate::sys_umbrella::kOfxStatErrFatal => Some(Self::ErrFatal),
            crate::sys_umbrella::kOfxStatErrFormat => Some(Self::ErrFormat),
            crate::sys_umbrella::kOfxStatErrImageFormat => Some(Self::ErrImageFormat),
            crate::sys_umbrella::kOfxStatErrMemory => Some(Self::ErrMemory),
            crate::sys_umbrella::kOfxStatErrMissingHostFeature => Some(Self::ErrMissingHostFeature),
            crate::sys_umbrella::kOfxStatErrUnknown => Some(Self::ErrUnknown),
            crate::sys_umbrella::kOfxStatErrUnsupported => Some(Self::ErrUnsupported),
            crate::sys_umbrella::kOfxStatErrValue => Some(Self::ErrValue),
            crate::sys_umbrella::kOfxStatFailed => Some(Self::Failed),
            crate::sys_umbrella::kOfxStatGPUOutOfMemory => Some(Self::GPUOutOfMemory),
            crate::sys_umbrella::kOfxStatGPURenderFailed => Some(Self::GPURenderFailed),
            crate::sys_umbrella::kOfxStatUnlicensed => Some(Self::Unlicensed),
            _ => None,
        }
    }
}
impl From<StatusError> for crate::sys_umbrella::OfxStatus {
    fn from(status: StatusError) -> Self {
        match status {
            StatusError::ErrBadHandle => crate::sys_umbrella::kOfxStatErrBadHandle,
            StatusError::ErrBadIndex => crate::sys_umbrella::kOfxStatErrBadIndex,
            StatusError::ErrExists => crate::sys_umbrella::kOfxStatErrExists,
            StatusError::ErrFatal => crate::sys_umbrella::kOfxStatErrFatal,
            StatusError::ErrFormat => crate::sys_umbrella::kOfxStatErrFormat,
            StatusError::ErrImageFormat => crate::sys_umbrella::kOfxStatErrImageFormat,
            StatusError::ErrMemory => crate::sys_umbrella::kOfxStatErrMemory,
            StatusError::ErrMissingHostFeature => {
                crate::sys_umbrella::kOfxStatErrMissingHostFeature
            }
            StatusError::ErrUnknown => crate::sys_umbrella::kOfxStatErrUnknown,
            StatusError::ErrUnsupported => crate::sys_umbrella::kOfxStatErrUnsupported,
            StatusError::ErrValue => crate::sys_umbrella::kOfxStatErrValue,
            StatusError::Failed => crate::sys_umbrella::kOfxStatFailed,
            StatusError::GPUOutOfMemory => crate::sys_umbrella::kOfxStatGPUOutOfMemory,
            StatusError::GPURenderFailed => crate::sys_umbrella::kOfxStatGPURenderFailed,
            StatusError::Unlicensed => crate::sys_umbrella::kOfxStatUnlicensed,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[must_use]
pub enum StatusReply {
    ReplyDefault,
    ReplyNo,
    ReplyYes,
}
impl StatusReply {
    pub fn try_from_sys(sys_status: crate::sys_umbrella::OfxStatus) -> Option<Self> {
        match sys_status {
            crate::sys_umbrella::kOfxStatReplyDefault => Some(Self::ReplyDefault),
            crate::sys_umbrella::kOfxStatReplyNo => Some(Self::ReplyNo),
            crate::sys_umbrella::kOfxStatReplyYes => Some(Self::ReplyYes),
            _ => None,
        }
    }
}
impl From<StatusReply> for crate::sys_umbrella::OfxStatus {
    fn from(status: StatusReply) -> Self {
        match status {
            StatusReply::ReplyDefault => crate::sys_umbrella::kOfxStatReplyDefault,
            StatusReply::ReplyNo => crate::sys_umbrella::kOfxStatReplyNo,
            StatusReply::ReplyYes => crate::sys_umbrella::kOfxStatReplyYes,
        }
    }
}

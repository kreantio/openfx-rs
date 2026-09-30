pub enum Status {
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
    OK,
    ReplyDefault,
    ReplyNo,
    ReplyYes,
    Unlicensed,
    Unknown(crate::sys_umbrella::OfxStatus),
}
const _: () = assert!(
    crate ::sys_umbrella::kOfxStatGPUOutOfMemory == crate
    ::sys_umbrella::kOfxStatGLOutOfMemory
);
const _: () = assert!(
    crate ::sys_umbrella::kOfxStatGPURenderFailed == crate
    ::sys_umbrella::kOfxStatGLRenderFailed
);
impl From<crate::sys_umbrella::OfxStatus> for Status {
    fn from(status: crate::sys_umbrella::OfxStatus) -> Self {
        match status {
            crate::sys_umbrella::kOfxStatErrBadHandle => Self::ErrBadHandle,
            crate::sys_umbrella::kOfxStatErrBadIndex => Self::ErrBadIndex,
            crate::sys_umbrella::kOfxStatErrExists => Self::ErrExists,
            crate::sys_umbrella::kOfxStatErrFatal => Self::ErrFatal,
            crate::sys_umbrella::kOfxStatErrFormat => Self::ErrFormat,
            crate::sys_umbrella::kOfxStatErrImageFormat => Self::ErrImageFormat,
            crate::sys_umbrella::kOfxStatErrMemory => Self::ErrMemory,
            crate::sys_umbrella::kOfxStatErrMissingHostFeature => {
                Self::ErrMissingHostFeature
            }
            crate::sys_umbrella::kOfxStatErrUnknown => Self::ErrUnknown,
            crate::sys_umbrella::kOfxStatErrUnsupported => Self::ErrUnsupported,
            crate::sys_umbrella::kOfxStatErrValue => Self::ErrValue,
            crate::sys_umbrella::kOfxStatFailed => Self::Failed,
            crate::sys_umbrella::kOfxStatGPUOutOfMemory => Self::GPUOutOfMemory,
            crate::sys_umbrella::kOfxStatGPURenderFailed => Self::GPURenderFailed,
            crate::sys_umbrella::kOfxStatOK => Self::OK,
            crate::sys_umbrella::kOfxStatReplyDefault => Self::ReplyDefault,
            crate::sys_umbrella::kOfxStatReplyNo => Self::ReplyNo,
            crate::sys_umbrella::kOfxStatReplyYes => Self::ReplyYes,
            crate::sys_umbrella::kOfxStatUnlicensed => Self::Unlicensed,
            _ => Self::Unknown(status),
        }
    }
}
impl From<Status> for crate::sys_umbrella::OfxStatus {
    fn from(status: Status) -> Self {
        match status {
            Status::ErrBadHandle => crate::sys_umbrella::kOfxStatErrBadHandle,
            Status::ErrBadIndex => crate::sys_umbrella::kOfxStatErrBadIndex,
            Status::ErrExists => crate::sys_umbrella::kOfxStatErrExists,
            Status::ErrFatal => crate::sys_umbrella::kOfxStatErrFatal,
            Status::ErrFormat => crate::sys_umbrella::kOfxStatErrFormat,
            Status::ErrImageFormat => crate::sys_umbrella::kOfxStatErrImageFormat,
            Status::ErrMemory => crate::sys_umbrella::kOfxStatErrMemory,
            Status::ErrMissingHostFeature => {
                crate::sys_umbrella::kOfxStatErrMissingHostFeature
            }
            Status::ErrUnknown => crate::sys_umbrella::kOfxStatErrUnknown,
            Status::ErrUnsupported => crate::sys_umbrella::kOfxStatErrUnsupported,
            Status::ErrValue => crate::sys_umbrella::kOfxStatErrValue,
            Status::Failed => crate::sys_umbrella::kOfxStatFailed,
            Status::GPUOutOfMemory => crate::sys_umbrella::kOfxStatGPUOutOfMemory,
            Status::GPURenderFailed => crate::sys_umbrella::kOfxStatGPURenderFailed,
            Status::OK => crate::sys_umbrella::kOfxStatOK,
            Status::ReplyDefault => crate::sys_umbrella::kOfxStatReplyDefault,
            Status::ReplyNo => crate::sys_umbrella::kOfxStatReplyNo,
            Status::ReplyYes => crate::sys_umbrella::kOfxStatReplyYes,
            Status::Unlicensed => crate::sys_umbrella::kOfxStatUnlicensed,
            Status::Unknown(status) => status,
        }
    }
}

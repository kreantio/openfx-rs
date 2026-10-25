use crate::sys_umbrella::{OfxStatus, kOfxStatOK};

pub mod enums {
    pub use crate::generated::low::c_enums::*;
    pub use crate::generated::low::meta_enums::*;
}

pub use crate::generated::low::c_statuses::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[must_use]
pub struct Status(OfxStatus);

impl Status {
    pub const OK: Status = Status(kOfxStatOK);

    pub fn sys(&self) -> OfxStatus {
        self.0
    }
}

impl From<OfxStatus> for Status {
    fn from(sys_status: OfxStatus) -> Self {
        Status(sys_status)
    }
}

impl From<StatusError> for Status {
    fn from(status: StatusError) -> Self {
        Status(status.into())
    }
}

impl From<StatusReply> for Status {
    fn from(reply: StatusReply) -> Self {
        Status(reply.into())
    }
}

impl From<Status> for OfxStatus {
    fn from(status: Status) -> Self {
        status.0
    }
}

/// A result going to the other side: if you are writing a plugin, it goes to
/// the host; if you are writing a host, it goes to the plugin.
pub type OutboundResult = Result<Status, StatusError>;

/// A “result” coming from the other side: if you are writing a plugin, it comes
/// from the host; if you are writing a host, it comes from the plugin.
///
/// Despite the name, [`std::ops::Try`] is not implemented for this type.
/// Callers are expected to handle the variants explicitly. This prevents a
/// status from being accidentally propagated back to the side it came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[must_use]
pub enum InboundResult<T> {
    Ok(T),
    KnownError(StatusError),
    KnownReply(StatusReply),
    /// Use [`Self::sys_matches`] for matching unknown statuses instead. Even if
    /// matching this variant for unrecognized statuses works now, it will break
    /// when this crate adds dedicated variants to [`StatusError`] or
    /// [`StatusReply`] in a future release.
    UnknownDontUseThisDirectly(OfxStatus),
}

impl InboundResult<()> {
    pub fn from_sys(sys_status: OfxStatus) -> Self {
        if sys_status == kOfxStatOK {
            return Self::Ok(());
        }
        if let Some(status) = StatusError::try_from_sys(sys_status) {
            return Self::KnownError(status);
        }
        if let Some(reply) = StatusReply::try_from_sys(sys_status) {
            return Self::KnownReply(reply);
        }

        Self::UnknownDontUseThisDirectly(sys_status)
    }
}

impl<T> InboundResult<T> {
    pub fn sys_status(&self) -> OfxStatus {
        match &self {
            Self::Ok(_) => kOfxStatOK,
            Self::KnownError(status) => (*status).into(),
            Self::KnownReply(reply) => (*reply).into(),
            Self::UnknownDontUseThisDirectly(sys_status) => *sys_status,
        }
    }

    pub fn sys_matches(&self, sys_status: OfxStatus) -> bool {
        self.sys_status() == sys_status
    }

    pub fn ok(self) -> Option<T> {
        match self {
            Self::Ok(value) => Some(value),
            _ => None,
        }
    }

    pub fn known_error(&self) -> Option<StatusError> {
        match self {
            Self::KnownError(status) => Some(*status),
            _ => None,
        }
    }

    pub fn known_reply(&self) -> Option<StatusReply> {
        match self {
            Self::KnownReply(reply) => Some(*reply),
            _ => None,
        }
    }

    pub fn is_ok(&self) -> bool {
        matches!(self, Self::Ok(_))
    }
    pub fn ok_or<E>(self, err: E) -> Result<T, E> {
        self.ok().ok_or(err)
    }
    pub fn map_ok<U, F>(self, f: F) -> InboundResult<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Self::Ok(value) => InboundResult::Ok(f(value)),
            Self::KnownError(status) => InboundResult::KnownError(status),
            Self::KnownReply(reply) => InboundResult::KnownReply(reply),
            Self::UnknownDontUseThisDirectly(sys_status) => {
                InboundResult::UnknownDontUseThisDirectly(sys_status)
            }
        }
    }
    pub fn ok_and_then<U, F>(self, f: F) -> InboundResult<U>
    where
        F: FnOnce(T) -> InboundResult<U>,
    {
        match self {
            Self::Ok(value) => f(value),
            Self::KnownError(status) => InboundResult::KnownError(status),
            Self::KnownReply(reply) => InboundResult::KnownReply(reply),
            Self::UnknownDontUseThisDirectly(sys_status) => {
                InboundResult::UnknownDontUseThisDirectly(sys_status)
            }
        }
    }

    pub fn is_known_error(&self) -> bool {
        self.known_error().is_some()
    }

    pub fn is_known_reply(&self) -> bool {
        self.known_reply().is_some()
    }
    pub fn known_reply_or<E>(self, err: E) -> Result<StatusReply, E> {
        self.known_reply().ok_or(err)
    }
}

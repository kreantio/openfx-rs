use std::convert::Infallible;

use crate::sys_umbrella::{OfxStatus, kOfxStatOK};

pub use crate::generated::low::c_statuses::*;

pub mod enums {
    pub use crate::generated::low::c_enums::*;
    pub use crate::generated::low::meta_enums::*;
}

/// A result coming from the other side: if you are writing a plugin, it comes
/// from the host; if you are writing a host, it comes from the plugin.
///
/// Note that at the API boundary, `T` is always `()`, and the `Ok` value, if
/// any, is typically retrieved by other means in functions that wrap the API
/// invocations, e.g., through an output parameter pointer.
pub type InboundResult<T> = Result<Okish<T>, StatusError>;
/// A result going to the other side: if you are writing a plugin, it goes
/// to the host; if you are writing a host, it goes to the plugin.
///
/// Note that at the API boundary, `T` is always `()`.
pub type OutboundResult<T> = Result<Okish<T>, OutboundError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Okish<T> {
    Ok(T),
    Reply(StatusReply),
    /// Use [`Self::sys_matches`] for matching unknown statuses instead. Even if
    /// matching this variant for unrecognized statuses works now, it will break
    /// when this crate adds dedicated variants for those statuses in a future
    /// release.
    ///
    /// This is also used to hold custom non-error statuses that this crate does
    /// not recognize but that you want to pass to the other side.
    UnknownOrCustomDontMatchAgainstThisDirectly(OfxStatus),
}

pub enum OutboundError {
    Known(StatusError),
    Custom(OfxStatus),
}

impl<T> From<StatusReply> for Okish<T> {
    fn from(reply: StatusReply) -> Self {
        Okish::Reply(reply)
    }
}

impl Okish<()> {
    pub fn inbound_result_from_sys(sys_status: OfxStatus) -> InboundResult<()> {
        if sys_status == kOfxStatOK {
            return Ok(Okish::Ok(()));
        }
        if let Some(status) = StatusError::try_from_sys(sys_status) {
            return Err(status);
        }
        if let Some(reply) = StatusReply::try_from_sys(sys_status) {
            return Ok(reply.into());
        }

        Ok(Okish::UnknownOrCustomDontMatchAgainstThisDirectly(
            sys_status,
        ))
    }

    /// Creates an [`Okish::UnknownOrCustomDontMatchAgainstThisDirectly`]
    /// without checking the type of the status.
    pub fn force_custom_from_sys(sys_status: OfxStatus) -> Self {
        Okish::UnknownOrCustomDontMatchAgainstThisDirectly(sys_status)
    }
}

impl<T> Okish<T> {
    pub fn sys_status(&self) -> OfxStatus {
        match &self {
            Okish::Ok(_) => kOfxStatOK,
            Okish::Reply(status) => (*status).into(),
            Okish::UnknownOrCustomDontMatchAgainstThisDirectly(sys_status) => *sys_status,
        }
    }

    pub fn sys_matches(&self, sys_status: OfxStatus) -> bool {
        self.sys_status() == sys_status
    }

    pub fn result_ok<E>(value: T) -> Result<Okish<T>, E> {
        Ok(Okish::Ok(value))
    }

    pub fn ok(self) -> Option<T> {
        match self {
            Okish::Ok(value) => Some(value),
            _ => None,
        }
    }
    pub fn is_ok(&self) -> bool {
        matches!(self, Okish::Ok(_))
    }
    pub fn ok_or<E>(self, err: E) -> Result<T, E> {
        self.ok().ok_or(err)
    }
    pub fn map_ok<U, F>(self, f: F) -> Okish<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Okish::Ok(value) => Okish::Ok(f(value)),
            Okish::Reply(reply) => Okish::Reply(reply),
            Okish::UnknownOrCustomDontMatchAgainstThisDirectly(sys_status) => {
                Okish::UnknownOrCustomDontMatchAgainstThisDirectly(sys_status)
            }
        }
    }
    pub fn ok_into_result(self) -> Result<T, Okish<Infallible>> {
        match self {
            Okish::Ok(value) => Ok(value),
            Okish::Reply(status_reply) => Err(Okish::Reply(status_reply)),
            Okish::UnknownOrCustomDontMatchAgainstThisDirectly(sys_status) => Err(
                Okish::UnknownOrCustomDontMatchAgainstThisDirectly(sys_status),
            ),
        }
    }

    pub fn reply(self) -> Option<StatusReply> {
        match self {
            Okish::Reply(value) => Some(value),
            _ => None,
        }
    }
    pub fn is_reply(&self) -> bool {
        matches!(self, Okish::Reply(_))
    }
    pub fn reply_or<E>(self, err: E) -> Result<StatusReply, E> {
        self.reply().ok_or(err)
    }
}

impl Okish<Infallible> {
    pub fn into_other<T>(self) -> Okish<T> {
        match self {
            Okish::Ok(value) => match value {},
            Okish::Reply(reply) => Okish::Reply(reply),
            Okish::UnknownOrCustomDontMatchAgainstThisDirectly(sys_status) => {
                Okish::UnknownOrCustomDontMatchAgainstThisDirectly(sys_status)
            }
        }
    }
}

impl From<StatusError> for OutboundError {
    fn from(err: StatusError) -> Self {
        OutboundError::Known(err)
    }
}

impl From<OutboundError> for OfxStatus {
    fn from(err: OutboundError) -> Self {
        match err {
            OutboundError::Known(status) => status.into(),
            OutboundError::Custom(sys_status) => sys_status,
        }
    }
}

impl OutboundError {
    /// Creates an [`OutboundError`] in which known error statuses become
    /// [`OutboundError::Known`], while all other statuses become
    /// [`OutboundError::Custom`] without verifying that they really are
    /// errors.
    pub fn force_from_sys(sys_status: OfxStatus) -> Self {
        if let Some(status) = StatusError::try_from_sys(sys_status) {
            return OutboundError::Known(status);
        }
        OutboundError::Custom(sys_status)
    }
}

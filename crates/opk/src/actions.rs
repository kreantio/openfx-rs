use openfx::low::{Status, StatusError, StatusReply};
use snafu::Snafu;

use crate::{host::FetchSuiteError, plugin_instance::PluginInstanceHelperError};

pub mod image_effect;

pub type CommonActionResult = Result<CommonActionOutput, CommonActionError>;

pub enum CommonActionOutput {
    Ok,
    ReplyDefault,
}

impl CommonActionOutput {
    pub fn status(&self) -> Status {
        match self {
            CommonActionOutput::Ok => Status::OK,
            CommonActionOutput::ReplyDefault => StatusReply::ReplyDefault.into(),
        }
    }
}

#[derive(Debug, Snafu)]
pub enum CommonActionError {
    CustomErrorStatus {
        status: StatusError,
    },
    CustomStatus {
        status: Status,
    },

    FetchSuite {
        source: FetchSuiteError,
    },
    PluginInstanceHelper {
        source: crate::plugin_instance::PluginInstanceHelperError,
    },
}

impl From<StatusError> for CommonActionError {
    fn from(status: StatusError) -> Self {
        CommonActionError::CustomErrorStatus { status }
    }
}

impl From<FetchSuiteError> for CommonActionError {
    fn from(source: FetchSuiteError) -> Self {
        CommonActionError::FetchSuite { source }
    }
}
impl From<&FetchSuiteError> for CommonActionError {
    fn from(source: &FetchSuiteError) -> Self {
        CommonActionError::FetchSuite {
            source: source.clone(),
        }
    }
}

impl From<PluginInstanceHelperError> for CommonActionError {
    fn from(source: PluginInstanceHelperError) -> Self {
        CommonActionError::PluginInstanceHelper { source }
    }
}

impl CommonActionError {
    pub fn status(&self) -> Status {
        match self {
            CommonActionError::CustomErrorStatus { status } => (*status).into(),
            CommonActionError::CustomStatus { status } => *status,
            CommonActionError::FetchSuite { source: _ } => {
                StatusError::ErrMissingHostFeature.into()
            }
            CommonActionError::PluginInstanceHelper { source: _ } => StatusError::Failed.into(),
        }
    }
}

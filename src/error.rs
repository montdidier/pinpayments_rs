use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::params::to_snakecase;

/// An error encountered using the Pin Payments API.
#[derive(Debug, Error)]
pub enum PinError {
    #[error("error reported by Pin Payments: {0}")]
    PinPayments(#[from] ErrorResponse),
    #[error("error serializing or deserializing a querystring: {0}")]
    QueryStringSerialize(#[from] serde_path_to_error::Error<serde_qs::Error>),
    #[error("error serializing or deserializing a request")]
    JSONSerialize(#[from] serde_path_to_error::Error<serde_json::Error>),
    #[error("error communicating with Pin Payments: {0}")]
    ClientError(String),
    #[error("timeout communicating with Pin Payments")]
    Timeout,
}

impl From<http_types::Error> for PinError {
    fn from(err: http_types::Error) -> PinError {
        PinError::ClientError(err.to_string())
    }
}

/// The list of possible values for a RequestError code.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ErrorCode {
    ParameterMissing,
    ParameterUnknown,
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", to_snakecase(&format!("{:?}", self)))
    }
}

#[derive(Debug, Default, Deserialize, Error)]
#[error("{error} message: {error_description}")]
pub struct ErrorResponse {
    pub error: String,
    pub error_description: String,
}

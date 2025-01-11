//! Error types for the crate.
use thiserror::Error;

use crate::api;

/// An alias for result types returned by this crate.
pub type JikanApiResult<T> = Result<T, JikanApiError>;

/// Errors from the jikan.moe api client.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum JikanApiError {
    /// Error from the jikan.moe API
    #[error("API error: {0}")]
    Api(#[from] api::ApiError<RestError>),
    /// Error parsing URL.
    #[error("url parse error: {0}")]
    Parse(#[from] url::ParseError),
}

/// Error communicating with the REST endpoint.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum RestError {
    /// Reqwest client error.
    #[error("communication: {0}")]
    Communication(#[from] reqwest::Error),
    /// HTTP protocol error.
    #[error("HTTP error: {0}")]
    Http(#[from] http::Error),
}

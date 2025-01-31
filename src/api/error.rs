use std::error::Error;

use thiserror::Error;

/// Errors from response.
#[derive(Debug, Error)]
pub enum ResponseError {
    #[error("Parsing JSON: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("Deserializing value: {source}")]
    DataType {
        source: serde_json::Error,
        value: serde_json::Value,
        type_name: &'static str,
    },
    #[error("HTTP error: {status}")]
    HttpStatus {
        value: serde_json::Value,
        status: http::StatusCode,
    },
}

/// Errors that occur when creating form data.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum BodyError {
    /// Error serializing body data from form paramaters.
    #[error("URL encode error: {0}")]
    UrlEncoded(#[from] serde_urlencoded::ser::Error),
    #[error("JSON encode error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Errors that occur from API endpoints.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    /// Error creating body data.
    #[error("failed to create form data: {0}")]
    Body(#[from] BodyError),
    /// The client encountered an error.
    #[error("client error: {0}")]
    Client(E),
    /// The URL failed to parse.
    #[error("url parse error: {0}")]
    Parse(#[from] url::ParseError),
    /// Error in response.
    #[error("Error in the HTTP response at url [{url}]: source")]
    Response {
        /// Source of the error.
        source: ResponseError,
        /// URL of the error.
        url: http::Uri,
    },
}

impl<E> ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    /// Create an API error from a client error.
    pub fn client(source: E) -> Self {
        Self::Client(source)
    }

    pub(crate) fn from_http_response(source: ResponseError, url: http::Uri) -> Self {
        Self::Response { source, url }
    }
}

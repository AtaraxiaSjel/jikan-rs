use std::error::Error;

use async_trait::async_trait;
use bytes::Bytes;
use http::{
    Response,
    request::Builder as RequestBuilder,
};
use url::Url;

use super::error::ApiError;

/// A parent trait representing a client which can communicate with jikan.moe
pub trait RestClient {
    /// The error that may occur for this client
    type Error: Error + Send + Sync + 'static;

    /// Get the URL for the endpoint for the client.
    ///
    /// This method adds the hostname for the target api.
    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>>;
}

/// A trait representing a blocking client which can communicate with jikan.moe
pub trait Client: RestClient {
    /// Send a REST query
    fn rest(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>>;
}

/// A trait representing an asynchronous client which can communicate with
/// jikan.moe
#[async_trait]
pub trait AsyncClient: RestClient {
    /// Send a REST query asynchronously
    async fn rest_async(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>>;
}

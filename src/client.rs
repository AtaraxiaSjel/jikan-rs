use std::convert::TryInto;

use async_trait::async_trait;
use futures::TryFutureExt;
use log::debug;
use reqwest::{
    Client as AsyncHttpClient,
    blocking::Client as HttpClient,
};
use url::Url;

use crate::{
    api,
    error::RestError,
};

const API_BASE_URL: &str = "https://api.jikan.moe/v4/";

/// A client for communicating with the jikan.moe API
#[derive(Clone, Debug)]
pub struct JikanApiClient {
    client: HttpClient,
    rest_url: Url,
}

impl JikanApiClient {
    /// Create a new jikan.moe API client.
    pub fn new() -> Self {
        let rest_url =
            Url::parse(API_BASE_URL).expect("Unable to parse API_BASE_URL into url::Url");
        Self {
            client: HttpClient::new(),
            rest_url,
        }
    }
}

impl Default for JikanApiClient {
    fn default() -> Self {
        Self::new()
    }
}

impl api::RestClient for JikanApiClient {
    type Error = RestError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, api::ApiError<Self::Error>> {
        debug!("REST api call {}", endpoint);
        self.rest_url
            .join(endpoint.trim_start_matches('/'))
            .map_err(From::from)
    }
}

impl api::Client for JikanApiClient {
    fn rest(
        &self,
        request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<http::Response<bytes::Bytes>, api::ApiError<Self::Error>> {
        let call = || -> Result<_, RestError> {
            let http_request = request.body(body)?;
            let request: reqwest::blocking::Request = http_request.try_into()?;
            let rsp = self.client.execute(request)?;

            let mut http_rsp = http::Response::builder()
                .status(rsp.status())
                .version(rsp.version());
            let headers = http_rsp.headers_mut().unwrap();
            for (key, val) in rsp.headers() {
                headers.insert(key, val.clone());
            }
            http_rsp.body(rsp.bytes()?).map_err(From::from)
        };
        call().map_err(api::ApiError::client)
    }
}

/// An asynchronous client for communicating with the jikan.moe API
#[derive(Clone, Debug)]
pub struct JikanApiClientAsync {
    client: AsyncHttpClient,
    rest_url: Url,
}

impl JikanApiClientAsync {
    /// Create a new asynchronous jikan.moe API client
    pub fn new() -> Self {
        let rest_url =
            Url::parse(API_BASE_URL).expect("Unable to parse API_BASE_URL into url::Url");
        let client = AsyncHttpClient::new();
        Self { client, rest_url }
    }
}

impl Default for JikanApiClientAsync {
    fn default() -> Self {
        Self::new()
    }
}

impl api::RestClient for JikanApiClientAsync {
    type Error = RestError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, api::ApiError<Self::Error>> {
        debug!("REST api call {}", endpoint);
        self.rest_url
            .join(endpoint.trim_start_matches('/'))
            .map_err(From::from)
    }
}

#[async_trait]
impl api::AsyncClient for JikanApiClientAsync {
    async fn rest_async(
        &self,
        request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<http::Response<bytes::Bytes>, api::ApiError<Self::Error>> {
        let call = || async {
            let http_request = request.body(body)?;
            let request: reqwest::Request = http_request.try_into()?;
            let rsp = self.client.execute(request).await?;

            let mut http_rsp = http::Response::builder()
                .status(rsp.status())
                .version(rsp.version());
            let headers = http_rsp.headers_mut().unwrap();
            for (key, val) in rsp.headers() {
                headers.insert(key, val.clone());
            }
            http_rsp.body(rsp.bytes().await?).map_err(From::from)
        };
        call().map_err(api::ApiError::client).await
    }
}

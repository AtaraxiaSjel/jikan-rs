use bytes::Bytes;
use http::{
    header,
    request::Builder as RequestBuilder,
};
use serde::de::DeserializeOwned;

use super::{
    ApiError,
    RestClient,
    Root,
    endpoint::Endpoint,
    error::ResponseError,
    page::{
        InnerState,
        Pageable,
    },
};

pub fn url_to_http_uri(url: url::Url) -> http::Uri {
    url.as_str()
        .parse::<http::Uri>()
        .expect("failed to parse url::Url as http::Uri")
}

pub(crate) fn build_request<E, C>(
    endpoint: &E,
    client: &C,
) -> Result<(RequestBuilder, Vec<u8>), ApiError<C::Error>>
where
    E: Endpoint,
    C: RestClient,
{
    let url = client.rest_endpoint(&endpoint.endpoint())?;
    build_request_internal(url, endpoint, client)
}

pub(crate) fn build_paged_request<E, C>(
    state: &InnerState<'_, E>,
    client: &C,
) -> Result<(RequestBuilder, Vec<u8>), ApiError<C::Error>>
where
    E: Endpoint + Pageable,
    C: RestClient,
{
    let mut url = client.rest_endpoint(&state.endpoint.endpoint())?;
    let mut params = state.endpoint.query_params()?;
    params.extend_from(&state)?;
    params.apply_to(&mut url);

    build_request_internal(url, state.endpoint, client)
}

pub(crate) fn build_request_internal<E, C>(
    mut url: url::Url,
    endpoint: &E,
    _client: &C,
) -> Result<(RequestBuilder, Vec<u8>), ApiError<C::Error>>
where
    E: Endpoint,
    C: RestClient,
{
    endpoint.query_params()?.apply_to(&mut url);

    let req = RequestBuilder::new()
        .method(endpoint.method())
        .uri(url_to_http_uri(url));
    if let Some((mime, data)) = endpoint.body()? {
        let req = req.header(header::CONTENT_TYPE, mime);
        Ok((req, data))
    } else {
        Ok((req, Vec::new()))
    }
}

pub(crate) fn deserialize_response<T>(rsp: http::Response<Bytes>) -> Result<Root<T>, ResponseError>
where
    T: DeserializeOwned,
{
    let status = rsp.status();
    let value = serde_json::from_slice(rsp.body())?;
    if !status.is_success() {
        return Err(ResponseError::HttpStatus { value, status });
    }

    serde_json::from_value::<Root<T>>(value.clone()).map_err(|err| ResponseError::DataType {
        source: err,
        value,
        type_name: std::any::type_name::<T>(),
    })
}

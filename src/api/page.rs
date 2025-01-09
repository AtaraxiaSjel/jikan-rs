use async_trait::async_trait;
use log::debug;
use serde::{
    Serialize,
    de::DeserializeOwned,
};

use super::{
    ApiError,
    AsyncClient,
    AsyncQuery,
    Client,
    Query,
    RestClient,
    endpoint::Endpoint,
    utils,
};
use crate::types::Pagination;

/// Marker trait to indicate that an endpoint is pageable.
pub trait Pageable {}

// Adapters specific to [`Pageable`] endpoints.
pub trait PagedEndpointExt<'a, E> {
    /// Create an Iterator over the results of the paginated endpoint.
    fn iter<T, C>(&'a self, client: &'a C) -> PagedIter<'a, E, C, T>
    where
        C: RestClient,
        T: DeserializeOwned;
}

pub trait AsyncIterator {
    type Item;

    // async fn next(&mut self) -> Option<Self::Item>;
    fn next(&mut self) -> impl Future<Output = Option<Self::Item>> + Send;
}

pub struct PagedIter<'a, E, C, T> {
    client: &'a C,
    state: InnerState<'a, E>,
    current_page: Vec<T>,
    last_page: bool,
}

#[derive(Debug, Serialize, Builder)]
#[builder(pattern = "owned")]
pub struct InnerState<'a, E> {
    #[serde(skip)]
    pub(crate) endpoint: &'a E,
    pub(crate) page: u32,
}

impl<'a, E, C, T> PagedIter<'a, E, C, T>
where
    E: Endpoint + Pageable,
{
    pub(crate) fn new(endpoint: &'a E, client: &'a C) -> Self {
        let state = InnerStateBuilder::default()
            .endpoint(endpoint)
            .page(1)
            .build()
            .unwrap();
        Self {
            client,
            state,
            current_page: Vec::new(),
            last_page: false,
        }
    }
}

impl<E, C, T> Iterator for PagedIter<'_, E, C, T>
where
    E: Endpoint + Pageable,
    T: DeserializeOwned,
    C: Client,
{
    type Item = Result<T, ApiError<C::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_page.is_empty() {
            if self.last_page {
                return None;
            }
            debug!("Fetching page {}", self.state.page);
            (self.current_page, self.last_page) = match self.state.query(self.client) {
                Ok((data, pagination)) => (data, !pagination.has_next_page),
                Err(err) => {
                    debug!("Error in query: {:?}", err);
                    return Some(Err(err));
                }
            };

            self.state.page += 1;

            self.current_page.reverse();
        }
        self.current_page.pop().map(Ok)
    }
}

impl<E, C, T> AsyncIterator for PagedIter<'_, E, C, T>
where
    E: Endpoint + Pageable + Sync,
    T: DeserializeOwned + Send + 'static,
    C: AsyncClient + Sync,
{
    type Item = Result<T, ApiError<C::Error>>;

    async fn next(&mut self) -> Option<Self::Item> {
        if self.current_page.is_empty() {
            if self.last_page {
                return None;
            }
            debug!("Fetching page {}", self.state.page);
            (self.current_page, self.last_page) = match self.state.query_async(self.client).await {
                Ok((data, pagination)) => (data, !pagination.has_next_page),
                Err(err) => {
                    debug!("Error in query: {:?}", err);
                    return Some(Err(err));
                }
            };

            self.state.page += 1;

            self.current_page.reverse();
        }
        self.current_page.pop().map(Ok)
    }
}

impl<'a, E> PagedEndpointExt<'a, E> for E
where
    E: Endpoint + Pageable,
{
    fn iter<T, C>(&'a self, client: &'a C) -> PagedIter<'a, E, C, T>
    where
        C: RestClient,
        T: DeserializeOwned,
    {
        PagedIter::new(self, client)
    }
}

impl<E, T, C> Query<(Vec<T>, Pagination), C> for InnerState<'_, E>
where
    E: Endpoint + Pageable,
    T: DeserializeOwned,
    C: Client,
{
    fn query(&self, client: &C) -> Result<(Vec<T>, Pagination), ApiError<C::Error>> {
        let (req, data) = utils::build_paged_request(self, client)?;

        let url = req.uri_ref().cloned().unwrap_or_default();

        let rsp = client.rest(req, data)?;

        utils::deserialize_response::<_>(rsp)
            .map(|value| (value.data, value.pagination.unwrap_or_default()))
            .map_err(|err| ApiError::from_http_response(err, url))
    }
}

#[async_trait]
impl<E, T, C> AsyncQuery<(Vec<T>, Pagination), C> for InnerState<'_, E>
where
    E: Endpoint + Pageable + Sync,
    T: DeserializeOwned + Send + 'static,
    C: AsyncClient + Sync,
{
    async fn query_async(&self, client: &C) -> Result<(Vec<T>, Pagination), ApiError<C::Error>> {
        let (req, data) = utils::build_paged_request(self, client)?;

        let url = req.uri_ref().cloned().unwrap_or_default();

        let rsp = client.rest_async(req, data).await?;

        utils::deserialize_response::<_>(rsp)
            .map(|value| (value.data, value.pagination.unwrap_or_default()))
            .map_err(|err| ApiError::from_http_response(err, url))
    }
}

use std::borrow::Cow;

use http::Method;
use serde::Serialize;

use crate::api::{
    endpoint::Endpoint,
    error::BodyError,
    page::Pageable,
    query_params::QueryParams,
};

/// Retrieves anime reviews.
#[derive(Debug, Clone, Builder, Serialize)]
#[builder(setter(into))]
pub struct Reviews {
    #[doc = r"`ID` of the anime"]
    #[serde(skip)]
    id: u32,
    #[doc = r"Any reviews left during an ongoing anime"]
    #[builder(default, setter(strip_option))]
    preliminary: Option<bool>,
    #[doc = r"Any reviews that are tagged as a spoiler"]
    #[builder(default, setter(strip_option))]
    spoilers: Option<bool>,
}

impl Reviews {
    /// Create a builder for this endpoint.
    pub fn builder() -> ReviewsBuilder {
        ReviewsBuilder::default()
    }
}

impl Endpoint for Reviews {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/anime/{}/reviews", self.id).into()
    }

    fn query_params(&self) -> Result<QueryParams<'_>, BodyError> {
        QueryParams::with(self)
    }
}

impl Pageable for Reviews {}

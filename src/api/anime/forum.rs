use std::borrow::Cow;

use http::Method;
use serde::Serialize;

use crate::api::{
    Query,
    endpoint::Endpoint,
    error::BodyError,
    query_params::QueryParams,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
enum ForumFilter {
    All,
    Episode,
    Other,
}

/// Retrieves a list of forum topics related to the entry.
#[derive(Debug, Clone, Builder, Serialize)]
#[builder(setter(into))]
pub struct Forum {
    #[doc = r"`ID` of the anime"]
    #[serde(skip)]
    id: u32,
    #[doc = r"Filter forum topics"]
    #[builder(default, setter(strip_option))]
    filter: Option<ForumFilter>,
}

impl Forum {
    /// Create a builder for this endpoint.
    pub fn builder() -> ForumBuilder {
        ForumBuilder::default()
    }
}

impl Endpoint for Forum {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/anime/{}/forum", self.id).into()
    }

    fn query_params(&self) -> Result<QueryParams<'_>, BodyError> {
        QueryParams::with(self)
    }
}

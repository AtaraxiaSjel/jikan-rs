use std::borrow::Cow;

use http::Method;

use crate::api::{
    endpoint::Endpoint,
    page::Pageable,
};

/// Retrieves a list of news articles related to the entry.
#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct News {
    #[doc = r"`ID` of the anime"]
    id: u32,
}

impl News {
    /// Create a builder for this endpoint.
    pub fn builder() -> NewsBuilder {
        NewsBuilder::default()
    }
}

impl Endpoint for News {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/anime/{}/news", self.id).into()
    }
}

impl Pageable for News {}

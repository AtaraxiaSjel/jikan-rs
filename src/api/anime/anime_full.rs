use std::borrow::Cow;

use http::Method;

use crate::api::endpoint::Endpoint;

/// Retrieves a complete anime resource data.
#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct AnimeFull {
    #[doc = r"`ID` of the anime"]
    id: u32,
}

impl AnimeFull {
    /// Create a builder for this endpoint.
    pub fn builder() -> AnimeFullBuilder {
        AnimeFullBuilder::default()
    }
}

impl Endpoint for AnimeFull {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/anime/{}/full", self.id).into()
    }
}

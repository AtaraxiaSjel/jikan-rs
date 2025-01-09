use std::borrow::Cow;

use http::Method;

use crate::api::{
    endpoint::Endpoint,
    page::Pageable,
};

/// Retrieves a list of anime episodes.
#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct Episodes {
    #[doc = r"`ID` of the anime"]
    id: u32,
}

impl Episodes {
    /// Create a builder for this endpoint.
    pub fn builder() -> EpisodesBuilder {
        EpisodesBuilder::default()
    }
}

impl Endpoint for Episodes {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/anime/{}/episodes", self.id).into()
    }
}

impl Pageable for Episodes {}

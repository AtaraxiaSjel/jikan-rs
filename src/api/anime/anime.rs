use std::borrow::Cow;

use http::Method;

use crate::api::endpoint::Endpoint;

/// Retrieves anime resource data.
#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct Anime {
    #[doc = r"`ID` of the anime"]
    id: u32,
}

impl Anime {
    /// Create a builder for this endpoint.
    pub fn builder() -> AnimeBuilder {
        AnimeBuilder::default()
    }
}

impl Endpoint for Anime {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/anime/{}", self.id).into()
    }
}

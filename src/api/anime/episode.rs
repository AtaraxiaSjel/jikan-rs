use std::borrow::Cow;

use http::Method;

use crate::api::endpoint::Endpoint;

/// Retrieves an anime episode resource.
#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct Episode {
    #[doc = r"`ID` of the anime"]
    id: u32,
    #[doc = r"`ID` of the episode"]
    episode: u32,
}

impl Episode {
    /// Create a builder for this endpoint.
    pub fn builder() -> EpisodeBuilder {
        EpisodeBuilder::default()
    }
}

impl Endpoint for Episode {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/anime/{}/episodes/{}", self.id, self.episode).into()
    }
}

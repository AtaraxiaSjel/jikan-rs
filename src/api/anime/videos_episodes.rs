use std::borrow::Cow;

use http::Method;

use crate::api::{
    endpoint::Endpoint,
    page::Pageable,
};

/// Retrieves episode videos related to the entry.
#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct VideosEpisodes {
    #[doc = r"`ID` of the anime"]
    id: u32,
}

impl VideosEpisodes {
    /// Create a builder for this endpoint.
    pub fn builder() -> VideosEpisodesBuilder {
        VideosEpisodesBuilder::default()
    }
}

impl Endpoint for VideosEpisodes {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/anime/{}/videos/episodes", self.id).into()
    }
}

impl Pageable for VideosEpisodes {}

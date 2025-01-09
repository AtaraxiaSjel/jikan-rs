use std::borrow::Cow;

use http::Method;

use crate::api::endpoint::Endpoint;

/// Retrieves videos related to the entry.
#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct Videos {
    #[doc = r"`ID` of the anime"]
    id: u32,
}

impl Videos {
    /// Create a builder for this endpoint.
    pub fn builder() -> VideosBuilder {
        VideosBuilder::default()
    }
}

impl Endpoint for Videos {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/anime/{}/videos", self.id).into()
    }
}

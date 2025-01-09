use std::borrow::Cow;

use http::Method;

use crate::api::endpoint::Endpoint;

/// Retrieves anime streaming links.
#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct Streaming {
    #[doc = r"`ID` of the anime"]
    id: u32,
}

impl Streaming {
    /// Create a builder for this endpoint.
    pub fn builder() -> StreamingBuilder {
        StreamingBuilder::default()
    }
}

impl Endpoint for Streaming {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/anime/{}/streaming", self.id).into()
    }
}

use std::borrow::Cow;

use http::Method;

use crate::api::endpoint::Endpoint;

/// Retrieves pictures related to the entry.
#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct Pictures {
    #[doc = r"`ID` of the anime"]
    id: u32,
}

impl Pictures {
    /// Create a builder for this endpoint.
    pub fn builder() -> PicturesBuilder {
        PicturesBuilder::default()
    }
}

impl Endpoint for Pictures {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/anime/{}/pictures", self.id).into()
    }
}

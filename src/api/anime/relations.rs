use std::borrow::Cow;

use http::Method;

use crate::api::endpoint::Endpoint;

/// Retrieves anime relations.
#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct Relations {
    #[doc = r"`ID` of the anime"]
    id: u32,
}

impl Relations {
    /// Create a builder for this endpoint.
    pub fn builder() -> RelationsBuilder {
        RelationsBuilder::default()
    }
}

impl Endpoint for Relations {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/anime/{}/relations", self.id).into()
    }
}

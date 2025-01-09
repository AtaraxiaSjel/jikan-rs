use std::borrow::Cow;

use http::Method;

use crate::api::endpoint::Endpoint;

/// Retrieves anime recommendations.
#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct Recommendations {
    #[doc = r"`ID` of the anime"]
    id: u32,
}

impl Recommendations {
    /// Create a builder for this endpoint.
    pub fn builder() -> RecommendationsBuilder {
        RecommendationsBuilder::default()
    }
}

impl Endpoint for Recommendations {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/anime/{}/recommendations", self.id).into()
    }
}

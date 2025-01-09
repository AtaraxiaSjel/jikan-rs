use std::borrow::Cow;

use http::Method;

use crate::api::endpoint::Endpoint;

/// Retrieves anime statistics.
#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct MoreInfo {
    #[doc = r"`ID` of the anime"]
    id: u32,
}

impl MoreInfo {
    /// Create a builder for this endpoint.
    pub fn builder() -> MoreInfoBuilder {
        MoreInfoBuilder::default()
    }
}

impl Endpoint for MoreInfo {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/anime/{}/moreinfo", self.id).into()
    }
}

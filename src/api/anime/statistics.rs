use std::borrow::Cow;

use http::Method;

use crate::api::endpoint::Endpoint;

/// Retrieves anime statistics.
#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct Statistics {
    #[doc = r"`ID` of the anime"]
    id: u32,
}

impl Statistics {
    /// Create a builder for this endpoint.
    pub fn builder() -> StatisticsBuilder {
        StatisticsBuilder::default()
    }
}

impl Endpoint for Statistics {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/anime/{}/statistics", self.id).into()
    }
}

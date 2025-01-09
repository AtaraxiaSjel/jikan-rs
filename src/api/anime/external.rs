use std::borrow::Cow;

use http::Method;

use crate::api::endpoint::Endpoint;

/// Retrieves anime external links.
#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct External {
    #[doc = r"`ID` of the anime"]
    id: u32,
}

impl External {
    /// Create a builder for this endpoint.
    pub fn builder() -> ExternalBuilder {
        ExternalBuilder::default()
    }
}

impl Endpoint for External {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/anime/{}/external", self.id).into()
    }
}

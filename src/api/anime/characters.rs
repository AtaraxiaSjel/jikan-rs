use std::borrow::Cow;

use http::Method;

use crate::api::endpoint::Endpoint;

/// Retrieves anime characters resource.
#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct Characters {
    #[doc = r"`ID` of the anime"]
    id: u32,
}

impl Characters {
    /// Create a builder for this endpoint.
    pub fn builder() -> CharactersBuilder {
        CharactersBuilder::default()
    }
}

impl Endpoint for Characters {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/anime/{}/characters", self.id).into()
    }
}

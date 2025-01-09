use std::borrow::Cow;

use http::Method;

use crate::api::endpoint::Endpoint;

/// Retrieves anime themes.
#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct Themes {
    #[doc = r"`ID` of the anime"]
    id: u32,
}

impl Themes {
    /// Create a builder for this endpoint.
    pub fn builder() -> ThemesBuilder {
        ThemesBuilder::default()
    }
}

impl Endpoint for Themes {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/anime/{}/themes", self.id).into()
    }
}

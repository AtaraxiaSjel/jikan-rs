use std::borrow::Cow;

use http::Method;

use crate::api::endpoint::Endpoint;

/// Retrieves anime staff resource.
#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct Staff {
    #[doc = r"`ID` of the anime"]
    id: u32,
}

impl Staff {
    /// Create a builder for this endpoint.
    pub fn builder() -> StaffBuilder {
        StaffBuilder::default()
    }
}

impl Endpoint for Staff {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/anime/{}/staff", self.id).into()
    }
}

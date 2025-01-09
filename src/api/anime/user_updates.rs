use std::borrow::Cow;

use http::Method;

use crate::api::{
    endpoint::Endpoint,
    page::Pageable,
};

/// Retrieves a list of users who have added/updated/removed
/// the entry on their list.
#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct UserUpdates {
    #[doc = r"`ID` of the anime"]
    id: u32,
}

impl UserUpdates {
    /// Create a builder for this endpoint.
    pub fn builder() -> UserUpdatesBuilder {
        UserUpdatesBuilder::default()
    }
}

impl Endpoint for UserUpdates {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/anime/{}/userupdates", self.id).into()
    }
}

impl Pageable for UserUpdates {}

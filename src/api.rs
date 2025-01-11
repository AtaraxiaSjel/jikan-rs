//! Jikan.moe api wrapper.

/// `/anime` endpoints.
pub mod anime;
mod client;
mod endpoint;
mod error;
mod page;
mod query;
mod query_params;
mod utils;

pub use client::{
    AsyncClient,
    Client,
    RestClient,
};
pub use error::ApiError;
pub use page::{
    AsyncIterator,
    PagedEndpointExt,
};
pub use query::{
    AsyncQuery,
    Query,
};

pub use crate::types::Root;

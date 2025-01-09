#![forbid(unsafe_code)]
#![warn(future_incompatible, rust_2024_compatibility, unused)]
#![warn(missing_docs)]
#![warn(clippy::all)]

#[macro_use]
extern crate derive_builder;
mod client;

pub mod api;
pub mod error;
pub mod types;

pub use client::{
    JikanApiClient,
    JikanApiClientAsync,
};

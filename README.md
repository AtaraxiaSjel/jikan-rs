# Jikan.moe API Wrapper for Rust

This crate provides a Rust API wrapper for the Jikan.moe API - an Unofficial MyAnimeList API, which serves as a comprehensive anime and manga database.

For more information about Jikan.moe, [read official docs](https://docs.api.jikan.moe/) or [download openapi specs](https://raw.githubusercontent.com/jikan-me/jikan-rest/master/storage/api-docs/api-docs.json).

## Key Points

- Easy to use, type-safe API for Jikan.moe v4 API (in progress, see [Roadmap](#roadmap))
- Pagination support for list endpoints
- Strongly typed responses
- Async support
- Support for custom HTTP client
- Zero configuration required

## Roadmap

* Implement all endpoints:
  - [x] `/anime`
  - [ ] `/characters`
  - [ ] `/clubs`
  - [ ] `/genres`
  - [ ] `/magazines`
  - [ ] `/manga`
  - [ ] `/people`
  - [ ] `/producers`
  - [ ] `/random`
  - [ ] `/recommendations`
  - [ ] `/reviews`
  - [ ] `/schedules`
  - [ ] `/users`
  - [ ] `/seasons`
  - [ ] `/top`
  - [ ] `/watch`
* Create response types for all endpoints:
  - [x] `/anime`
  - [ ] `/characters`
  - [ ] `/clubs`
  - [ ] `/genres`
  - [ ] `/magazines`
  - [ ] `/manga`
  - [ ] `/people`
  - [ ] `/producers`
  - [ ] `/random`
  - [ ] `/recommendations`
  - [ ] `/reviews`
  - [ ] `/schedules`
  - [ ] `/users`
  - [ ] `/seasons`
  - [ ] `/top`
  - [ ] `/watch`
* [ ] Implement rate limiting for default client
* [ ] Logging with tracing library
* [ ] Add usage examples
* And maybe more.

## Design

This crate's API design is inspired by:
- [speedrun-api](https://docs.rs/speedrun-api/) crate's architecture
- [gitlab](https://docs.rs/gitlab/) crate's API design
- [Designing Rust bindings for REST APIs](https://plume.benboeckel.net/~/JustAnotherBlog/designing-rust-bindings-for-rest-ap-is) by Ben Boeckel

## License

This project is dual-licensed under either:

* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.
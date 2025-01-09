use std::borrow::Cow;

use http::Method;
use serde::Serialize;

use crate::api::{
    endpoint::Endpoint,
    error::BodyError,
    page::Pageable,
    query_params::QueryParams,
};

/// Enum for the 'type' query parameter.
#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnimeType {
    Tv,
    Movie,
    Ova,
    Special,
    Ona,
    Music,
    Cm,
    Pv,
    TvSpecial,
}

/// Enum for the 'status' query parameter.
#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnimeStatus {
    Airing,
    Complete,
    Upcoming,
}

/// Enum for the 'rating' query parameter.
#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnimeRating {
    G,
    PG,
    PG13,
    R17,
    R,
    Rx,
}

/// Enum for the 'order_by' query parameter.
#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnimeOrderBy {
    MalId,
    Title,
    StartDate,
    EndDate,
    Episodes,
    Score,
    ScoredBy,
    Rank,
    Popularity,
    Members,
    Favorites,
}

/// Enum for the 'sort' query parameter.
#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnimeSorting {
    Asc,
    Desc,
}

/// Search and retrieve anime resource data.
#[derive(Debug, Clone, Builder, Serialize)]
#[builder(setter(into))]
pub struct AnimeSearch<'a> {
    // unapproved: bool,
    /// The number of maximum results to return.
    limit: u32,
    /// Query string.
    #[serde(rename = "q")]
    query: Option<Cow<'a, str>>,
    /// The type of anime to search.
    #[serde(rename = "type")]
    query_type: Option<AnimeType>,
    /// Score of anime to search.
    score: Option<f32>,
    /// The minimum score to search.
    min_score: Option<f32>,
    /// The maximum score to search.
    max_score: Option<f32>,
    /// Status of anime to search.
    status: Option<AnimeStatus>,
    /// Audience rating of anime.
    rating: Option<AnimeRating>,
    /// Filter adult entries.
    sfw: Option<bool>,
    // TODO: custom type instead of ids
    #[doc = r"Filter by genre(s) `ID`s. Can pass multiple with a comma as a delimiter"]
    genres: Option<Cow<'a, str>>,
    // TODO: custom type instead of ids
    #[doc = r"Filter by genre(s) `ID`s. Can pass multiple with a comma as a delimiter"]
    genres_exclude: Option<Cow<'a, str>>,
    /// Sorting result by this parameter.
    order_by: Option<AnimeOrderBy>,
    /// Sorting order.
    sorting: Option<AnimeSorting>,
    /// Search entries starting with the given letter.
    letter: Option<Cow<'a, str>>,
    // TODO: custom type instead of ids
    #[doc = r"Filter by producer(s) `ID`s. Can pass multiple with a comma as a delimiter"]
    producers: Option<Cow<'a, str>>,
    // TODO: custom date serializer
    #[doc = r"Filter by starting date. Format: `YYYY-MM-DD`."]
    start_date: Option<Cow<'a, str>>,
    // TODO: custom date serializer
    #[doc = r"Filter by ending date. Format: `YYYY-MM-DD`."]
    end_date: Option<Cow<'a, str>>,
}

impl<'a> AnimeSearch<'a> {
    /// Create a builder for this endpoint.
    pub fn builder() -> AnimeSearchBuilder<'a> {
        AnimeSearchBuilder::default()
    }
}

impl Endpoint for AnimeSearch<'_> {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "/anime".into()
    }

    fn query_params(&self) -> Result<QueryParams<'_>, BodyError> {
        QueryParams::with(self)
    }
}

impl Pageable for AnimeSearch<'_> {}

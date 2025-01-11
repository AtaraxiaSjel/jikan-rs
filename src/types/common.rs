use serde::{
    Deserialize,
    Serialize,
};

/// Represents the root structure of a response, containing data and optional.
/// pagination.
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Root<T> {
    /// The main data payload of the response.
    pub data: T,
    /// Optional pagination details for the response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
}

/// Represents pagination details for a response.
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Pagination {
    /// The last visible page in the pagination.
    pub last_visible_page: u64,
    /// Indicates whether there is a next page available.
    pub has_next_page: bool,
    /// Optional details about the pagination items (count, total, per_page).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<PaginationItems>,
}

/// Represents pagination items, including count, total, and per-page details.
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PaginationItems {
    /// Number of items on the current page.
    pub count: u64,
    /// Total number of items across all pages.
    pub total: u64,
    /// Number of items per page.
    pub per_page: u64,
}

/// Represents the type of anime (e.g., TV, Movie, OVA, etc.).
#[derive(Serialize, Deserialize, Debug)]
pub enum AnimeType {
    /// TV series.
    #[serde(rename = "TV")]
    Tv,
    /// Original Video Animation (OVA).
    #[serde(rename = "OVA")]
    Ova,
    /// Original Net Animation (ONA).
    #[serde(rename = "ONA")]
    Ona,
    /// Movie.
    Movie,
    /// Special episode.
    Special,
    /// Music video or promotional video.
    Music,
}

/// Represents the airing status of an anime.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum AnimeStatus {
    /// The anime is currently airing.
    #[serde(rename = "Currently Airing")]
    Airing,
    /// The anime has finished airing.
    #[serde(rename = "Finished Airing")]
    Complete,
    /// The anime has not yet aired.
    #[serde(rename = "Not yet aired")]
    Upcoming,
}

/// Represents the audience rating of an anime.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum AnimeRating {
    /// Suitable for all ages.
    #[serde(rename = "G - All Ages")]
    G,
    /// Suitable for children.
    #[serde(rename = "PG - Children")]
    Pg,
    /// Suitable for teens 13 or older.
    #[serde(rename = "PG-13 - Teens 13 or older")]
    Pg13,
    /// Restricted to viewers 17 or older (violence & profanity).
    #[serde(rename = "R - 17+ (violence & profanity)")]
    R17,
    /// Restricted to viewers 17 or older (mild nudity).
    #[serde(rename = "R+ - Mild Nudity")]
    R,
    /// Hentai (explicit content).
    #[serde(rename = "Rx - Hentai")]
    Rx,
}

/// Represents the season in which an anime aired.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum AnimeSeason {
    /// Anime aired in the summer season.
    Summer,
    /// Anime aired in the winter season.
    Winter,
    /// Anime aired in the spring season.
    Spring,
    /// Anime aired in the fall season.
    Fall,
}

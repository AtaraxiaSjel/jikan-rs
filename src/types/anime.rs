use iso8601_timestamp::Timestamp;
use serde::{
    Deserialize,
    Serialize,
};

use super::{
    AnimeRating,
    AnimeSeason,
    AnimeStatus,
    AnimeType,
};

/// Represents an anime resource.
/// Root type for the `/anime/{id}` endpoint.
#[derive(Serialize, Deserialize, Debug)]
pub struct Anime {
    /// MyAnimeList ID.
    pub mal_id: u32,

    /// MyAnimeList URL.
    pub url: String,

    /// Images associated with the anime.
    pub images: AnimeImages,

    /// Trailer information.
    pub trailer: TrailerBase,

    /// Whether the entry is pending approval on MyAnimeList.
    pub approved: bool,

    /// All titles associated with the anime.
    pub titles: Vec<Title>,

    /// Main title of the anime.
    pub title: String,

    /// English title of the anime (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title_english: Option<String>,

    /// Japanese title of the anime (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title_japanese: Option<String>,

    /// Other titles or synonyms for the anime.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub title_synonyms: Vec<String>,

    /// Type of the anime (e.g., TV, Movie, OVA, etc.).
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub anime_type: Option<AnimeType>,

    /// Source material the anime is adapted from (e.g., manga, light novel,
    /// etc.).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// Number of episodes in the anime (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub episodes: Option<u32>,

    /// Airing status of the anime (e.g., "Finished Airing", "Currently Airing",
    /// etc.).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<AnimeStatus>,

    /// Whether the anime is currently airing.
    pub airing: bool,

    /// Date range during which the anime aired.
    pub aired: DateRange,

    /// Duration of each episode (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,

    /// Audience rating of the anime (e.g., "G - All Ages", "PG - Children",
    /// etc.).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rating: Option<AnimeRating>,

    /// Average score of the anime (1.00 - 10.00).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,

    /// Number of users who scored the anime.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scored_by: Option<u32>,

    /// Ranking of the anime based on popularity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<u32>,

    /// Popularity rank of the anime.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub popularity: Option<u32>,

    /// Number of users who have added the anime to their list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members: Option<u32>,

    /// Number of users who have favorited the anime.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub favorites: Option<u32>,

    /// Synopsis of the anime.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synopsis: Option<String>,

    /// Background information about the anime.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,

    /// Season in which the anime aired (e.g., "summer", "winter", etc.).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub season: Option<AnimeSeason>,

    /// Year in which the anime aired.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub year: Option<u32>,

    /// Broadcast details of the anime.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub broadcast: Option<Broadcast>,

    /// List of producers involved in the anime.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub producers: Vec<MalUrl>,

    /// List of licensors for the anime.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub licensors: Vec<MalUrl>,

    /// List of studios that produced the anime.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub studios: Vec<MalUrl>,

    /// List of genres associated with the anime.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub genres: Vec<MalUrl>,

    /// List of explicit genres associated with the anime.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub explicit_genres: Vec<MalUrl>,

    /// List of themes associated with the anime.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub themes: Vec<MalUrl>,

    /// List of demographics associated with the anime.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub demographics: Vec<MalUrl>,
}

/// Represents a complete anime resource with additional details.
/// Root type for the `/anime/{id}/full` endpoint.
#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeFull {
    /// MyAnimeList ID.
    pub mal_id: u32,

    /// MyAnimeList URL.
    pub url: String,

    /// Images associated with the anime.
    pub images: AnimeImages,

    /// Trailer information.
    pub trailer: TrailerBase,

    /// Whether the entry is pending approval on MyAnimeList.
    pub approved: bool,

    /// All titles associated with the anime.
    pub titles: Vec<Title>,

    /// Main title of the anime.
    pub title: String,

    /// English title of the anime (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title_english: Option<String>,

    /// Japanese title of the anime (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title_japanese: Option<String>,

    /// Other titles or synonyms for the anime.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub title_synonyms: Vec<String>,

    /// Type of the anime (e.g., TV, Movie, OVA, etc.).
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub anime_type: Option<AnimeType>,

    /// Source material the anime is adapted from (e.g., manga, light novel,
    /// etc.).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// Number of episodes in the anime (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub episodes: Option<u32>,

    /// Airing status of the anime (e.g., "Finished Airing", "Currently Airing",
    /// etc.).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<AnimeStatus>,

    /// Whether the anime is currently airing.
    pub airing: bool,

    /// Date range during which the anime aired.
    pub aired: DateRange,

    /// Duration of each episode (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,

    /// Audience rating of the anime (e.g., "G - All Ages", "PG - Children",
    /// etc.).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rating: Option<AnimeRating>,

    /// Average score of the anime (1.00 - 10.00).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,

    /// Number of users who scored the anime.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scored_by: Option<u32>,

    /// Ranking of the anime based on popularity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<u32>,

    /// Popularity rank of the anime.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub popularity: Option<u32>,

    /// Number of users who have added the anime to their list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members: Option<u32>,

    /// Number of users who have favorited the anime.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub favorites: Option<u32>,

    /// Synopsis of the anime.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synopsis: Option<String>,

    /// Background information about the anime.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,

    /// Season in which the anime aired (e.g., "summer", "winter", etc.).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub season: Option<AnimeSeason>,

    /// Year in which the anime aired.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub year: Option<u32>,

    /// Broadcast details of the anime.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub broadcast: Option<Broadcast>,

    /// List of producers involved in the anime.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub producers: Vec<MalUrl>,

    /// List of licensors for the anime.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub licensors: Vec<MalUrl>,

    /// List of studios that produced the anime.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub studios: Vec<MalUrl>,

    /// List of genres associated with the anime.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub genres: Vec<MalUrl>,

    /// List of explicit genres associated with the anime.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub explicit_genres: Vec<MalUrl>,

    /// List of themes associated with the anime.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub themes: Vec<MalUrl>,

    /// List of demographics associated with the anime.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub demographics: Vec<MalUrl>,

    /// List of related anime.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relations: AnimeRelations,

    /// Opening and ending themes of the anime.
    pub theme: Theme,

    /// External links related to the anime.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub external: AnimeExternal,

    /// Streaming links for the anime.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub streaming: AnimeStreaming,
}

/// Represents a list of characters in an anime.
/// Root type for the `/anime/{id}/characters` endpoint.
pub type AnimeCharacters = Vec<AnimeCharacter>;

/// Represents a character in an anime.
#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeCharacter {
    /// Metadata about the character.
    pub character: CharacterMeta,
    /// Role of the character in the anime.
    pub role: String,
    /// List of voice actors who voiced the character.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub voice_actors: Vec<VoiceActor>,
}

/// Represents metadata about a character.
#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterMeta {
    /// MyAnimeList ID of the character.
    pub mal_id: u32,
    /// MyAnimeList URL of the character.
    pub url: String,
    /// Images of the character.
    pub images: CharacterImages,
    /// Name of the character.
    pub name: String,
}

/// Represents a voice actor for a character.
#[derive(Serialize, Deserialize, Debug)]
pub struct VoiceActor {
    /// Metadata about the voice actor.
    pub person: PersonMeta,
    /// Language in which the character was voiced.
    pub language: String,
}

/// Represents metadata about a person (e.g., voice actor, staff member).
#[derive(Serialize, Deserialize, Debug)]
pub struct PersonMeta {
    /// MyAnimeList ID of the person.
    pub mal_id: u32,
    /// MyAnimeList URL of the person.
    pub url: String,
    /// Images of the person.
    pub images: PeopleImages,
    /// Name of the person.
    pub name: String,
}

/// Represents a list of staff members involved in an anime.
/// Root type for the `/anime/{id}/staff` endpoint.
pub type AnimeStaff = Vec<StaffRole>;

/// Represents a staff member's role in an anime.
#[derive(Serialize, Deserialize, Debug)]
pub struct StaffRole {
    /// Metadata about the staff member.
    pub person: PersonMeta,
    /// Positions held by the staff member.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub positions: Vec<String>,
}

/// Represents a list of episodes in an anime.
/// Root type for the `/anime/{id}/episodes` endpoint.
pub type AnimeEpisodes = Vec<AnimeEpisode>;

/// Represents an episode in an anime.
#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeEpisode {
    /// MyAnimeList ID of the episode.
    pub mal_id: u32,
    /// MyAnimeList URL of the episode.
    pub url: String,
    /// Title of the episode.
    pub title: String,
    /// Japanese title of the episode (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title_japanese: Option<String>,
    /// Romanized title of the episode (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title_romanji: Option<String>,
    /// Duration of the episode in seconds (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,
    /// Date the episode aired (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aired: Option<String>,
    /// Whether the episode is a filler.
    pub filler: bool,
    /// Whether the episode is a recap.
    pub recap: bool,
    /// MyAnimeList forum URL for the episode (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forum_url: Option<String>,
}

/// Represents a list of news articles related to an anime.
/// Root type for the `/anime/{id}/news` endpoint.
pub type AnimeNews = Vec<AnimeNewsMeta>;

/// Represents metadata about a news article related to an anime.
#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeNewsMeta {
    /// MyAnimeList ID of the news article.
    pub mal_id: u32,
    /// URL of the news article.
    pub url: String,
    /// Title of the news article.
    pub title: String,
    /// Date the news article was published.
    pub date: String,
    /// Username of the author.
    pub author_username: String,
    /// URL of the author's profile.
    pub author_url: String,
    /// MyAnimeList forum URL for the news article (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forum_url: Option<String>,
    /// Images associated with the news article.
    pub images: CommonImages,
    /// Number of comments on the news article.
    pub comments: u32,
    /// Excerpt of the news article (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub excerpt: Option<String>,
}

/// Represents a list of forum posts related to an anime.
/// Root type for the `/anime/{id}/forum` endpoint.
pub type AnimeForumPosts = Vec<AnimeForumPost>;

/// Represents a forum post related to an anime.
#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeForumPost {
    /// MyAnimeList ID of the forum post.
    pub mal_id: u32,
    /// URL of the forum post.
    pub url: String,
    /// Title of the forum post.
    pub title: String,
    /// Date the forum post was created.
    pub date: Timestamp,
    /// Username of the author.
    pub author_username: String,
    /// URL of the author's profile.
    pub author_url: String,
    /// Number of comments on the forum post.
    pub comments: u32,
    /// Metadata about the last comment on the forum post.
    pub last_comment: LastComment,
}

/// Represents metadata about the last comment on a forum post.
#[derive(Serialize, Deserialize, Debug)]
pub struct LastComment {
    /// URL of the last comment.
    pub url: String,
    /// Username of the last comment's author.
    pub author_username: String,
    /// URL of the last comment's author.
    pub author_url: String,
    /// Date the last comment was made (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<Timestamp>,
}

/// Represents videos related to an anime.
/// Root type for the `/anime/{id}/videos` endpoint.
#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeVideos {
    /// List of promotional videos.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub promo: Vec<PromoVideo>,
    /// List of episode videos.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub episodes: Vec<EpisodeVideo>,
    /// List of music videos.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub music_videos: Vec<MusicVideo>,
}

/// Represents a promotional video for an anime.
#[derive(Serialize, Deserialize, Debug)]
pub struct PromoVideo {
    /// Title of the promotional video.
    pub title: String,
    /// Trailer information for the promotional video.
    pub trailer: Trailer,
}

/// Represents a list of episode videos for an anime.
/// Root type for the `/anime/{id}/videos/episodes` endpoint.
pub type AnimeVideosEpisodes = Vec<EpisodeVideo>;

/// Represents an episode video for an anime.
#[derive(Serialize, Deserialize, Debug)]
pub struct EpisodeVideo {
    /// MyAnimeList ID of the episode video.
    pub mal_id: u32,
    /// URL of the episode video.
    pub url: String,
    /// Title of the episode video.
    pub title: String,
    /// Episode number of the video.
    pub episode: String,
    /// Images associated with the episode video.
    pub images: CommonImages,
}

/// Represents a music video for an anime.
#[derive(Serialize, Deserialize, Debug)]
pub struct MusicVideo {
    /// Title of the music video.
    pub title: String,
    /// Video information for the music video.
    pub video: Trailer,
    /// Metadata about the music video (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<MusicVideoMeta>,
}

/// Represents metadata about a music video.
#[derive(Serialize, Deserialize, Debug)]
pub struct MusicVideoMeta {
    /// Title of the music video (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Author of the music video (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
}

/// Represents a list of pictures related to an anime.
/// Root type for the `/anime/{id}/pictures` endpoint.
pub type AnimePictures = Vec<AnimePicture>;

/// Represents a picture related to an anime.
#[derive(Serialize, Deserialize, Debug)]
pub struct AnimePicture {
    /// Images associated with the anime picture.
    pub images: CommonImages,
}

/// Represents statistics about an anime.
/// Root type for the `/anime/{id}/statistics` endpoint.
#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeStatistics {
    /// Number of users watching the anime.
    pub watching: u32,
    /// Number of users who completed the anime.
    pub completed: u32,
    /// Number of users who put the anime on hold.
    pub on_hold: u32,
    /// Number of users who dropped the anime.
    pub dropped: u32,
    /// Number of users planning to watch the anime.
    pub plan_to_watch: u32,
    /// Total number of users who have the anime on their list.
    pub total: u32,
    /// List of scores given by users.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scores: Vec<Score>,
}

/// Represents a score given by users for an anime.
#[derive(Serialize, Deserialize, Debug)]
pub struct Score {
    /// Score given by users.
    pub score: u32,
    /// Number of votes for the score.
    pub votes: u32,
    /// Percentage of users who gave this score.
    pub percentage: f32,
}

/// Represents additional information about an anime.
/// Root type for the `/anime/{id}/moreinfo` endpoint.
#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeMoreInfo {
    /// Additional information about the anime.
    pub moreinfo: Option<String>,
}

/// Represents a list of recommendations for an anime.
/// Root type for the `/anime/{id}/recommendations` endpoint.
pub type AnimeRecommendations = Vec<AnimeRecommendation>;

/// Represents a recommendation for an anime.
#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeRecommendation {
    /// MyAnimeList ID of the recommendation.
    pub mal_id: String,
    /// List of entries recommended.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entry: Vec<EntryMeta>,
    /// Content of the recommendation.
    pub content: String,
    /// User who made the recommendation.
    pub user: UserById,
}

/// Represents metadata about an entry (e.g., anime, manga).
#[derive(Serialize, Deserialize, Debug)]
pub struct EntryMeta {
    /// MyAnimeList ID of the entry.
    pub mal_id: u32,
    /// MyAnimeList URL of the entry.
    pub url: String,
    /// Images associated with the entry.
    pub images: AnimeImages,
    /// Title of the entry.
    pub title: String,
}

/// Represents a user by their ID.
#[derive(Serialize, Deserialize, Debug)]
pub struct UserById {
    /// URL of the user's profile.
    pub url: String,
    /// Username of the user.
    pub username: String,
}

/// Represents a list of user updates for an anime.
/// Root type for the `/anime/{id}/userupdates` endpoint.
pub type AnimeUserUpdates = Vec<AnimeUserUpdate>;

/// Represents a user's update for an anime.
#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeUserUpdate {
    /// Metadata about the user.
    pub user: UserMeta,
    /// Score given by the user (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<u32>,
    /// Status of the anime in the user's list.
    pub status: String,
    /// Number of episodes seen by the user (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub episodes_seen: Option<u32>,
    /// Total number of episodes in the anime (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub episodes_total: Option<u32>,
    /// Date of the user's update.
    pub date: String,
}

/// Represents metadata about a user.
#[derive(Serialize, Deserialize, Debug)]
pub struct UserMeta {
    /// Username of the user.
    pub username: String,
    /// URL of the user's profile.
    pub url: String,
    /// Images associated with the user.
    pub images: UserImages,
}

/// Represents an image URL.
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageUrl {
    /// URL of the image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

/// Represents a list of reviews for an anime.
/// Root type for the `/anime/{id}/reviews` endpoint.
pub type AnimeReviews = Vec<AnimeReview>;

/// Represents a review for an anime.
#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeReview {
    /// MyAnimeList ID of the review.
    pub mal_id: u32,
    /// URL of the review.
    pub url: String,
    /// Type of the review.
    #[serde(rename = "type")]
    pub review_type: String,
    /// Reactions to the review.
    pub reactions: Reactions,
    /// Date the review was posted.
    pub date: String,
    /// Content of the review.
    pub review: String,
    /// Score given in the review.
    pub score: u32,
    /// Tags associated with the review.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    /// Whether the review contains spoilers.
    pub is_spoiler: bool,
    /// Whether the review is preliminary.
    pub is_preliminary: bool,
    /// Number of episodes watched by the reviewer.
    pub episodes_watched: u32,
}

/// Represents reactions to a review.
#[derive(Serialize, Deserialize, Debug)]
pub struct Reactions {
    /// Overall reaction count.
    pub overall: u32,
    /// Number of "nice" reactions.
    pub nice: u32,
    /// Number of "love it" reactions.
    pub love_it: u32,
    /// Number of "funny" reactions.
    pub funny: u32,
    /// Number of "confusing" reactions.
    pub confusing: u32,
    /// Number of "informative" reactions.
    pub informative: u32,
    /// Number of "well-written" reactions.
    pub well_written: u32,
    /// Number of "creative" reactions.
    pub creative: u32,
}

/// Represents a list of related anime.
/// Root type for the `/anime/{id}/relations` endpoint.
pub type AnimeRelations = Vec<AnimeRelation>;

/// Represents a relation between anime.
#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeRelation {
    /// Type of relation (e.g., sequel, prequel, etc.).
    pub relation: String,
    /// List of related entries.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entry: Vec<MalUrl>,
}

/// Represents a MyAnimeList URL.
#[derive(Serialize, Deserialize, Debug)]
pub struct MalUrl {
    /// MyAnimeList ID of the entry.
    pub mal_id: u32,
    /// Type of the entry.
    #[serde(rename = "type")]
    pub url_type: String,
    /// Name of the entry.
    pub name: String,
    /// URL of the entry.
    pub url: String,
}

/// Represents themes (openings and endings) of an anime.
/// Root type for the `/anime/{id}/themes` endpoint.
#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeThemes {
    /// Theme data (openings and endings).
    pub data: Theme,
}

/// Represents opening and ending themes of an anime.
#[derive(Serialize, Deserialize, Debug)]
pub struct Theme {
    /// List of opening themes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub openings: Vec<String>,
    /// List of ending themes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endings: Vec<String>,
}

/// Represents a list of external links related to an anime.
/// Root type for the `/anime/{id}/external` endpoint.
pub type AnimeExternal = Vec<ExternalLink>;

/// Represents a list of streaming links for an anime.
/// Root type for the `/anime/{id}/streaming` endpoint.
pub type AnimeStreaming = Vec<ExternalLink>;

/// Represents an external link.
#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalLink {
    /// Name of the external link.
    pub name: String,
    /// URL of the external link.
    pub url: String,
}

/// Represents a list of anime search results.
/// Root type for the `/anime` endpoint.
pub type AnimeSearchResults = Vec<Anime>;

/// Represents image URLs in different sizes.
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageUrls {
    /// URL of the image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// URL of the small-sized image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub small_image_url: Option<String>,
    /// URL of the large-sized image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub large_image_url: Option<String>,
}

/// Represents basic trailer information.
#[derive(Serialize, Deserialize, Debug)]
pub struct TrailerBase {
    /// YouTube ID of the trailer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub youtube_id: Option<String>,
    /// URL of the trailer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Embed URL of the trailer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embed_url: Option<String>,
}

/// Represents detailed trailer information.
#[derive(Serialize, Deserialize, Debug)]
pub struct Trailer {
    /// YouTube ID of the trailer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub youtube_id: Option<String>,
    /// URL of the trailer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Embed URL of the trailer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embed_url: Option<String>,
    /// Images associated with the trailer.
    pub images: TrailerImages,
}

/// Represents a date range.
#[derive(Serialize, Deserialize, Debug)]
pub struct DateRange {
    /// Start date of the range (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// End date of the range (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// Detailed date properties.
    pub prop: DateProp,
}

/// Represents detailed date properties.
#[derive(Serialize, Deserialize, Debug)]
pub struct DateProp {
    /// Start date details.
    pub from: DatePropDetail,
    /// End date details.
    pub to: DatePropDetail,
    /// String representation of the date range (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub string: Option<String>,
}

/// Represents detailed date properties (day, month, year).
#[derive(Serialize, Deserialize, Debug)]
pub struct DatePropDetail {
    /// Day of the date (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub day: Option<u32>,
    /// Month of the date (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub month: Option<u32>,
    /// Year of the date (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub year: Option<u32>,
}

/// Represents broadcast details of an anime.
#[derive(Serialize, Deserialize, Debug)]
pub struct Broadcast {
    /// Day of the week the anime is broadcasted (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub day: Option<String>,
    /// Time of the broadcast (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    /// Timezone of the broadcast (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// String representation of the broadcast schedule (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub string: Option<String>,
}

/// Represents a title (e.g., main title, English title, etc.).
#[derive(Serialize, Deserialize, Debug)]
pub struct Title {
    /// Type of the title (e.g., "Default", "English", etc.).
    #[serde(rename = "type")]
    pub title_type: String,
    /// Title text.
    pub title: String,
}

/// Represents common image formats.
#[derive(Serialize, Deserialize, Debug)]
pub struct CommonImages {
    /// JPG format image.
    pub jpg: ImageUrl,
}

/// Represents character images in different formats.
#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterImages {
    /// JPG format image.
    pub jpg: ImageUrl,
    /// WebP format image.
    pub webp: ImageUrl,
}

/// Represents images of a person (e.g., voice actor, staff member).
#[derive(Serialize, Deserialize, Debug)]
pub struct PeopleImages {
    /// JPG format image.
    pub jpg: ImageUrl,
}

/// Represents anime images in different formats.
#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeImages {
    /// JPG format images.
    pub jpg: ImageUrls,
    /// WebP format images.
    pub webp: ImageUrls,
}

/// Represents user images in different formats.
#[derive(Serialize, Deserialize, Debug)]
pub struct UserImages {
    /// JPG format image.
    pub jpg: ImageUrl,
    /// WebP format image.
    pub webp: ImageUrl,
}

/// Represents trailer images in different sizes.
#[derive(Serialize, Deserialize, Debug)]
pub struct TrailerImages {
    /// URL of the image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// URL of the small-sized image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub small_image_url: Option<String>,
    /// URL of the medium-sized image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub medium_image_url: Option<String>,
    /// URL of the large-sized image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub large_image_url: Option<String>,
    /// URL of the maximum-sized image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_image_url: Option<String>,
}

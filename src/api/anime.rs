#[allow(clippy::module_inception)]
mod anime;
mod anime_full;
mod characters;
mod episode;
mod episodes;
mod external;
mod forum;
mod more_info;
mod news;
mod pictures;
mod recommendations;
mod relations;
mod reviews;
mod search;
mod staff;
mod statistics;
mod streaming;
mod themes;
mod user_updates;
mod videos;
mod videos_episodes;

pub use self::{
    anime::*,
    anime_full::*,
    characters::*,
    episode::*,
    episodes::*,
    external::*,
    forum::*,
    more_info::*,
    news::*,
    pictures::*,
    recommendations::*,
    relations::*,
    reviews::*,
    search::*,
    staff::*,
    statistics::*,
    streaming::*,
    themes::*,
    user_updates::*,
    videos::*,
    videos_episodes::*,
};

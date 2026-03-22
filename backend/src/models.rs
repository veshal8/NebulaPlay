// src/models.rs
use serde::{Deserialize, Serialize};

/// Global state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub tmdb_key: String,
    pub rd_token: String,
}

// -------------------------------
// PUBLIC MODELS (what Flutter sees)
// -------------------------------

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    /// Optional kind for search: "movie" (default) or "anime"
    #[serde(default)]
    pub kind: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct MovieCard {
    pub tmdb_id: i64,
    pub title: String,
    pub poster_url: String,
    pub backdrop_url: Option<String>,
    pub year: Option<i32>,
    pub rating: Option<f32>,
    /// "movie" or "tv"
    pub media_type: Option<String>,
}


#[derive(Debug, Serialize)]
pub struct MovieSection {
    pub title: String,
    pub items: Vec<MovieCard>,
}

#[derive(Debug, Serialize)]
pub struct HomeResponse {
    pub sections: Vec<MovieSection>,
}

#[derive(Debug, Serialize)]
pub struct MovieDetails {
    pub tmdb_id: i64,
    pub imdb_id: Option<String>,
    pub title: String,
    pub overview: String,
    pub year: Option<i32>,
    pub runtime: Option<i32>,
    pub poster_url: Option<String>,
    pub backdrop_url: Option<String>,
    pub rating: Option<f32>,
}

#[derive(Debug, Serialize)]
pub struct SourceItem {
    pub id: String,
    pub title: String,
    pub quality: String,
    pub size_bytes: u64,
    pub seeds: i64,
    pub info_hash: String,
    pub is_cached: bool,
    pub provider: Option<String>, // currently always false (RD removed instantAvailability)
}

#[derive(Debug, Serialize)]
pub struct SourcesResponse {
    pub tmdb_id: i64,
    pub sources: Vec<SourceItem>,
}

#[derive(Debug, Deserialize)]
pub struct PlayRequest {
    // we send the info hash from the client
    pub info_hash: String,
    // OPTIONAL: something like "S06E25" so we can pick correct file
    pub episode_tag: Option<String>,
}


#[derive(Debug, Serialize)]
pub struct PlayResponse {
    // "ready", "pending", or "error"
    pub status: String,
    // present only when ready
    pub stream_url: Option<String>,
    // optional human-readable message
    pub message: Option<String>,
}

// -------------------------------
// TMDB INTERNAL MODELS
// -------------------------------

#[derive(Debug, Deserialize)]
pub struct TmdbMovieListResponse {
    pub results: Vec<TmdbMovieResult>,
}

#[derive(Debug, Deserialize)]
pub struct TmdbMovieResult {
    pub id: i64,
    pub title: String,
    pub poster_path: Option<String>,
    pub release_date: Option<String>,
    pub vote_average: f32,
    /// Extra fields used for filtering (e.g. anime search)
    pub genre_ids: Option<Vec<i64>>,
    pub original_language: Option<String>,
    pub backdrop_path: Option<String>,
}

// === INTERNAL TMDB TV RESPONSES ===

#[derive(Debug, Deserialize)]
pub struct TmdbTvDetailsResponse {
    pub id: i64,
    pub name: String,
    pub overview: String,
    pub first_air_date: Option<String>,
    pub last_air_date: Option<String>,
    pub number_of_seasons: i32,
    pub number_of_episodes: i32,
    pub vote_average: f32,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub seasons: Vec<TmdbTvSeasonShort>,
    pub external_ids: Option<TmdbExternalIds>,
}

#[derive(Debug, Deserialize)]
pub struct TmdbTvSeasonShort {
    pub season_number: i32,
    pub episode_count: i32,
    pub air_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TmdbTvSeasonResponse {
    pub id: i64,
    pub season_number: i32,
    pub episodes: Vec<TmdbTvEpisode>,
}

#[derive(Debug, Deserialize)]
pub struct TmdbTvEpisode {
    pub episode_number: i32,
    pub name: String,
    pub overview: String,
    pub air_date: Option<String>,
    pub still_path: Option<String>,
    pub runtime: Option<i32>,
    pub vote_average: f32,
}


// TV search/discover responses

#[derive(Debug, Deserialize)]
pub struct TmdbTvListResponse {
    pub results: Vec<TmdbTvResult>,
}

#[derive(Debug, Deserialize)]
pub struct TmdbTvResult {
    pub id: i64,
    pub name: String,
    pub poster_path: Option<String>,
    pub first_air_date: Option<String>,
    pub vote_average: f32,
    pub genre_ids: Option<Vec<i64>>,
    pub original_language: Option<String>,
    pub backdrop_path: Option<String>,
}


#[derive(Debug, Deserialize)]
pub struct TmdbMovieDetailsResponse {
    pub id: i64,
    pub title: String,
    pub overview: String,
    pub release_date: Option<String>,
    pub runtime: Option<i32>,
    pub vote_average: f32,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub external_ids: Option<TmdbExternalIds>,
}

#[derive(Debug, Deserialize)]
pub struct TmdbExternalIds {
    pub imdb_id: Option<String>,
}


// High-level TV series info
#[derive(Debug, Serialize)]
pub struct TvSeriesDetails {
    pub tmdb_id: i64,
    pub name: String,
    pub overview: String,
    pub poster_url: Option<String>,
    pub backdrop_url: Option<String>,
    pub first_air_date: Option<String>,
    pub last_air_date: Option<String>,
    pub number_of_seasons: i32,
    pub number_of_episodes: i32,
    pub rating: Option<f32>,
    pub seasons: Vec<TvSeasonSummary>,
}

#[derive(Debug, Serialize)]
pub struct TvSeasonSummary {
    pub season_number: i32,
    pub name: String,
    pub episode_count: i32,
    pub air_date: Option<String>,
    pub poster_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TvEpisode {
    pub season_number: i32,
    pub episode_number: i32,
    pub name: String,
    pub overview: String,
    pub still_url: Option<String>,
    pub air_date: Option<String>,
    pub runtime: Option<i32>,
    pub rating: Option<f32>,
}

#[derive(Debug, Serialize)]
pub struct TvSeasonEpisodesResponse {
    pub tmdb_id: i64,
    pub season_number: i32,
    pub episodes: Vec<TvEpisode>,
}


// -------------------------------
// TORRENTIO INTERNAL MODELS
// -------------------------------

#[derive(Debug, Deserialize)]
pub struct TorrentioResponse {
    pub streams: Vec<TorrentioStream>,
}

#[derive(Debug, Deserialize)]
pub struct TorrentioStream {
    pub name: Option<String>,
    pub title: Option<String>,
    pub infoHash: Option<String>,
    pub size: Option<u64>,
    pub seeds: Option<i64>,
}

// -------------------------------
// REAL-DEBRID INTERNAL MODELS
// -------------------------------

#[derive(Debug, Deserialize)]
pub struct RdAddMagnetResponse {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct RdTorrentFile {
    pub id: i32,
    pub path: String,
    pub bytes: i64,
    pub selected: i32,
}

#[derive(Debug, Deserialize)]
pub struct RdTorrentInfo {
    pub id: String,
    pub status: Option<String>,
    pub links: Option<Vec<String>>,
    pub files: Option<Vec<RdTorrentFile>>,
}

#[derive(Debug, Deserialize)]
pub struct RdUnrestrictResponse {
    pub download: String,
}

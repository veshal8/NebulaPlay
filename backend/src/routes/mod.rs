// src/routes/mod.rs
use axum::{
    routing::{get, post},
    Router,
};

use crate::models::AppState;

pub mod home;
pub mod movie;
pub mod play;
pub mod search;
pub mod tv;
pub mod tv_sources;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(home::health_handler))

        // HOME
        .route("/api/home", get(home::home_handler))
        .route("/api/home/anime", get(home::home_anime_handler))
        .route("/api/home/countries", get(home::home_countries_handler))
        .route("/api/home/tv", get(home::home_tv_handler))

        // SEARCH
        .route("/api/search", get(search::search_handler))

        // MOVIES
        .route("/api/movie/:tmdb_id", get(movie::movie_details_handler))
        .route("/api/movie/:tmdb_id/sources", get(movie::movie_sources_handler))

        // TV SHOWS / ANIME SERIES
        .route("/api/tv/:tmdb_id", get(tv::tv_details_handler))
        .route("/api/tv/:tmdb_id/season/:season_number", get(tv::tv_season_handler))

        // ⭐ NEW — EPISODE SOURCES
        .route(
            "/api/tv/:tmdb_id/season/:season/episode/:episode/sources",
            get(tv_sources::tv_episode_sources_handler),
        )

        // PLAYBACK (RealDebrid)
        .route("/api/play", post(play::play_handler))

        // STATE
        .with_state(state)
}

// src/config.rs
use crate::models::AppState;

pub fn build_app_state() -> AppState {
    let tmdb_key =
        std::env::var("TMDB_API_KEY").expect("TMDB_API_KEY env var not set");
    let rd_token =
        std::env::var("RD_TOKEN").expect("RD_TOKEN env var not set");

    AppState { tmdb_key, rd_token }
}

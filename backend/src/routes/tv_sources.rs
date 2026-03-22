use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use reqwest::Client;

use crate::{
    models::{AppState, SourcesResponse},
    torrentio::streams_to_sources,
};

/// GET /api/tv/:tmdb_id/season/:season/episode/:episode/sources
pub async fn tv_episode_sources_handler(
    State(state): State<AppState>,
    Path((tmdb_id, season, episode)): Path<(i64, i32, i32)>,
) -> impl IntoResponse {
    let client = Client::new();
    let api_key = &state.tmdb_key;

    //
    // Step 1: TMDB → get IMDB ID
    //
    let details_url = format!(
        "https://api.themoviedb.org/3/tv/{}?api_key={}&language=en-US&append_to_response=external_ids",
        tmdb_id, api_key
    );

    let details_resp = match client.get(&details_url).send().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("[tv_episode_sources_handler] TMDB error: {}", e);
            return (
                StatusCode::BAD_GATEWAY,
                Json(SourcesResponse { tmdb_id, sources: vec![] }),
            );
        }
    };

    if !details_resp.status().is_success() {
        eprintln!(
            "[tv_episode_sources_handler] TMDB returned {}",
            details_resp.status()
        );
        return (
            StatusCode::BAD_GATEWAY,
            Json(SourcesResponse { tmdb_id, sources: vec![] }),
        );
    }

    let json: serde_json::Value = match details_resp.json().await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("[tv_episode_sources_handler] JSON error: {}", e);
            return (
                StatusCode::BAD_GATEWAY,
                Json(SourcesResponse { tmdb_id, sources: vec![] }),
            );
        }
    };

    let imdb_id = json["external_ids"]["imdb_id"]
        .as_str()
        .unwrap_or("")
        .to_string();

    if imdb_id.is_empty() {
        eprintln!("[tv_episode_sources_handler] Missing IMDB ID for {}", tmdb_id);
        return (
            StatusCode::OK,
            Json(SourcesResponse { tmdb_id, sources: vec![] }),
        );
    }

    //
    // Step 2: Torrentio → fetch episode streams
    //
    let torrentio_url = format!(
        "https://torrentio.strem.fun/stream/series/{}:{}:{}.json?qualityfilter=all&lang=en",
        imdb_id, season, episode
    );

    let torr_resp = match client.get(&torrentio_url).send().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("[tv_episode_sources_handler] Torrentio request error: {}", e);
            return (
                StatusCode::BAD_GATEWAY,
                Json(SourcesResponse { tmdb_id, sources: vec![] }),
            );
        }
    };

    if !torr_resp.status().is_success() {
        eprintln!(
            "[tv_episode_sources_handler] Torrentio returned {}",
            torr_resp.status()
        );
        return (
            StatusCode::BAD_GATEWAY,
            Json(SourcesResponse { tmdb_id, sources: vec![] }),
        );
    }

    let torr_json: crate::models::TorrentioResponse = match torr_resp.json().await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("[tv_episode_sources_handler] Torrentio JSON error: {}", e);
            return (
                StatusCode::BAD_GATEWAY,
                Json(SourcesResponse { tmdb_id, sources: vec![] }),
            );
        }
    };

    //
    // Step 3: Map → SourceItem[]
    //
    let sources = streams_to_sources(torr_json, tmdb_id);

    //
    // Step 4: Return
    //
    (StatusCode::OK, Json(SourcesResponse { tmdb_id, sources }))
}

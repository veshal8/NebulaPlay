// src/routes/movie.rs
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use reqwest::Client;

use crate::{
    models::{
        AppState, MovieDetails, SourcesResponse, TmdbMovieDetailsResponse,
        TorrentioResponse,
    },
    tmdb::extract_year,
    torrentio::streams_to_sources,
};

/// GET /api/movie/:tmdb_id
pub async fn movie_details_handler(
    Path(tmdb_id): Path<i64>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let client = Client::new();

    let url = format!(
        "https://api.themoviedb.org/3/movie/{}?api_key={}&language=en-US&append_to_response=external_ids",
        tmdb_id,
        state.tmdb_key,
    );

    let resp = match client.get(&url).send().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("[movie_details_handler] request error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json::<MovieDetails>(MovieDetails {
                    tmdb_id,
                    imdb_id: None,
                    title: "".into(),
                    overview: "".into(),
                    year: None,
                    runtime: None,
                    poster_url: None,
                    backdrop_url: None,
                    rating: None,
                }),
            );
        }
    };

    if !resp.status().is_success() {
        eprintln!(
            "[movie_details_handler] TMDB returned status {}",
            resp.status()
        );
        return (
            StatusCode::BAD_GATEWAY,
            Json::<MovieDetails>(MovieDetails {
                tmdb_id,
                imdb_id: None,
                title: "".into(),
                overview: "".into(),
                year: None,
                runtime: None,
                poster_url: None,
                backdrop_url: None,
                rating: None,
            }),
        );
    }

    let tmdb: TmdbMovieDetailsResponse = match resp.json().await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("[movie_details_handler] json error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json::<MovieDetails>(MovieDetails {
                    tmdb_id,
                    imdb_id: None,
                    title: "".into(),
                    overview: "".into(),
                    year: None,
                    runtime: None,
                    poster_url: None,
                    backdrop_url: None,
                    rating: None,
                }),
            );
        }
    };

    let imdb_id = tmdb
        .external_ids
        .as_ref()
        .and_then(|e| e.imdb_id.clone())
        .filter(|s| !s.is_empty());

    let details = MovieDetails {
        tmdb_id: tmdb.id,
        imdb_id,
        title: tmdb.title,
        overview: tmdb.overview,
        year: extract_year(&tmdb.release_date),
        runtime: tmdb.runtime,
        poster_url: tmdb
            .poster_path
            .map(|p| format!("https://image.tmdb.org/t/p/w500{}", p)),
        backdrop_url: tmdb
            .backdrop_path
            .map(|p| format!("https://image.tmdb.org/t/p/w1280{}", p)),
        rating: Some(tmdb.vote_average),
    };

    (StatusCode::OK, Json(details))
}

/// GET /api/movie/:tmdb_id/sources
pub async fn movie_sources_handler(
    Path(tmdb_id): Path<i64>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let client = Client::new();

    // 1) get imdb_id from TMDB
    let tmdb_url = format!(
        "https://api.themoviedb.org/3/movie/{}?api_key={}&language=en-US&append_to_response=external_ids",
        tmdb_id,
        state.tmdb_key,
    );

    let resp = match client.get(&tmdb_url).send().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("[movie_sources_handler] TMDB request error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SourcesResponse {
                    tmdb_id,
                    sources: vec![],
                }),
            );
        }
    };

    if !resp.status().is_success() {
        eprintln!(
            "[movie_sources_handler] TMDB returned status {}",
            resp.status()
        );
        return (
            StatusCode::BAD_GATEWAY,
            Json(SourcesResponse {
                tmdb_id,
                sources: vec![],
            }),
        );
    }

    let tmdb_details: TmdbMovieDetailsResponse = match resp.json().await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("[movie_sources_handler] TMDB json error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SourcesResponse {
                    tmdb_id,
                    sources: vec![],
                }),
            );
        }
    };

    let imdb_id = match tmdb_details
        .external_ids
        .and_then(|e| e.imdb_id)
        .filter(|s| !s.is_empty())
    {
        Some(id) => id,
        None => {
            eprintln!(
                "[movie_sources_handler] No imdb_id for tmdb_id {}",
                tmdb_id
            );
            return (
                StatusCode::OK,
                Json(SourcesResponse {
                    tmdb_id,
                    sources: vec![],
                }),
            );
        }
    };

    // 2) call Torrentio
    let torrentio_url = format!(
        "https://torrentio.strem.fun/sort=quality|safe=true|language=en|qualityfilter=1080p,2160p/stream/movie/{}.json",
        imdb_id
    );

    let resp = match client.get(&torrentio_url).send().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!(
                "[movie_sources_handler] Torrentio request error: {}",
                e
            );
            return (
                StatusCode::BAD_GATEWAY,
                Json(SourcesResponse {
                    tmdb_id,
                    sources: vec![],
                }),
            );
        }
    };

    if !resp.status().is_success() {
        eprintln!(
            "[movie_sources_handler] Torrentio returned status {}",
            resp.status()
        );
        return (
            StatusCode::BAD_GATEWAY,
            Json(SourcesResponse {
                tmdb_id,
                sources: vec![],
            }),
        );
    }

    let torrentio: TorrentioResponse = match resp.json().await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("[movie_sources_handler] Torrentio json error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SourcesResponse {
                    tmdb_id,
                    sources: vec![],
                }),
            );
        }
    };

    let sources = streams_to_sources(torrentio, tmdb_id);

    (StatusCode::OK, Json(SourcesResponse { tmdb_id, sources }))
}

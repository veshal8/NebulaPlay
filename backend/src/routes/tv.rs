use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use reqwest::Client;

use crate::models::{
    AppState,
    TmdbTvDetailsResponse,
    TmdbTvSeasonResponse,
    TvSeriesDetails,
    TvSeasonSummary,
    TvEpisode,
    TvSeasonEpisodesResponse,
};

/// GET /api/tv/:tmdb_id
pub async fn tv_details_handler(
    State(state): State<AppState>,
    Path(tmdb_id): Path<i64>,
) -> impl IntoResponse {
    let client = Client::new();
    let api_key = &state.tmdb_key;
    let lang = "en-US";

    let url = format!(
        "https://api.themoviedb.org/3/tv/{}?api_key={}&language={}&append_to_response=external_ids",
        tmdb_id, api_key, lang
    );

    let resp = match client.get(&url).send().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("[tv_details_handler] request error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json::<TvSeriesDetails>(TvSeriesDetails {
                    tmdb_id,
                    name: "Unknown".into(),
                    overview: "".into(),
                    poster_url: None,
                    backdrop_url: None,
                    first_air_date: None,
                    last_air_date: None,
                    number_of_seasons: 0,
                    number_of_episodes: 0,
                    rating: None,
                    seasons: vec![],
                }),
            );
        }
    };

    if !resp.status().is_success() {
        eprintln!(
            "[tv_details_handler] TMDB returned status {}",
            resp.status()
        );
        return (
            StatusCode::BAD_GATEWAY,
            Json(TvSeriesDetails {
                tmdb_id,
                name: "Unknown".into(),
                overview: "".into(),
                poster_url: None,
                backdrop_url: None,
                first_air_date: None,
                last_air_date: None,
                number_of_seasons: 0,
                number_of_episodes: 0,
                rating: None,
                seasons: vec![],
            }),
        );
    }

    let tmdb: TmdbTvDetailsResponse = match resp.json().await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("[tv_details_handler] json error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TvSeriesDetails {
                    tmdb_id,
                    name: "Unknown".into(),
                    overview: "".into(),
                    poster_url: None,
                    backdrop_url: None,
                    first_air_date: None,
                    last_air_date: None,
                    number_of_seasons: 0,
                    number_of_episodes: 0,
                    rating: None,
                    seasons: vec![],
                }),
            );
        }
    };

    let poster_url = tmdb
        .poster_path
        .map(|p| format!("https://image.tmdb.org/t/p/w500{}", p));
    let backdrop_url = tmdb
        .backdrop_path
        .map(|p| format!("https://image.tmdb.org/t/p/w1280{}", p));

    let seasons: Vec<TvSeasonSummary> = tmdb
        .seasons
        .into_iter()
        .map(|s| TvSeasonSummary {
            season_number: s.season_number,
            name: format!("Season {}", s.season_number),
            episode_count: s.episode_count,
            air_date: s.air_date,
            poster_url: None, // you can fill later
        })
        .collect();

    let details = TvSeriesDetails {
        tmdb_id: tmdb.id,
        name: tmdb.name,
        overview: tmdb.overview,
        poster_url,
        backdrop_url,
        first_air_date: tmdb.first_air_date,
        last_air_date: tmdb.last_air_date,
        number_of_seasons: tmdb.number_of_seasons,
        number_of_episodes: tmdb.number_of_episodes,
        rating: Some(tmdb.vote_average),
        seasons,
    };

    (StatusCode::OK, Json(details))
}

/// GET /api/tv/:tmdb_id/season/:season_number
pub async fn tv_season_handler(
    State(state): State<AppState>,
    Path((tmdb_id, season_number)): Path<(i64, i32)>,
) -> impl IntoResponse {
    let client = Client::new();
    let api_key = &state.tmdb_key;
    let lang = "en-US";

    let url = format!(
        "https://api.themoviedb.org/3/tv/{}/season/{}?api_key={}&language={}",
        tmdb_id, season_number, api_key, lang
    );

    let resp = match client.get(&url).send().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("[tv_season_handler] request error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TvSeasonEpisodesResponse {
                    tmdb_id,
                    season_number,
                    episodes: vec![],
                }),
            );
        }
    };

    if !resp.status().is_success() {
        eprintln!(
            "[tv_season_handler] TMDB returned status {}",
            resp.status()
        );
        return (
            StatusCode::BAD_GATEWAY,
            Json(TvSeasonEpisodesResponse {
                tmdb_id,
                season_number,
                episodes: vec![],
            }),
        );
    }

    let tmdb: TmdbTvSeasonResponse = match resp.json().await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("[tv_season_handler] json error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TvSeasonEpisodesResponse {
                    tmdb_id,
                    season_number,
                    episodes: vec![],
                }),
            );
        }
    };

    let episodes: Vec<TvEpisode> = tmdb
        .episodes
        .into_iter()
        .map(|ep| TvEpisode {
            season_number: tmdb.season_number,
            episode_number: ep.episode_number,
            name: ep.name,
            overview: ep.overview,
            still_url: ep
                .still_path
                .map(|p| format!("https://image.tmdb.org/t/p/w300{}", p)),
            air_date: ep.air_date,
            runtime: ep.runtime,
            rating: Some(ep.vote_average),
        })
        .collect();

    let resp_body = TvSeasonEpisodesResponse {
        tmdb_id,
        season_number: tmdb.season_number,
        episodes,
    };

    (StatusCode::OK, Json(resp_body))
}

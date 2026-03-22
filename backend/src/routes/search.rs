// src/routes/search.rs
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use reqwest::Client;

use crate::{
    models::{
        AppState,
        SearchQuery,
        MovieCard,
        MovieSection,
        HomeResponse,
        TmdbMovieListResponse,
        TmdbTvListResponse,
    },
    tmdb::extract_year,
};

/// GET /api/search?q=...&kind=movie|anime
pub async fn search_handler(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> impl IntoResponse {
    let query = params.q.trim().to_string();
    if query.is_empty() {
        return (
            StatusCode::OK,
            Json(HomeResponse { sections: vec![] }),
        );
    }

    // default kind = "movie"
    let kind = params.kind.clone().unwrap_or_else(|| "movie".to_string());
    let is_anime_search = kind == "anime";

    let client = Client::new();

    // ============================================
    // NORMAL MOVIE SEARCH
    // ============================================
    if !is_anime_search {
        let url = "https://api.themoviedb.org/3/search/movie";

        let resp = match client
            .get(url)
            .query(&[
                ("api_key", state.tmdb_key.as_str()),
                ("language", "en-US"),
                ("query", query.as_str()),
                ("page", "1"),
                ("include_adult", "false"),
            ])
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => {
                eprintln!("[search_handler] request error: {}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(HomeResponse { sections: vec![] }),
                );
            }
        };

        if !resp.status().is_success() {
            eprintln!("[search_handler] TMDB returned status {}", resp.status());
            return (
                StatusCode::BAD_GATEWAY,
                Json(HomeResponse { sections: vec![] }),
            );
        }

        let tmdb: TmdbMovieListResponse = match resp.json().await {
            Ok(data) => data,
            Err(e) => {
                eprintln!("[search_handler] json error: {}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(HomeResponse { sections: vec![] }),
                );
            }
        };

        let items: Vec<MovieCard> = tmdb
            .results
            .into_iter()
            .map(|m| MovieCard {
                tmdb_id: m.id,
                title: m.title,
                poster_url: m
                    .poster_path
                    .map(|p| format!("https://image.tmdb.org/t/p/w342{}", p))
                    .unwrap_or_default(),
                backdrop_url: m
                    .backdrop_path
                    .map(|p| format!("https://image.tmdb.org/t/p/w1280{}", p)),
                year: extract_year(&m.release_date),
                rating: Some(m.vote_average),
                media_type: Some("movie".into()),   // <-- ADDED
            })
            .collect();

        let section = MovieSection {
            title: format!("Results for \"{}\"", query),
            items,
        };

        return (StatusCode::OK, Json(HomeResponse { sections: vec![section] }));
    }

    // ============================================
    // ANIME SEARCH (COMBINE MOVIE + TV)
    // ============================================

    let movie_url = "https://api.themoviedb.org/3/search/movie";
    let tv_url = "https://api.themoviedb.org/3/search/tv";

    let (movie_resp, tv_resp) = tokio::join!(
        client
            .get(movie_url)
            .query(&[
                ("api_key", state.tmdb_key.as_str()),
                ("language", "en-US"),
                ("query", query.as_str()),
                ("page", "1"),
                ("include_adult", "false"),
            ])
            .send(),
        client
            .get(tv_url)
            .query(&[
                ("api_key", state.tmdb_key.as_str()),
                ("language", "en-US"),
                ("query", query.as_str()),
                ("page", "1"),
                ("include_adult", "false"),
            ])
            .send(),
    );

    let movie_resp = match movie_resp {
        Ok(r) => r,
        Err(e) => {
            eprintln!("[search_handler] movie request error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(HomeResponse { sections: vec![] }),
            );
        }
    };

    let tv_resp = match tv_resp {
        Ok(r) => r,
        Err(e) => {
            eprintln!("[search_handler] tv request error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(HomeResponse { sections: vec![] }),
            );
        }
    };

    if !movie_resp.status().is_success() || !tv_resp.status().is_success() {
        eprintln!(
            "[search_handler] TMDB movie status={} tv status={}",
            movie_resp.status(),
            tv_resp.status()
        );
        return (
            StatusCode::BAD_GATEWAY,
            Json(HomeResponse { sections: vec![] }),
        );
    }

    let movie_results: TmdbMovieListResponse = match movie_resp.json().await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("[search_handler] movie json error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(HomeResponse { sections: vec![] }),
            );
        }
    };

    let tv_results: TmdbTvListResponse = match tv_resp.json().await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("[search_handler] tv json error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(HomeResponse { sections: vec![] }),
            );
        }
    };

    // ANIME FILTER: animation genre OR Japanese language
    let mut items: Vec<MovieCard> = Vec::new();

    // ----------------------------------------
    // Anime MOVIES
    // ----------------------------------------
    items.extend(
        movie_results
            .results
            .into_iter()
            .filter(|m| {
                let is_anim_genre = m.genre_ids.as_ref().map_or(false, |ids| ids.contains(&16));
                let is_japanese_lang = m.original_language.as_deref() == Some("ja");
                is_anim_genre || is_japanese_lang
            })
            .map(|m| MovieCard {
                tmdb_id: m.id,
                title: m.title,
                poster_url: m
                    .poster_path
                    .map(|p| format!("https://image.tmdb.org/t/p/w342{}", p))
                    .unwrap_or_default(),
                backdrop_url: m
                    .backdrop_path
                    .map(|p| format!("https://image.tmdb.org/t/p/w1280{}", p)),
                year: extract_year(&m.release_date),
                rating: Some(m.vote_average),
                media_type: Some("movie".into()),   // <-- ADDED
            }),
    );

    // ----------------------------------------
    // Anime TV SHOWS
    // ----------------------------------------
    items.extend(
        tv_results
            .results
            .into_iter()
            .filter(|t| {
                let is_anim_genre = t.genre_ids.as_ref().map_or(false, |ids| ids.contains(&16));
                let is_japanese_lang = t.original_language.as_deref() == Some("ja");
                is_anim_genre || is_japanese_lang
            })
            .map(|t| MovieCard {
                tmdb_id: t.id,
                title: t.name,
                poster_url: t
                    .poster_path
                    .map(|p| format!("https://image.tmdb.org/t/p/w342{}", p))
                    .unwrap_or_default(),
                backdrop_url: t
                    .backdrop_path
                    .map(|p| format!("https://image.tmdb.org/t/p/w1280{}", p)),
                year: extract_year(&t.first_air_date),
                rating: Some(t.vote_average),
                media_type: Some("tv".into()),   // <-- ADDED
            }),
    );

    let section = MovieSection {
        title: format!("Anime results for \"{}\"", query),
        items,
    };

    (StatusCode::OK, Json(HomeResponse { sections: vec![section] }))
}

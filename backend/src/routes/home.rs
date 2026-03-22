// src/routes/home.rs
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use reqwest::Client;

use crate::{
    models::{AppState, HealthResponse, HomeResponse, MovieSection, MovieCard},
    tmdb::{fetch_tmdb_movie_list, fetch_tmdb_tv_list},
};


/// GET /health
pub async fn health_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(HealthResponse {
            status: "OK".into(),
        }),
    )
}

/// GET /api/home
pub async fn home_handler(State(state): State<AppState>) -> impl IntoResponse {
    let client = Client::new();
    let api_key = &state.tmdb_key;
    let lang = "en-US";

    let trending_url = format!(
        "https://api.themoviedb.org/3/trending/movie/week?api_key={}",
        api_key
    );
    let popular_url = format!(
        "https://api.themoviedb.org/3/movie/popular?api_key={}&language={}&page=1",
        api_key, lang
    );
    let top_rated_url = format!(
        "https://api.themoviedb.org/3/movie/top_rated?api_key={}&language={}&page=1",
        api_key, lang
    );
    let upcoming_url = format!(
        "https://api.themoviedb.org/3/movie/upcoming?api_key={}&language={}&page=1",
        api_key, lang
    );
    let now_playing_url = format!(
        "https://api.themoviedb.org/3/movie/now_playing?api_key={}&language={}&page=1",
        api_key, lang
    );
    // Example genre rows via discover:
    // Action = 28, Sci-Fi = 878, Horror = 27
    let action_url = format!(
        "https://api.themoviedb.org/3/discover/movie?api_key={}&language={}&with_genres=28&sort_by=popularity.desc&page=1",
        api_key, lang
    );
    let scifi_url = format!(
        "https://api.themoviedb.org/3/discover/movie?api_key={}&language={}&with_genres=878&sort_by=popularity.desc&page=1",
        api_key, lang
    );
    let horror_url = format!(
        "https://api.themoviedb.org/3/discover/movie?api_key={}&language={}&with_genres=27&sort_by=popularity.desc&page=1",
        api_key, lang
    );

    let (trending, popular, top_rated, upcoming, now_playing, action, scifi, horror) = tokio::join!(
        fetch_tmdb_movie_list(&client, &trending_url),
        fetch_tmdb_movie_list(&client, &popular_url),
        fetch_tmdb_movie_list(&client, &top_rated_url),
        fetch_tmdb_movie_list(&client, &upcoming_url),
        fetch_tmdb_movie_list(&client, &now_playing_url),
        fetch_tmdb_movie_list(&client, &action_url),
        fetch_tmdb_movie_list(&client, &scifi_url),
        fetch_tmdb_movie_list(&client, &horror_url),
    );

    let mut sections = Vec::new();

    if !trending.is_empty() {
        sections.push(MovieSection {
            title: "Trending Now".into(),
            items: trending,
        });
    }
    if !popular.is_empty() {
        sections.push(MovieSection {
            title: "Popular on NebulaPlay".into(),
            items: popular,
        });
    }
    if !top_rated.is_empty() {
        sections.push(MovieSection {
            title: "Top Rated".into(),
            items: top_rated,
        });
    }
    if !now_playing.is_empty() {
        sections.push(MovieSection {
            title: "Now in Theaters".into(),
            items: now_playing,
        });
    }
    if !upcoming.is_empty() {
        sections.push(MovieSection {
            title: "Coming Soon".into(),
            items: upcoming,
        });
    }
    if !action.is_empty() {
        sections.push(MovieSection {
            title: "Action Thrills".into(),
            items: action,
        });
    }
    if !scifi.is_empty() {
        sections.push(MovieSection {
            title: "Sci-Fi & Fantasy".into(),
            items: scifi,
        });
    }
    if !horror.is_empty() {
        sections.push(MovieSection {
            title: "Horror".into(),
            items: horror,
        });
    }

    (StatusCode::OK, Json(HomeResponse { sections }))
}

/// GET /api/home/anime
/// Uses TMDB discover for anime-style movies *and* TV series (Animation + JP origin).
pub async fn home_anime_handler(State(state): State<AppState>) -> impl IntoResponse {
    let client = Client::new();
    let api_key = &state.tmdb_key;
    let lang = "en-US";

    let base_movie = "https://api.themoviedb.org/3/discover/movie";
    let base_tv = "https://api.themoviedb.org/3/discover/tv";

    // --- Movies ---
    let anime_popular_movies_url = format!(
        "{base}?api_key={api}&language={lang}&with_genres=16&with_origin_country=JP&sort_by=popularity.desc&page=1",
        base = base_movie,
        api = api_key,
        lang = lang
    );

    let anime_top_rated_movies_url = format!(
        "{base}?api_key={api}&language={lang}&with_genres=16&with_origin_country=JP&sort_by=vote_average.desc&vote_count.gte=50&page=1",
        base = base_movie,
        api = api_key,
        lang = lang
    );

    let anime_recent_movies_url = format!(
        "{base}?api_key={api}&language={lang}&with_genres=16&with_origin_country=JP&sort_by=primary_release_date.desc&page=1",
        base = base_movie,
        api = api_key,
        lang = lang
    );

    // --- TV series ---
    let anime_popular_tv_url = format!(
        "{base}?api_key={api}&language={lang}&with_genres=16&with_origin_country=JP&sort_by=popularity.desc&page=1",
        base = base_tv,
        api = api_key,
        lang = lang
    );

    let anime_top_rated_tv_url = format!(
        "{base}?api_key={api}&language={lang}&with_genres=16&with_origin_country=JP&sort_by=vote_average.desc&vote_count.gte=50&page=1",
        base = base_tv,
        api = api_key,
        lang = lang
    );

    let anime_recent_tv_url = format!(
        "{base}?api_key={api}&language={lang}&with_genres=16&with_origin_country=JP&sort_by=first_air_date.desc&page=1",
        base = base_tv,
        api = api_key,
        lang = lang
    );

    let (
        popular_movies,
        top_rated_movies,
        recent_movies,
        popular_tv,
        top_rated_tv,
        recent_tv,
    ): (
        Vec<MovieCard>,
        Vec<MovieCard>,
        Vec<MovieCard>,
        Vec<MovieCard>,
        Vec<MovieCard>,
        Vec<MovieCard>,
    ) = tokio::join!(
        fetch_tmdb_movie_list(&client, &anime_popular_movies_url),
        fetch_tmdb_movie_list(&client, &anime_top_rated_movies_url),
        fetch_tmdb_movie_list(&client, &anime_recent_movies_url),
        fetch_tmdb_tv_list(&client, &anime_popular_tv_url),
        fetch_tmdb_tv_list(&client, &anime_top_rated_tv_url),
        fetch_tmdb_tv_list(&client, &anime_recent_tv_url),
    );


    let mut sections = Vec::new();

    // I’m putting series first since that’s what you noticed missing,
    // but you can reorder however you like.

    if !popular_tv.is_empty() {
        sections.push(MovieSection {
            title: "Popular Anime Series".into(),
            items: popular_tv,
        });
    }
    if !top_rated_tv.is_empty() {
        sections.push(MovieSection {
            title: "Top Rated Anime Series".into(),
            items: top_rated_tv,
        });
    }
    if !recent_tv.is_empty() {
        sections.push(MovieSection {
            title: "New & Recent Anime Series".into(),
            items: recent_tv,
        });
    }

    if !popular_movies.is_empty() {
        sections.push(MovieSection {
            title: "Popular Anime Movies".into(),
            items: popular_movies,
        });
    }
    if !top_rated_movies.is_empty() {
        sections.push(MovieSection {
            title: "Top Rated Anime Movies".into(),
            items: top_rated_movies,
        });
    }
    if !recent_movies.is_empty() {
        sections.push(MovieSection {
            title: "New & Recent Anime Movies".into(),
            items: recent_movies,
        });
    }

    (StatusCode::OK, Json(HomeResponse { sections }))
}


/// GET /api/home/countries
/// Curated rows by origin country: KR, JP, US, IN.
pub async fn home_countries_handler(State(state): State<AppState>) -> impl IntoResponse {
    let client = Client::new();
    let api_key = &state.tmdb_key;
    let lang = "en-US";

    let base = "https://api.themoviedb.org/3/discover/movie";

    let kr_url = format!(
        "{base}?api_key={api}&language={lang}&with_origin_country=KR&sort_by=popularity.desc&page=1",
        base = base,
        api = api_key,
        lang = lang
    );
    let jp_url = format!(
        "{base}?api_key={api}&language={lang}&with_origin_country=JP&sort_by=popularity.desc&page=1",
        base = base,
        api = api_key,
        lang = lang
    );
    let us_url = format!(
        "{base}?api_key={api}&language={lang}&with_origin_country=US&sort_by=popularity.desc&page=1",
        base = base,
        api = api_key,
        lang = lang
    );
    let in_url = format!(
        "{base}?api_key={api}&language={lang}&with_origin_country=IN&sort_by=popularity.desc&page=1",
        base = base,
        api = api_key,
        lang = lang
    );

    let (kr, jp, us, ind) = tokio::join!(
        fetch_tmdb_movie_list(&client, &kr_url),
        fetch_tmdb_movie_list(&client, &jp_url),
        fetch_tmdb_movie_list(&client, &us_url),
        fetch_tmdb_movie_list(&client, &in_url),
    );

    let mut sections = Vec::new();

    if !kr.is_empty() {
        sections.push(MovieSection {
            title: "Korean Cinema".into(),
            items: kr,
        });
    }
    if !jp.is_empty() {
        sections.push(MovieSection {
            title: "Japanese Cinema".into(),
            items: jp,
        });
    }
    if !us.is_empty() {
        sections.push(MovieSection {
            title: "Hollywood Hits".into(),
            items: us,
        });
    }
    if !ind.is_empty() {
        sections.push(MovieSection {
            title: "Indian Cinema".into(),
            items: ind,
        });
    }

    (StatusCode::OK, Json(HomeResponse { sections }))
}


/// GET /api/home/tv
/// Home rows for TV series (popular, top rated, etc).
pub async fn home_tv_handler(State(state): State<AppState>) -> impl IntoResponse {
    let client = Client::new();
    let api_key = &state.tmdb_key;
    let lang = "en-US";

    // TV endpoints
    let trending_tv_url = format!(
        "https://api.themoviedb.org/3/trending/tv/week?api_key={}",
        api_key
    );
    let popular_tv_url = format!(
        "https://api.themoviedb.org/3/tv/popular?api_key={}&language={}&page=1",
        api_key, lang
    );
    let top_rated_tv_url = format!(
        "https://api.themoviedb.org/3/tv/top_rated?api_key={}&language={}&page=1",
        api_key, lang
    );
    let on_air_tv_url = format!(
        "https://api.themoviedb.org/3/tv/on_the_air?api_key={}&language={}&page=1",
        api_key, lang
    );

    let (trending, popular, top_rated, on_air): (
        Vec<MovieCard>,
        Vec<MovieCard>,
        Vec<MovieCard>,
        Vec<MovieCard>,
    ) = tokio::join!(
        fetch_tmdb_tv_list(&client, &trending_tv_url),
        fetch_tmdb_tv_list(&client, &popular_tv_url),
        fetch_tmdb_tv_list(&client, &top_rated_tv_url),
        fetch_tmdb_tv_list(&client, &on_air_tv_url),
    );

    let mut sections = Vec::new();

    if !trending.is_empty() {
        sections.push(MovieSection {
            title: "Trending TV".into(),
            items: trending,
        });
    }
    if !popular.is_empty() {
        sections.push(MovieSection {
            title: "Popular TV Shows".into(),
            items: popular,
        });
    }
    if !top_rated.is_empty() {
        sections.push(MovieSection {
            title: "Top Rated Series".into(),
            items: top_rated,
        });
    }
    if !on_air.is_empty() {
        sections.push(MovieSection {
            title: "Currently Airing".into(),
            items: on_air,
        });
    }

    (StatusCode::OK, Json(HomeResponse { sections }))
}

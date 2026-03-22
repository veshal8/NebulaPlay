use reqwest::Client;

use crate::models::{
    MovieCard,
    TmdbMovieListResponse,
    TmdbTvListResponse,
};

/// Generic helper for TMDB movie lists (trending, popular, discover, etc.)
pub async fn fetch_tmdb_movie_list(client: &Client, url: &str) -> Vec<MovieCard> {
    let resp = match client.get(url).send().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("[fetch_tmdb_movie_list] request error: {}", e);
            return vec![];
        }
    };

    if !resp.status().is_success() {
        eprintln!(
            "[fetch_tmdb_movie_list] TMDB returned status {}",
            resp.status()
        );
        return vec![];
    }

    let tmdb: TmdbMovieListResponse = match resp.json().await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("[fetch_tmdb_movie_list] json error: {}", e);
            return vec![];
        }
    };

    tmdb.results
        .into_iter()
        .map(|m| MovieCard {
            tmdb_id: m.id,
            title: m.title,
            poster_url: m
                .poster_path
                .map(|p| format!("https://image.tmdb.org/t/p/w500{}", p))
                .unwrap_or_default(),
            backdrop_url: m
                .backdrop_path
                .map(|p| format!("https://image.tmdb.org/t/p/w1280{}", p)),
            year: extract_year(&m.release_date),
            rating: Some(m.vote_average),

            // 🔥 NEW FIELD
            media_type: Some("movie".into()),
        })
        .collect()
}

/// Same idea, but for TMDB TV lists (discover/tv, search/tv, etc.)
pub async fn fetch_tmdb_tv_list(client: &Client, url: &str) -> Vec<MovieCard> {
    let resp = match client.get(url).send().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("[fetch_tmdb_tv_list] request error: {}", e);
            return vec![];
        }
    };

    if !resp.status().is_success() {
        eprintln!(
            "[fetch_tmdb_tv_list] TMDB returned status {}",
            resp.status()
        );
        return vec![];
    }

    let tmdb: TmdbTvListResponse = match resp.json().await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("[fetch_tmdb_tv_list] json error: {}", e);
            return vec![];
        }
    };

    tmdb.results
        .into_iter()
        .map(|t| MovieCard {
            tmdb_id: t.id,
            title: t.name, // TV uses `name` instead of `title`
            poster_url: t
                .poster_path
                .map(|p| format!("https://image.tmdb.org/t/p/w500{}", p))
                .unwrap_or_default(),
            backdrop_url: t
                .backdrop_path
                .map(|p| format!("https://image.tmdb.org/t/p/w1280{}", p)),
            year: extract_year(&t.first_air_date),
            rating: Some(t.vote_average),

            // 🔥 NEW FIELD
            media_type: Some("tv".into()),
        })
        .collect()
}

/// Reusable helper to map "YYYY-MM-DD" -> year
pub fn extract_year(date: &Option<String>) -> Option<i32> {
    if let Some(d) = date {
        if d.len() >= 4 {
            if let Ok(y) = d[0..4].parse::<i32>() {
                return Some(y);
            }
        }
    }
    None
}

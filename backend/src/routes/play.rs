// src/routes/play.rs
use std::time::Duration;

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use reqwest::Client;
use tokio::time::sleep;

use crate::models::{
    AppState,
    PlayRequest,
    PlayResponse,
    RdAddMagnetResponse,
    RdTorrentInfo,
    RdTorrentFile,
    RdUnrestrictResponse,
};

/// POST /api/play
/// Body: { "info_hash": "<btih>", "episode_tag": "S06E25" (optional) }
pub async fn play_handler(
    State(state): State<AppState>,
    Json(body): Json<PlayRequest>,
) -> impl IntoResponse {
    let client = Client::new();

    let magnet = format!("magnet:?xt=urn:btih:{}", body.info_hash);
    let add_url = "https://api.real-debrid.com/rest/1.0/torrents/addMagnet";

    println!("[play_handler] addMagnet for hash {}", body.info_hash);

    // 1) add magnet
    let resp = match client
        .post(add_url)
        .bearer_auth(&state.rd_token)
        .form(&[("magnet", magnet.as_str())])
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            eprintln!("[play_handler] addMagnet error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(PlayResponse {
                    status: "error".into(),
                    stream_url: None,
                    message: Some(
                        "Failed to talk to Real-Debrid (addMagnet).".into(),
                    ),
                }),
            );
        }
    };

    if !resp.status().is_success() {
        eprintln!("[play_handler] addMagnet status {}", resp.status());
        return (
            StatusCode::BAD_GATEWAY,
            Json(PlayResponse {
                status: "error".into(),
                stream_url: None,
                message: Some("Real-Debrid rejected the magnet.".into()),
            }),
        );
    }

    let add: RdAddMagnetResponse = match resp.json().await {
        Ok(d) => d,
        Err(e) => {
            eprintln!("[play_handler] addMagnet json error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(PlayResponse {
                    status: "error".into(),
                    stream_url: None,
                    message: Some(
                        "Failed to parse Real-Debrid addMagnet response."
                            .into(),
                    ),
                }),
            );
        }
    };

    let torrent_id = add.id;
    println!("[play_handler] torrent id = {}", torrent_id);

    // 2) select all files to start it
    let select_url = format!(
        "https://api.real-debrid.com/rest/1.0/torrents/selectFiles/{}",
        torrent_id
    );
    let _ = client
        .post(&select_url)
        .bearer_auth(&state.rd_token)
        .form(&[("files", "all")])
        .send()
        .await;

    // 3) poll torrents/info for a short while waiting for links
    let info_url = format!(
        "https://api.real-debrid.com/rest/1.0/torrents/info/{}",
        torrent_id
    );

    let mut info_opt: Option<RdTorrentInfo> = None;

    for attempt in 0..8 {
        println!("[play_handler] polling torrents/info attempt {}", attempt + 1);

        let resp = match client
            .get(&info_url)
            .bearer_auth(&state.rd_token)
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => {
                eprintln!("[play_handler] torrents/info error: {}", e);
                break;
            }
        };

        if !resp.status().is_success() {
            eprintln!(
                "[play_handler] torrents/info status {} (attempt {})",
                resp.status(),
                attempt + 1
            );
            break;
        }

        let info: RdTorrentInfo = match resp.json().await {
            Ok(d) => d,
            Err(e) => {
                eprintln!("[play_handler] torrents/info json error: {}", e);
                break;
            }
        };

        if let Some(links) = &info.links {
            if !links.is_empty() {
                println!(
                    "[play_handler] torrents/info has {} link(s) at attempt {}",
                    links.len(),
                    attempt + 1
                );
                info_opt = Some(info);
                break;
            }
        }

        // no links yet -> wait a bit and retry
        sleep(Duration::from_secs(3)).await;
    }

    let info = match info_opt {
        Some(i) => i,
        None => {
            println!("[play_handler] no links after polling, returning pending");
            return (
                StatusCode::ACCEPTED,
                Json(PlayResponse {
                    status: "pending".into(),
                    stream_url: None,
                    message: Some(
                        "Torrent added to Real-Debrid, but no download links yet. Open RD or try again shortly."
                            .into(),
                    ),
                }),
            );
        }
    };

    // 4) choose the correct link:
    //    - if episode_tag given: pick file whose path matches that tag
    //    - else / fallback: largest video file
    //    - else / final fallback: last link (old behaviour)
    let mut chosen_link: Option<String> = None;

    if let (Some(files), Some(links)) = (info.files.as_ref(), info.links.as_ref()) {
        // 4a) episode tag match
        if let Some(tag) = body.episode_tag.as_ref() {
            let tag_lower = tag.to_lowercase();

            if let Some((_f, link)) = files
                .iter()
                .zip(links.iter())
                .find(|(f, _)| f.path.to_lowercase().contains(&tag_lower))
            {
                println!("[play_handler] matched episode_tag '{}' to file", tag);
                chosen_link = Some(link.clone());
            }
        }

        // 4b) fallback: largest video file
        if chosen_link.is_none() {
            let mut best_index: Option<usize> = None;

            for (idx, f) in files.iter().enumerate() {
                let lower = f.path.to_lowercase();
                let is_video = lower.ends_with(".mkv")
                    || lower.ends_with(".mp4")
                    || lower.ends_with(".avi")
                    || lower.ends_with(".mov");

                if !is_video {
                    continue;
                }

                match best_index {
                    None => best_index = Some(idx),
                    Some(bi) => {
                        if f.bytes > files[bi].bytes {
                            best_index = Some(idx);
                        }
                    }
                }
            }

            if let Some(idx) = best_index {
                if idx < links.len() {
                    println!(
                        "[play_handler] using largest video file: {}",
                        files[idx].path
                    );
                    chosen_link = Some(links[idx].clone());
                }
            }
        }
    }

    // 4c) final fallback – behave like before: pick last link
    if chosen_link.is_none() {
        if let Some(links) = info.links.as_ref() {
            if let Some(last) = links.last() {
                println!("[play_handler] fallback to last link in list");
                chosen_link = Some(last.clone());
            }
        }
    }

    let link = match chosen_link {
        Some(l) => l,
        None => {
            println!("[play_handler] no usable links, returning pending");
            return (
                StatusCode::ACCEPTED,
                Json(PlayResponse {
                    status: "pending".into(),
                    stream_url: None,
                    message: Some(
                        "Real-Debrid has the torrent, but no usable links yet. Try again later."
                            .into(),
                    ),
                }),
            );
        }
    };

    // 5) unrestrict hoster link to get final stream URL
    let unrestrict_url = "https://api.real-debrid.com/rest/1.0/unrestrict/link";

    let resp = match client
        .post(unrestrict_url)
        .bearer_auth(&state.rd_token)
        .form(&[("link", link.as_str())])
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            eprintln!("[play_handler] unrestrict/link error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(PlayResponse {
                    status: "error".into(),
                    stream_url: None,
                    message: Some(
                        "Failed to unrestrict Real-Debrid link.".into(),
                    ),
                }),
            );
        }
    };

    if !resp.status().is_success() {
        eprintln!("[play_handler] unrestrict/link status {}", resp.status());
        return (
            StatusCode::BAD_GATEWAY,
            Json(PlayResponse {
                status: "error".into(),
                stream_url: None,
                message: Some(
                    "Real-Debrid did not unrestrict the link.".into(),
                ),
            }),
        );
    }

    let unrestrict: RdUnrestrictResponse = match resp.json().await {
        Ok(d) => d,
        Err(e) => {
            eprintln!("[play_handler] unrestrict/link json error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(PlayResponse {
                    status: "error".into(),
                    stream_url: None,
                    message: Some(
                        "Failed to parse unrestrict/link response.".into(),
                    ),
                }),
            );
        }
    };

    println!("[play_handler] READY, returning stream URL");
    (
        StatusCode::OK,
        Json(PlayResponse {
            status: "ready".into(),
            stream_url: Some(unrestrict.download),
            message: None,
        }),
    )
}

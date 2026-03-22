use crate::models::{SourceItem, TorrentioResponse};

pub fn detect_quality(title: &str) -> String {
    let lower = title.to_lowercase();
    if lower.contains("2160p") || lower.contains("4k") {
        "2160p".into()
    } else if lower.contains("1080p") {
        "1080p".into()
    } else if lower.contains("720p") {
        "720p".into()
    } else {
        "unknown".into()
    }
}

pub fn quality_rank(q: &str) -> i32 {
    match q {
        "2160p" => 0,
        "1080p" => 1,
        "720p" => 2,
        _ => 3,
    }
}

pub fn detect_provider(title: &str) -> Option<String> {
    let lower = title.to_lowercase();

    let providers = [
        "1337x", "yts", "rarbg", "piratebay", "thepiratebay", "torrent9",
        "nyaa", "sokudo", "ember", "judas", "tigole", "evo", "qxr",
        "mejortorrent", "cinecalidad", "btm", "trial", "sp3ddy94",
    ];

    for p in providers {
        if lower.contains(p) {
            return Some(p.to_string());
        }
    }

    // detect [Uploader] tags
    if lower.starts_with('[') {
        if let Some(end_idx) = lower.find(']') {
            let tag = &title[1..end_idx];
            return Some(tag.to_string());
        }
    }

    None
}

pub fn streams_to_sources(torrentio: TorrentioResponse, tmdb_id: i64) -> Vec<SourceItem> {
    let mut sources: Vec<SourceItem> = torrentio
        .streams
        .into_iter()
        .filter_map(|s| {
            let hash = s.infoHash?;
            let title = s.title.or(s.name).unwrap_or_else(|| "Unknown".to_string());

            let quality = detect_quality(&title);
            let provider = detect_provider(&title);
            let size = s.size.unwrap_or(0);
            let seeds = s.seeds.unwrap_or(0);

            Some(SourceItem {
                id: format!("{}-{}", tmdb_id, &hash),
                title,
                quality,
                provider,          // ⭐ added
                size_bytes: size,
                seeds,
                info_hash: hash,
                is_cached: false,
            })
        })
        .collect();

    // sort best → worst
    sources.sort_by(|a, b| {
        let qa = quality_rank(&a.quality);
        let qb = quality_rank(&b.quality);
        qa.cmp(&qb).then(b.size_bytes.cmp(&a.size_bytes))
    });

    sources
}

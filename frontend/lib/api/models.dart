import 'dart:convert';

// ---------- HOME / MOVIES ----------

class MovieCard {
  final int tmdbId;
  final String title;
  final String posterUrl;
  final String? backdropUrl;
  final int? year;
  final double? rating;
  final String? mediaType; // <-- added

  MovieCard({
    required this.tmdbId,
    required this.title,
    required this.posterUrl,
    this.backdropUrl,
    this.year,
    this.rating,
    this.mediaType, // <-- added
  });

  factory MovieCard.fromJson(Map<String, dynamic> json) {
    return MovieCard(
      tmdbId: json['tmdb_id'] as int,
      title: json['title'] as String,
      posterUrl: json['poster_url'] as String,
      backdropUrl: json['backdrop_url'] as String?,
      year: json['year'] as int?,
      rating: (json['rating'] as num?)?.toDouble(),
      mediaType: json['media_type'] as String?, // <-- added
    );
  }
}

class MovieSection {
  final String title;
  final List<MovieCard> items;

  MovieSection({
    required this.title,
    required this.items,
  });

  factory MovieSection.fromJson(Map<String, dynamic> json) {
    return MovieSection(
      title: json['title'] as String,
      items: (json['items'] as List<dynamic>)
          .map((e) => MovieCard.fromJson(e as Map<String, dynamic>))
          .toList(),
    );
  }
}

class HomeResponse {
  final List<MovieSection> sections;

  HomeResponse({required this.sections});

  factory HomeResponse.fromJson(Map<String, dynamic> json) {
    return HomeResponse(
      sections: (json['sections'] as List<dynamic>)
          .map((e) => MovieSection.fromJson(e as Map<String, dynamic>))
          .toList(),
    );
  }
}

// ---------- MOVIE DETAILS ----------

class MovieDetails {
  final int tmdbId;
  final String? imdbId;
  final String title;
  final String overview;
  final int? year;
  final int? runtime;
  final String? posterUrl;
  final String? backdropUrl;
  final double? rating;

  MovieDetails({
    required this.tmdbId,
    required this.imdbId,
    required this.title,
    required this.overview,
    required this.year,
    required this.runtime,
    required this.posterUrl,
    required this.backdropUrl,
    required this.rating,
  });

  factory MovieDetails.fromJson(Map<String, dynamic> json) {
    return MovieDetails(
      tmdbId: json['tmdb_id'] as int,
      imdbId: json['imdb_id'] as String?,
      title: json['title'] as String,
      overview: json['overview'] as String,
      year: json['year'] as int?,
      runtime: json['runtime'] as int?,
      posterUrl: json['poster_url'] as String?,
      backdropUrl: json['backdrop_url'] as String?,
      rating: (json['rating'] as num?)?.toDouble(),
    );
  }
}

// ---------- SOURCES + PLAY ----------

class SourceItem {
  final String id;
  final String title;
  final String quality;
  final int sizeBytes;
  final int seeds;
  final String infoHash;
  final bool isCached;
  final String? provider;

  SourceItem({
    required this.id,
    required this.title,
    required this.quality,
    required this.sizeBytes,
    required this.seeds,
    required this.infoHash,
    required this.isCached,
    this.provider,
  });

  factory SourceItem.fromJson(Map<String, dynamic> json) {
    return SourceItem(
      id: json['id'] as String,
      title: json['title'] as String,
      quality: json['quality'] as String,
      sizeBytes: (json['size_bytes'] as num).toInt(),
      seeds: (json['seeds'] as num).toInt(),
      infoHash: json['info_hash'] as String,
      isCached: json['is_cached'] as bool,
      provider: json['provider'] as String?,
    );
  }
}

class SourcesResponse {
  final int tmdbId;
  final List<SourceItem> sources;

  SourcesResponse({
    required this.tmdbId,
    required this.sources,
  });

  factory SourcesResponse.fromJson(Map<String, dynamic> json) {
    return SourcesResponse(
      tmdbId: json['tmdb_id'] as int,
      sources: (json['sources'] as List<dynamic>)
          .map((e) => SourceItem.fromJson(e as Map<String, dynamic>))
          .toList(),
    );
  }
}

class PlayResponse {
  final String status; // ready | pending | error
  final String? streamUrl;
  final String? message;

  PlayResponse({
    required this.status,
    required this.streamUrl,
    required this.message,
  });

  factory PlayResponse.fromJson(Map<String, dynamic> json) {
    return PlayResponse(
      status: json['status'] as String,
      streamUrl: json['stream_url'] as String?,
      message: json['message'] as String?,
    );
  }
}

// ---------- TV / SERIES ----------

class TvSeriesDetails {
  final int tmdbId;
  final String name;
  final String overview;
  final String? posterUrl;
  final String? backdropUrl;
  final String? firstAirDate;
  final String? lastAirDate;
  final int numberOfSeasons;
  final int numberOfEpisodes;
  final double? rating;
  final List<TvSeasonSummary> seasons;

  TvSeriesDetails({
    required this.tmdbId,
    required this.name,
    required this.overview,
    required this.posterUrl,
    required this.backdropUrl,
    required this.firstAirDate,
    required this.lastAirDate,
    required this.numberOfSeasons,
    required this.numberOfEpisodes,
    required this.rating,
    required this.seasons,
  });

  factory TvSeriesDetails.fromJson(Map<String, dynamic> json) {
    return TvSeriesDetails(
      tmdbId: json['tmdb_id'] as int,
      name: json['name'] as String,
      overview: json['overview'] as String,
      posterUrl: json['poster_url'] as String?,
      backdropUrl: json['backdrop_url'] as String?,
      firstAirDate: json['first_air_date'] as String?,
      lastAirDate: json['last_air_date'] as String?,
      numberOfSeasons: json['number_of_seasons'] as int,
      numberOfEpisodes: json['number_of_episodes'] as int,
      rating: (json['rating'] as num?)?.toDouble(),
      seasons: (json['seasons'] as List<dynamic>)
          .map((e) => TvSeasonSummary.fromJson(e as Map<String, dynamic>))
          .toList(),
    );
  }
}

class TvSeasonSummary {
  final int seasonNumber;
  final String name;
  final int episodeCount;
  final String? airDate;
  final String? posterUrl;

  TvSeasonSummary({
    required this.seasonNumber,
    required this.name,
    required this.episodeCount,
    required this.airDate,
    required this.posterUrl,
  });

  factory TvSeasonSummary.fromJson(Map<String, dynamic> json) {
    return TvSeasonSummary(
      seasonNumber: json['season_number'] as int,
      name: json['name'] as String,
      episodeCount: json['episode_count'] as int,
      airDate: json['air_date'] as String?,
      posterUrl: json['poster_url'] as String?,
    );
  }
}

class TvEpisode {
  final int seasonNumber;
  final int episodeNumber;
  final String name;
  final String overview;
  final String? stillUrl;
  final String? airDate;
  final int? runtime;
  final double? rating;

  TvEpisode({
    required this.seasonNumber,
    required this.episodeNumber,
    required this.name,
    required this.overview,
    required this.stillUrl,
    required this.airDate,
    required this.runtime,
    required this.rating,
  });

  factory TvEpisode.fromJson(Map<String, dynamic> json) {
    return TvEpisode(
      seasonNumber: json['season_number'] as int,
      episodeNumber: json['episode_number'] as int,
      name: json['name'] as String,
      overview: json['overview'] as String,
      stillUrl: json['still_url'] as String?,
      airDate: json['air_date'] as String?,
      runtime: json['runtime'] as int?,
      rating: (json['rating'] as num?)?.toDouble(),
    );
  }
}

class TvSeasonEpisodesResponse {
  final int tmdbId;
  final int seasonNumber;
  final List<TvEpisode> episodes;

  TvSeasonEpisodesResponse({
    required this.tmdbId,
    required this.seasonNumber,
    required this.episodes,
  });

  factory TvSeasonEpisodesResponse.fromJson(Map<String, dynamic> json) {
    return TvSeasonEpisodesResponse(
      tmdbId: json['tmdb_id'] as int,
      seasonNumber: json['season_number'] as int,
      episodes: (json['episodes'] as List<dynamic>)
          .map((e) => TvEpisode.fromJson(e as Map<String, dynamic>))
          .toList(),
    );
  }
}

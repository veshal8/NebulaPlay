// lib/api/api.dart
import 'dart:convert';
import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:http/http.dart' as http;

import 'models.dart';

class BackendApi {
  static const String baseUrl = 'http://127.0.0.1:8080';

  // ----- HOME -----

  Future<HomeResponse> getHome({required String kind}) async {
    String path;
    switch (kind) {
      case 'anime':
        path = '/api/home/anime';
        break;
      case 'countries':
        path = '/api/home/countries';
        break;
      case 'tv':
        path = '/api/home/tv';
        break;
      case 'movie':
      default:
        path = '/api/home';
        break;
    }

    final uri = Uri.parse('$baseUrl$path');
    final res = await http.get(uri);

    if (res.statusCode != 200) {
      throw Exception('getHome($kind) failed: ${res.statusCode}');
    }
    return HomeResponse.fromJson(jsonDecode(res.body));
  }

  Future<HomeResponse> search(String query, {required String kind}) async {
    final uri = Uri.parse('$baseUrl/api/search').replace(
      queryParameters: {
        'q': query,
        'kind': kind,
      },
    );

    final res = await http.get(uri);
    if (res.statusCode != 200) {
      throw Exception('search failed: ${res.statusCode}');
    }
    return HomeResponse.fromJson(jsonDecode(res.body));
  }

  // ----- MOVIES -----

  Future<MovieDetails> getMovieDetails(int tmdbId) async {
    final uri = Uri.parse('$baseUrl/api/movie/$tmdbId');
    final res = await http.get(uri);

    if (res.statusCode != 200) {
      throw Exception('getMovieDetails failed: ${res.statusCode}');
    }
    return MovieDetails.fromJson(jsonDecode(res.body));
  }

  Future<SourcesResponse> getSources(int tmdbId) async {
    final uri = Uri.parse('$baseUrl/api/movie/$tmdbId/sources');
    final res = await http.get(uri);

    if (res.statusCode != 200) {
      throw Exception('getSources failed: ${res.statusCode}');
    }
    return SourcesResponse.fromJson(jsonDecode(res.body));
  }

  /// Now supports optional episodeTag (for TV), movies can ignore it.
  Future<PlayResponse> play(
    String infoHash, {
    String? episodeTag,
  }) async {
    final uri = Uri.parse('$baseUrl/api/play');

    final body = <String, dynamic>{
      'info_hash': infoHash,
    };
    if (episodeTag != null) {
      body['episode_tag'] = episodeTag;
    }

    final res = await http.post(
      uri,
      headers: {'Content-Type': 'application/json'},
      body: jsonEncode(body),
    );

    if (res.statusCode != 200 && res.statusCode != 202) {
      throw Exception('play failed: ${res.statusCode}');
    }
    return PlayResponse.fromJson(jsonDecode(res.body));
  }

  // ----- TV / SERIES -----

  Future<TvSeriesDetails> getTvDetails(int tmdbId) async {
    final uri = Uri.parse('$baseUrl/api/tv/$tmdbId');
    final res = await http.get(uri);

    if (res.statusCode != 200) {
      throw Exception('getTvDetails failed: ${res.statusCode}');
    }
    return TvSeriesDetails.fromJson(jsonDecode(res.body));
  }

  Future<TvSeasonEpisodesResponse> getTvSeason(
      int tmdbId, int seasonNumber) async {
    final uri = Uri.parse('$baseUrl/api/tv/$tmdbId/season/$seasonNumber');
    final res = await http.get(uri);

    if (res.statusCode != 200) {
      throw Exception('getTvSeason failed: ${res.statusCode}');
    }
    return TvSeasonEpisodesResponse.fromJson(jsonDecode(res.body));
  }

  /// Fetch sources for a specific TV episode
  Future<SourcesResponse> getTvEpisodeSources(
      int tmdbId, int season, int episode) async {
    final uri = Uri.parse(
        '$baseUrl/api/tv/$tmdbId/season/$season/episode/$episode/sources');

    final res = await http.get(uri);

    if (res.statusCode != 200) {
      throw Exception(
          'getTvEpisodeSources failed: ${res.statusCode} (${res.body})');
    }

    return SourcesResponse.fromJson(jsonDecode(res.body));
  }
}

// ------------------------------------------------------------
// REAL MPV LAUNCH FUNCTION (Windows, Linux, macOS Compatible)
// ------------------------------------------------------------
Future<void> playWithMpv(String url) async {
  try {
    debugPrint('Launching MPV with: $url');

    // MPV is in PATH, so we can call it directly
    await Process.start(
      'mpv',
      [url],
      mode: ProcessStartMode.detached, // Run in separate window
    );
  } catch (e) {
    debugPrint('mpv launch error: $e');

    // Attempt fallback using mpv.com (sometimes required on Windows)
    try {
      await Process.start(
        'mpv.com',
        [url],
        mode: ProcessStartMode.detached,
      );
    } catch (e2) {
      debugPrint('mpv.com launch error: $e2');
    }
  }
}

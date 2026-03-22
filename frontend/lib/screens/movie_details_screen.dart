// lib/screens/movie_details_screen.dart

import 'package:flutter/material.dart';

import '../api/api.dart';
import '../api/models.dart';
import '../widgets/glass.dart';
import '../widgets/hero_banner.dart' show HeroTag;
import 'series_details_screen.dart';

class MovieDetailsScreen extends StatefulWidget {
  final int tmdbId;
  final String? mediaType; // <--- IMPORTANT (movie / tv)

  const MovieDetailsScreen({
    super.key,
    required this.tmdbId,
    this.mediaType,
  });

  @override
  State<MovieDetailsScreen> createState() => _MovieDetailsScreenState();
}

class _MovieDetailsScreenState extends State<MovieDetailsScreen> {
  final _api = BackendApi();

  Future<MovieDetails>? _futureDetails;
  Future<SourcesResponse>? _futureSources;

  @override
  void initState() {
    super.initState();

    // 👇 If mis-clicked but mediaType=tv → auto redirect
    if (widget.mediaType == "tv") {
      Future.microtask(() {
        Navigator.pushReplacement(
          context,
          MaterialPageRoute(
            builder: (_) => SeriesDetailsScreen(tmdbId: widget.tmdbId),
          ),
        );
      });
      return;
    }

    _futureDetails = _api.getMovieDetails(widget.tmdbId);
    _futureSources = _api.getSources(widget.tmdbId);
  }

  // ---------- PLAY SOURCE ----------
  Future<void> _playSource(SourceItem source) async {
    try {
      final resp = await _api.play(source.infoHash);

      if (resp.status == 'ready' && resp.streamUrl != null) {
        await playWithMpv(resp.streamUrl!);
      } else {
        final msg =
            resp.message ?? 'Source not ready (status: ${resp.status})';

        if (!mounted) return;
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text(msg)),
        );
      }
    } catch (e) {
      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Failed to play: $e')),
      );
    }
  }

  // ---------------------------------------------------------

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF020617),
      appBar: AppBar(
        title: const Text('Details'),
      ),
      body: Row(
        children: [
          // ======================================================
          // LEFT: MOVIE DETAILS PANEL
          // ======================================================
          Expanded(
            flex: 2,
            child: FutureBuilder<MovieDetails>(
              future: _futureDetails,
              builder: (context, snapshot) {
                if (snapshot.connectionState == ConnectionState.waiting) {
                  return const Center(child: CircularProgressIndicator());
                }
                if (!snapshot.hasData) {
                  return Center(
                    child: Text("Error: ${snapshot.error}"),
                  );
                }

                final m = snapshot.data!;

                return SingleChildScrollView(
                  padding: const EdgeInsets.all(20),
                  child: glass(
                    radius: 24,
                    padding: const EdgeInsets.all(16),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        // ---------- Poster / Backdrop ----------
                        if (m.backdropUrl != null)
                          ClipRRect(
                            borderRadius: BorderRadius.circular(18),
                            child: AspectRatio(
                              aspectRatio: 16 / 9,
                              child: Image.network(
                                m.backdropUrl!,
                                fit: BoxFit.cover,
                                filterQuality: FilterQuality.high,
                              ),
                            ),
                          )
                        else if (m.posterUrl != null)
                          SizedBox(
                            height: 320,
                            child: ClipRRect(
                              borderRadius: BorderRadius.circular(18),
                              child: Image.network(
                                m.posterUrl!,
                                fit: BoxFit.cover,
                              ),
                            ),
                          ),

                        const SizedBox(height: 16),

                        // ---------- TITLE ----------
                        Text(
                          '${m.title} (${m.year ?? ''})',
                          style: Theme.of(context)
                              .textTheme
                              .headlineSmall
                              ?.copyWith(
                                fontWeight: FontWeight.w800,
                              ),
                        ),

                        const SizedBox(height: 8),

                        // ---------- TAGS ----------
                        Wrap(
                          spacing: 8,
                          runSpacing: 4,
                          children: [
                            if (m.runtime != null)
                              HeroTag(label: '${m.runtime} min'),
                            if (m.rating != null)
                              HeroTag(
                                icon: Icons.star_rounded,
                                label: m.rating!.toStringAsFixed(1),
                              ),
                            if (m.imdbId != null)
                              HeroTag(label: 'IMDB: ${m.imdbId}'),
                          ],
                        ),

                        const SizedBox(height: 16),

                        // ---------- OVERVIEW ----------
                        Text(
                          m.overview,
                          style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                                color: Colors.white.withOpacity(0.9),
                              ),
                        ),
                      ],
                    ),
                  ),
                );
              },
            ),
          ),

          const VerticalDivider(width: 1, color: Colors.white10),

          // ======================================================
          // RIGHT: SOURCES PANEL
          // ======================================================
          Expanded(
            flex: 3,
            child: FutureBuilder<SourcesResponse>(
              future: _futureSources,
              builder: (context, snapshot) {
                if (snapshot.connectionState == ConnectionState.waiting) {
                  return const Center(child: CircularProgressIndicator());
                }
                if (!snapshot.hasData) {
                  return Center(
                    child: Text("Error: ${snapshot.error}"),
                  );
                }

                final sources = snapshot.data!.sources;

                if (sources.isEmpty) {
                  return const Center(
                    child: Text("No sources found"),
                  );
                }

                return Padding(
                  padding: const EdgeInsets.all(20),
                  child: glass(
                    radius: 24,
                    padding: const EdgeInsets.only(
                      left: 12,
                      right: 12,
                      top: 12,
                      bottom: 4,
                    ),
                    child: ListView.separated(
                      itemCount: sources.length,
                      separatorBuilder: (_, __) =>
                          const Divider(height: 1, color: Colors.white12),
                      itemBuilder: (context, index) {
                        final s = sources[index];
                        final sizeGb =
                            (s.sizeBytes / (1024 * 1024 * 1024))
                                .toStringAsFixed(1);

                        return ListTile(
                          contentPadding:
                              const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                          title: Text(
                            s.title,
                            maxLines: 2,
                            overflow: TextOverflow.ellipsis,
                          ),
                          subtitle: Text(
                            '${s.quality} • $sizeGb GB • Seeds: ${s.seeds}',
                            style: TextStyle(
                              color: Colors.white.withOpacity(0.8),
                            ),
                          ),
                          trailing: IconButton(
                            icon: const Icon(Icons.play_arrow_rounded),
                            onPressed: () => _playSource(s),
                          ),
                        );
                      },
                    ),
                  ),
                );
              },
            ),
          ),
        ],
      ),
    );
  }
}

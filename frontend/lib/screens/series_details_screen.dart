// lib/screens/series_details_screen.dart
import 'dart:ui';
import 'package:flutter/material.dart';

import '../api/api.dart';
import '../api/models.dart';

class SeriesDetailsScreen extends StatefulWidget {
  final int tmdbId;

  const SeriesDetailsScreen({super.key, required this.tmdbId});

  @override
  State<SeriesDetailsScreen> createState() => _SeriesDetailsScreenState();
}

class _SeriesDetailsScreenState extends State<SeriesDetailsScreen> {
  final _api = BackendApi();

  Future<TvSeriesDetails>? _futureSeries;
  Future<TvSeasonEpisodesResponse>? _futureSeason;

  int _selectedSeason = 1;

  @override
  void initState() {
    super.initState();
    _futureSeries = _api.getTvDetails(widget.tmdbId);
    _futureSeason = _api.getTvSeason(widget.tmdbId, _selectedSeason);
  }

  void _switchSeason(int seasonNumber) {
    setState(() {
      _selectedSeason = seasonNumber;
      _futureSeason = _api.getTvSeason(widget.tmdbId, seasonNumber);
    });
  }

  // -----------------------------------------------------------
  // OPEN SOURCE SELECTOR FOR EACH EPISODE
  // -----------------------------------------------------------

  Future<void> _openSourcesForEpisode(TvEpisode ep) async {
    try {
      final resp = await _api.getTvEpisodeSources(
        widget.tmdbId,
        ep.seasonNumber,
        ep.episodeNumber,
      );

      if (!mounted) return;

      if (resp.sources.isEmpty) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('No sources found')),
        );
        return;
      }

      // Show modal with available sources
      showModalBottomSheet(
        context: context,
        backgroundColor: const Color(0xFF0B1120),
        shape: const RoundedRectangleBorder(
          borderRadius: BorderRadius.vertical(top: Radius.circular(20)),
        ),
        builder: (context) => _buildSourceList(ep, resp.sources),
      );
    } catch (e) {
      if (!mounted) return;
      ScaffoldMessenger.of(context)
          .showSnackBar(SnackBar(content: Text('Error loading sources: $e')));
    }
  }

  Widget _buildSourceList(TvEpisode episode, List<SourceItem> sources) {
    return Padding(
      padding: const EdgeInsets.all(16),
      child: ListView.separated(
        itemCount: sources.length,
        separatorBuilder: (_, __) =>
            const Divider(color: Colors.white12, height: 1),
        itemBuilder: (context, index) {
          final s = sources[index];
          final sizeGb =
              (s.sizeBytes / (1024 * 1024 * 1024)).toStringAsFixed(2);

          return ListTile(
            title: Text(
              s.title,
              maxLines: 2,
              overflow: TextOverflow.ellipsis,
            ),
            subtitle: Text(
              [
                s.quality,
                if (s.provider != null && s.provider!.isNotEmpty)
                  'Source: ${s.provider}',
                '$sizeGb GB',
                'Seeds: ${s.seeds}',
              ].join(' • '),
              style: TextStyle(color: Colors.white.withOpacity(0.8)),
            ),
            trailing: const Icon(Icons.play_arrow_rounded),
            onTap: () => _playSource(episode, s),
          );
        },
      ),
    );
  }

  Future<void> _playSource(TvEpisode episode, SourceItem source) async {
    try {
      final tag =
          'S${episode.seasonNumber.toString().padLeft(2, '0')}E${episode.episodeNumber.toString().padLeft(2, '0')}';

      debugPrint('[_playSource] epTag=$tag, hash=${source.infoHash}');

      final resp = await _api.play(
        source.infoHash,
        episodeTag: tag,
      );

      if (!mounted) return;

      if (resp.status == 'ready' && resp.streamUrl != null) {
        await playWithMpv(resp.streamUrl!);
      } else {
        final msg = resp.message ?? 'Not ready: ${resp.status}';
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text(msg)),
        );
      }
    } catch (e) {
      if (!mounted) return;
      ScaffoldMessenger.of(context)
          .showSnackBar(SnackBar(content: Text('Play failed: $e')));
    }
  }

  // -----------------------------------------------------------

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF020617),
      appBar: AppBar(
        title: const Text('Series'),
      ),
      body: Row(
        children: [
          // LEFT — HERO PANEL
          Expanded(
            flex: 2,
            child: FutureBuilder<TvSeriesDetails>(
              future: _futureSeries,
              builder: (context, snapshot) {
                if (!snapshot.hasData) {
                  return const Center(child: CircularProgressIndicator());
                }

                final series = snapshot.data!;

                return SingleChildScrollView(
                  padding: const EdgeInsets.all(20),
                  child: _glass(
                    radius: 24,
                    padding: const EdgeInsets.all(16),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        if (series.backdropUrl != null)
                          ClipRRect(
                            borderRadius: BorderRadius.circular(18),
                            child: AspectRatio(
                              aspectRatio: 16 / 9,
                              child: Image.network(
                                series.backdropUrl!,
                                fit: BoxFit.cover,
                              ),
                            ),
                          )
                        else if (series.posterUrl != null)
                          SizedBox(
                            height: 320,
                            child: ClipRRect(
                              borderRadius: BorderRadius.circular(18),
                              child: Image.network(
                                series.posterUrl!,
                                fit: BoxFit.cover,
                              ),
                            ),
                          ),

                        const SizedBox(height: 16),

                        Text(
                          series.name,
                          style: Theme.of(context)
                              .textTheme
                              .headlineSmall
                              ?.copyWith(fontWeight: FontWeight.w800),
                        ),

                        const SizedBox(height: 8),
                        Wrap(
                          spacing: 8,
                          children: [
                            _HeroChip(
                              label: _formatYearRange(
                                  series.firstAirDate, series.lastAirDate),
                            ),
                            _HeroChip(
                              label:
                                  '${series.numberOfSeasons} season${series.numberOfSeasons == 1 ? '' : 's'}',
                            ),
                            _HeroChip(
                              label:
                                  '${series.numberOfEpisodes} episode${series.numberOfEpisodes == 1 ? '' : 's'}',
                            ),
                            if (series.rating != null)
                              _HeroChip(
                                icon: Icons.star_rounded,
                                label: series.rating!.toStringAsFixed(1),
                              ),
                          ],
                        ),

                        const SizedBox(height: 16),
                        Text(
                          series.overview,
                          style: TextStyle(
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

          // RIGHT — EPISODES PANEL
          Expanded(
            flex: 3,
            child: Column(
              children: [
                FutureBuilder<TvSeriesDetails>(
                  future: _futureSeries,
                  builder: (context, snapshot) {
                    if (!snapshot.hasData) return const SizedBox.shrink();

                    final seasons = snapshot.data!.seasons;

                    return SingleChildScrollView(
                      scrollDirection: Axis.horizontal,
                      padding: const EdgeInsets.all(12),
                      child: Row(
                        children: [
                          for (final s in seasons)
                            Padding(
                              padding: const EdgeInsets.only(right: 8),
                              child: GestureDetector(
                                onTap: () => _switchSeason(s.seasonNumber),
                                child: _glass(
                                  radius: 999,
                                  padding: const EdgeInsets.symmetric(
                                      horizontal: 12, vertical: 6),
                                  child: Text(
                                    'Season ${s.seasonNumber}',
                                    style: TextStyle(
                                      fontSize: 12,
                                      fontWeight: FontWeight.w600,
                                      color: _selectedSeason == s.seasonNumber
                                          ? Colors.white
                                          : Colors.white70,
                                    ),
                                  ),
                                ),
                              ),
                            )
                        ],
                      ),
                    );
                  },
                ),

                const Divider(height: 1, color: Colors.white12),

                Expanded(
                  child: FutureBuilder<TvSeasonEpisodesResponse>(
                    future: _futureSeason,
                    builder: (context, snapshot) {
                      if (!snapshot.hasData) {
                        return const Center(
                            child: CircularProgressIndicator());
                      }

                      final episodes = snapshot.data!.episodes;

                      return ListView.separated(
                        padding: const EdgeInsets.all(16),
                        itemCount: episodes.length,
                        separatorBuilder: (_, __) =>
                            const SizedBox(height: 10),
                        itemBuilder: (context, i) {
                          final ep = episodes[i];

                          return _EpisodeCard(
                            episode: ep,
                            onPlay: () => _openSourcesForEpisode(ep),
                          );
                        },
                      );
                    },
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  // Format years
  String _formatYearRange(String? first, String? last) {
    String year(String? d) =>
        (d != null && d.length >= 4) ? d.substring(0, 4) : '';
    final f = year(first);
    final l = year(last);

    if (f.isEmpty && l.isEmpty) return 'TV Series';
    if (l.isEmpty || f == l) return f;

    return '$f – $l';
  }
}

class _HeroChip extends StatelessWidget {
  final IconData? icon;
  final String label;

  const _HeroChip({this.icon, required this.label});

  @override
  Widget build(BuildContext context) {
    return _glass(
      radius: 999,
      padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 6),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          if (icon != null) ...[
            Icon(icon, size: 14, color: Colors.amberAccent),
            const SizedBox(width: 4),
          ],
          Text(label, style: const TextStyle(fontSize: 12)),
        ],
      ),
    );
  }
}

class _EpisodeCard extends StatelessWidget {
  final TvEpisode episode;
  final VoidCallback onPlay;

  const _EpisodeCard({required this.episode, required this.onPlay});

  @override
  Widget build(BuildContext context) {
    return _glass(
      radius: 18,
      padding: const EdgeInsets.all(10),
      child: Row(
        children: [
          SizedBox(
            width: 140,
            height: 80,
            child: ClipRRect(
              borderRadius: BorderRadius.circular(12),
              child: episode.stillUrl != null
                  ? Image.network(episode.stillUrl!, fit: BoxFit.cover)
                  : Container(
                      color: Colors.grey[900],
                      child: const Icon(Icons.tv, color: Colors.white38),
                    ),
            ),
          ),
          const SizedBox(width: 12),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  'E${episode.episodeNumber.toString().padLeft(2, '0')} · ${episode.name}',
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                  style: const TextStyle(
                    fontSize: 14,
                    fontWeight: FontWeight.w700,
                  ),
                ),
                const SizedBox(height: 4),
                Text(
                  episode.overview,
                  maxLines: 2,
                  overflow: TextOverflow.ellipsis,
                  style: TextStyle(
                    fontSize: 12,
                    color: Colors.white.withOpacity(0.8),
                  ),
                ),
                const SizedBox(height: 4),
                Row(
                  children: [
                    if (episode.runtime != null)
                      Text(
                        '${episode.runtime} min',
                        style: const TextStyle(fontSize: 11),
                      ),
                    if (episode.airDate != null) ...[
                      const SizedBox(width: 8),
                      Text(
                        episode.airDate!,
                        style: const TextStyle(fontSize: 11),
                      ),
                    ],
                    if (episode.rating != null) ...[
                      const SizedBox(width: 8),
                      Icon(
                        Icons.star_rounded,
                        size: 12,
                        color: Colors.amber.shade400,
                      ),
                      Text(
                        episode.rating!.toStringAsFixed(1),
                        style: const TextStyle(fontSize: 11),
                      )
                    ],
                  ],
                ),
              ],
            ),
          ),
          IconButton(
            icon: const Icon(Icons.play_arrow_rounded),
            onPressed: onPlay,
          ),
        ],
      ),
    );
  }
}

// Glass
Widget _glass({
  required Widget child,
  double radius = 24,
  EdgeInsets padding = const EdgeInsets.all(16),
  EdgeInsets margin = EdgeInsets.zero,
}) {
  return Container(
    margin: margin,
    child: ClipRRect(
      borderRadius: BorderRadius.circular(radius),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 20, sigmaY: 20),
        child: Container(
          padding: padding,
          decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(radius),
            border: Border.all(color: Colors.white.withOpacity(0.08)),
            gradient: LinearGradient(
              colors: [
                Colors.white.withOpacity(0.12),
                Colors.white.withOpacity(0.03),
              ],
              begin: Alignment.topLeft,
              end: Alignment.bottomRight,
            ),
          ),
          child: child,
        ),
      ),
    ),
  );
}

import 'package:flutter/material.dart';

import '../api/models.dart';
import 'glass.dart';

class HeroBanner extends StatelessWidget {
  final MovieCard movie;
  final VoidCallback onOpenDetails;

  const HeroBanner({
    super.key,
    required this.movie,
    required this.onOpenDetails,
  });

  @override
  Widget build(BuildContext context) {
    final backgroundUrl = movie.backdropUrl?.isNotEmpty == true
        ? movie.backdropUrl!
        : movie.posterUrl;

    return AspectRatio(
      aspectRatio: 16 / 6,
      child: ClipRRect(
        borderRadius: BorderRadius.circular(32),
        child: Stack(
          fit: StackFit.expand,
          children: [
            // Background image (prefer backdrop)
            if (backgroundUrl.isNotEmpty)
              Image.network(
                backgroundUrl,
                fit: BoxFit.cover,
                filterQuality: FilterQuality.high,
              ),
            // Nebula gradient overlay
            Container(
              decoration: const BoxDecoration(
                gradient: LinearGradient(
                  colors: [
                    Color(0xFF020617),
                    Color(0x66020F2F),
                    Colors.transparent,
                  ],
                  begin: Alignment.centerLeft,
                  end: Alignment.centerRight,
                ),
              ),
            ),
            // Content
            Padding(
              padding: const EdgeInsets.fromLTRB(28, 20, 24, 24),
              child: Row(
                children: [
                  // Glass poster card on the left
                  Expanded(
                    flex: 2,
                    child: Align(
                      alignment: Alignment.centerLeft,
                      child: glass(
                        radius: 24,
                        padding: const EdgeInsets.all(8),
                        child: AspectRatio(
                          aspectRatio: 2 / 3,
                          child: ClipRRect(
                            borderRadius: BorderRadius.circular(18),
                            child: Image.network(
                              movie.posterUrl,
                              fit: BoxFit.cover,
                              filterQuality: FilterQuality.high,
                            ),
                          ),
                        ),
                      ),
                    ),
                  ),
                  const SizedBox(width: 24),
                  // Text + CTAs
                  Expanded(
                    flex: 4,
                    child: Column(
                      mainAxisAlignment: MainAxisAlignment.center,
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        const Text(
                          'Featured in the Nebula',
                          style: TextStyle(
                            fontSize: 13,
                            fontWeight: FontWeight.w500,
                            color: Colors.white70,
                            letterSpacing: 1.1,
                          ),
                        ),
                        const SizedBox(height: 8),
                        Text(
                          movie.title,
                          maxLines: 2,
                          overflow: TextOverflow.ellipsis,
                          style: const TextStyle(
                            fontSize: 28,
                            fontWeight: FontWeight.w800,
                            letterSpacing: 0.4,
                          ),
                        ),
                        const SizedBox(height: 12),
                        Row(
                          children: [
                            if (movie.year != null)
                              HeroTag(label: movie.year.toString()),
                            if (movie.rating != null) ...[
                              const SizedBox(width: 8),
                              HeroTag(
                                icon: Icons.star_rounded,
                                label: movie.rating!.toStringAsFixed(1),
                              ),
                            ],
                          ],
                        ),
                        const SizedBox(height: 16),
                        Row(
                          children: [
                            ElevatedButton.icon(
                              icon: const Icon(Icons.play_arrow_rounded),
                              label: const Text('Open Details'),
                              style: ElevatedButton.styleFrom(
                                padding: const EdgeInsets.symmetric(
                                    horizontal: 20, vertical: 12),
                                shape: RoundedRectangleBorder(
                                  borderRadius: BorderRadius.circular(999),
                                ),
                              ),
                              onPressed: onOpenDetails,
                            ),
                            const SizedBox(width: 12),
                            OutlinedButton.icon(
                              icon: const Icon(Icons.info_outline),
                              label: const Text('More info'),
                              style: OutlinedButton.styleFrom(
                                padding: const EdgeInsets.symmetric(
                                    horizontal: 18, vertical: 12),
                                shape: RoundedRectangleBorder(
                                  borderRadius: BorderRadius.circular(999),
                                ),
                                side: BorderSide(
                                  color: Colors.white.withOpacity(0.35),
                                ),
                              ),
                              onPressed: onOpenDetails,
                            ),
                          ],
                        ),
                      ],
                    ),
                  ),
                  const Spacer(),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class HeroTag extends StatelessWidget {
  final IconData? icon;
  final String label;

  const HeroTag({
    super.key,
    this.icon,
    required this.label,
  });

  @override
  Widget build(BuildContext context) {
    return glass(
      radius: 999,
      padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 6),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          if (icon != null) ...[
            Icon(icon, size: 14, color: Colors.amberAccent),
            const SizedBox(width: 4),
          ],
          Text(
            label,
            style: const TextStyle(fontSize: 12),
          ),
        ],
      ),
    );
  }
}

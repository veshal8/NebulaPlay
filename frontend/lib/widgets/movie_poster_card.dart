import 'package:flutter/material.dart';

import '../api/models.dart';
import 'glass.dart';

class MoviePosterCard extends StatefulWidget {
  final MovieCard movie;
  final VoidCallback onTap;

  const MoviePosterCard({
    super.key,
    required this.movie,
    required this.onTap,
  });

  @override
  State<MoviePosterCard> createState() => _MoviePosterCardState();
}

class _MoviePosterCardState extends State<MoviePosterCard> {
  bool _hovering = false;

  void _setHover(bool value) {
    if (!mounted) return;
    setState(() {
      _hovering = value;
    });
  }

  @override
  Widget build(BuildContext context) {
    final movie = widget.movie;

    return MouseRegion(
      onEnter: (_) => _setHover(true),
      onExit: (_) => _setHover(false),
      child: GestureDetector(
        onTap: widget.onTap,
        child: AnimatedScale(
          scale: _hovering ? 1.06 : 1.0,
          duration: const Duration(milliseconds: 150),
          curve: Curves.easeOut,
          child: glass(
            radius: 18,
            padding: const EdgeInsets.all(6),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                Expanded(
                  child: ClipRRect(
                    borderRadius: BorderRadius.circular(14),
                    child: Image.network(
                      movie.posterUrl,
                      fit: BoxFit.cover,
                      filterQuality: FilterQuality.high,
                      errorBuilder: (_, __, ___) => Container(
                        color: Colors.grey[900],
                        child: const Icon(
                          Icons.movie,
                          size: 32,
                          color: Colors.white38,
                        ),
                      ),
                    ),
                  ),
                ),
                const SizedBox(height: 6),
                Text(
                  movie.title,
                  maxLines: 2,
                  overflow: TextOverflow.ellipsis,
                  style: const TextStyle(
                    fontSize: 13,
                    fontWeight: FontWeight.w600,
                  ),
                ),
                if (movie.year != null || movie.rating != null)
                  Row(
                    children: [
                      if (movie.year != null)
                        Text(
                          movie.year.toString(),
                          style: TextStyle(
                            fontSize: 11,
                            color: Colors.white.withOpacity(0.7),
                          ),
                        ),
                      if (movie.year != null && movie.rating != null)
                        Text(
                          ' • ',
                          style: TextStyle(
                            fontSize: 11,
                            color: Colors.white.withOpacity(0.5),
                          ),
                        ),
                      if (movie.rating != null)
                        Row(
                          children: [
                            Icon(
                              Icons.star_rounded,
                              size: 12,
                              color: Colors.amber.shade400,
                            ),
                            const SizedBox(width: 2),
                            Text(
                              movie.rating!.toStringAsFixed(1),
                              style: TextStyle(
                                fontSize: 11,
                                color: Colors.white.withOpacity(0.85),
                              ),
                            ),
                          ],
                        ),
                    ],
                  ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}

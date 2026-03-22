import 'package:flutter/material.dart';

import '../api/models.dart';
import 'movie_poster_card.dart';

class SectionRow extends StatelessWidget {
  final MovieSection section;
  final void Function(MovieCard movie) onTapMovie;

  const SectionRow({
    super.key,
    required this.section,
    required this.onTapMovie,
  });

  @override
  Widget build(BuildContext context) {
    if (section.items.isEmpty) return const SizedBox.shrink();

    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 6, horizontal: 8),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Section header
          Padding(
            padding: const EdgeInsets.fromLTRB(4, 4, 4, 8),
            child: Row(
              children: [
                Text(
                  section.title,
                  style: Theme.of(context).textTheme.titleMedium?.copyWith(
                        fontWeight: FontWeight.w700,
                      ),
                ),
                const SizedBox(width: 8),
                Container(
                  width: 28,
                  height: 2,
                  decoration: const BoxDecoration(
                    gradient: LinearGradient(
                      colors: [
                        Color(0xFF38BDF8),
                        Color(0xFFA855F7),
                      ],
                    ),
                    borderRadius: BorderRadius.all(Radius.circular(999)),
                  ),
                ),
              ],
            ),
          ),
          SizedBox(
            height: 250,
            child: ListView.separated(
              scrollDirection: Axis.horizontal,
              padding: const EdgeInsets.only(right: 8),
              itemCount: section.items.length,
              separatorBuilder: (_, __) => const SizedBox(width: 12),
              itemBuilder: (context, index) {
                final movie = section.items[index];
                return SizedBox(
                  width: 155,
                  child: MoviePosterCard(
                    movie: movie,
                    onTap: () => onTapMovie(movie),
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

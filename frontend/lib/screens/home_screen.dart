import 'dart:async';
import 'package:flutter/material.dart';

import '../api/api.dart';
import '../api/models.dart';
import '../widgets/glass.dart';
import '../widgets/nav_pill.dart';
import '../widgets/hero_banner.dart';
import '../widgets/movie_poster_card.dart';
import '../widgets/section_row.dart';
import 'movie_details_screen.dart';
import 'series_details_screen.dart';

class HomeScreen extends StatefulWidget {
  const HomeScreen({super.key});

  @override
  State<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen> {
  final _api = BackendApi();
  Future<HomeResponse>? _futureHome;

  final TextEditingController _searchController = TextEditingController();
  Timer? _searchDebounce;

  bool _isSearching = false;
  bool _isLoadingSearch = false;
  List<MovieCard>? _searchResults;

  String _selectedNav = 'Movies'; // Movies | TV Shows | Anime | Countries

  String _currentSearchKind() {
    switch (_selectedNav) {
      case 'Anime':
        return 'anime';
      case 'Movies':
      case 'TV Shows':
      case 'Countries':
      default:
        return 'movie';
    }
  }

  @override
  void initState() {
    super.initState();
    _futureHome = _api.getHome(kind: 'movie');
  }

  @override
  void dispose() {
    _searchDebounce?.cancel();
    _searchController.dispose();
    super.dispose();
  }

  void _clearSearch() {
    _searchDebounce?.cancel();
    _searchController.clear();
    setState(() {
      _isSearching = false;
      _isLoadingSearch = false;
      _searchResults = null;
    });
  }

  void _onSearchChanged(String value) {
    final query = value.trim();

    if (query.isEmpty) {
      _clearSearch();
      return;
    }

    setState(() {
      _isSearching = true;
    });

    _searchDebounce?.cancel();
    _searchDebounce = Timer(const Duration(milliseconds: 400), () {
      _runSearch(query);
    });
  }

  void _onSearchSubmitted(String value) {
    final query = value.trim();
    if (query.isEmpty) {
      _clearSearch();
      return;
    }
    _searchDebounce?.cancel();
    _runSearch(query);
  }

  Future<void> _runSearch(String query) async {
    setState(() {
      _isLoadingSearch = true;
    });

    try {
      final home = await _api.search(
        query,
        kind: _currentSearchKind(),
      );

      if (!mounted || _searchController.text.trim().isEmpty) return;
      if (_searchController.text.trim() != query) return;

      final List<MovieCard> items =
          home.sections.isNotEmpty ? home.sections.first.items : <MovieCard>[];

      setState(() {
        _searchResults = items;
        _isLoadingSearch = false;
      });
    } catch (e) {
      if (!mounted) return;
      setState(() {
        _searchResults = [];
        _isLoadingSearch = false;
      });
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Search failed: $e')),
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    final query = _searchController.text.trim();

    return Scaffold(
      extendBodyBehindAppBar: true,
      appBar: AppBar(
        toolbarHeight: 60,
        titleSpacing: 24,
        title: Row(
          children: [
            const Text(
              'NebulaPlay',
              style: TextStyle(
                fontSize: 24,
                fontWeight: FontWeight.w800,
                letterSpacing: 1.4,
              ),
            ),
            const SizedBox(width: 24),
            _buildTopNavChips(),
            const Spacer(),
            _buildProfilePill(),
          ],
        ),
      ),
      body: Container(
        decoration: const BoxDecoration(
          gradient: LinearGradient(
            colors: [
              Color(0xFF020617),
              Color(0xFF020617),
              Color(0xFF0B1120),
              Color(0xFF111827),
            ],
            begin: Alignment.topCenter,
            end: Alignment.bottomCenter,
          ),
        ),
        child: SafeArea(
          top: true,
          child: Column(
            children: [
              const SizedBox(height: 12),
              Padding(
                padding: const EdgeInsets.symmetric(horizontal: 24),
                child: _buildSearchBar(query),
              ),
              const SizedBox(height: 16),
              Expanded(
                child: Padding(
                  padding: const EdgeInsets.symmetric(horizontal: 16),
                  child: _isSearching
                      ? _buildSearchBody(query)
                      : _buildHomeBody(),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }

  // ------------------------------------------------
  // TOP NAV
  // ------------------------------------------------

  Widget _buildTopNavChips() {
    final options = ['Movies', 'TV Shows', 'Anime', 'Countries'];

    return Container(
      height: 36,
      padding: const EdgeInsets.symmetric(horizontal: 8),
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(999),
        color: Colors.white.withOpacity(0.02),
        border: Border.all(
          color: Colors.white.withOpacity(0.08),
        ),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          for (final label in options) ...[
            NavPill(
              label: label,
              selected: _selectedNav == label,
              onTap: () {
                setState(() {
                  _selectedNav = label;
                  _clearSearch();

                  switch (label) {
                    case 'Movies':
                      _futureHome = _api.getHome(kind: 'movie');
                      break;
                    case 'Anime':
                      _futureHome = _api.getHome(kind: 'anime');
                      break;
                    case 'Countries':
                      _futureHome = _api.getHome(kind: 'countries');
                      break;
                    case 'TV Shows':
                      _futureHome = _api.getHome(kind: 'tv');
                      break;
                    default:
                      _futureHome = _api.getHome(kind: 'movie');
                      break;
                  }
                });
              },
            ),
            if (label != options.last) const SizedBox(width: 4),
          ]
        ],
      ),
    );
  }

  Widget _buildProfilePill() {
    return glass(
      radius: 999,
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: const [
          CircleAvatar(
            radius: 12,
            backgroundColor: Color(0xFF38BDF8),
            child: Icon(
              Icons.person,
              size: 16,
              color: Colors.black,
            ),
          ),
          SizedBox(width: 8),
          Text(
            'Guest',
            style: TextStyle(fontSize: 13, fontWeight: FontWeight.w500),
          ),
        ],
      ),
    );
  }

  // ------------------------------------------------
  // SEARCH
  // ------------------------------------------------

  Widget _buildSearchBar(String query) {
    return glass(
      radius: 999,
      padding: const EdgeInsets.symmetric(horizontal: 16),
      child: Row(
        children: [
          const Icon(Icons.search, size: 20, color: Colors.white70),
          const SizedBox(width: 8),
          Expanded(
            child: TextField(
              controller: _searchController,
              onChanged: _onSearchChanged,
              onSubmitted: _onSearchSubmitted,
              style: const TextStyle(fontSize: 14),
              decoration: const InputDecoration(
                hintText: 'Search across the nebula...',
                border: InputBorder.none,
                isDense: true,
              ),
            ),
          ),
          if (query.isNotEmpty)
            IconButton(
              icon: const Icon(Icons.close, size: 18),
              onPressed: _clearSearch,
              splashRadius: 18,
            ),
        ],
      ),
    );
  }

  Widget _buildSearchBody(String query) {
    if (_isLoadingSearch) {
      return const Center(child: CircularProgressIndicator());
    }

    if (_searchResults == null) {
      return Center(
        child: Text(
          'Searching for "$query"...',
          style: Theme.of(context).textTheme.titleMedium,
        ),
      );
    }

    if (_searchResults!.isEmpty) {
      return Center(
        child: Text(
          'No results for "$query".',
          style: Theme.of(context).textTheme.titleMedium,
        ),
      );
    }

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        if (query.isNotEmpty)
          Padding(
            padding: const EdgeInsets.fromLTRB(8, 0, 8, 8),
            child: Text(
              'Results for "$query"',
              style: Theme.of(context).textTheme.titleMedium?.copyWith(
                    fontWeight: FontWeight.w600,
                  ),
            ),
          ),
        Expanded(
          child: GridView.builder(
            padding: const EdgeInsets.only(bottom: 24),
            gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
              crossAxisCount: 5,
              crossAxisSpacing: 14,
              mainAxisSpacing: 14,
              childAspectRatio: 2 / 3,
            ),
            itemCount: _searchResults!.length,
            itemBuilder: (context, index) {
              final movie = _searchResults![index];
              return MoviePosterCard(
                movie: movie,
                onTap: () {
                  if (movie.mediaType == 'tv') {
                    Navigator.push(
                      context,
                      MaterialPageRoute(
                        builder: (_) =>
                            SeriesDetailsScreen(tmdbId: movie.tmdbId),
                      ),
                    );
                  } else {
                    Navigator.push(
                      context,
                      MaterialPageRoute(
                        builder: (_) =>
                            MovieDetailsScreen(tmdbId: movie.tmdbId),
                      ),
                    );
                  }
                },
              );
            },
          ),
        ),
      ],
    );
  }

  // ------------------------------------------------
  // HOME BODY
  // ------------------------------------------------

  Widget _buildHomeBody() {
    return FutureBuilder<HomeResponse>(
      future: _futureHome,
      builder: (context, snapshot) {
        if (snapshot.connectionState == ConnectionState.waiting) {
          return const Center(child: CircularProgressIndicator());
        }
        if (snapshot.hasError || !snapshot.hasData) {
          return Center(
            child: Text('Error: ${snapshot.error}'),
          );
        }

        final home = snapshot.data!;
        if (home.sections.isEmpty) {
          return const Center(child: Text('No movies'));
        }

        return _buildHomeContent(home);
      },
    );
  }

  Widget _buildHomeContent(HomeResponse home) {
    final sections = home.sections;
    final MovieCard? heroMovie =
        sections.isNotEmpty && sections.first.items.isNotEmpty
            ? sections.first.items.first
            : null;

    return CustomScrollView(
      physics: const BouncingScrollPhysics(),
      slivers: [
        if (heroMovie != null)
          SliverToBoxAdapter(
            child: Padding(
              padding: const EdgeInsets.symmetric(horizontal: 8),
              child: HeroBanner(
                movie: heroMovie,
                onOpenDetails: () {
                  if (heroMovie.mediaType == 'tv') {
                    Navigator.push(
                      context,
                      MaterialPageRoute(
                        builder: (_) =>
                            SeriesDetailsScreen(tmdbId: heroMovie.tmdbId),
                      ),
                    );
                  } else {
                    Navigator.push(
                      context,
                      MaterialPageRoute(
                        builder: (_) =>
                            MovieDetailsScreen(tmdbId: heroMovie.tmdbId),
                      ),
                    );
                  }
                },
              ),
            ),
          ),
        const SliverToBoxAdapter(child: SizedBox(height: 16)),
        for (final section in sections)
          SliverToBoxAdapter(
            child: SectionRow(
              section: section,
              onTapMovie: (movie) {
                if (movie.mediaType == 'tv') {
                  Navigator.push(
                    context,
                    MaterialPageRoute(
                      builder: (_) =>
                          SeriesDetailsScreen(tmdbId: movie.tmdbId),
                    ),
                  );
                } else {
                  Navigator.push(
                    context,
                    MaterialPageRoute(
                      builder: (_) =>
                          MovieDetailsScreen(tmdbId: movie.tmdbId),
                    ),
                  );
                }
              },
            ),
          ),
        const SliverToBoxAdapter(child: SizedBox(height: 24)),
      ],
    );
  }
}

import 'package:flutter/material.dart';

ThemeData buildNebulaTheme() {
  final base = ThemeData.dark(useMaterial3: true);

  return base.copyWith(
    scaffoldBackgroundColor: const Color(0xFF020617),
    colorScheme: base.colorScheme.copyWith(
      primary: const Color(0xFF7DD3FC),
      secondary: const Color(0xFFA855F7),
      surface: const Color(0xFF020617),
    ),
    appBarTheme: const AppBarTheme(
      backgroundColor: Colors.transparent,
      elevation: 0,
      centerTitle: false,
      titleTextStyle: TextStyle(
        fontSize: 22,
        fontWeight: FontWeight.w700,
        letterSpacing: 1.2,
        color: Colors.white,
      ),
    ),
    textTheme: base.textTheme.apply(
      bodyColor: Colors.white,
      displayColor: Colors.white,
    ),
  );
}

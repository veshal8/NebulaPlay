import 'package:flutter/material.dart';

import 'theme/nebula_theme.dart';
import 'screens/home_screen.dart';

void main() {
  runApp(const NebulaPlayApp());
}

class NebulaPlayApp extends StatelessWidget {
  const NebulaPlayApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'NebulaPlay',
      debugShowCheckedModeBanner: false,
      theme: buildNebulaTheme(),
      home: const HomeScreen(),
    );
  }
}

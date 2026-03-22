import 'dart:ui';
import 'package:flutter/material.dart';

Widget glass({
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
            border: Border.all(
              color: Colors.white.withOpacity(0.08),
              width: 1.1,
            ),
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

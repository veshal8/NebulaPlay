import 'package:flutter/material.dart';

class NavPill extends StatelessWidget {
  final String label;
  final bool selected;
  final VoidCallback onTap;

  const NavPill({
    super.key,
    required this.label,
    required this.selected,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    final bgGradient = selected
        ? const LinearGradient(
            colors: [
              Color(0xFF38BDF8),
              Color(0xFFA855F7),
            ],
          )
        : null;

    return GestureDetector(
      onTap: onTap,
      child: AnimatedContainer(
        duration: const Duration(milliseconds: 160),
        padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
        decoration: BoxDecoration(
          borderRadius: BorderRadius.circular(999),
          gradient: bgGradient,
          color: bgGradient == null
              ? Colors.white.withOpacity(0.0)
              : null,
        ),
        child: Text(
          label,
          style: TextStyle(
            fontSize: 12,
            fontWeight: selected ? FontWeight.w700 : FontWeight.w500,
            color: selected ? Colors.black : Colors.white70,
            letterSpacing: 0.4,
          ),
        ),
      ),
    );
  }
}

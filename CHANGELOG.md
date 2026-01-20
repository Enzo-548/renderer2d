# Changelog

## [0.1.0] - Initial Public Release

- CPU-based 2D software renderer implemented from scratch in Rust
- Custom framebuffer with explicit pixel addressing and clipping
- Explicit rasterization of basic primitives (lines, rectangles, circles, polygons)
- Layered rendering model with base and overlay framebuffers
- Shape system with explicit transforms and world-space materialization
- Interactive playground using minifb as a window backend

Notes:
- This release focuses on architectural clarity and explicit data flow.
- Performance optimizations and API stability are explicitly non-goals at this stage.
# Architecture.md
**renderer2d — CPU-based 2D Software Renderer (Rust)**

## 1. Project Goal

This project is a **CPU-based 2D software renderer**, fully implemented in Rust.

It is **not** intended to be fast, complete, or comparable to modern GPU APIs.  
Its goals are instead to:

- Make the **rendering pipeline explicit**
- Allow **direct experimentation** with rasterization algorithms
- Maintain **low coupling** between data, transformations, and drawing
- Serve as an **educational and exploratory** codebase

## 2. Architectural Overview

The architecture is structured around four core concepts:

```
Input / Playground (main.rs)
        ↓
     Shape (data + transforms)
        ↓
  world_shape() (materialization)
        ↓
     Render (orchestration)
        ↓
  Framebuffer (raw pixel state)
```

Each layer has **strict responsibilities** and does not leak concerns into others.

---

## 3. Framebuffer

### Responsibility

`Framebuffer` is the **lowest-level structure** in the system. It:

- Stores pixels (`Vec<Color>`)
- Knows its width and height
- Enforces **structural clipping**
- Contains no geometric or algorithmic logic

### Invariants

- No out-of-bounds writes are possible
- All writes go through `put_pixel` or `return_pixel`
- The framebuffer **never decides what to draw**, only how to store it

This guarantees that **all clipping is centralized**, regardless of the rasterization algorithm.

---

## 4. Render

### Responsibility

`Render` is the **drawing orchestrator**. It:

- Owns multiple `Framebuffer`s as layers
- Decides **how** each `ShapeKind` is rasterized
- Implements algorithms for:
  - Lines (Bresenham)
  - Circles
  - Rectangles
  - Polygons
  - Geometric fills
  - Bucket fill

### Layers

The renderer maintains a vector of layers:

- Base layer: persistent drawing
- Overlay layer: temporary preview / feedback

The overlay layer is cleared every frame, while the base layer accumulates state.

### Invariants

- `Render` **never mutates Shapes**
- `Render` receives Shapes already in world space
- `Render` does not know about input, UI, or user state
- All drawing goes through the `Framebuffer`

The concentration of responsibilities in `Render` is **intentional**: it represents the physical center of the renderer.

---

## 5. Shape and ShapeKind

### ShapeKind

`ShapeKind` is a **pure geometric data type**, containing only:

- Coordinates
- Dimensions
- Vertices (for polygons)

It contains:
- No rasterization logic
- No framebuffer access
- No renderer dependency

### Shape

`Shape` is a **higher-level entity** composed of:

- A `ShapeKind`
- Accumulated transforms
- Outline and fill colors

### Invariants

- Shapes are **never rasterized directly**
- Local Shapes represent the original model
- All transforms are accumulated separately

---

## 6. Transforms

### Model

Each `Shape` owns a set of accumulated transforms:

```
translate: (f32, f32)
scale:     (f32, f32)
angle:     f32
```

These transforms:

- Do not modify the original `ShapeKind`
- Can be applied incrementally
- Allow granular undo if external history is stored

### Application

Transforms are applied **only during materialization**.

---

## 7. world_shape(): Explicit Materialization

`world_shape()` is a central pipeline boundary.

It:
1. Clones the original `ShapeKind`
2. Applies scale
3. Applies rotation
4. Applies translation
5. Returns a **new Shape** ready for rasterization

### Critical Invariants

- The original Shape is **never mutated**
- The returned Shape is disposable
- Rasterization **only happens in world space**

This ensures a clean, explicit pipeline and prevents conceptual ordering errors.

---

## 8. Rasterization Flow

Rasterization follows this path:

```
Shape.rasterize()
    → world_shape()
        → Render.draw_shape()
            → algorithm
                → Framebuffer.put_pixel()
```

### Properties

- No direct buffer access outside the framebuffer
- All algorithms inherit clipping automatically
- Thickness is treated as a discrete stroke expansion

---

## 9. main.rs as a Playground

`main.rs` is **not** a production application layer.

It functions as:

- A visual testing environment
- A debug playground
- An experimental input system

Intentional characteristics:
- Dense code
- Temporary artifacts
- Ad-hoc shapes
- Easy insertion and removal of experiments

### Invariants

- `main` never performs rasterization
- `main` never writes directly to a `Framebuffer`
- All drawing goes through `Render`

This keeps the renderer decoupled and reusable.

---

## 10. Conscious Decisions and Non-Goals

### Conscious Decisions

- Monolithic renderer (no ECS)
- Shapes as data, not systems
- Destructive transforms only on cloned data
- Pipeline expressed in code, not heavy abstractions

### Non-Goals

- Extreme performance
- Stable public API

---

## 11. Current State and Evolution

At its current stage:

- The architectural model is defined
- Experimentation is prioritized
- Refactors are cheap
- Decisions are progressively documented

This document exists to **state invariants**, not to freeze the design.
# Architecture.md

**renderer2d — CPU-based 2D Software Renderer (Rust)**

## 1. Project Goal

This project is a **CPU-based 2D software renderer**, fully implemented in Rust.

It is **not** intended to be fast, complete, or comparable to modern GPU APIs.
Its goals are instead to:

* Make the **rendering pipeline explicit**
* Allow **direct experimentation** with rasterization algorithms
* Maintain **low coupling** between data, transformations, and drawing
* Serve as an **educational and exploratory** codebase

---

## 2. Architectural Overview

The architecture is structured around a clear, linear pipeline:

```
Input / Playground (main.rs)
        ↓
     Shape (data + transforms)
        ↓
  world_shape() (materialization)
        ↓
     Render (orchestration)
        ↓
  Framebuffer (final pixel state)
```

Each stage has **explicit responsibilities** and communicates only through well-defined data.

---

## 3. Framebuffer

### Responsibility

`Framebuffer` is the **lowest-level structure** in the system. It:

* Stores pixels (`Vec<Color>`)
* Knows its width and height
* Enforces **structural clipping**
* Retains only the **final pixel state**
* Contains no geometric or algorithmic logic

### Invariants

* No out-of-bounds writes are possible
* All pixel writes go through framebuffer methods
* Clipping is enforced exclusively at the framebuffer level
* The framebuffer **never decides what to draw**, only how pixels are stored

This guarantees that **all rasterizers inherit clipping automatically**, without duplicating bounds logic.

---

## 4. Render

### Responsibility

`Render` is the **central drawing authority**. It:

* Owns and manages one or more `Framebuffer`s
* Decides **how** each `ShapeKind` is rasterized
* Implements all rasterization algorithms

### Layers

The renderer may operate with multiple framebuffers:

* Base layer: retains the final drawing
* Overlay layer: temporary preview and debug visualization

The overlay layer is cleared every frame. The base layer retains the accumulated pixel state.

### Invariants

* `Render` never mutates `Shape` or `ShapeKind`
* `Render` receives shapes already materialized in world space
* `Render` has no knowledge of input, UI, or user intent
* All pixel mutation occurs through the framebuffer API

All abstractions that produce pixels (brush, debug tools, previews) are **logical sub-modes of Render**, not independent writers.

---

## 5. Shape and ShapeKind

### ShapeKind

`ShapeKind` is a **pure geometric data structure**, containing only:

* Coordinates
* Dimensions
* Vertices (for polygons)

It contains:

* No rasterization logic
* No framebuffer access
* No renderer dependency

### Shape

`Shape` is a higher-level construct composed of:

* A `ShapeKind`
* Accumulated transforms
* Stroke and fill color data

### Invariants

* Shapes are **never rasterized directly**
* Shapes represent intent, not pixels
* No persistent scene or command list is retained

---

## 6. Transforms

### Model

Each `Shape` owns a set of accumulated transforms:

```
translate: (f32, f32)
scale:     (f32, f32)
angle:     f32
```

### Properties

* Transforms do not mutate the original `ShapeKind`
* Transforms may be applied incrementally
* No transform history is stored internally

### Application

Transforms are applied **only during materialization**, never during rasterization.

---

## 7. world_shape(): Explicit Materialization

`world_shape()` defines a strict pipeline boundary.

It:

1. Clones the original `ShapeKind`
2. Applies scale
3. Applies rotation
4. Applies translation
5. Produces a disposable, world-space shape

### Invariants

* The original shape data is never mutated
* The returned shape has no identity or history
* Rasterization occurs **only in world space**

---

## 8. Rasterization Flow

The rasterization path is:

```
Shape
  → world_shape()
      → Render.draw_*()
          → raster algorithm
              → Framebuffer.put_pixel()
```

### Properties

* No direct access to the pixel buffer outside the framebuffer
* All clipping is centralized
* Stroke thickness is handled as discrete pixel expansion

---

## 9. main.rs as a Playground

`main.rs` is intentionally **not** a production application layer.

It exists as:

* A visual test harness
* A debugging surface
* An experimentation space

Characteristics:

* Dense, mutable code
* Temporary experiments
* Explicit non-goals around cleanliness

### Invariants

* `main` never rasterizes shapes
* `main` never writes directly to a framebuffer
* All drawing goes through `Render`

---

## 10. Retention Model and Ordering

* The renderer retains **only the final framebuffer state**
* No shape, command, or history retention exists
* There is no scene graph
* There is no undo/redo system

### Drawing Order

* No Z-axis or depth model exists
* Drawing order is defined solely by execution order (FIFO)
* Later draws overwrite earlier pixel data

---

## 11. Conscious Decisions and Non-Goals

### Conscious Decisions

* Monolithic renderer (no ECS)
* Shapes as data, not behavior
* Centralized clipping
* Explicit materialization step
* FIFO drawing model

### Non-Goals

* High performance
* Stable public API
* GPU abstraction
* Persistent scene representation

---

## 12. Evolution

This architecture is intentionally flexible.

* The model is defined
* Constraints are explicit
* Refactors are expected

This document exists to **state contracts and invariants**, not to freeze the design.

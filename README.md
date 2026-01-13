# Minimal 2D Software Renderer (Rust)

This project is a **CPU-based 2D software renderer written in Rust**.
It is designed to make every step of the rendering pipeline explicit — from input handling, to shape rasterization, to layered framebuffer composition.

The focus is not performance or GPU acceleration, but **architectural clarity** and a concrete understanding of how each pixel ends up on the screen.

It implements a custom framebuffer, explicit rendering layers, basic drawing operations, keyboard and mouse-driven interaction, and uses **minifb** purely as a lightweight window backend.

🚧 Work in progress — issues and feedback welcome

---

## ✨ Features

* **Explicit layered framebuffer model** (overlay vs committed content)
* Custom **RGBA framebuffer** stored entirely in CPU memory
* CPU-based rasterization of basic primitives:

  * lines
  * rectangles
  * triangles
  * circles
* Geometry-driven filling for shapes (not dependent on global flood fill)
* Brush **preview / overlay** separated from committed drawing
* Keyboard and mouse input mapped to rendering state
* Saving the current framebuffer state to an image
* `minifb` backend for window creation and pixel presentation
* Clear separation between:

  * rendering logic
  * framebuffer data
  * window and input handling

---

## 🧱 Project Structure

```
src/
├── main.rs              # Application entry point and minifb integration
└── renderer/
    ├── mod.rs           # Renderer module definition
    ├── color.rs         # RGBA color representation and constants
    ├── framebuffer.rs   # Framebuffer data structure and buffer conversion
    └── render.rs        # Renderer logic (draw operations)
```

### Module Responsibilities

* **`Color`**
  Simple RGBA color type with predefined constants (RED, GREEN, BLUE, ALPHA).

* **`Framebuffer`**
  Stores pixel data and dimensions. Provides conversion to a `Vec<u32>` suitable for `minifb`, and a `Vec<u8>` suitable for image export.

* **`Render`**
  Owns the background color, the main framebuffer, and an explicit array of layers. Provides drawing operations such as `clear`, `put_pixel`, `return_pixel`, and `draw_*` primitives.

* **`main.rs`**
  Orchestrates the application loop, handles keyboard and mouse input, and presents the composed framebuffer using `minifb`.

---

## 🧩 Layered Rendering Model

The renderer uses **explicit framebuffer layers**:

* One or more layers for **preview / overlay drawing**
* One layer for **committed framebuffer content**

This separation allows:

* clean distinction between previewed strokes and finalized pixels
* deterministic rendering behavior
* no reliance on hidden state or implicit clearing

Layers are composed explicitly during the render pass.

---

## 🎮 Controls

The numeric keypad controls screen color and drawing parameters:

| Key          | Action                                               |
| ------------ | ---------------------------------------------------- |
| NumPad 0     | White screen                                         |
| NumPad 1     | Black screen                                         |
| NumPad 2     | Blue screen                                          |
| NumPad 3     | Green screen                                         |
| NumPad 4     | Red screen                                           |
| NumPad Plus  | Increase thickness                                   |
| NumPad Minus | Decrease thickness                                   |
| S            | Draw a square                                        |
| T            | Draw a triangle                                      |
| C            | Draw a circle                                        |
| F            | Bucket fill at the center of the screen              |
| Arrow Up     | Draw a vertical line from the top-middle             |
| Arrow Left   | Draw a horizontal line from the middle-left          |
| Arrow Down   | Draw a diagonal line from the top-right              |
| Arrow Right  | Draw a diagonal line from the top-left               |
| Mouse Left   | Square brush (preview on overlay, commit on release) |
| NumPad Enter | Save the current buffer to `assets/output`           |
| ESC          | Exit the program                                     |

---

## ▶️ Running the Project

```bash
cargo run
```

Make sure you have Rust installed and a platform supported by `minifb`.

---

## 🧠 Design Notes

* The scope of this project is deliberately constrained to prioritize **architectural clarity** and **explicit data flow** over feature completeness or performance optimizations.
* Rendering logic is **backend-agnostic**.
* `minifb` is used only for prototyping and visualization.
* All drawing happens on the CPU via the framebuffer.
* Algorithms are introduced only when they become architecturally necessary.
* The architecture is intentionally simple to make the rendering pipeline transparent.
* Thickness is implemented as a rasterization-time pixel offset applied to shape outlines, not as a geometric transform. This is a visual-only parameter and may cause distortions at higher values.

```
Input → Render State → Shapes → Rasterization → Layers → Framebuffer → Window
```

---

## 🚧 Current Limitations

* No coordinate transforms
* No user-defined custom primitives
* CPU-only rendering by design
* Trade-offs favor simplicity and readability over raw performance

These limitations are intentional at this stage.

---

## 🛣️ Possible Next Steps (not commitments)

* Introduce basic coordinate transforms (translation, scaling)
* Formalize shapes as data structures with explicit bounds
* Optional exploration of a GPU backend (`wgpu`) without changing the core architecture

---

## 📌 Motivation

This project was built as a **learning exercise** for:

* applying Rust ownership, borrowing, and lifetimes in a renderer-style codebase
* understanding low-level rendering concepts
* practicing modular design and explicit architectural trade-offs

---

## 📜 License

This project is provided for educational purposes. Use it freely to learn and experiment.

---

## 🖼️ Screenshots

![Lines Demo](assets/screenshots/colored-lines-with-all-colors.png)
![Triangle Demo](assets/screenshots/empty-triangle.png)
![Circle Demo](assets/screenshots/green-circle-with-blue-outline.png)
![Square Demo](assets/screenshots/red-square-with-black-outline.png)
![Square Brush Demo](assets/screenshots/glad-and-stilish-girl.png)
![Drawing Demo](assets/img_artifacts/worst_enemy_of_a_red_crab.png)

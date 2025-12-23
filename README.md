# Minimal 2D Software Renderer (Rust)

This project is a **minimal 2D software renderer written in Rust**, built for learning and experimentation with computer graphics fundamentals.

It implements a custom framebuffer, basic drawing operations, keyboard-driven interaction, and uses **minifb** as a lightweight window backend.


---

## ✨ Features

* Custom **RGBA framebuffer** stored in CPU memory
* Minimal `Render` layer responsible for drawing operations
* Basic drawing primitive: **screen fill**
* Keyboard input mapped to rendering state (color changes)
* `minifb` backend for window creation and presenting pixels
* Clean separation between:

  * rendering logic
  * framebuffer data
  * window/input handling

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
  Simple RGBA color type with predefined constants (RED, GREEN, BLUE, etc.).

* **`Framebuffer`**
  Stores pixel data and dimensions. Provides conversion to a `Vec<u32>` suitable for `minifb`.

* **`Render`**
  Owns a framebuffer and provides drawing operations such as `clear` and `put_pixel`.

* **`main.rs`**
  Orchestrates the application loop, handles keyboard input, and presents the framebuffer using `minifb`.

---

## 🎮 Controls

The numeric keypad controls the screen color:

| Key      | Action       |
| -------- | ------------ |
| NumPad 0 | White screen |
| NumPad 1 | Black screen |
| NumPad 2 | Blue screen  |
| NumPad 3 | Green screen |
| NumPad 4 | Red screen   |
| ESC      | Exit program |

---

## ▶️ Running the Project

```bash
cargo run
```

Make sure you have Rust installed and a platform supported by `minifb`.

---

## 🧠 Design Notes
* The scope of this project is deliberately constrained to prioritize architectural clarity
and explicit data flow over feature completeness or performance optimizations.
* Rendering logic is **backend-agnostic**.
* `minifb` is used only for prototyping and visualization.
* All drawing happens on the CPU via the framebuffer.
* The architecture is intentionally simple to make the data flow explicit:

```
Input → Render → Framebuffer → Window
```

---

## 🚧 Current Limitations

* Only one drawing primitive (`clear`)
* No geometric primitives yet (lines, rectangles, etc.)
* No coordinate transforms
* CPU-only rendering, as it is used intentionally to keep the pipeline explicit
* Trade-offs favor simplicity and readability over raw performance


These limitations are intentional at this stage.

---

## 🛣️ Possible Next Steps

* Implement drawing primitives (line, rectangle)
* Add simple coordinate transforms (translation, scaling)
* Mouse input handling
* Explore a GPU-based backend using `winit` + `wgpu`

---

## 📌 Motivation

This project was built as a **learning exercise** for:
* applying Rust ownership, borrowing, and lifetimes in a renderer-style codebase
* understanding low-level rendering concepts
* practicing modular design and architectural trade-offs

---

## 📜 License

This project is provided for educational purposes. Use it freely to learn and experiment.

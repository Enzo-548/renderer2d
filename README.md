# Minimal 2D Software Renderer (Rust)

This project is a **minimal 2D software renderer written in Rust**, built for learning and experimentation with computer graphics fundamentals.

It implements a custom framebuffer, basic drawing operations, keyboard-driven interaction, and uses **minifb** as a lightweight window backend.

🚧 Work in progress — issues and feedback welcome
---

## ✨ Features

- Custom **RGBA framebuffer** stored in CPU memory  
- Minimal `Render` layer responsible for drawing operations  
- Basic drawing primitives: **screen clear, bucket fill, lines in eight directions, triangle, circle, rectangle**  
- Keyboard input mapped to rendering state (color changes / brush changes)  
- Brush **preview / overlay** on the canvas  
- Saving the current state of the framebuffer to an image  
- `minifb` backend for window creation and pixel presentation  
- Clean separation between:
  - rendering logic  
  - framebuffer data  
  - window and input handling  

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
  Stores pixel data and dimensions. Provides conversion to a `Vec<u32>` suitable for `minifb`, and `Vec<u8>` suitable for the image library.

* **`Render`**
  Owns some structs as a main framebuffer, a definition of a background color and a array of layers, also provides drawing operations such as `clear`, `put_pixel`, `return_pixel` and `draw_*`(primitive).

* **`main.rs`**
  Orchestrates the application loop, handles keyboard input, and presents the framebuffer using `minifb`.

---

## 🎮 Controls

The numeric keypad controls screen color and drawing parameters:

| Key            | Action |
|---------------|--------|
| NumPad 0      | White screen |
| NumPad 1      | Black screen |
| NumPad 2      | Blue screen |
| NumPad 3      | Green screen |
| NumPad 4      | Red screen |
| NumPad Plus   | Increase thickness |
| NumPad Minus  | Decrease thickness |
| S             | Draw a square |
| T             | Draw a triangle |
| C             | Draw a circle |
| F             | Bucket fill at the center of the screen |
| Arrow Up      | Draw a vertical line from the top-middle |
| Arrow Left    | Draw a horizontal line from the middle-left |
| Arrow Down    | Draw a diagonal line from the top-right |
| Arrow Right   | Draw a diagonal line from the top-left |
| Mouse Left    | Acts as a square brush; drag to draw on the canvas |
| NumPad Enter  | Save the current buffer to `assets/output` |
| ESC           | Exit the program |

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
* The project will implement certain algorithms as the nescessity is needed.
* The architecture is intentionally simple to make the data flow explicit.
* Thickness is implemented as a rasterization-time pixel offset applied to shape outlines, not as a geometric transform. It is a visual-only parameter and may cause distortions in some shapes at higher values.

```
Input → Render → Framebuffer → Window
```

---

## 🚧 Current Limitations

* No coordinate transforms
* No custom user primitives or draws guided by the user
* CPU-only rendering, as it is used intentionally to keep the pipeline explicit
* Trade-offs favor simplicity and readability over raw performance


These limitations are intentional at this stage.

---

## 🛣️ Possible Next Steps

* Add simple coordinate transforms (translation, scaling)
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

## SCREENSHOTS
 ![Lines Demo](assets/screenshots/colored-lines-with-all-colors.png)
 ![Triangle Demo](assets/screenshots/empty-triangle.png)
 ![Circle Demo](assets/screenshots/green-circle-with-blue-outline.png)
 ![Square Demo](assets/screenshots/red-square-with-black-outline.png)
 ![Square Brush Demo](assets/screenshots/glad-and-stilish-girl.png)
 ![Drawing Demo1](assets/img_artifacts/worst_enemy_of_a_red_crab.png)

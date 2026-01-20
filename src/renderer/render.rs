use crate::renderer::{
    color::Color,
    framebuffer::{*},
    shape::{*}
};

/// Render is the orchestration layer of the CPU renderer.
///
/// It owns one or more framebuffers ("layers") and is responsible for:
/// - Dispatching rasterization based on ShapeKind
/// - Coordinating fill vs outline passes
/// - Managing layering / overlay semantics
///
/// This struct intentionally keeps logic explicit and imperative,
/// favoring clarity over abstraction or performance.
#[derive(Debug)]
pub struct Render {
    /* 
     * Layer Vector:
     * Stores multiple framebuffers used as drawing layers.
     *
     * Architectural note:
     * - Index 0 is currently reserved as an overlay layer
     * - Future use includes true layer compositing or multiple render targets
     * - This is a deliberate architectural pivot to support overlays and UI
     */
    pub layers: Vec<Framebuffer>,
}

impl Render {

    /// Creates a new Render instance.
    ///
    /// The provided framebuffer is treated as the main drawing surface.
    /// An additional overlay layer is automatically created with the same dimensions.
    pub fn new(framebuffer: Framebuffer) -> Render {
        let (width, height) = (framebuffer.width, framebuffer.height);

        Self {
            // Convention:
            // layers[0] -> overlay
            // layers[1] -> main framebuffer
            layers: vec![
                Render::new_layer(width, height),
                framebuffer,
            ],
        }
    }

    /// Creates a new empty framebuffer layer.
    ///
    /// This function exists mainly to centralize layer creation
    /// and keep Render::new readable.
    pub fn new_layer(width: u32, height: u32) -> Framebuffer {
        Framebuffer::new(width, height)
    }

    /// Returns a mutable reference to the overlay layer, if it exists.
    ///
    /// This is currently a convenience accessor and may evolve
    /// into a more explicit layer management API.
    pub fn overlay_mut(&mut self) -> Option<&mut Framebuffer> {
        self.layers.get_mut(0)
    }

    /// Dispatches rasterization based on ShapeKind.
    ///
    /// This is the central entry point for drawing shapes.
    /// Responsibilities:
    /// - Decide whether the shape needs fill, outline, or both
    /// - Convert floating-point shape data into raster operations
    /// - Delegate to primitive-specific drawing routines
    pub fn draw_shape(&mut self, shape: &Shape, layer: usize, thickness: i32) {
        match &shape.kind {

            rect @ ShapeKind::Rect { center: origin, half_w: w, half_h: h } => {
                // Fill pass (if applicable)
                self.geo_fill(rect, layer, shape.inline_color);

                // Outline pass
                self.draw_rectangle(
                    layer,
                    origin.0 as i32,
                    origin.1 as i32,
                    *w as i32,
                    *h as i32,
                    thickness,
                    shape.outline_color,
                );
            }

            circle @ ShapeKind::Circle { center, r } => {
                self.geo_fill(circle, layer, shape.inline_color);

                self.draw_circle(
                    layer,
                    center.0 as i32,
                    center.1 as i32,
                    *r as i32,
                    thickness,
                    shape.outline_color,
                );
            }

            ShapeKind::Ellipse { center, rx, ry } => {
                // Ellipse support planned but not implemented yet
                todo!()
            }

            ShapeKind::Line { a, b, .. } => {
                // Lines currently ignore outline vs fill distinction
                self.draw_line(
                    layer,
                    a.0 as i32,
                    a.1 as i32,
                    b.0 as i32,
                    b.1 as i32,
                    thickness,
                    shape.inline_color,
                );
            }

            polygon @ ShapeKind::Polygon { vertices } => {
                self.geo_fill(polygon, layer, shape.inline_color);

                // Currently assumes triangles (3 vertices)
                // This is a known limitation, not an oversight
                let (x1, y1) = *vertices.get(0).unwrap();
                let (x2, y2) = *vertices.get(1).unwrap();
                let (x3, y3) = *vertices.get(2).unwrap();

                self.draw_triangle(
                    layer,
                    x1 as i32, y1 as i32,
                    x2 as i32, y2 as i32,
                    x3 as i32, y3 as i32,
                    thickness,
                    shape.outline_color,
                );
            }
        }
    }

    /// Utility function used by interactive / dynamic drawing modes.
    ///
    /// `draw_sel` selects the primitive type.
    /// This is intentionally simple and not meant as a final API.
    pub fn draw_dynam(
        &mut self,
        layer: usize,
        draw_sel: u32,
        ref_x: u32,
        ref_y: u32,
        thickness: i32,
        color: Color,
        destination: (u32, u32),    
    ) {
        match draw_sel {
            0 => {
                Shape::new_defined_monochrome(
                    ShapeKind::Rect {
                        center: (ref_x as f32, ref_y as f32),
                        half_w: 10.0,
                        half_h: 10.0,
                    },
                    color,
                )
                .rasterize(self, layer, thickness);
            }

            1 => {
                Shape::new_defined_monochrome(
                    ShapeKind::Polygon {
                        vertices: vec![
                            (ref_x as f32, ref_y as f32 - 25.0),
                            (ref_x as f32 - 25.0, ref_y as f32 + 25.0),
                            (ref_x as f32 + 25.0, ref_y as f32 + 25.0),
                        ],
                    },
                    color,
                )
                .rasterize(self, layer, thickness);
            }

            2 => {
                Shape::new_defined_monochrome(
                    ShapeKind::Circle {
                        center: (ref_x as f32, ref_y as f32),
                        r: 10.0,
                    },
                    color,
                )
                .rasterize(self, layer, thickness);
            }

            3 => {
                // Flood fill / bucket tool
                self.bucket_fill(layer, ref_x, ref_y, color);
            }

            4 => todo!(),
            _ => (),
        }
    }

    /// Draws a thick line using an integer-based incremental algorithm.
    ///
    /// This is a Bresenham-style implementation with a thickness extension
    /// applied orthogonally to the dominant axis.
    pub fn draw_line(
        &mut self,
        layer: usize,
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
        thickness: i32,
        color: Color,
    ) {
        let fb = &mut self.layers[layer];

        let mut x = x0;
        let mut y = y0;

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();

        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        let mut err = dx - dy;

        loop {
            // Thickness is applied perpendicular to the line direction
            if dx >= dy {
                for o in -thickness..=thickness {
                    fb.put_pixel(x as u32, (y + o) as u32, color);
                }
            } else {
                for o in -thickness..=thickness {
                    fb.put_pixel((x + o) as u32, y as u32, color);
                }
            }

            if x == x1 && y == y1 {
                break;
            }

            let e2 = 2 * err;

            if e2 > -dy {
                err -= dy;
                x += sx;
            }

            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }

    /// Draws a rectangle outline centered at a reference point.
    ///
    /// Uses four thick line calls.
    pub fn draw_rectangle(
        &mut self,
        layer: usize,
        x_ref_point: i32,
        y_ref_point: i32,
        width: i32,
        height: i32,
        thickness: i32,
        color: Color,
    ) {
        let x_r = x_ref_point + width;
        let x_l = x_ref_point - width;
        let y_u = y_ref_point + height;
        let y_d = y_ref_point - height;

        self.draw_line(layer, x_r, y_d - thickness, x_r, y_u + thickness, thickness, color);
        self.draw_line(layer, x_l, y_d - thickness, x_l, y_u + thickness, thickness, color);
        self.draw_line(layer, x_l - thickness, y_d, x_r + thickness, y_d, thickness, color);
        self.draw_line(layer, x_l - thickness, y_u, x_r + thickness, y_u, thickness, color);
    }

    /// Draws a triangle outline and caps vertices with circles.
    ///
    /// This avoids visible gaps at joins when thickness > 1.
    pub fn draw_triangle(
        &mut self,
        layer: usize,
        x1: i32, y1: i32,
        x2: i32, y2: i32,
        x3: i32, y3: i32,
        thickness: i32,
        color: Color,
    ) {
        let r = thickness;

        self.draw_line(layer, x1, y1, x2, y2, thickness, color);
        self.draw_line(layer, x1, y1, x3, y3, thickness, color);
        self.draw_line(layer, x2, y2, x3, y3, thickness, color);

        // Vertex caps
        self.draw_shape(
            &Shape::new_defined_monochrome(
                ShapeKind::Circle { center: (x1 as f32, y1 as f32), r: r as f32 },
                color,
            ),
            layer,
            0,
        );

        self.draw_shape(
            &Shape::new_defined_monochrome(
                ShapeKind::Circle { center: (x2 as f32, y2 as f32), r: r as f32 },
                color,
            ),
            layer,
            0,
        );

        self.draw_shape(
            &Shape::new_defined_monochrome(
                ShapeKind::Circle { center: (x3 as f32, y3 as f32), r: r as f32 },
                color,
            ),
            layer,
            0,
        );
    }

    /// Draws a circle, supporting both filled and ring-style outlines.
    ///
    /// - thickness <= 0 → filled circle (midpoint algorithm)
    /// - thickness > 0  → ring defined by inner and outer radius
    pub fn draw_circle(
        &mut self,
        layer: usize,
        cx: i32,
        cy: i32,
        r: i32,
        thickness: i32,
        color: Color,
    ) {
        let fb = &mut self.layers[layer];

        // Solid circle case
        if thickness <= 0 {
            let mut x = r;
            let mut y = 0;
            let mut d = 1 - r;

            while x >= y {
                fb.put_pixel((cx + x) as u32, (cy + y) as u32, color);
                fb.put_pixel((cx + y) as u32, (cy + x) as u32, color);
                fb.put_pixel((cx - x) as u32, (cy + y) as u32, color);
                fb.put_pixel((cx - y) as u32, (cy + x) as u32, color);
                fb.put_pixel((cx - x) as u32, (cy - y) as u32, color);
                fb.put_pixel((cx - y) as u32, (cy - x) as u32, color);
                fb.put_pixel((cx + x) as u32, (cy - y) as u32, color);
                fb.put_pixel((cx + y) as u32, (cy - x) as u32, color);

                y += 1;

                if d < 0 {
                    d += 2 * y + 1;
                } else {
                    x -= 1;
                    d += 2 * (y - x) + 1;
                }
            }
            return;
        }

        // Ring case
        let r_outer = r + thickness;
        let r_inner = (r - thickness).max(0);

        let ro2 = r_outer * r_outer;
        let ri2 = r_inner * r_inner;

        for y in (cy - r_outer)..=(cy + r_outer) {
            for x in (cx - r_outer)..=(cx + r_outer) {
                let dx = x - cx;
                let dy = y - cy;
                let d2 = dx * dx + dy * dy;

                if d2 <= ro2 && d2 >= ri2 {
                    fb.put_pixel(x as u32, y as u32, color);
                }
            }
        }
    }

    /// Geometric fill routines for supported shapes.
    ///
    /// This function is intentionally shape-specific and not generalized.
    /// Each fill algorithm is explicit and easy to inspect.
    fn geo_fill(&mut self, shape: &ShapeKind, layer: usize, color: Color) {
        let fb = &mut self.layers[layer];

        match shape {
            ShapeKind::Rect { center, half_w, half_h } => {
                let y_min = (center.1 - *half_h) as i32;
                let y_max = (center.1 + *half_h) as i32;
                let x_min = (center.0 - *half_w) as i32;
                let x_max = (center.0 + *half_w) as i32;

                for y in y_min.min(y_max)..=y_min.max(y_max) {
                    if y < 0 || y >= fb.height as i32 { continue; }
                    for x in x_min.min(x_max)..=x_min.max(x_max) {
                        if x < 0 || x >= fb.width as i32 { continue; }
                        fb.put_pixel(x as u32, y as u32, color);
                    }
                }
            }

            ShapeKind::Polygon { vertices } => {
                // Triangle-only scanline fill
                let mut v: [(i32, i32); 3] = [
                    (vertices[0].0 as i32, vertices[0].1 as i32),
                    (vertices[1].0 as i32, vertices[1].1 as i32),
                    (vertices[2].0 as i32, vertices[2].1 as i32),
                ];

                v.sort_by_key(|(_, y)| *y);

                let (x0, y0) = v[0];
                let (x1, y1) = v[1];
                let (x2, y2) = v[2];

                let interp = |x0, y0, x1, y1, y| {
                    if y1 == y0 { x0 }
                    else { x0 + (x1 - x0) * (y - y0) / (y1 - y0) }
                };

                for y in y0..=y2 {
                    let (xa, xb) = if y < y1 {
                        (
                            interp(x0, y0, x2, y2, y),
                            interp(x0, y0, x1, y1, y),
                        )
                    } else {
                        (
                            interp(x0, y0, x2, y2, y),
                            interp(x1, y1, x2, y2, y),
                        )
                    };

                    let (xmin, xmax) = if xa < xb { (xa, xb) } else { (xb, xa) };
                    for x in xmin..=xmax {
                        fb.put_pixel(x as u32, y as u32, color);
                    }
                }
            }

            ShapeKind::Circle { r, center } => {
                let r = *r as i32;
                for dy in -r..=r {
                    let y = center.1 as i32 + dy;
                    let dx = ((r * r - dy * dy) as f32).sqrt() as i32;
                    for x in (center.0 as i32 - dx)..=(center.0 as i32 + dx) {
                        fb.put_pixel(x as u32, y as u32, color);
                    }
                }
            }

            ShapeKind::Ellipse { .. } => todo!(),
            ShapeKind::Line { .. } => todo!(),
        }
    }

    /// Flood fill (bucket tool) using an explicit stack.
    ///
    /// Uses a classic 4-connected fill.
    /// Recursion is avoided to prevent stack overflow.
    pub fn bucket_fill(
        &mut self,
        layer: usize,
        x_ref_point: u32,
        y_ref_point: u32,
        color: Color,
    ) {
        let fb = &mut self.layers[layer];

        match fb.return_pixel(x_ref_point, y_ref_point) {
            Some(addr_paint_col) => {
                let paint_col = *addr_paint_col;
                if paint_col == color { return; }

                let mut stack = Vec::new();
                stack.push((x_ref_point as i32, y_ref_point as i32));

                while let Some((x, y)) = stack.pop() {
                    if x < 0 || y < 0 { continue; }
                    if x >= fb.width as i32 || y >= fb.height as i32 { continue; }

                    if let Some(pixel) = fb.return_pixel(x as u32, y as u32) {
                        if *pixel != paint_col { continue; }

                        *pixel = color;

                        stack.push((x + 1, y));
                        stack.push((x - 1, y));
                        stack.push((x, y + 1));
                        stack.push((x, y - 1));
                    }
                }
            }

            None => {
                println!("No pixel found at reference point");
            }
        }
    }
}
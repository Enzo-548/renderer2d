use crate::renderer::{color::Color, render::Render};
#[derive (Clone)]

/// Defines the geometric data of a shape in local space.
/// This enum stores untransformed shape data and is intentionally
/// kept free of rendering or transformation state.
pub enum ShapeKind {
    Rect { center: (f32, f32), half_w: f32, half_h: f32 },
    Circle { center: (f32, f32), r: f32 },
    Ellipse { center: (f32, f32), rx: f32, ry: f32 },
    /// Line segment defined by two endpoints.
    /// The `start` flag is used to control which endpoint is affected
    /// by certain local-space operations (e.g. scaling experiments).
    Line { a: (f32,f32), b:(f32,f32), start: bool},
    Polygon { vertices: Vec<(f32, f32)> },
    
}
#[derive (Clone)]
/// Accumulates transformation intents (translation, scale, rotation)
/// to be applied explicitly during shape materialization.
/// This avoids mutating the original shape data.
struct Transforms {
    translate: (f32,f32),
    scale: (f32,f32),
    angle: f32,
}

/// High-level renderable shape composed of:
/// - immutable local-space geometry (ShapeKind)
/// - accumulated transform state
/// - rendering attributes (colors)
///
/// The shape itself is not directly rasterized; it must first be
/// materialized into world space.
pub struct Shape{
    pub kind: ShapeKind,
    transforms: Transforms,
    pub outline_color: Color,
    pub inline_color: Color,
}

impl Shape{
    /// Constructs a shape with distinct outline and fill colors.
    /// The shape starts with identity transforms (no deformation applied).
    pub fn new_defined_polychrome
    (
        kind: ShapeKind, 
        outline_color: Color, 
        inline_color: Color
    ) -> Shape{
        Self{
            kind,
            transforms: Transforms { translate: (0.0,0.0), scale: (1.0,1.0), angle: 0.0},
            outline_color,
            inline_color,
        }
    }
    /// Constructs a shape using a single color for both outline and fill.
    /// Useful for simple primitives and early experiments.
    pub fn new_defined_monochrome
    (
        kind: ShapeKind, 
        color: Color, 
    ) -> Shape{
        Self{
            kind,
            transforms: Transforms { translate: (0.0,0.0), scale: (1.0,1.0), angle: 0.0},
            outline_color: color,
            inline_color: color,
        }
    }
    pub fn reset_transforms(&mut self){
        self.transforms = Transforms { translate: (0.0, 0.0), scale: (1.0, 1.0), angle: 0.0 }
    }
    /// Materializes the shape into world space by applying all accumulated
    /// transforms to the underlying ShapeKind.
    /// The original local-space shape data remains unchanged.
    fn world_shape(&self) -> Shape{
        let mut global_kind = self.kind.clone();        
            global_kind.scale(self.transforms.scale.0, self.transforms.scale.1);
        
            global_kind.rotate(self.transforms.angle);

            global_kind.translate(self.transforms.translate.0, self.transforms.translate.1);

            Shape::new_defined_polychrome(global_kind, self.outline_color, self.inline_color)
    
    }
    /// Transform operations accumulate intent rather than directly
    /// modifying the underlying shape geometry.
    /// All transforms are applied lazily during materialization.
    pub fn translate(&mut self, dx: f32, dy: f32) {
        let (mut x, mut y) = self.transforms.translate;
        x += dx; y+=dy;
        self.transforms.translate = (x,y);
    }
    
    pub fn scale(&mut self, sx: f32, sy: f32) {
        let (mut x, mut y) = self.transforms.scale;
        x += sx; y+=sy;
        self.transforms.scale = (x,y);
    }
    
    pub fn rotate(&mut self, angle: f32) {
        let mut rot = self.transforms.angle;
        rot += angle;
        self.transforms.angle = rot;
    }
    /// Materializes the shape and submits it to the renderer.
    /// Rasterization always operates on a world-space representation.
    pub fn rasterize(&self, renderer: &mut Render, layer:usize, thickness:i32){
        renderer.draw_shape(&self.world_shape(),layer,thickness);
    }
}

impl ShapeKind {
    /// Applies a translation directly to the local-space geometry.
    /// This is only called during shape materialization.
    fn translate(&mut self, dx: f32, dy: f32) {
        match self {
            ShapeKind::Rect { center: origin, .. } => {
                origin.0 += dx;
                origin.1 += dy;
            }
            ShapeKind::Circle { center, .. } => {
                center.0 += dx;
                center.1 += dy;
            }
            ShapeKind::Ellipse { center, .. } => {
                center.0 += dx;
                center.1 += dy;
            }
            ShapeKind::Line { a, b, ..} => {
                a.0 += dx;
                a.1 += dy;
                b.0 += dx;
                b.1 += dy;
            }
            ShapeKind::Polygon { vertices } => {
                for v in vertices {
                    v.0 += dx;
                    v.1 += dy;
                }
            }
        }
    }
    /// Applies non-uniform scaling in local space.
    /// Different shape variants define their own scaling semantics.
    fn scale(&mut self, sx: f32, sy: f32){
        match self {
            ShapeKind::Rect {half_w: w, half_h: h, ..} => {
                    *w *= sx;
                    *h *= sy;
            }
            ShapeKind::Circle { r, .. } => {
                    *r *= sx; 
            }
            ShapeKind::Ellipse { rx, ry, .. } => {
                    *rx *= sx;
                    *ry *= sy; 
            }
            ShapeKind::Line { a, b, start } => {
            if *start{
                a.0 *= sx;
                a.1 *= sy;
            } else {
                b.0 *= sx;
                b.1 *= sy;
                }
            }
            ShapeKind::Polygon { vertices } => {
                fn centroid(vertices: &Vec<(f32,f32)>) -> (f32,f32) {
                    let mut x = 0.0;
                    let mut y = 0.0;

                    for v in vertices {
                        x += v.0;
                        y += v.1;
                    }

                    let n = vertices.len() as f32;
                    (x / n, y / n)
                }

                let (cx, cy) = centroid(vertices);

                for v in vertices {
                    v.0 = cx + (v.0 - cx) * sx;
                    v.1 = cy + (v.1 - cy) * sy;
                }
            }
        }
    }
    /// Applies rotation in local space around a shape-defined pivot
    /// (e.g. centroid or midpoint, depending on the shape).
    fn rotate(&mut self, angle: f32) {
        fn rotate_point(
            x: f32,
            y: f32,
            cx: f32,
            cy: f32,
            angle: f32,
        ) -> (f32, f32) {
            let sin = angle.sin();
            let cos = angle.cos();

            let dx = x - cx;
            let dy = y - cy;

            (
                cx + dx * cos - dy * sin,
                cy + dx * sin + dy * cos,
            )
        }

        match self {
            ShapeKind::Rect { .. } => {}
            ShapeKind::Circle { .. } => {}
            ShapeKind::Ellipse { .. } => {}

            ShapeKind::Line { a, b, .. } => {
                let cx = (a.0 + b.0) * 0.5;
                let cy = (a.1 + b.1) * 0.5;

                let (ax, ay) = rotate_point(a.0, a.1, cx, cy, angle);
                let (bx, by) = rotate_point(b.0, b.1, cx, cy, angle);

                *a = (ax, ay);
                *b = (bx, by);
            }

           ShapeKind::Polygon { vertices } => {
                let mut cx = 0.0;
                let mut cy = 0.0;

                for v in vertices.iter() {
                    cx += v.0;
                    cy += v.1;
                }

                let n = vertices.len() as f32;
                cx /= n;
                cy /= n;

                for v in vertices.iter_mut() {
                    let (rx, ry) = rotate_point(v.0, v.1, cx, cy, angle);
                    v.0 = rx;
                    v.1 = ry;
                }
            }
        }
    }
}

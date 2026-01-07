use crate::renderer::color::Color;
pub enum ShapeKind {
    Rect { origin: (f32, f32), w: f32, h: f32 },
    Circle { center: (f32, f32), r: f32 },
    Ellipse { center: (f32, f32), rx: f32, ry: f32 },
    Line { a: (f32,f32), b:(f32,f32) },
    Polygon { vertices: Vec<(f32, f32)> },
    
}

struct Shape{
    pub kind: ShapeKind,
    pub outline_color: Color,
    pub inline_color: Color,
}

impl Shape{
    ///Cria shape com duas cores
    pub fn new_defined_polychrome
    (
        kind: ShapeKind, 
        outline_color: Color, 
        inline_color: Color
    ) -> Shape{
        Self{
            kind,
            outline_color,
            inline_color,
        }
    }
    ///Cria shape com uma cor unitaria
    pub fn new_defined_monochrome
    (
        kind: ShapeKind, 
        color: Color, 
    ) -> Shape{
        Self{
            kind,
            outline_color: color,
            inline_color: color,
        }
    }
    
    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.kind.translate(dx, dy);
    }

    pub fn scale(&mut self, sx: f32, sy: f32) {
        self.kind.scale(sx,sy);
    }

    pub fn rotate(&mut self, angle: f32) {
        self.kind.rotate(angle);
    }


}

impl ShapeKind {
    fn translate(&mut self, dx: f32, dy: f32) {
        match self {
            ShapeKind::Rect { origin, .. } => {
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
            ShapeKind::Line { a, b } => {
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
    fn scale(&mut self, sx: f32, sy: f32){
        match self {
            ShapeKind::Rect {w, h, ..} => {
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
            ShapeKind::Line { a, b } => {
                a.0 *= sx;
                a.1 *= sy;
                b.0 *= sx;
                b.1 *= sy;
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
    fn rotate(&mut self, angle: f32){
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
            ShapeKind::Rect { origin, .. } => {
                let (x, y) = *origin;
                let (rx, ry) = rotate_point(x, y, 0.0, 0.0, angle);
                *origin = (rx, ry);
            }

            ShapeKind::Circle { center, .. } => {
                let (x, y) = *center;
                let (rx, ry) = rotate_point(x, y, 0.0, 0.0, angle);
                *center = (rx, ry);
            }

            ShapeKind::Ellipse { center, .. } => {
                let (x, y) = *center;
                let (rx, ry) = rotate_point(x, y, 0.0, 0.0, angle);
                *center = (rx, ry);
            }

            ShapeKind::Line { a, b } => {
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

use crate::renderer::{color::Color, render::Render};
#[derive (Clone)]
pub enum ShapeKind {
    Rect { center: (f32, f32), half_w: f32, half_h: f32 },
    Circle { center: (f32, f32), r: f32 },
    Ellipse { center: (f32, f32), rx: f32, ry: f32 },
    Line { a: (f32,f32), b:(f32,f32), start: bool},
    Polygon { vertices: Vec<(f32, f32)> },
    
}
#[derive (Clone)]
struct Transforms {
    translate: (f32,f32),
    scale: (f32,f32),
    angle: f32,
}

pub struct Shape{
    pub kind: ShapeKind,
    transforms: Transforms,
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
            transforms: Transforms { translate: (0.0,0.0), scale: (1.0,1.0), angle: 0.0},
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
            transforms: Transforms { translate: (0.0,0.0), scale: (1.0,1.0), angle: 0.0},
            outline_color: color,
            inline_color: color,
        }
    }
    
    pub fn reset_transforms(&mut self){
        self.transforms = Transforms { translate: (0.0, 0.0), scale: (0.0, 0.0), angle: 0.0 }
    }

    fn world_shape(&self) -> Shape{
        let mut global_kind = self.kind.clone();        
            global_kind.scale(self.transforms.scale.0, self.transforms.scale.1);
        
            global_kind.rotate(self.transforms.angle);

            global_kind.translate(self.transforms.translate.0, self.transforms.translate.1);

            Shape::new_defined_polychrome(global_kind, self.outline_color, self.inline_color)
    
    }

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

    pub fn rasterize(&self, renderer: &mut Render, layer:usize, thickness:i32){
        renderer.draw_shape(&self.world_shape(),layer,thickness);
    }
}

impl ShapeKind {
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

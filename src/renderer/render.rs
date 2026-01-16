use crate::renderer::{color::Color, framebuffer::{*}, shape::{*}};
#[derive (Debug)]
pub struct Render{
    /* Layer Vector: carrega varias listas de cores, vai ser usado para implementação de layers;
     * eh uma mudanca arquitetural importante pois aqui vai ser onde o overlay vai ser implementado, 
     * podendo ser usado também para carregar multiplos framebuffers ou camadas especificas de desenho
     * */ 
    
    pub layers : Vec<Framebuffer>,
}

impl Render{
    /// Cria o renderer com um framebuffer inicial
    pub fn new(framebuffer: Framebuffer) -> Render {
        let (width, height) = (framebuffer.width, framebuffer.height);
        Self {
            layers: vec![Render::new_layer(width,height), framebuffer],
        }
    }
    /// Cria um novo layer para o renderer inicial
    pub fn new_layer(width:u32, height:u32) -> Framebuffer {
        Framebuffer::new(width,height)
    }

    pub fn overlay_mut(&mut self) -> Option<&mut Framebuffer>{
        self.layers.get_mut(0)
    }
    
    pub fn draw_shape(&mut self, shape: &Shape, layer:usize, thickness: i32){
        match &shape.kind{
            rect @ ShapeKind::Rect { center: origin, half_w: w, half_h: h }  => {
                let is_filled = {
                    if shape.inline_color == shape.outline_color {
                        true
                    } else {
                        false
                    }
                };
                 if shape.inline_color != Color::ZERO && is_filled == true{
                    self.geo_fill(rect, layer, shape.inline_color);
                }
                self.draw_rectangle(layer, origin.0 as i32, origin.1 as i32, *w as i32, *h as i32, thickness, shape.outline_color);
                
            },
            circle @ ShapeKind::Circle { center, r } => {
                let is_filled = {
                    if shape.inline_color == shape.outline_color {
                        true
                    } else {
                        false
                    }
                };
                self.draw_circle(layer, center.0 as i32, center.1 as i32, *r as i32, thickness, shape.outline_color);
                if shape.inline_color == Color::ZERO && is_filled == true{
                    self.geo_fill(circle, layer, shape.inline_color);
                }
            },
            ShapeKind::Ellipse { center, rx, ry } => {
                todo!()
            },
            ShapeKind::Line { a, b } => {
                self.draw_line(layer, a.0 as i32, a.1 as i32, b.0 as i32, b.1 as i32, thickness, shape.inline_color);
                todo!()
            },
            polygon @ ShapeKind::Polygon { vertices } => {
                let is_filled = {
                    if shape.inline_color == shape.outline_color {
                        true
                    } else {
                        false
                    }
                };
                let (x_ref_vertex1, y_ref_vertex1) = *vertices.get(0).unwrap();
                let (x_ref_vertex2, y_ref_vertex2) = *vertices.get(1).unwrap();
                let (x_ref_vertex3, y_ref_vertex3) = *vertices.get(2).unwrap();
                self.draw_triangle(layer, 
                    x_ref_vertex1 as i32, y_ref_vertex1 as i32, 
                    x_ref_vertex2 as i32, y_ref_vertex2 as i32, 
                    x_ref_vertex3 as i32, y_ref_vertex3 as i32, thickness, shape.outline_color);
                
                if shape.inline_color != Color::ZERO && is_filled == true{
                    self.geo_fill(polygon, layer, shape.inline_color);
                }
            },
        }
    }
    pub fn draw_dynam(&mut self, layer: usize, draw_sel:u32, ref_x:u32, ref_y:u32, thickness: i32, color: Color, destination:(u32,u32)){
            match draw_sel{
                0 => {
                    Shape::new_defined_monochrome(
                        ShapeKind::Rect { center: (ref_x as f32, ref_y as f32), half_w: 10.0, half_h: 10.0 },
                        color
                ).rasterize(self, layer, thickness);
                }
                1 => {
                    
                        Shape::new_defined_monochrome(
                        ShapeKind::Polygon { vertices: vec![(ref_x as f32, (ref_y as f32)-25.0),((ref_x as f32) - 25.0, (ref_y as f32)+25.0),(ref_x as f32 + 25.0, (ref_y as f32)+25.0)]},
                        color).rasterize(self, layer, thickness);
                    
                },
                2 => {
                    Shape::new_defined_monochrome(
                        ShapeKind::Circle { center: (ref_x as f32, ref_y as f32), r: 10.0 },
                        color).rasterize(self, layer, thickness);
                    
                },
                3=> {
                    self.bucket_fill(layer,ref_x, ref_y, color);
                    
                },
                4=> todo!(),
                _ => ()
            }
        }


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
        // thickness discreto
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

    pub fn draw_rectangle(&mut self, layer: usize, x_ref_point: i32, y_ref_point: i32, width:i32, height:i32, thickness:i32, color:Color){
        //x_start and x_end indicate the width of the rectangle and y_pos where those lines will be drawn
        //although it is possible that is nescesserary to indicate a reference point for the shape
        //as the shape shall be drawn from this point and may dictate the height and width of the shape.
        //draw_rectangle(&mut self, x_ref_point: u32, y_ref_point: u32, width:i32, height:i32, thickness:i32, color:Color)
        //self.draw_horizontal_line()
        //rightside
        let x_coordinate_right_side = x_ref_point+width; //355
        //leftside
        let x_coordinate_left_side = x_ref_point-width; //245
        //upline
        let y_coordinate_up_side = y_ref_point+height;
        //downline
        let y_coordinate_down_side = y_ref_point-height;
        //self.draw_line(x0, y0, x1, y1, thickness, color);
        self.draw_line(layer, x_coordinate_right_side, y_coordinate_down_side - thickness, x_coordinate_right_side, y_coordinate_up_side + thickness, thickness, color);
        self.draw_line(layer,x_coordinate_left_side,y_coordinate_down_side - thickness, x_coordinate_left_side, y_coordinate_up_side + thickness,thickness,color);
        self.draw_line(layer,x_coordinate_left_side - thickness, y_coordinate_down_side, x_coordinate_right_side + thickness, y_coordinate_down_side, thickness, color);
        self.draw_line(layer,x_coordinate_left_side - thickness, y_coordinate_up_side, x_coordinate_right_side + thickness, y_coordinate_up_side, thickness, color);
    }

    pub fn draw_triangle(&mut self, layer: usize,
    x_ref_vertex1:i32, y_ref_vertex1:i32,
    x_ref_vertex2:i32, y_ref_vertex2:i32, 
    x_ref_vertex3:i32, y_ref_vertex3:i32, thickness: i32, color:Color){
        let r = thickness;
        
        self.draw_line(layer,x_ref_vertex1, y_ref_vertex1, x_ref_vertex2, y_ref_vertex2, thickness, color);
        self.draw_line(layer,x_ref_vertex1, y_ref_vertex1, x_ref_vertex3, y_ref_vertex3, thickness, color);
        self.draw_line(layer,x_ref_vertex2, y_ref_vertex2, x_ref_vertex3, y_ref_vertex3, thickness, color);

        self.draw_circle(layer,x_ref_vertex1, y_ref_vertex1, r, 0, color);
        self.draw_circle(layer,x_ref_vertex2, y_ref_vertex2, r, 0, color);
        self.draw_circle(layer, x_ref_vertex3, y_ref_vertex3, r, 0, color);
        }

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
    // caso especial: círculo sólido
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
                d += 2*y + 1;
            } else {
                x -= 1;
                d += 2*(y - x) + 1;
            }
        }
        return;
    }

    // anel (outline)
    let r_outer = r + thickness;
    let r_inner = (r - thickness).max(0);

    let ro2 = r_outer * r_outer;
    let ri2 = r_inner * r_inner;

    for y in (cy - r_outer)..=(cy + r_outer) {
        for x in (cx - r_outer)..=(cx + r_outer) {
            let dx = x - cx;
            let dy = y - cy;
            let d2 = dx*dx + dy*dy;

            if d2 <= ro2 && d2 >= ri2 {
                fb.put_pixel(x as u32, y as u32, color);
            }
        }
    }
}
    fn geo_fill(&mut self, shape: &ShapeKind, layer: usize, color: Color) {
        let fb = &mut self.layers[layer];
        match shape {
            ShapeKind::Rect { center: origin, half_w: w, half_h: h } => {    
                            let y_min = (origin.1 - *h) as i32;
                let y_max = (origin.1 + *h) as i32;
                let x_min = (origin.0 - *w) as i32;
                let x_max = (origin.0 + *w) as i32;

                let y0 = y_min.min(y_max);
                let y1 = y_min.max(y_max);
                let x0 = x_min.min(x_max);
                let x1 = x_min.max(x_max);

                for y in y0..=y1 {
                    if y < 0 || y >= fb.height as i32 { continue; }

                    for x in x0..=x1 {
                        if x < 0 || x >= fb.width as i32 { continue; }

                        fb.put_pixel(x as u32, y as u32, color);
                    }
                }
            }
            ShapeKind::Polygon { vertices } => { 
                let (x_ref_vertex1, y_ref_vertex1) = *vertices.get(0).unwrap();
                let (x_ref_vertex2, y_ref_vertex2) = *vertices.get(1).unwrap();
                let (x_ref_vertex3, y_ref_vertex3) = *vertices.get(2).unwrap();
                
                let mut v = [
                (x_ref_vertex1 as i32, y_ref_vertex1 as i32),
                (x_ref_vertex2 as i32, y_ref_vertex2 as i32),
                (x_ref_vertex3 as i32, y_ref_vertex3 as i32),
            ];

            v.sort_by_key(|(_, y)| *y);

            let (x0, y0) = v[0];
            let (x1, y1) = v[1];
            let (x2, y2) = v[2];
                let fb = &mut self.layers[layer];

            let interp = |x0, y0, x1, y1, y| {
                if y1 == y0 { x0 }
                else { x0 + (x1 - x0) * (y - y0) / (y1 - y0) }
            };

            for y in y0..=y2 {
                let xa;
                let xb;

                if y < y1 {
                    xa = interp(x0, y0, x2, y2, y);
                    xb = interp(x0, y0, x1, y1, y);
                } else {
                    xa = interp(x0, y0, x2, y2, y);
                    xb = interp(x1, y1, x2, y2, y);
                }

                let (xmin, xmax) = if xa < xb { (xa, xb) } else { (xb, xa) };

                    for x in xmin..=xmax {
                        fb.put_pixel(x as u32, y as u32, color);
                    }
                }
            },
            ShapeKind::Circle { r, center } => { 
                let r = *r as i32;
                for dy in -r..=r {
                    let y = center.1 as i32 + dy;
                    let dx = ((r*r - dy*dy) as f32).sqrt() as i32;
                    for x in (center.0 as i32 - dx)..=(center.0 as i32 + dx) {
                        fb.put_pixel(x as u32, y as u32, color);
                    }
                }
            }
            ShapeKind::Ellipse { .. } => todo!(),
            ShapeKind::Line { .. } => todo!(),
        }
    }

    pub fn bucket_fill(&mut self, layer: usize, x_ref_point: u32, y_ref_point: u32, color: Color){
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

            match fb.return_pixel(x as u32, y as u32){
                Some(pixel) => {
                            
                    if *pixel != paint_col { continue; }

                    *pixel = color;

                    stack.push((x + 1, y));
                    stack.push((x - 1, y));
                    stack.push((x, y + 1));
                    stack.push((x, y - 1));
                }
                None => {
                    println!("Não achei pixel nenhum!")
                }
            }
        }

            }
            None => {
                println!("Não achei cor nenhuma!")
            }
        }
    }
    /* later */
    //pub fn skew()
}
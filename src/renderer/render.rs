use crate::renderer::{color::Color, framebuffer::{*}};

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
    
    //REESCREVER PARA RETORNAR UM ENUM DE SHAPE
    pub fn draw_dynam(&mut self, layer: usize, draw_sel:u32, fix_x:u32, fix_y:u32, thickness: i32, color: Color, destination:(u32,u32)) {
                    /*for i in -thickness..thickness{
            let x = x as i32 + i;
            let y = y as i32 + i;
            self.put_pixel(x as u32, y as u32, color);
        }*/
            match draw_sel{
                0 => self.draw_rectangle(layer,fix_x as i32, fix_y as i32, 1, 1, true, thickness, color),
                1 => {
                    let x_ref_vertex1 = fix_x as i32; let y_ref_vertex1 = (fix_y as i32 -25) as i32;
                    let x_ref_vertex2 = (fix_x-25) as i32; let y_ref_vertex2 = (fix_y+25) as i32;
                    let x_ref_vertex3 = (fix_x+25) as i32; let y_ref_vertex3 = (fix_y+25) as i32;
                    self.draw_triangle(layer,x_ref_vertex1, y_ref_vertex1, x_ref_vertex2, y_ref_vertex2, x_ref_vertex3, y_ref_vertex3, true, thickness, color)
                },
                2 => self.draw_circle(layer,fix_x as i32, fix_y as i32, thickness, true, thickness, color),
                3=> self.fill(layer,fix_x, fix_y, color),
                4=> self.draw_line(layer,fix_x as i32, fix_y as i32, destination.0 as i32, destination.1 as i32, thickness, color),
                _ => println!("não aceito")
            }
        }


    pub fn draw_line(&mut self, layer:usize, x0:i32, y0:i32, x1:i32,y1:i32, thickness:i32, color: Color){
        let fb = &mut self.layers[layer];
        let dx:i32 = i32::abs(x1 - x0);
        let dy:i32 = i32::abs(y1 - y0);
        let sx:i32 = { if x0 < x1 { 1 } else { -1 } };
        let sy:i32 = { if y0 < y1 { 1 } else { -1 } };

        let mut error:i32 = (if dx > dy  { dx } else { -dy }) / 2 ;

        let mut x = x0;
        let mut y = y0;
            loop {
            // desenha com thickness discreto
            if dx >= dy {
                for o in -thickness..=thickness {
                    fb.put_pixel(x as u32, (y + o) as u32, color);
                }
            } else {
                for o in -thickness..=thickness {
                    fb.put_pixel((x + o) as u32, y as u32, color);
                }
            }

            if x == x1 && y == y1 { break; }

            let e2 = 2 * error;
            if e2 > -dy { error -= dy; x += sx; }
            if e2 <  dx { error += dx; y += sy; }
        }
    }

     pub fn draw_rectangle(&mut self, layer: usize, x_ref_point: i32, y_ref_point: i32, width:i32, height:i32, is_filled:bool, thickness:i32, color:Color){
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
        
        if is_filled == true{
            self.fill(layer, x_ref_point as u32, y_ref_point as u32, color);
        }
    }

  pub fn draw_triangle(&mut self, layer: usize,
    x_ref_vertex1:i32, y_ref_vertex1:i32,
    x_ref_vertex2:i32, y_ref_vertex2:i32, 
    x_ref_vertex3:i32, y_ref_vertex3:i32, is_filled:bool, thickness: i32, color:Color){
        let r = thickness;

        self.draw_circle(layer,x_ref_vertex1, y_ref_vertex1, r, true, 0, color);
        self.draw_circle(layer,x_ref_vertex2, y_ref_vertex2, r, true, 0, color);
        self.draw_circle(layer, x_ref_vertex3, y_ref_vertex3, r, true, 0, color);

        
        self.draw_line(layer,x_ref_vertex1, y_ref_vertex1, x_ref_vertex2, y_ref_vertex2, thickness, color);
        self.draw_line(layer,x_ref_vertex1, y_ref_vertex1, x_ref_vertex3, y_ref_vertex3, thickness, color);
        self.draw_line(layer,x_ref_vertex2, y_ref_vertex2, x_ref_vertex3, y_ref_vertex3, thickness, color);

        
        if is_filled == true{
            let center_x = (x_ref_vertex1 + x_ref_vertex2 + x_ref_vertex3) / 3;
            let center_y = (y_ref_vertex1 + y_ref_vertex2 + y_ref_vertex3) / 3;
            self.fill(layer, center_x as u32, center_y as u32, color);
        }
    }
    pub fn fill(&mut self, layer: usize, x_ref_point: u32, y_ref_point: u32, color: Color){
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

    pub fn draw_circle(
    &mut self,
    layer: usize,
    cx: i32,
    cy: i32,
    r: i32,
    is_filled: bool,
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

    if is_filled {
        self.fill(layer, cx as u32, cy as u32, color);
    }
}
    //  pub fn draw_filled circle
    
    
    /* later */
    //pub fn skew()
}
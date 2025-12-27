use crate::renderer::{color::Color, framebuffer::{*}};

#[derive (Debug)]
pub struct Render{
    pub framebuffer : Framebuffer,
    //window??
}

impl Render{  
    /// Cria o renderer com um framebuffer inicial
    pub fn new(framebuffer: Framebuffer) -> Render{
        return Self { framebuffer };
    }
    /// Limpa o framebuffer com uma cor
    pub fn clear(&mut self, color: Color){
        for pixel in &mut self.framebuffer.pixels_buffer{
            *pixel = color;
        }
    }
    /// Desenha um pixel (com bounds check)
    pub fn put_pixel(&mut self, x:u32, y:u32, color: Color){
        //X = largura; Y=altura;
        if x>= self.framebuffer.width || y>= self.framebuffer.height {
            return;
        }

        let index = (y*self.framebuffer.width + x) as usize;
        self.framebuffer.pixels_buffer[index] = color;
    }

    pub fn return_pixel(&mut self, x:u32, y:u32) -> Option<&Color>{
            if x>= self.framebuffer.width || y>= self.framebuffer.height {
            return None;
        } else {
            let index = (y*self.framebuffer.width + x) as usize;
            let col_ref = &self.framebuffer.pixels_buffer[index];
            return Some(col_ref);
        }
    }

    pub fn draw_line(&mut self, x0:i32, y0:i32, x1:i32,y1:i32, thickness:i32, color: Color){
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
                    self.put_pixel(x as u32, (y + o) as u32, color);
                }
            } else {
                for o in -thickness..=thickness {
                    self.put_pixel((x + o) as u32, y as u32, color);
                }
            }

            if x == x1 && y == y1 { break; }

            let e2 = 2 * error;
            if e2 > -dy { error -= dy; x += sx; }
            if e2 <  dx { error += dx; y += sy; }
        }
    }

     pub fn draw_rectangle_unfilled(&mut self, x_ref_point: i32, y_ref_point: i32, width:i32, height:i32, thickness:i32, color:Color){
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
        self.draw_line(x_coordinate_right_side, y_coordinate_down_side - thickness, x_coordinate_right_side, y_coordinate_up_side + thickness, thickness, color);
        self.draw_line(x_coordinate_left_side,y_coordinate_down_side - thickness, x_coordinate_left_side, y_coordinate_up_side + thickness,thickness,color);
        self.draw_line(x_coordinate_left_side - thickness, y_coordinate_down_side, x_coordinate_right_side + thickness, y_coordinate_down_side, thickness, color);
        self.draw_line(x_coordinate_left_side - thickness, y_coordinate_up_side, x_coordinate_right_side + thickness, y_coordinate_up_side, thickness, color);
    }


  pub fn draw_rectangle_as_filled(&mut self, x_ref_point: i32, y_ref_point: i32, width:i32, height:i32, thickness:i32, outline_color:Color, inline_color:Color){

        //rightside
        let x_coordinate_right_side = x_ref_point+width; //355
        //leftside
        let x_coordinate_left_side = x_ref_point-width; //245
        //upline
        let y_coordinate_up_side = y_ref_point+height;
        //downline
        let y_coordinate_down_side = y_ref_point-height;

        self.draw_line(x_coordinate_right_side, y_coordinate_down_side - thickness, x_coordinate_right_side, y_coordinate_up_side + thickness, thickness, outline_color);
        self.draw_line(x_coordinate_left_side,y_coordinate_down_side - thickness, x_coordinate_left_side, y_coordinate_up_side + thickness,thickness,outline_color);
        self.draw_line(x_coordinate_left_side - thickness, y_coordinate_down_side, x_coordinate_right_side + thickness, y_coordinate_down_side, thickness, outline_color);
        self.draw_line(x_coordinate_left_side - thickness, y_coordinate_up_side, x_coordinate_right_side + thickness, y_coordinate_up_side, thickness, outline_color);

        /*for mut i in 0..self.framebuffer.pixels_buffer.len()-1{
            let curr_color = self.framebuffer.pixels_buffer[i];
            
            
        }        */
  }
  pub fn draw_triangle(&mut self, 
    x_ref_vertex1:i32, y_ref_vertex1:i32,
    x_ref_vertex2:i32, y_ref_vertex2:i32, 
    x_ref_vertex3:i32, y_ref_vertex3:i32, thickness: i32, color:Color){
        //implementar um fix para desenhar nas bordas usando um circulo cheio
        self.draw_line(x_ref_vertex1, y_ref_vertex1, x_ref_vertex2, y_ref_vertex2, thickness, color);
        self.draw_line(x_ref_vertex1, y_ref_vertex1, x_ref_vertex3, y_ref_vertex3, thickness, color);
        self.draw_line(x_ref_vertex2, y_ref_vertex2, x_ref_vertex3, y_ref_vertex3, thickness, color);
    }    
    /// Acesso somente-leitura ao buffer
    pub fn buffer(&self) -> &[Color]{
        &self.framebuffer.pixels_buffer
    }
}
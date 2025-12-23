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
    //Generalized draw, should unify draw vertical and horizontal lines along the center of the canvas
    /*pub fn draw_line(&mut self, mut x0:i32, x1:i32, mut y0:i32, y1:i32, size:i32, color: Color){
        //aqui deveria receber o eixo de desenho, seja x ou  ou os dois
        //mas e se depois eu quiser reutilizar para o mouse ?
        //vai ter que passar os dois eixos
        //x1 e y1 são limites no meu código
        
    }*/
    pub fn draw_vertical_line(&mut self, x: i32, y_start:i32, y_end:i32, thickness:i32, color: Color){
        for i in -thickness..=thickness{
                let mut y:i32 = y_start;
                if y_start < y_end{
                //less than size
                while y <= y_end{
                    //implict clamp
                    let cur_thickness = (x+i) as u32;
                    self.put_pixel(cur_thickness, y as u32, color);
                    y+=1;
                }
            } else {
                //more than size
                    while y >= y_end{
                    //implict clamp
                    let cur_thickness = (x+i) as u32;
                    self.put_pixel(cur_thickness, y as u32, color);
                    y-=1;
                }
            }
        }
    }
    pub fn draw_horizontal_line(&mut self, x_start:i32, y:i32, x_end:i32, thickness:i32, color: Color){
        for i in -thickness..=thickness{
                let mut x = x_start as i32;
                if x_start < x_end{
                //less than size
                    while x <= x_end{
                        //implict clamp
                        let cur_thickness = (y+i) as u32;
                        self.put_pixel(x as u32, cur_thickness, color);
                        x+=1;
                    }
                } else {
                //more than size
                    while x >= x_end{
                        //implict clamp
                        let cur_thickness = (y+i) as u32;
                        self.put_pixel(x as u32, cur_thickness, color);
                        x-=1;
                    }
                }
            }
    }
    pub fn draw_perfect_diagonal_line(&mut self, x_start:i32, y_start:i32, x_end:i32, y_end:i32, thickness:i32, color: Color){
        for i in -thickness..=thickness{
                let mut x = x_start as i32;
                let mut y = y_start as i32;
                if x_start < x_end && y_start < y_end{
                    //less than size
                    while x <= x_end &&  y <= y_end{
                        //implict clamp
                        let cur_thickness_y = (y+i) as u32;
                        let cur_thickness_x = (x+i) as u32;
                        self.put_pixel(x as u32, cur_thickness_y, color);
                        self.put_pixel(y as u32, cur_thickness_x, color);
                        x+=1;
                        y+=1;
                        }
                println!("veio pro lado menor !")
                } 
                if  x_start > x_end && y_start > y_end{
                    //more than size
                    while x >= x_end &&  y >= y_end{
                        //implict clamp
                        let cur_thickness_y = (y+i) as u32;
                        let cur_thickness_x = (x+i) as u32;
                        self.put_pixel(x as u32, cur_thickness_y, color);
                        self.put_pixel(y as u32, cur_thickness_x, color);
                        x-=1;
                        y-=1;
                        }
                    println!("veio pro lado maior !");
                } else {
                    if x_start >= x_end{
                        // x axis is dominant
                        while x >= x_end &&  y <= y_end{
                        //implict clamp
                        let cur_thickness_y = (y+i) as u32;
                        let cur_thickness_x = (x+i) as u32;
                        self.put_pixel(x as u32, cur_thickness_y, color);
                        self.put_pixel(y as u32, cur_thickness_x, color);
                        x-=1;
                        y+=1;
                        }
                    println!("veio pro lado x eh maior !");
                    }
                    if y_start >= y_end{
                        // y axis is dominant
                        while x <= x_end &&  y >= y_end{
                        //implict clamp
                        let cur_thickness_y = (y+i) as u32;
                        let cur_thickness_x = (x+i) as u32;
                        self.put_pixel(x as u32, cur_thickness_y, color);
                        self.put_pixel(y as u32, cur_thickness_x, color);
                        x+=1;
                        y-=1;
                        }
                        println!("veio pro lado y eh maior !");
                    }
                }
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
        self.draw_vertical_line (x_coordinate_right_side,y_coordinate_down_side - thickness,y_coordinate_up_side + thickness,thickness,color);
        self.draw_vertical_line (x_coordinate_left_side,y_coordinate_down_side - thickness,y_coordinate_up_side + thickness,thickness,color);
        self.draw_horizontal_line(x_coordinate_left_side - thickness, y_coordinate_down_side, x_coordinate_right_side + thickness, thickness, color);
        self.draw_horizontal_line(x_coordinate_left_side - thickness, y_coordinate_up_side, x_coordinate_right_side + thickness, thickness, color);
    }
//  pub fn draw_rectangle_as_filled(&mut self, x_ref_point: i32, y_ref_point: i32, width:i32, height:i32, thickness:i32, outline_color:Color, inline_color::Color){}
//  pub fn draw_triangle(&mut self, x_ref_point: i32, y_ref_point: i32, vertex1:i32, vertex2:i32, vertex3:i32, color:Color){}    
    /// Acesso somente-leitura ao buffer
    pub fn buffer(&self) -> &[Color]{
        &self.framebuffer.pixels_buffer
    }
}
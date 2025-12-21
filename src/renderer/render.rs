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
                while y < y_end{
                    //implict clamp
                    let cur_thickness = (x+i) as u32;
                    self.put_pixel(cur_thickness, y as u32, color);
                    y+=1;
                }
        }
    }
    pub fn draw_horizontal_line(&mut self, x_start:i32, y:i32, x_end:i32, thickness:i32, color: Color){
        for i in -thickness..=thickness{
                let mut x = x_start as i32;
                while x < x_end{
                    //implict clamp
                    let cur_thickness = (y+i) as u32;
                    self.put_pixel(x as u32, cur_thickness, color);
                    x+=1;
                }
            }
    }
    /// Acesso somente-leitura ao buffer
    pub fn buffer(&self) -> &[Color]{
        &self.framebuffer.pixels_buffer
    }
}
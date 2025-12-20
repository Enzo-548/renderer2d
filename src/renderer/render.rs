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

    pub fn draw_vertical_line(&mut self, color: Color){
        let mut i = 0;
                while i < self.framebuffer.height{
                self.put_pixel(self.framebuffer.width/2-3, i, color);
                self.put_pixel(self.framebuffer.width/2-2, i, color);
                self.put_pixel(self.framebuffer.width/2-1, i, color);
                self.put_pixel(self.framebuffer.width/2, i, color);
                self.put_pixel(self.framebuffer.width/2+1, i, color);
                self.put_pixel(self.framebuffer.width/2+2, i, color);
                self.put_pixel(self.framebuffer.width/2+3, i, color);
                i+=1;
            }
    }

    pub fn draw_horizontal_line(&mut self, color: Color){
        let mut i = 0;
                while i < self.framebuffer.width{
                self.put_pixel(i, self.framebuffer.height/2, color);
                self.put_pixel(i, self.framebuffer.height/2+1, color);
                self.put_pixel(i, self.framebuffer.height/2+2, color);
                self.put_pixel(i, self.framebuffer.height/2, color);
                self.put_pixel(i, self.framebuffer.height/2-1, color);
                self.put_pixel(i, self.framebuffer.height/2-2,color);
                self.put_pixel(i, self.framebuffer.height/2-3, color);
                i+=1;
            }
    }
    /// Acesso somente-leitura ao buffer
    pub fn buffer(&self) -> &[Color]{
        &self.framebuffer.pixels_buffer
    }
}
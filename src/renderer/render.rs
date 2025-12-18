use crate::renderer::{color::Color, framebuffer::{self, *}};

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
        if x>= self.framebuffer.width || y>= self.framebuffer.height {
            return;
        }

        let index = (y*self.framebuffer.width + x) as usize;
        self.framebuffer.pixels_buffer[index] = color;
    }
    /// Acesso somente-leitura ao buffer
    pub fn buffer(&self) -> &[Color]{
        &self.framebuffer.pixels_buffer
    }
}
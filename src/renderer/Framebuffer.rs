use crate::renderer::color::*;

#[derive (Debug)]
pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pixels_buffer: Vec<Color>,
    background_color : Color,
}

impl Framebuffer{
    pub fn new(width:u32,height:u32) -> Framebuffer{
        let size = (width*height) as usize;
        println!("am tryna fill");
        Self{
            width,
            height,
            background_color: Color::ZERO,
            pixels_buffer: vec![Color::ZERO; size],
        }
    }
    pub fn as_u32_buffer(&self) -> Vec<u32> {
        self.pixels_buffer
            .iter()
            .map(|c| {
                ((c.a as u32) << 24)
              | ((c.r as u32) << 16)
              | ((c.g as u32) << 8)
              |  (c.b as u32)
            })
            .collect()
    }
    pub fn as_u8_buffer(&self) -> Vec<u8>{
        self.pixels_buffer
        .iter()
        .flat_map(|c| [c.r, c.g, c.b, c.a])
        .collect()
    }

    /// Limpa o framebuffer com uma cor
    pub fn clear(&mut self, color: Color){
        for pixel in &mut self.pixels_buffer{
            *pixel = color;
        }
        self.background_color = color;
    }
    /// Desenha um pixel (com bounds check)
    pub fn put_pixel(&mut self, x:u32, y:u32, color: Color){
        //X = largura; Y=altura;
        if x>= self.width || y>= self.height {
            return;
        }

        let index = (y*self.width + x) as usize;
        self.pixels_buffer[index] = color;
    }

    pub fn return_pixel(&mut self, x:u32, y:u32) -> Option<&mut Color>{
            if x>= self.width || y>= self.height {
            return None;
        } else {
            let index = (y*self.width + x) as usize;
            let col_ref = &mut self.pixels_buffer[index];
            return Some(col_ref);
        }
    }
    /// Acesso somente-leitura ao buffer
    pub fn cur_buffer(&self) -> &[Color]{
        &self.pixels_buffer
    }
    /// Acesso leitura e escrita ao buffer
    fn cur_buffer_as_mut(&mut self) -> &mut [Color]{
        self.pixels_buffer.as_mut_slice()
    }
    pub fn update_buffer(&mut self, buffer: &[Color]){
        self.pixels_buffer.copy_from_slice(buffer);
    }
    pub fn update_color(&mut self, col: Color, index: usize){
        self.cur_buffer_as_mut()[index] = col;
    }
}

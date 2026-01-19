use crate::renderer::color::*;

#[derive (Debug)]
pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pixels_buffer: Vec<Color>,
    background_color : Color,
}

impl Framebuffer{
    ///Creates a new framebuffer
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
    ///Returns the pixel buffer as an u32 list, is used for updating the event loop
    pub fn as_u32_buffer(&self) -> Vec<u32> {
        self.cur_buffer()
            .iter()
            .map(|c| {
                ((c.a as u32) << 24)
              | ((c.r as u32) << 16)
              | ((c.g as u32) << 8)
              |  (c.b as u32)
            })
            .collect()
    }
    ///Returns the pixel buffer as an u8 list, is used for saving the buffer onto a .png image
    pub fn as_u8_buffer(&self) -> Vec<u8>{
        self.cur_buffer()
        .iter()
        .flat_map(|c| [c.r, c.g, c.b, c.a])
        .collect()
    }
    ///Uses the specified color to clean the buffer
    pub fn clear(&mut self, color: Color){
        for pixel in &mut self.pixels_buffer{
            *pixel = color;
        }
        self.background_color = color;
    }
    ///Updates the index at the (x,y) coordinate provided with the Color provided.
    pub fn put_pixel(&mut self, x:u32, y:u32, color: Color){
        if x>= self.width || y>= self.height {
            return;
        }

        let index = (y*self.width + x) as usize;
        self.pixels_buffer[index] = color;
    }
    ///Returns the Color at the (x,y) coordinate provided.
    pub fn return_pixel(&mut self, x:u32, y:u32) -> Option<&mut Color>{
            if x>= self.width || y>= self.height {
            return None;
        } else {
            let index = (y*self.width + x) as usize;
            let col_ref = &mut self.pixels_buffer[index];
            return Some(col_ref);
        }
    }
    ///Returns the buffer as read-only, used for conversion to images. 
    pub fn cur_buffer(&self) -> &[Color]{
        &self.pixels_buffer
    }
    ///Returns the current buffer as mutable, this is not intended for other classes to use.
    fn cur_buffer_as_mut(&mut self) -> &mut [Color]{
        self.pixels_buffer.as_mut_slice()
    }
    ///Updates the whole buffer.
    pub fn update_buffer(&mut self, buffer: &[Color]){
        self.cur_buffer_as_mut().copy_from_slice(buffer);
    }
    ///Updates the color at the specified index in the buffer.
    pub fn update_color(&mut self, col: Color, index: usize){
        self.cur_buffer_as_mut()[index] = col;
    }
}

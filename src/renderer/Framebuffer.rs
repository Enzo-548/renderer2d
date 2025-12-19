use crate::renderer::color::*;

#[derive (Debug)]
pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub pixels_buffer: Vec<Color>,
}

impl Framebuffer{
    pub fn new(width:u32,height:u32) -> Framebuffer{
        let size = (width*height) as usize;
        println!("am tryna fill");
        Self{
            width,
            height,
            pixels_buffer: vec![Color::zero(); size],
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
}

use crate::renderer::color::*;

#[derive (Debug)]
pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub pixels_buffer: Vec<Color>,
}

impl Framebuffer{
    pub fn new(width:u32,height:u32,pixels_buffer: Vec<Color>) -> Framebuffer{
        return Self{width,height,pixels_buffer};
    }
    pub fn new_as_filled(width:u32,height:u32) -> Framebuffer{
        let size = (width*height) as usize;
        Self{
            width,
            height,
            pixels_buffer: vec![Color::zero(); size],
        }
    }
}

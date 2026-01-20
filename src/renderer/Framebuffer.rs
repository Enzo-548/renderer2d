use crate::renderer::color::*;

#[derive (Debug)]
/// CPU-side framebuffer abstraction.
/// Owns a contiguous pixel buffer and provides explicit read/write
/// access for software rasterization and presentation.
pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    /// Contiguous pixel storage in row-major order.
    /// Indexing follows (y * width + x).
    pixels_buffer: Vec<Color>,
    /// Tracks the last clear color used on this framebuffer.
    /// This is informational and not automatically reapplied.
    background_color : Color,
}

impl Framebuffer{
    /// Allocates a framebuffer with the given dimensions.
    /// The pixel buffer is initialized to a uniform background color.
    pub fn new(width:u32,height:u32) -> Framebuffer{
        let size = (width*height) as usize;
        Self{
            width,
            height,
            background_color: Color::ZERO,
            pixels_buffer: vec![Color::ZERO; size],
        }
    }
    /// Packs the framebuffer into a u32 buffer (ARGB layout).
    /// Intended for presentation via window backends.
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
    /// Packs the framebuffer into a byte buffer (RGBA order).
    /// Intended for image export (e.g. PNG encoding).
    pub fn as_u8_buffer(&self) -> Vec<u8>{
        self.cur_buffer()
        .iter()
        .flat_map(|c| [c.r, c.g, c.b, c.a])
        .collect()
    }
    /// Fills the entire framebuffer with a single color.
    /// This is an explicit operation and overwrites all pixels.
    pub fn clear(&mut self, color: Color){
        for pixel in &mut self.pixels_buffer{
            *pixel = color;
        }
        self.background_color = color;
    }
    /// Writes a single pixel at the given coordinates.
    /// Out-of-bounds writes are safely ignored.
    pub fn put_pixel(&mut self, x:u32, y:u32, color: Color){
        if x>= self.width || y>= self.height {
            return;
        }

        let index = (y*self.width + x) as usize;
        self.pixels_buffer[index] = color;
    }
    /// Returns a mutable reference to a pixel at the given coordinates.
    /// Used by algorithms that need read-modify-write semantics.
    pub fn return_pixel(&mut self, x:u32, y:u32) -> Option<&mut Color>{
            if x>= self.width || y>= self.height {
            return None;
        } else {
            let index = (y*self.width + x) as usize;
            let col_ref = &mut self.pixels_buffer[index];
            return Some(col_ref);
        }
    }
    /// Exposes the framebuffer as a read-only slice.
    /// Intended for inspection, conversion, or presentation.
    pub fn cur_buffer(&self) -> &[Color]{
        &self.pixels_buffer
    }
    /// Internal mutable access to the pixel buffer.
    /// Not exposed publicly to limit uncontrolled writes.
    fn cur_buffer_as_mut(&mut self) -> &mut [Color]{
        self.pixels_buffer.as_mut_slice()
    }
    /// Replaces the entire pixel buffer with external data.
    /// The input slice must match the framebuffer size.
    pub fn update_buffer(&mut self, buffer: &[Color]){
        self.cur_buffer_as_mut().copy_from_slice(buffer);
    }
    /// Updates a pixel directly by linear index.
    /// Intended for internal compositing and overlay operations.
    pub fn update_color(&mut self, col: Color, index: usize){
        self.cur_buffer_as_mut()[index] = col;
    }
}

#[derive (Debug, Clone, Copy, PartialEq, Eq)]
/// Represents a raw RGBA color used throughout the rendering pipeline.
/// 
/// This type is intentionally minimal and value-based:
/// - No color space conversion
/// - No gamma correction
/// - No blending logic(handled by the window api)
///
/// All color interpretation is deferred to rasterization or framebuffer composition.
pub struct Color{
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color{
    /// Common predefined colors for testing, debugging, and simple drawing.
    pub const RED: Color   = Color { r: 255, g: 0,   b: 0,   a: 255 };
    pub const GREEN: Color = Color { r: 0,   g: 255, b: 0,   a: 255 };
    pub const BLUE: Color  = Color { r: 0,   g: 0,   b: 255, a: 255 };
    pub const BLACK: Color = Color { r: 0,   g: 0,   b: 0,   a: 255 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };
    /// Fully transparent color and commonly used to clear overlay layers or represent absence of color.
    pub const ZERO: Color  = Color { r:0,    g:0,    b:0,    a:0    };
    pub fn new(r:u8,g:u8,b:u8,a:u8) -> Self{
        Self {r,g,b,a}
    }
}
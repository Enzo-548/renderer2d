#[derive (Debug, Clone, Copy)]
pub struct Color{
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color{
    pub const RED: Color   = Color { r: 255, g: 0,   b: 0,   a: 255 };
    pub const GREEN: Color = Color { r: 0,   g: 255, b: 0,   a: 255 };
    pub const BLUE: Color  = Color { r: 0,   g: 0,   b: 255, a: 255 };
    pub const BLACK: Color = Color { r: 0,   g: 0,   b: 0,   a: 255 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };

    pub fn new(r:u8,g:u8,b:u8,a:u8) -> Self{
        Self {r,g,b,a}
    }
    pub fn zero() -> Self{
        Self{r:0,g:0,b:0,a:0}
    }
    pub fn full() -> Self{
        Self{r:255,g:255,b:255,a:255}
    }
    pub fn red() -> Self{
        Color::RED
    }
    pub fn green() -> Self{
        Color::GREEN
    }pub fn blue() -> Self{
        Color::BLUE
    }pub fn white() -> Self{
        Color::WHITE
    }pub fn black() -> Self{
        Color::BLACK
    }
}
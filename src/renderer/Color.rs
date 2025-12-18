#[derive (Debug, Clone, Copy)]
pub struct Color{
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color{
    pub fn new(r:u8,g:u8,b:u8,a:u8) -> Self{
        Self {r,g,b,a}
    }
    pub fn zero() -> Self{
        Self{r:0,g:0,b:0,a:0}
    }
}
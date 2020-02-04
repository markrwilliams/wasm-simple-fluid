use std::convert::From;

pub const WIDTH: usize = 200;
pub const HEIGHT: usize = 200;
pub const N: usize = WIDTH;

pub type Screen = [u32; WIDTH * HEIGHT];

pub fn p(x: usize, y: usize) -> usize {
    y * WIDTH + x
}

#[derive(Debug, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<Color> for u32 {
    fn from(c: Color) -> u32 {
        (c.a as u32) << 24 | (c.b as u32) << 16 | (c.g as u32) << 8 | (c.r as u32)
    }
}

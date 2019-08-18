use std::ops;
use std::ops::AddAssign;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color {r: (r / 255) as f32, g: (g / 255) as f32, b: (b / 255) as f32}
    }
}


impl ops::Div<f32> for Color {
    type Output = Color;

    fn div(self, val: f32) -> Color {
        Color {r: self.r / val, g :self.g / val, b: self.b / val}
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {r: self.r * other.r, g :self.g * other.g, b: self.b * other.b}
    }
}

impl ops::Mul<f32> for Color {
    type Output = Color;

    fn mul(self, val: f32) -> Color {
        Color {r: self.r * val, g :self.g * val, b: self.b * val}
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        };
    }
}

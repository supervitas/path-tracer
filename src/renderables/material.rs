use crate::math::color::Color;

pub struct Material {
    pub albedo: Color,
    pub opacity: f32,
}

impl Material {
    pub fn new(albedo: Color, opacity: f32) -> Self {
        Material {albedo, opacity}
    }
}

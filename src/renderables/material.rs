use crate::math::color::Color;

pub struct Material {
    albedo: Color,
    pub opacity: f32,
    pub roughness: f32,
    pub metalness: f32

}

impl Material {
    pub fn new(albedo: Color) -> Self {
        Material {
            albedo,
            opacity: 1.,
            roughness: 0.5,
            metalness: 0.5,
        }
    }

    pub fn get_albedo(&self) -> &Color {
        &self.albedo
    }

}

use crate::math::vec3::Vector3;
use crate::math::color::Color;

#[derive(Clone, Debug)]
pub struct Light {
    pub color: Color,
    pub intensity: f32,
    pub position: Vector3<f32>
}

impl Light {
    pub fn new(color: Color, intensity: f32, position: Vector3<f32>) -> Self {
        Light {
            color,
            intensity,
            position,
        }
    }
}

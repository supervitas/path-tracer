use crate::math::vec3::Vector3;

pub struct Light {
    pub color: [u8; 3],
    pub intensity: f32,
    pub position: Vector3<f32>
}

impl Light {
    pub fn new(color: [u8; 3], intensity: f32, position: Vector3<f32>) -> Self {
        Light {
            color,
            intensity,
            position,
        }
    }
}

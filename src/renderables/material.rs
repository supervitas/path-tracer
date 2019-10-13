use crate::math::color::Color;

#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub diffuse_color: Color,
    pub shininess: f32,
    pub opacity: f32,
}

impl Material {
    pub fn new() -> Self {
        Material {
            diffuse_color: Color::new(255.,255.,255.),
            opacity: 1.,
            shininess: 1.
        }
    }
}

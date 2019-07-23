pub struct Material {
    pub color: [u8; 3],
    pub opacity: f32,
}

impl Material {
    pub fn new(color: [u8; 3], opacity: f32) -> Self {
        Material {color, opacity}
    }
}

use crate::math::vec3::Vector3;

pub struct Camera {
    position: Vector3<f32>,
    direction: Vector3<f32>,
    target: Vector3<f32>,
    fov: f32,
    z_near: f32,
    z_far: f32,
}

impl Camera {
    pub fn new(fov: f32, z_near: f32, z_far: f32, position: Vector3<f32>, direction: Vector3<f32>) -> Self {
        Camera {
            fov,
            z_far,
            z_near,
            position,
            target: Vector3::new(0.,0.,0.),
            direction
        }
    }

    pub fn set_position(&mut self, position: Vector3<f32>) {
        self.position = position;
    }

    pub fn position(&self) -> &Vector3<f32> {
        &self.position
    }
    pub fn target(&self) -> &Vector3<f32> {
        &self.target
    }
}

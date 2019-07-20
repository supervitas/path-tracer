use crate::math::vec3::Vector3;
use crate::primitives::renderable::Renderable;

pub struct Camera {
    position: Vector3<f64>,
    direction: Vector3<f64>,
    fov: f32,
    z_near: f32,
    z_far: f32,
}

impl Camera {
    pub fn new(fov: f32, z_near: f32, z_far: f32, position: Vector3<f64>, direction: Vector3<f64>) -> Self {
        Camera {
            fov,
            z_far,
            z_near,
            position,
            direction
        }
    }

    pub fn position(&self) -> &Vector3<f64> {
        &self.position
    }

}

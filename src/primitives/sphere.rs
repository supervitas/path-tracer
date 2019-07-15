use crate::primitives::renderable::Renderable;
use crate::math::ray::Ray;
use crate::math;

pub struct Sphere {
    radius: f32,
    position: math::vec3::Vector3<f64>
}

impl Renderable for Sphere {
    fn new() -> Self {
        Sphere {
            radius: 0.5,
            position: math::vec3::Vector3{x:0., y: 0., z: -5.}
        }
    }

    fn intersects(&self, ray: &Ray) -> bool {
        return false;
    }
}

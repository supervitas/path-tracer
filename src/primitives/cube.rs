use crate::math;
use crate::primitives::renderable::Renderable;
use crate::math::ray::Ray;

pub struct Cube {
    width: f64,
    height: f64,
    depth: f64,
    position: math::vec3::Vector3<f64>
}

impl Renderable for Cube {
    fn new() -> Self {
        Cube {
            width: 32.,
            height: 10.,
            depth: 15.,
            position: math::vec3::Vector3{x:0., y: 0., z: -25.}
        }
    }

    fn intersects(&self, ray: &Ray) -> bool {
        return false;
    }
}

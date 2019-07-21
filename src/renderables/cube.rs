use crate::math;
use crate::renderables::renderable::Renderable;
use crate::math::ray::Ray;
use crate::math::vec3::Vector3;

pub struct Cube {
    width: f64,
    height: f64,
    depth: f64,
    position: math::vec3::Vector3<f64>
}

impl Cube {
    pub fn new(width: f64, height: f64, depth: f64, position: Vector3<f64>) -> Self {
        Cube {
            width,
            height,
            depth,
            position
        }
    }
}

impl Renderable for Cube {
    fn intersects(&self, ray: &Ray) -> bool {
        return false;
    }
}

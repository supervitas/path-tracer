use crate::math;
use crate::renderables::renderable::Renderable;
use crate::math::ray::Ray;
use crate::math::vec3::Vector3;
use crate::renderables::material::Material;

pub struct Cube {
    width: f64,
    height: f64,
    depth: f64,
    material: Material,
    position: math::vec3::Vector3<f64>
}

impl Cube {
    pub fn new(width: f64, height: f64, depth: f64, position: Vector3<f64>, material: Material) -> Self {
        Cube {
            width,
            height,
            depth,
            position,
            material
        }
    }
}

impl Renderable for Cube {
    fn intersects(&self, ray: &Ray) -> bool {
        return false;
    }

    fn get_material(&self) -> &Material {
        &self.material
    }
}

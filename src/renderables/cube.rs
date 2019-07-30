use crate::math;
use crate::renderables::renderable::Renderable;
use crate::math::ray::Ray;
use crate::math::vec3::Vector3;
use crate::renderables::material::Material;

pub struct Cube {
    width: f32,
    height: f32,
    depth: f32,
    material: Material,
    position: math::vec3::Vector3<f32>
}

impl Cube {
    pub fn new(width: f32, height: f32, depth: f32, position: Vector3<f32>, material: Material) -> Self {
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
    fn intersects(&self, ray: &Ray) -> Option<f32> {
        None
    }

    fn get_material(&self) -> &Material {
        &self.material
    }
}

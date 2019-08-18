use crate::math::ray::Ray;
use crate::renderables::material::Material;
use crate::math::vec3::Vector3;

pub trait Renderable {
    fn intersects(&self, ray: &Ray) -> Option<f32>;
    fn get_material(&self) -> Option<&Material>;
    fn get_normal(&self, hit: &Vector3<f32>) -> Vector3<f32>;
}

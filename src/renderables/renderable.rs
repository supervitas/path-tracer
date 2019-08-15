use crate::math::ray::Ray;
use crate::renderables::material::Material;

pub trait Renderable {
    fn intersects(&mut self, ray: &Ray) -> Option<f32>;
    fn get_material(&self) -> Option<&Material>;
}

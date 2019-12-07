use crate::math::ray::Ray;
use crate::renderables::material::Material;
use crate::math::vec3::Vector3;

pub struct IntersectionData {
    pub distance: f32,
    pub normal: Vector3<f32>,
}

pub trait Renderable : Sync{
    fn intersects(&self, ray: &Ray) -> Option<IntersectionData>;
    fn get_material(&self) -> &Material;
}

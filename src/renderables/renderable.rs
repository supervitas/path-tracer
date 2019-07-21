use crate::math::ray::Ray;

pub trait Renderable {
    fn intersects(&self, ray: &Ray) -> bool;
}

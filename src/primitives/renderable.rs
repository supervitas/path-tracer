use crate::math::ray::Ray;

pub trait Renderable {
    fn new() -> Self where Self: Sized;
    fn intersects(&self, ray: &Ray) -> bool;
}

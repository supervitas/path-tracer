use crate::renderables::renderable::Renderable;
use crate::math::ray::Ray;
use crate::math;
use crate::math::vec3::Vector3;
use crate::renderables::material::Material;

pub struct Sphere {
    radius: f64,
    material: Material,
    position: Vector3<f64>
}

impl Sphere {
    pub fn new(radius: f64, position: Vector3<f64>, material: Material) -> Self {
        Sphere {
            radius,
            material,
            position
        }
    }
}

impl Renderable for Sphere {
    fn intersects(&self, ray: &Ray) -> bool {
        let l = &self.position - &ray.origin;
        let adj2 = l.dot(&ray.direction);
        let d2 = l.dot(&l) - (adj2 * adj2);
        d2 < (self.radius * self.radius)
    }

    fn get_material(&self) -> &Material {
        &self.material
    }
}

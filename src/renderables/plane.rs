use crate::renderables::renderable::{Renderable, IntersectionData};
use crate::math::ray::Ray;
use crate::math::vec3::Vector3;
use crate::renderables::material::Material;

pub struct Plane {
    material: Material,
    center: Vector3<f32>,
    normal: Vector3<f32>,
}

impl Plane {
    pub fn new(center: Vector3<f32>, material: Material, normal: Vector3<f32>) -> Self {
        Plane {
            center,
            material,
            normal
        }
    }

    pub fn get_normal(&self) -> Vector3<f32> {
        self.normal
    }
}

impl Renderable for Plane {
    fn intersects(&self, ray: &Ray) -> Option<IntersectionData> {
        let denominator = self.normal.dot(&ray.direction);

        if denominator.abs() > std::f32::EPSILON {
            let difference = &self.center - &ray.origin;
            let t = difference.dot(&self.normal) / denominator;

            if t > std::f32::EPSILON {
                return Some(IntersectionData{
                    distance: t,
                    normal: self.get_normal()
                });
            }
        }

        return None
    }

    fn get_material(&self) -> &Material {
        &self.material
    }
}

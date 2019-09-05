use crate::renderables::renderable::Renderable;
use crate::math::ray::Ray;
use crate::math::vec3::Vector3;
use crate::renderables::material::Material;

pub struct Plane {
    material: Option<Material>,
    center: Vector3<f32>,
    normal: Vector3<f32>,
}

impl Plane {
    pub fn new(center: Vector3<f32>, material: Option<Material>, normal: Vector3<f32>) -> Self {
        Plane {
            center,
            material,
            normal
        }
    }
}

impl Renderable for Plane {
    fn intersects(&self, ray: &Ray) -> Option<f32> {
        let denominator = self.normal.dot(&ray.direction);

        if denominator.abs() > std::f32::EPSILON {
            let difference = &self.center - &ray.origin;
            let t = difference.dot(&self.normal) / denominator;

            if t > std::f32::EPSILON {
                return Some(t);
            }
        }

        return None
    }

    fn get_material(&self) -> Option<&Material> {
        match &self.material {
            Some(material) => Some(material),
            None => None,
        }
    }

    fn get_normal(&self, _: &Vector3<f32>) -> Vector3<f32> {
        self.normal
    }
}

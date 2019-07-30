use crate::renderables::renderable::Renderable;
use crate::math::ray::Ray;
use crate::math;
use crate::math::vec3::Vector3;
use crate::renderables::material::Material;

pub struct Sphere {
    radius: f32,
    material: Material,
    position: Vector3<f32>
}

impl Sphere {
    pub fn new(radius: f32, position: Vector3<f32>, material: Material) -> Self {
        Sphere {
            radius,
            material,
            position
        }
    }
}

impl Renderable for Sphere {
    fn intersects(&self, ray: &Ray) -> Option<f32> {
        let from_center_to_camera = &self.position - &ray.origin;
        let projection_length = from_center_to_camera.dot(&ray.direction);

        if projection_length < 0. {
            return None;
        }

        let from_center_to_point = from_center_to_camera.dot(&from_center_to_camera) - (projection_length * projection_length);

        let quad_radius = self.radius * self.radius;

        if from_center_to_point > quad_radius {
            return None;
        }

        let from_center_to_sphere_end = f32::sqrt(quad_radius - from_center_to_point);

        let first_intersection_distance = projection_length - from_center_to_sphere_end;
        let second_intersection_distance = projection_length + from_center_to_sphere_end;

        if first_intersection_distance < second_intersection_distance {
           return Some(first_intersection_distance);
        }

        return Some(second_intersection_distance);
    }

    fn get_material(&self) -> &Material {
        &self.material
    }
}

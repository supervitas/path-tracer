use crate::math::vec3::Vector3;
use crate::renderables::triangle::Triangle;
use crate::math::ray::Ray;
use std::mem;

pub struct BBox {
    pub min: Vector3<f32>,
    pub max: Vector3<f32>,
}

impl BBox {
    pub fn new(min: Vector3<f32>, max: Vector3<f32>) -> Self {
        BBox {
            min,
            max
        }
    }

    pub fn ray_intersect_box(&self, ray: &Ray) -> bool {
        let mut dir_inv = ray.direction.clone();
        dir_inv.x = 1. / dir_inv.x;
        dir_inv.y = 1. / dir_inv.y;
        dir_inv.z = 1. / dir_inv.z;


        let mut tmin = (self.min.x - ray.origin.x) * dir_inv.x;
        let mut tmax = (self.max.x - ray.origin.x) * dir_inv.x;

        if tmin > tmax {
            mem::swap(&mut tmin, &mut tmax);
        }

        let mut tymin = (self.min.y - ray.origin.y) * dir_inv.y;
        let mut tymax = (self.max.y - ray.origin.y) * dir_inv.y;

        if tymin > tymax {
            mem::swap(&mut tymin, &mut tymax);
        }

        if tmin > tymax || tymin > tmax {
            return false;
        }

        tmin = f32::max(tmin, tymin);
        tmax = f32::min(tymax, tmax);

        let mut tzmin = (self.min.z - ray.origin.z) * dir_inv.z;
        let mut tzmax = (self.max.z - ray.origin.z) * dir_inv.z;

        if tzmin > tzmax {
            mem::swap(&mut tzmin, &mut tzmax);
        }

        if tmin > tzmax || tzmin > tmax {
            return false;
        }

        true
    }

    pub fn new_from_triangles(triangles: &Vec<Triangle>) -> Self {
        let mut min = Vector3::new(0.,0.,0.);
        let mut max = Vector3::new(0.,0.,0.);

        for triangle in triangles {
            for vertex in &triangle.get_vertices() {
                max.x = f32::max(max.x, vertex.x);
                min.x = f32::min(min.x, vertex.x);

                max.y = f32::max(max.y, vertex.y);
                min.y = f32::min(min.y, vertex.y);

                max.z = f32::max(max.z, vertex.z);
                min.z = f32::min(min.z, vertex.z);
            }
        }

        BBox {
            min,
            max
        }
    }
}

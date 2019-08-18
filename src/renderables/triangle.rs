use crate::renderables::renderable::Renderable;
use crate::math::ray::Ray;
use crate::math;
use crate::math::vec3::Vector3;
use crate::renderables::material::Material;

pub struct Triangle {
    v0: Vector3<f32>,
    v1: Vector3<f32>,
    v2: Vector3<f32>,
    edge1: Vector3<f32>,
    edge2: Vector3<f32>,
    normal: Vector3<f32>,
    material: Option<Material>,
}

impl Triangle {
    pub fn new(v0: Vector3<f32>, v1: Vector3<f32>, v2: Vector3<f32>,
               normal: Option<Vector3<f32>>, material: Option<Material>) -> Self {

        let edge1 = &v1.clone() - &v0;
        let edge2 = &v2.clone() - &v0;

        let normal = normal.unwrap_or_else(|| {
            let mut edge1 = edge1.clone();
            edge1.cross(&edge2).normalize();
            edge1
        });

        Triangle {
            v0,
            v1,
            v2,
            edge1,
            edge2,
            normal,
            material
        }
    }

    pub fn get_triangle_normal(&self) -> &Vector3<f32> {
        &self.normal
    }

    pub fn get_vertices(&self) -> [&Vector3<f32>; 3] {
        [&self.v0, &self.v1, &self.v2]
    }
}

impl Renderable for Triangle {
    fn intersects(&self, ray: &Ray) -> Option<f32> {
        let mut pvec = ray.direction.clone();
        pvec.cross(&self.edge2);

        let det = self.edge1.dot(&pvec);
        if det < std::f32::EPSILON && det > -std::f32::EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;
        let tvec = &ray.origin - &self.v0;

        let u = tvec.dot(&pvec) * inv_det;
        if u < 0. || u > 1. {
            return None;
        }

        let mut qvec = tvec.clone();
        qvec.cross(&self.edge1);

        let v = ray.direction.dot(&qvec) * inv_det;
        if v < 0. || u + v > 1. {
            return None;
        }

        Some(self.edge2.dot(&qvec) * inv_det)
    }

    fn get_material(&self) -> Option<&Material> {
        match &self.material {
            Some(material) => Some(material),
            None => None,
        }
    }

    fn get_normal(&self, hit: &Vector3<f32>) -> Vector3<f32> {
        self.normal.clone()
    }
}

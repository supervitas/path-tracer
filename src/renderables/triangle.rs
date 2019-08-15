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

    pvec: Vector3<f32>,
    tvec: Vector3<f32>,
    qvec: Vector3<f32>
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
            material,
            pvec: Vector3::new(0.,0.,0.),
            tvec: Vector3::new(0.,0.,0.),
            qvec: Vector3::new(0.,0.,0.)
        }
    }

    pub fn get_normal(&self) -> &Vector3<f32> {
        &self.normal
    }
}

impl Renderable for Triangle {
    fn intersects(&mut self, ray: &Ray) -> Option<f32> {
        self.pvec.copy(&ray.direction);
        self.pvec.cross(&self.edge2);

        let det = self.edge1.dot(&self.pvec);
        if det < 1e-8 && det > -1e-8 {
            return None;
        }

        let inv_det = 1.0 / det;
        self.tvec = ray.origin;
        self.tvec = &self.tvec - &self.v0;

        let u = self.tvec.dot(&self.pvec) * inv_det;
        if u < 0. || u > 1. {
            return None;
        }

        self.qvec = self.tvec;
        self.qvec.cross(&self.edge1);

        let v = ray.direction.dot(&self.qvec) * inv_det;
        if v < 0. || u + v > 1. {
            return None;
        }

        Some(self.edge2.dot(&self.qvec) * inv_det)
    }

    fn get_material(&self) -> Option<&Material> {
        match &self.material {
            Some(material) => Some(material),
            None => None,
        }
    }
}

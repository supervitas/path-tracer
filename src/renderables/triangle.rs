use crate::math::ray::Ray;
use crate::math::vec3::Vector3;

pub struct Triangle {
    v0: Vector3<f32>,
    v1: Vector3<f32>,
    v2: Vector3<f32>,
    edge1: Vector3<f32>,
    edge2: Vector3<f32>,
    normal: Vector3<f32>,
}

const EPSILON: f32 = 0.00001;

impl Triangle {
    pub fn new(v0: Vector3<f32>, v1: Vector3<f32>, v2: Vector3<f32>) -> Self {
        let edge1 = &v1.clone() - &v0;
        let edge2 = &v2.clone() - &v0;

        let normal = {
            let mut edge1 = edge1.clone();
            edge1.cross(&edge2).normalize();
            edge1
        };

        Triangle {
            v0,
            v1,
            v2,
            edge1,
            edge2,
            normal,
        }
    }

    pub fn get_vertices(&self) -> [&Vector3<f32>; 3] {
        [&self.v0, &self.v1, &self.v2]
    }

    pub fn get_normal(&self) -> Vector3<f32> {
        self.normal.clone()
    }


    pub fn intersects(&self, ray: &Ray) -> Option<f32> {
        let mut pvec = ray.direction.clone();

        let h = pvec.cross(&self.edge2);
        let a = self.edge1.dot(h);

        if f32::abs(a) < EPSILON {
            return None;
        }

        let f = 1.0 / a;
        let s = &ray.origin - &(self.v0);
        let u = f * (s.dot(h));

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let mut st = s.clone();
        let q = st.cross(&self.edge1);
        let v = f * ray.direction.dot(q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * self.edge2.dot(q);

        if t > EPSILON {
            return Some(t);
        }

        return None;
    }
}

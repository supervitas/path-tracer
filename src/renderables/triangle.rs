use crate::renderables::renderable::Renderable;
use crate::math::ray::Ray;
use crate::math;
use crate::math::vec3::Vector3;
use crate::renderables::material::Material;
use sdl2::mouse::SystemCursor::No;

pub struct Triangle {
    a: Vector3<f32>,
    b: Vector3<f32>,
    c: Vector3<f32>,
    material: Material,
    position: Vector3<f32>
}

impl Triangle {
//    pub fn new(radius: f32, position: Vector3<f32>, material: Material) -> Self {
//        Triangle {
//            radius,
//            material,
//            position
//        }
//    }
}

impl Renderable for Triangle {
    fn intersects(&self, ray: &Ray) -> Option<f32> {
        None
    }

    fn get_material(&self) -> &Material {
        &self.material
    }
}

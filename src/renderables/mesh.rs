use crate::renderables::renderable::Renderable;
use crate::math::ray::Ray;
use crate::math;
use crate::math::vec3::Vector3;
use crate::renderables::material::Material;
use std::ops::Deref;
use crate::renderables::triangle::Triangle;

pub struct Mesh {
    material: Option<Material>,
    triangles: Vec<Triangle>,
    name: String,
}

impl Mesh {
    pub fn new(material: Option<Material>, triangles: Vec<Triangle>, name: String) -> Self {
        Mesh {
            name,
            triangles,
            material
        }
    }

}

impl Renderable for Mesh {
    fn intersects(&self, ray: &Ray) -> Option<f32> {
        None
    }

    fn get_material(&self) -> Option<&Material> {
        match &self.material {
            Some(material) => Some(material),
            None => None,
        }
    }
}
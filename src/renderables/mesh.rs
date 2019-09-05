use crate::renderables::renderable::Renderable;
use crate::math::ray::Ray;
use crate::math::vec3::Vector3;
use crate::renderables::material::Material;
use crate::renderables::triangle::Triangle;
use crate::math::bbox::BBox;

pub struct Mesh {
    material: Option<Material>,
    triangles: Vec<Triangle>,
    bbox: BBox,
    name: String,
}

impl Mesh {
    pub fn new(material: Option<Material>, triangles: Vec<Triangle>, name: String) -> Self {
        let bbox = BBox::new_from_triangles(&triangles);

        Mesh {
            name,
            triangles,
            material,
            bbox
        }
    }
}

impl Renderable for Mesh {
    fn intersects(&self, ray: &Ray) -> Option<f32> {
        if !self.bbox.ray_intersect_box(ray) {
            return None;
        }

        let mut min_distance = std::f32::MAX;

        let mut is_intersecting = false;

        for triangle in &self.triangles {
            match triangle.intersects(ray) {
                Some(distance) => {
                    if min_distance > distance {
                        is_intersecting = true;
                        min_distance = distance;
                    }
                },
                _ => {},
            };
        }

        match is_intersecting {
            false => {
                return None;
            },
            _ => {Some(min_distance)}
        }
    }

    fn get_material(&self) -> Option<&Material> {
        match &self.material {
            Some(material) => Some(material),
            None => None,
        }
    }

    fn get_normal(&self, hit: &Vector3<f32>) -> Vector3<f32> {
        Vector3::new(0.,0.,0.) // todo
    }
}

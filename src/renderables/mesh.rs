use crate::renderables::renderable::{Renderable, IntersectionData};
use crate::math::ray::Ray;
use crate::renderables::material::Material;
use crate::renderables::triangle::Triangle;
use crate::math::bbox::BBox;

pub struct Mesh {
    material: Material,
    triangles: Vec<Triangle>,
    bbox: BBox,
    name: String,
}

impl Mesh {
    pub fn new(material: Material, triangles: Vec<Triangle>, name: String) -> Self {
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
    fn intersects(&self, ray: &Ray) -> Option<IntersectionData> {
        if !self.bbox.ray_intersect_box(ray) {
            return None;
        }

        let mut min_distance = std::f32::MAX;
        let mut intersected_triangle: Option<&Triangle> = None;

        for triangle in &self.triangles {
            match triangle.intersects(ray) {
                Some(distance) => {
                    if min_distance > distance {
                        intersected_triangle = Some(triangle);
                        min_distance = distance;
                    }
                },
                _ => {},
            };
        }

        match intersected_triangle {
             Some(triangle) => {
                 return Some(IntersectionData {
                     distance: min_distance,
                     normal: triangle.get_normal()
                 })
             }
            _ => {
                return None;
            }
        }
    }

    fn get_material(&self) -> &Material {
        &self.material
    }
}

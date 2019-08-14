use std::path::Path;
use tobj;
use crate::renderables::mesh::Mesh;
use std::collections::HashMap;
use crate::renderables::material::Material;
use crate::math::vec3::Vector3;
use crate::renderables::triangle::Triangle;


pub fn load_obj(path: &str) -> Vec<Mesh> {
    let cornell_box = tobj::load_obj(&Path::new(path));
    assert!(cornell_box.is_ok());

    let (models, materials) = cornell_box.unwrap();
    let mut meshes: Vec<Mesh> = Vec::new();

    for (i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;

        let mat = &materials[mesh.material_id.unwrap()];
        let diffuse_color = [
            (mat.diffuse[0] * 255.) as u8,
            (mat.diffuse[1] * 255.) as u8,
            (mat.diffuse[2] * 255.) as u8
        ];

        let material = Material::new(diffuse_color, 1.0);

        let mut triangles: Vec<Triangle> = Vec::new();

        for f in 0..mesh.indices.len() / 3 {
            let index1 = mesh.indices[3 * f] as usize;
            let index2 = mesh.indices[3 * f + 1] as usize;
            let index3 = mesh.indices[3 * f + 2] as usize;

            let v0 = Vector3::new(
                mesh.positions[3 * index1].clone(),
                mesh.positions[3 * index1 + 1].clone(),
                mesh.positions[3 * index1 + 2].clone()
            );

            let v1 = Vector3::new(
                mesh.positions[3 * index2].clone(),
                mesh.positions[3 * index2 + 1].clone(),
                mesh.positions[3 * index2 + 2].clone()
            );

            let v2 = Vector3::new(
                mesh.positions[3 * index3].clone(),
                mesh.positions[3 * index3 + 1].clone(),
                mesh.positions[3 * index3 + 2].clone()
            );

            let triangle = Triangle::new(v0, v1, v2, None, None);
            triangles.push(triangle);
        }

        meshes.push(Mesh::new(Some(material), triangles, m.name.clone()));
    }
    meshes
}

use std::path::Path;
use tobj;
use crate::renderables::mesh::Mesh;
use crate::renderables::material::Material;
use crate::math::vec3::Vector3;
use crate::renderables::triangle::Triangle;
use crate::math::color::Color;


pub fn load_obj(path: &str) -> Vec<Mesh> {
    let model = tobj::load_obj(&Path::new(path));
    assert!(model.is_ok());

    let (models, materials) = model.unwrap();
    let mut meshes: Vec<Mesh> = Vec::new();

    for (_i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;

        let mat = &materials[mesh.material_id.unwrap()];
        let diffuse_color = Color::new(
            mat.diffuse[0] * 255.,
            mat.diffuse[1] * 255.,
            mat.diffuse[2] * 255.
        );



        let mut material = Material::new();
        material.diffuse_color = diffuse_color;
        material.shininess = mat.shininess;
        material.opacity = mat.dissolve;

        let mut triangles: Vec<Triangle> = Vec::new();

        for f in 0..mesh.indices.len() / 3 {
            let index1 = mesh.indices[3 * f] as usize;
            let index2 = mesh.indices[3 * f + 1] as usize;
            let index3 = mesh.indices[3 * f + 2] as usize;

            let v0 = Vector3::new(
                mesh.positions[3 * index1],
                mesh.positions[3 * index1 + 1],
                mesh.positions[3 * index1 + 2]
            );

            let v1 = Vector3::new(
                mesh.positions[3 * index2],
                mesh.positions[3 * index2 + 1],
                mesh.positions[3 * index2 + 2]
            );

            let v2 = Vector3::new(
                mesh.positions[3 * index3],
                mesh.positions[3 * index3 + 1],
                mesh.positions[3 * index3 + 2]
            );

            let triangle = Triangle::new(v0, v1, v2);
            triangles.push(triangle);
        }

        meshes.push(Mesh::new(material, triangles, m.name.clone()));
    }

    meshes
}

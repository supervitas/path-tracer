use crate::math::vec3::Vector3;
use crate::renderables::renderable::Renderable;
use crate::renderer::scene::Scene;
use crate::renderer::camera::Camera;
use crate::math::ray::Ray;
use std::f32;


pub struct Renderer {
    width: u32,
    height: u32,
    image: Vec<u8>
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        let image: Vec<u8> = vec![0; ((width * height) * 3) as usize];

        Renderer {
            width,
            height,
            image
        }
    }

    fn check_intersections(&self, camera: &Camera, scene: &Scene, x: u32, y: u32) -> [u8; 3] {

        let fov_adjustment = (45.0_f32.to_radians() / 2.0).tan();
        let aspect_ratio = (self.width as f32) / (self.height as f32);
        let dir_x = ((((x as f32 + 0.5) / self.width as f32) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let dir_y = (1.0 - ((y as f32 + 0.5) / self.height as f32) * 2.0) * fov_adjustment;


        let mut direction = Vector3::new(dir_x, dir_y, -1.0);
        direction.normalize();

        let ray = Ray {
            origin: camera.position().clone(),
            direction
        };

        let mut result_color = scene.get_background().clone();

        let mut near = f32::INFINITY;
        for renderable in scene.get_renderables() {
            match renderable.intersects(&ray) {
                Some(point) => {
                    if point < near {
                        near = point;
                        match renderable.get_material() {
                            Some(material) => {
                                result_color = material.color;
                            },
                            _ => {},
                        };
                    }
                },
                None => {},
            }
        }

        return result_color
    }

    pub fn render(&mut self, scene: &Scene, camera: &Camera) -> &Vec<u8> {
        for h in 0..self.height {
            for w in 0..self.width {
                let color = self.check_intersections(&camera, &scene, w, h);
                let offset = (h * self.width * 3 + w * 3) as usize;

                self.image[offset] = color[0];
                self.image[offset + 1] = color[1];
                self.image[offset + 2] = color[2];
            }
        }

        return &self.image;
    }
}

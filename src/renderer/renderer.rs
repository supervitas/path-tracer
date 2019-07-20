use crate::math::vec3::Vector3;
use crate::primitives::renderable::Renderable;
use crate::renderer::scene::Scene;
use crate::renderer::camera::Camera;
use rand::Rng;
use crate::math::ray::Ray;

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

    fn cast_ray(&self, camera: &Camera, x: usize, y: usize) -> [u8; 3] {
        let dir_x = ((x as f64 + 0.5) / self.width as f64) * 2.0 - 1.0;
        let dir_y = 1.0 - ((y as f64 + 0.5) / self.height as f64) * 2.0;

        let ray = Ray {
            origin: camera.position().clone(),
            direction: Vector3 {
                x: dir_x,
                y: dir_y,
                z: -1.0,
            }
        };

        return [21,33,44];
    }

    pub fn render(&mut self, scene: &Scene, camera: &Camera) -> &Vec<u8> {
        let mut rng = rand::thread_rng();

        let mut i = 0;
        for w in 0..self.width {
            for h in 0..self.height {
//                self.image[i] = rng.gen_range(1, 255);
//                self.image[i + 1] = rng.gen_range(1, 255);
//                self.image[i + 2] = rng.gen_range(1, 255);

                i+= 3;
            }
        }

        return &self.image;
    }
}

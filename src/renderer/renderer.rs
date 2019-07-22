use crate::math::vec3::Vector3;
use crate::renderables::renderable::Renderable;
use crate::renderer::scene::Scene;
use crate::renderer::camera::Camera;
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

    fn check_intersections(&self, camera: &Camera, scene: &Scene, x: u32, y: u32) -> Vector3<f64> {

        let fov_adjustment = (45.0_f64.to_radians() / 2.0).tan();
        let aspect_ratio = (self.width as f64) / (self.height as f64);
        let dir_x = ((((x as f64 + 0.5) / self.width as f64) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let dir_y = (1.0 - ((y as f64 + 0.5) / self.height as f64) * 2.0) * fov_adjustment;


        let mut direction = Vector3::new(dir_x, dir_y, -1.0);
        direction.normalize();

        let mut ray = Ray {
            origin: camera.position().clone(),
            direction
        };

        let mut result_color = scene.get_background().clone();

        for renderable in scene.get_renderables() {
            if renderable.intersects(&ray) {
                result_color.set(0., 250., 0.);
            }
        }

        return result_color
    }

    pub fn render(&mut self, scene: &Scene, camera: &Camera) -> &Vec<u8> {
        for h in 0..self.height {
            for w in 0..self.width {
                let color = self.check_intersections(&camera, &scene, w, h);
                let offset = (h * self.width * 3 + w * 3) as usize;

                self.image[offset] = color.x as u8;
                self.image[offset + 1] = color.y as u8;
                self.image[offset + 2] = color.z as u8;
            }
        }

        return &self.image;
    }
}

use crate::math::vec3::Vector3;
use crate::renderables::renderable::Renderable;
use crate::renderer::scene::Scene;
use crate::renderer::camera::Camera;
use crate::math::ray::Ray;
use std::f32;
use crate::renderer::light::Light;
use crate::math::color::Color;

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

    fn calculate_light(&self, ray: &Ray, hit_distance: f32, renderable: &Box<dyn Renderable>, lights: &Vec<Light>) -> Color {
        let mut color = Color::new(0,0,0);

        let mut albedo = Color::new(0,0,0);

        match renderable.get_material() {
            Some(material) => {
                albedo = material.albedo;
            },
            _ => {},
        };

        let hit_point = &ray.origin + &(ray.direction * hit_distance);
        let renderable_normal = renderable.get_normal(&hit_point);

        for light in lights {
            let direction_to_light = -light.direction;
            let light_power = f32::max(renderable_normal.dot(&direction_to_light), 0.0) * light.intensity;

            let light_reflected = albedo / std::f32::consts::PI;

            color += albedo * light.color * light_power * light_reflected;

        }

        color
    }

    fn check_intersections(&self, camera: &Camera, scene: &mut Scene, x: u32, y: u32) -> Color {
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

        let mut pixel_color = scene.get_background().clone();
        let mut lights = scene.get_lights();

        let mut near = f32::INFINITY;
        for renderable in scene.get_renderables() {
            match renderable.intersects(&ray) {
                Some(point) => {
                    if point > near {continue};

                    near = point;

                    pixel_color = self.calculate_light(&ray, near, &renderable, &lights);

                },
                None => {},
            }
        }

        pixel_color
    }

    pub fn render(&mut self, scene: &mut Scene, camera: &Camera) -> &Vec<u8> {
        for h in 0..self.height {
            for w in 0..self.width {
                let color = self.check_intersections(&camera, scene, w, h);
                let offset = (h * self.width * 3 + w * 3) as usize;

                self.image[offset] = (color.r * 255.) as u8;
                self.image[offset + 1] = (color.g * 255.) as u8;
                self.image[offset + 2] = (color.b * 255.) as u8;
            }
        }

        return &self.image;
    }
}

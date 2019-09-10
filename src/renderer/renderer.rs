use crate::math::vec3::Vector3;
use crate::renderables::renderable::{Renderable, IntersectionData};
use crate::renderer::scene::Scene;
use crate::renderer::camera::Camera;
use crate::math::ray::Ray;
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

    fn calculate_light(&self, ray: &Ray, intersection_data: IntersectionData, renderable: &Box<dyn Renderable>, lights: &Vec<Light>) -> Color {
        let material = renderable.get_material();

        let mut color = Color::new(0.,0.,0.);
        let mut albedo = Color::new(0.,0.,0.);

        albedo = *material.get_albedo();

        let mut roughness = material.roughness;
        let mut metalness = material.metalness;

        let hit_point = &ray.origin + &(ray.direction * intersection_data.distance);
        let renderable_normal = intersection_data.normal;

        for light in lights {
            let mut light_direction = &light.position - &hit_point;
            light_direction.normalize();

            let light_to_normal = f32::max(0., light_direction.dot(&renderable_normal));

            let diffuse = light.intensity * light_to_normal;
            let specular = light.intensity * f32::powf(light_to_normal, 0.5) * metalness;


            color += albedo * (diffuse + specular);
        }

        color
    }

    fn get_camera_ray(&self, camera: &Camera, x: u32, y: u32) -> Ray {
        let fov_adjustment = (45.0_f32.to_radians() / 2.0).tan();
        let aspect_ratio = (self.width as f32) / (self.height as f32);
        let dir_x = ((((x as f32 + 0.5) / self.width as f32) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let dir_y = (1.0 - ((y as f32 + 0.5) / self.height as f32) * 2.0) * fov_adjustment;

        let mut direction = Vector3::new(dir_x, dir_y, -1.0);
        direction.normalize();

        Ray {
            origin: camera.position().clone(),
            direction
        }
    }

    fn check_intersections(&self, ray: Ray, scene: &Scene) -> Color {
        let mut pixel_color = scene.get_background().clone();
        let lights = scene.get_lights();

        let mut near = std::f32::INFINITY;
        let mut intersected_renderable = None;
        let mut result_intersected_data:Option<IntersectionData> = None;

        for renderable in scene.get_renderables() {
            match renderable.intersects(&ray) {
                Some(intersection_data) => {
                    if intersection_data.distance < near {
                        near = intersection_data.distance;
                        result_intersected_data = Some(intersection_data);
                        intersected_renderable = Some(renderable);
                    }
                },
                None => {},
            }
        }

        match intersected_renderable {
            Some(renderable) => {
                pixel_color = self.calculate_light(&ray, result_intersected_data.unwrap(), &renderable, &lights);
            },
            None => {},
        }

        pixel_color
    }

    pub fn render(&mut self, scene: &Scene, camera: &Camera) -> &Vec<u8> {
        for h in 0..self.height {
            for w in 0..self.width {
                let offset = (h * self.width * 3 + w * 3) as usize;
                let camera_ray = self.get_camera_ray(&camera, w, h);
                let color = self.check_intersections(camera_ray, scene).as_u8();

                self.image[offset] = color[0];
                self.image[offset + 1] = color[1];
                self.image[offset + 2] = color[2];
            }
        }

        return &self.image;
    }
}

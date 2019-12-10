use crate::math::vec3::Vector3;
use crate::renderables::renderable::{Renderable, IntersectionData};
use crate::renderer::scene::Scene;
use crate::renderer::camera::Camera;
use crate::renderer::thread_pool::ThreadPool;
use crate::math::ray::Ray;
use crate::renderer::light::Light;
use crate::math::color::Color;
use rand::distributions::{Uniform, Distribution};
use std::sync::Arc;

const RAY_COUNT: usize = 16;

struct RenderScene {
    camera: Camera,
    scene: Scene,
    image: Vec<u8>
}

pub struct Renderer {
    width: u32,
    height: u32,
    last_frame_camera_position: Vector3<f32>,
    thread_pool: ThreadPool,
    render_scene: Arc<RenderScene>
}

impl Renderer {
    pub fn new(width: u32, height: u32, camera: Camera, scene: Scene) -> Self {
        let image: Vec<u8> = vec![0; ((width * height) * 3) as usize];

        Renderer {
            width,
            height,
            thread_pool: ThreadPool::new(),
            last_frame_camera_position: Vector3::new(0.,0.,0.),
            render_scene: Arc::new(RenderScene { image, scene, camera })
        }
    }

//    fn create_scatter_direction(normal: &Vector3<f32>) -> (Vector3<f32>, f32) {
//        let range = Uniform::new(0.0, PI);
//        let mut rng = rand::thread_rng();
//        let r1 = range.sample(&mut rng);
//        let r2 = range.sample(&mut rng);
//
//    }


    pub fn get_render_camera(&mut self) -> &mut Camera {&mut Arc::get_mut(&mut self.render_scene).unwrap().camera}

    fn calculate_direct_light(ray: &Ray, scene: &Scene, intersection_data: IntersectionData, renderable: &Box<dyn Renderable + Send + 'static>) -> Color {
        let material = renderable.get_material();

        let mut color = Color::new(0.,0.,0.);

        let hit_point = &ray.origin + &(ray.direction * intersection_data.distance);
        let renderable_normal = intersection_data.normal;

        for light in scene.get_lights() {
            let mut light_direction = &light.position - &hit_point;
            light_direction.normalize();


//
//            let dot_result = ray.direction.dot(&renderable_normal);
//            let shadow_point;
//            if dot_result < 0.0 {
//                shadow_point = &hit_point + &renderable_normal;
//            } else {
//                shadow_point = &hit_point - &renderable_normal;
//            }
//
//            let shadow_ray = Ray::new(shadow_point, light_direction);
//
//            let in_shadow = match Renderer::check_intersections(&shadow_ray, &scene) {
//                Some(data) => { 0.},
//                None => {1.}
//            };

            let in_shadow = 1.0;

            let light_to_normal = f32::max(0., light_direction.dot(&renderable_normal));

            let diffuse = in_shadow * light.intensity * light_to_normal;

            color += material.diffuse_color * diffuse;
        }

        color
    }

    fn check_intersections<'a>(ray: &Ray, scene: &'a Scene) -> Option<(IntersectionData, &'a Box<dyn Renderable + Send + 'static>)> {
        let mut near = std::f32::INFINITY;
        let mut intersected_renderable = None;
        let mut result_intersected_data: Option<IntersectionData> = None;

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

        match result_intersected_data {
            Some(data) => {
                return Some((data, &intersected_renderable.unwrap()))
            },
            None => return None
        }
    }

    fn trace(ray: Ray, scene: &Scene) -> Color {
        let mut pixel_color = scene.get_background().clone();

        match Renderer::check_intersections(&ray, &scene) {
            Some((result_intersected_data, renderable)) => {
                pixel_color = Renderer::calculate_direct_light(&ray, &scene, result_intersected_data, &renderable);
            }
            None => {}
        }

        pixel_color
    }

    pub fn render(&mut self) -> &Vec<u8> {
        let render_scene = Arc::get_mut(&mut self.render_scene).unwrap();

        if self.last_frame_camera_position != render_scene.camera.position { // clear buffer when camera moved;
            self.last_frame_camera_position = render_scene.camera.position;
            for i in 0..render_scene.image.len() { render_scene.image[i] = 0; }
        }

        let workers_num = self.thread_pool.get_workers_num() as u32;
        let height_per_thread = self.height / workers_num;

        for i in 0..workers_num {
            let start_height = height_per_thread * i;
            let end_height = start_height + height_per_thread;


            let width = self.width.clone();
            let height = self.height.clone();
            let mut render_scene_thread = Arc::clone(&self.render_scene);

            let task = move || {
                for h in start_height..end_height {
                    for w in 0..width {
                        let offset = (h * width * 3 + w * 3) as usize;
                        let camera_ray = render_scene_thread.camera.get_camera_ray(w, h, width, height);
                        let color = Renderer::trace(camera_ray, &render_scene_thread.scene).as_u8();

                        unsafe {
                            Arc::get_mut_unchecked(&mut render_scene_thread).image[offset] = color[0];
                            Arc::get_mut_unchecked(&mut render_scene_thread).image[offset + 1] = color[1];
                            Arc::get_mut_unchecked(&mut render_scene_thread).image[offset + 2] = color[2];
                        }
                    }
                }
            };

            self.thread_pool.add_task(Box::new(task));
        }

        self.thread_pool.wait_all();

        return &self.render_scene.image;
    }
}

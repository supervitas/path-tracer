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
use std::f32::consts::PI;
use crate::math::lerp;
use rand::prelude::ThreadRng;

const MAX_DEPTH: usize = 2;
const INDIRECT_RAYS: usize = 12;
const REFLECTION_FACTOR: f32 = 0.17;


struct RenderScene {
    camera: Camera,
    scene: Scene,
    image: Vec<f32>
}

pub struct Renderer {
    width: u32,
    height: u32,
    frames_total: u32,
    last_frame_camera_position: Vector3<f32>,
    thread_pool: ThreadPool,
    render_scene: Arc<RenderScene>
}

impl Renderer {
    pub fn new(width: u32, height: u32, camera: Camera, scene: Scene) -> Self {
        let image: Vec<f32> = vec![0.; ((width * height) * 3) as usize];
        let range = Uniform::new(0.0, 1.0);
        let mut rng: ThreadRng = rand::thread_rng();

        Renderer {
            width,
            height,
            frames_total: 0,
            thread_pool: ThreadPool::new(),
            last_frame_camera_position: Vector3::new(0.,0.,0.),
            render_scene: Arc::new(RenderScene { image, scene, camera })
        }
    }

    pub fn get_render_camera(&mut self) -> &mut Camera {&mut Arc::get_mut(&mut self.render_scene).unwrap().camera}

    fn create_scatter_direction(normal: &Vector3<f32>, r1: f32, r2: f32) -> (Vector3<f32>, f32) {
        let y = r1;
        let azimuth = r2 * 2.0 * PI;
        let sin_elevation = f32::sqrt(1.0 - y * y);
        let x = sin_elevation * f32::cos(azimuth);
        let z = sin_elevation * f32::sin(azimuth);

        let hemisphere_vec = Vector3 { x, y, z };

        let (n_t, n_b) = Renderer::create_coordinate_system(&normal);

        let scatter = Vector3 {
            x: hemisphere_vec.x * n_b.x + hemisphere_vec.y * normal.x + hemisphere_vec.z * n_t.x,
            y: hemisphere_vec.x * n_b.y + hemisphere_vec.y * normal.y + hemisphere_vec.z * n_t.y,
            z: hemisphere_vec.x * n_b.z + hemisphere_vec.y * normal.z + hemisphere_vec.z * n_t.z,
        };

        let weight = 1.0 / scatter.dot(&normal);
        (scatter, weight)
    }

    fn calculate_indirect_light(ray: &Ray, scene: &Scene, intersection_data: &IntersectionData, depth: usize) -> Color {
        let mut indirect_light = Color::new(0.,0.,0.);

        let range = Uniform::new(0.0, 1.0);
        let mut rng = rand::thread_rng();

        let hit_point = &ray.origin + &(ray.direction * intersection_data.distance);

        let indirect_count = INDIRECT_RAYS >> depth;

        for n in 0..indirect_count {
            let r1 = range.sample(&mut rng);
            let r2 = range.sample(&mut rng);

            let (direction, weight) = Renderer::create_scatter_direction(&intersection_data.normal, r1, r2);

            let indirect_ray = Ray::new(&hit_point + &(direction * 0.0001), direction);
            let cosine_angle = direction.dot(&intersection_data.normal);

            indirect_light += Renderer::trace(indirect_ray,  &scene, depth + 1) * cosine_angle ;
        }

        indirect_light = indirect_light * f32::powf(REFLECTION_FACTOR, depth as f32);

        indirect_light = indirect_light / indirect_count as f32;

        indirect_light
    }

    fn calculate_direct_light(ray: &Ray, scene: &Scene, intersection_data: &IntersectionData, renderable: &Box<dyn Renderable + Send + 'static>) -> f32 {
        let hit_point = &ray.origin + &(ray.direction * intersection_data.distance);
        let normal = intersection_data.normal;
        let material = renderable.get_material();

        let shadow_point;
        if ray.direction.dot(&normal) < 0.0 {
            shadow_point = &hit_point + &(normal * 0.0001);
        } else {
            shadow_point = &hit_point - &(normal* 0.0001);
        }

        let mut diffuse = 0.0;

        for light in scene.get_lights() {
            let mut light_direction = &light.position - &hit_point;
            light_direction.normalize();

            let shadow_ray = Ray::new(shadow_point, light_direction);

            let in_light = match Renderer::check_intersections(&shadow_ray, &scene) {
                Some(_) => 0.,
                None => 1.
            };

            let light_to_normal = f32::max(0., light_direction.dot(&normal));

            diffuse += in_light * light.intensity * light_to_normal;
        }

        diffuse
    }

    fn create_coordinate_system(normal: &Vector3<f32>) -> (Vector3<f32>, Vector3<f32>) {
        let mut n_t = if normal.x.abs() > normal.y.abs() {
            Vector3::new(normal.z, 0.0, -normal.x)
        } else {
            Vector3::new (0.0, -normal.z, normal.y)
        };

        n_t.normalize();

        let mut n_b = normal.clone();
        n_b.cross(&n_t);

        (n_t, n_b)
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

        return match result_intersected_data {
            Some(data) => Some((data, &intersected_renderable.unwrap())),
            None => None
        }
    }

    fn trace(ray: Ray, scene: &Scene, depth: usize) -> Color {
        let mut pixel_color = Color::new(0.,0.,0.);

        if depth > MAX_DEPTH {
            return pixel_color;
        }

        match Renderer::check_intersections(&ray, &scene) {
            Some((result_intersected_data, renderable)) => {
                let direct_light = Renderer::calculate_direct_light(&ray, &scene, &result_intersected_data, &renderable);
                let indirect_light_color = Renderer::calculate_indirect_light(&ray, &scene, &result_intersected_data, depth);

                let diffuse_color = renderable.get_material().diffuse_color;
                pixel_color = (indirect_light_color + &(diffuse_color * direct_light));
            }
            None => {
                if depth == 0 {
                    pixel_color = *scene.get_background();
                }
            }
        }

        pixel_color
    }

    pub fn render(&mut self) -> &Vec<f32> {
        let render_scene = Arc::get_mut(&mut self.render_scene).unwrap();

        if self.last_frame_camera_position != render_scene.camera.position {
            self.frames_total = 0;
            self.last_frame_camera_position = render_scene.camera.position;
            for i in 0..render_scene.image.len() { render_scene.image[i] = 0.; }
        }

        let workers_num = self.thread_pool.get_workers_num() as u32;
        let height_per_thread = self.height / workers_num;

        for i in 0..workers_num {
            let start_height = height_per_thread * i;
            let end_height = start_height + height_per_thread;

            let width = self.width.clone();
            let height = self.height.clone();
            let frames_total = self.frames_total.clone();
            let mut render_scene_thread = Arc::clone(&self.render_scene);

            let task = move || {
                for h in start_height..end_height {
                    for w in 0..width {
                        let offset = (h * width * 3 + w * 3) as usize;
                        let camera_ray = render_scene_thread.camera.get_camera_ray(w, h, width, height);
                        let rendered_color = Renderer::trace(camera_ray, &render_scene_thread.scene, 0);

                        let _image = &render_scene_thread.image;

                        let frames = (frames_total as f32 / (frames_total + 1) as f32);

                        let r = _image[offset] * frames + (rendered_color.r / (frames_total + 1) as f32);
                        let g = _image[offset + 1] * frames + (rendered_color.g / (frames_total + 1) as f32);
                        let b = _image[offset + 2] * frames + (rendered_color.b / (frames_total + 1) as f32);


                        unsafe {
                            let image = &mut Arc::get_mut_unchecked(&mut render_scene_thread).image;

                            image[offset] = r;
                            image[offset + 1] = g;
                            image[offset + 2] = b;
                        }
                    }
                }
            };

            self.thread_pool.add_task(Box::new(task));
        }

        self.frames_total += 1;
        self.thread_pool.wait_all();

        return &self.render_scene.image;
    }
}

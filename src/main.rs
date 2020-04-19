extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use pathtracer::renderer::camera::Camera;
use pathtracer::math::vec3::Vector3;
use pathtracer::renderer::renderer::Renderer;
use pathtracer::renderer::scene::Scene;
use pathtracer::gl::display::Display;
use pathtracer::renderables::material::Material;
use pathtracer::renderables::sphere::Sphere;
use std::time::{Duration, Instant};
use pathtracer::renderer::light::Light;
use pathtracer::renderables::plane::Plane;
use pathtracer::math::color::Color;

use pathtracer::renderer::camera_controller::CameraController;

pub fn main() {
    let width = 800;
    let height = 600;

    let mut scene = Scene::new(Color::new(255., 255., 255.));
    let light = Light::new(Color::new(255., 255., 255.), 1.2, Vector3::new(0., 55., 70.));
    scene.add_light(light);

    scene.load_model(String::from("./assets/cornell_box/default.obj"));

    let camera = Camera::new(65., Vector3::new(-10.,35.,55.), Vector3::new(-10.,40.,-1.));
    let mut camera_controller = CameraController::new(&camera);

    let mut renderer = Renderer::new(width, height, camera, scene);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Pathtracer", width, height)
        .build()
        .map_err(|e| e.to_string()).unwrap();

    let mut canvas = window.into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string()).unwrap();

    let texture_creator = canvas.texture_creator();

    let mut display = Display::new(width, height, &texture_creator).unwrap();
    let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string()).unwrap();

    'running: loop {
        let now = Instant::now();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }

        camera_controller.update( renderer.get_render_camera(), &event_pump);
        let image = renderer.render();

        display.show(&mut canvas, &image).unwrap();

        println!("Render time: {}", now.elapsed().as_millis());
    }
}

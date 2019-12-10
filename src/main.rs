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
use pathtracer::gl::obj_loader::load_obj;
use std::time::{Duration, Instant};
use pathtracer::renderer::light::Light;
use pathtracer::renderables::plane::Plane;
use pathtracer::math::color::Color;

use pathtracer::renderer::camera_controller::CameraController;
use std::sync::Arc;

pub fn main() {
    let width = 800;
    let height = 600;

    let mut scene = Scene::new(Color::new(0., 233., 255.));
    let light = Light::new(Color::new(255., 255., 255.), 1.2, Vector3::new(2., 150., 1.));
    scene.add_light(light);

//    scene.load_model(String::from("./assets/cornell_box/CornellBox-Sphere.obj"));
    scene.load_model(String::from("./assets/chair.obj"));
    add_test_renderables(&mut scene);

    let camera = Camera::new(65., 0.1, 1000., Vector3::new(0.,5.,25.), Vector3::new(0.,5.,-5.));
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

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn add_test_renderables(scene: &mut Scene) {
    for i in 0..6 {
        let step = i as f32;
        let mut material = Material::new();
        material.opacity = 0.65 + 1. / i as f32;
        material.diffuse_color = Color::new(80.,  10., 15.);

        let sphere = Sphere::new(1.5, Vector3::new( -10. + step * 4. , 2., -10.), material);
        scene.add_renderable(Box::new(sphere));
    }

    for i in 0..2 {
        let step = i as f32;
        let mut material = Material::new();
        material.opacity = 0.95;
        material.diffuse_color = Color::new(80.,  10., 15.);
        let sphere = Sphere::new(5.5, Vector3::new( 10. - step * 15. , 15., -8.), material);
        scene.add_renderable(Box::new(sphere));
    }

    {
        let mut plane_material = Material::new();
        plane_material.diffuse_color = Color::new(100., 100., 100.0);

        let plane_floor = Plane::new(Vector3::new(0., 0., 0.), plane_material, Vector3::new(0., 1., 0.));
        scene.add_renderable(Box::new(plane_floor));
    }


    {
        let mut plane_material = Material::new();
        plane_material.diffuse_color = Color::new(10.,  160., 10.);

        let plane_top = Plane::new(Vector3::new(0.,0., -50.),  plane_material, Vector3::new(0., 0.,1.));
        scene.add_renderable(Box::new(plane_top));
    }

    {
        let mut plane_material = Material::new();
        plane_material.diffuse_color = Color::new(120.,  0., 10.);

        let plane_behind = Plane::new(Vector3::new(0.,0., 50.),  plane_material, Vector3::new(0., 0.,-1.));
        scene.add_renderable(Box::new(plane_behind));
    }

    {
        let mut plane_material = Material::new();
        plane_material.diffuse_color = Color::new(10.,  10., 110.);

        let plane_left = Plane::new(Vector3::new(50., 0., 0.), plane_material, Vector3::new(-1., 0., 0.));
        scene.add_renderable(Box::new(plane_left));
    }
}

extern crate sdl2;
extern crate rand;

use rand::Rng;

use sdl2::pixels::PixelFormatEnum;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;


use pathtracer::renderer::camera::Camera;
use pathtracer::math::vec3::Vector3;
use pathtracer::renderer::renderer::Renderer;
use pathtracer::renderer::scene::Scene;
use pathtracer::gl::display::Display;
use pathtracer::renderables::material::Material;
use pathtracer::renderables::sphere::Sphere;
use pathtracer::renderables::triangle::Triangle;
use pathtracer::gl::obj_loader::load_obj;
use std::time::Duration;

pub fn main() {
    let width = 800;
    let height = 600;

    let mut scene = Scene::new([255, 255, 255]);
    load_model(&mut scene);

    let camera = Camera::new(65., 0.1, 1000., Vector3::new(0.,0.,50.), Vector3::new(0.,0.,1.));
    let mut renderer = Renderer::new(width, height);

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
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }

        let image = renderer.render(&mut scene, &camera);
        display.show(&mut canvas, &image);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn load_model(scene: &mut Scene) {
    let mut meshes = load_obj("./assets/simple.obj");

    for mesh in meshes {
        scene.add_renderable(Box::new(mesh));
    }
}

fn test_renderables(scene: &mut Scene) {
    let material = Material::new([255, 0, 0], 1.0);
    let triangle = Triangle::new(
        Vector3::new(0.,0.,-5.),
        Vector3::new(5., 0., -5.),
        Vector3::new(5.,5., -5.),
        None, Some(material));

    scene.add_renderable(Box::new(triangle));
    for i in 0..5 {
        let material = Material::new([15 * i, 10 * i + 1, 7 * i], 1.0);
        let mut sphere = Sphere::new(1.0, Vector3::new( -8. + i as f32 * 4. , 0., -5.), Some(material));
        scene.add_renderable(Box::new(sphere));
    }
}

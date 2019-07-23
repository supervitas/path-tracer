extern crate sdl2;
extern crate rand;

use rand::Rng;

use sdl2::pixels::PixelFormatEnum;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::renderables::renderable::Renderable;
use crate::renderables::cube::Cube;
use crate::math::vec3::Vector3;
use crate::renderer::camera::Camera;
use crate::renderer::renderer::Renderer;
use crate::renderables::sphere::Sphere;
use crate::renderer::scene::Scene;
use crate::gl::display::Display;
use crate::renderables::material::Material;

mod math;
mod gl;
mod renderables;
mod renderer;

pub fn main() {
    let width = 800;
    let height = 600;

    let mut scene = renderer::scene::Scene::new([255, 255, 255]);
    add_renderables(&mut scene);

    let camera = Camera::new(65., 0.1, 1000., Vector3::new(0.,0.,15.), Vector3::new(0.,0.,1.));
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

        let image = renderer.render(&scene, &camera);
        display.show(&mut canvas, &image);
    }
}

fn add_renderables(scene: &mut Scene) {
    for i in 0..5 {
        let material = Material::new([15 * i, 10 * i, 7 * i], 1.0);
        let mut sphere = Sphere::new(1.0, Vector3::new( -8. + i as f64 * 4. , 0., -5.), material);
        scene.add_renderable(Box::new(sphere));
    }
}

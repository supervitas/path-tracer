extern crate sdl2;
extern crate rand;

use rand::Rng;

use sdl2::pixels::PixelFormatEnum;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::primitives::renderable::Renderable;
use crate::primitives::cube::Cube;
use crate::math::vec3::Vector3;

mod math;
mod gl;
mod primitives;
mod renderer;

pub fn main() {
    let camera_position = math::vec3::Vector3{x:0, y:0, z: 15};
    let mut cube: Cube = Cube::new();

    let mut scene = renderer::scene::Scene::new(Vector3{x: 0.5, y: 0.1, z: 0.3});
    scene.add_renderable(Box::new(cube));


    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let width = 800;
    let height = 600;

    let window = video_subsystem
        .window("Pathtracer", width, height)
        .build()
        .map_err(|e| e.to_string()).unwrap();

    let mut canvas = window.into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string()).unwrap();

    let texture_creator = canvas.texture_creator();

    let mut display = gl::display::Display::new(width, height, &texture_creator).unwrap();
    let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string()).unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }

        display.show(&mut canvas);
    }
}

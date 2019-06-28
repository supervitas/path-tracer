extern crate sdl2;
extern crate rand;

use rand::Rng;

use sdl2::pixels::PixelFormatEnum;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod math;
mod gl;

pub fn main() {
    let camera_position = math::vec3::Vector3{x:0, y:0, z: 15};

    let mut renderer = gl::renderer::Renderer::new(800, 600).unwrap();
    renderer.run();
}

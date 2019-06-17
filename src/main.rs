extern crate sdl2;
extern crate rand;

use rand::Rng;

use sdl2::pixels::PixelFormatEnum;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod math;

pub fn main() -> Result<(), String> {

    let camera_position = math::vec3::Vector3{x:0, y:0, z: 15};

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let screen_width = 800;
    let screen_height = 600;

    let window = video_subsystem
        .window("Pathtracer", screen_width, screen_height).build().map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().present_vsync().build().map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::IYUV, screen_width, screen_height)
        .map_err(|e| e.to_string())?;

    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        // `pitch` is the width of the Y component
        // The U and V components are half the width and height of Y

        let w = screen_width as usize;
        let h = screen_height as usize;

        // Set Y (constant)
        for y in 0..h {
            for x in 0..w {
                let offset = y*pitch + x;
                buffer[offset] = 128;
            }
        }

        let y_size = pitch*h;

        let mut rng = rand::thread_rng();


        // Set U and V (X and Y)
        for y in 0..h/2 {
            for x in 0..w/2 {
                let u_offset = y_size + y*pitch/2 + x;
                let v_offset = y_size + (pitch/2 * h/2) + y*pitch/2 + x;

                let n1: u8 = rng.gen();
                let n2: u8 = rng.gen();

                buffer[u_offset] = n1;
                buffer[v_offset] = n2;
            }
        }
    })?;

    canvas.clear();
    canvas.copy(&texture, None, Some(Rect::new(0, 0, screen_width, screen_height)))?;
    canvas.present();

    let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }

//        canvas.set_draw_color(Color::RGB(0, 0, 0));
//        canvas.clear();
//        canvas.present();
    }

    Ok(())
}

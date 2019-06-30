extern crate rand;
use rand::Rng;

extern crate sdl2;
use sdl2::pixels::PixelFormatEnum;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::{Sdl, VideoSubsystem};
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Texture, Canvas, TextureCreator};
use sdl2::mouse::SystemCursor::No;


pub struct Display<'a> {
    width: u32,
    height: u32,
    texture: Texture<'a>,
}

impl<'a> Display <'a>{
     pub fn new(width: u32, height: u32, texture_creator: &'a TextureCreator<WindowContext>) -> Result<Display, String> {
         let texture = texture_creator.create_texture_streaming(PixelFormatEnum::IYUV, width, height)
             .map_err(|e| e.to_string()).unwrap();

         let display = Display {
             width,
             height,
             texture
         };

        Ok(display)
    }

    fn random_texture(&mut self) {
        let w = self.width as usize;
        let h = self.height as usize;

        self.texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            // `pitch` is the width of the Y component
            // The U and V components are half the width and height of Y

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
        }).unwrap();
    }


    pub fn show (&mut self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        self.random_texture();
        canvas.clear();
        canvas.copy(&self.texture, None, Some(Rect::new(0, 0, self.width, self.height)))?;
        canvas.present();

        Ok(())
    }
}

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

    fn write_image_to_texture(&mut self, image: &Vec<u8>) {
        let width = self.width as usize;
        let height = self.height as usize;

        self.texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            let y_size = pitch*height;

            for x in 0..width {
                for y in 0..height {
                    let offset = x * width + y;

                    buffer[offset] = image[offset];
                    buffer[offset + 1] = image[offset + 1];
                    buffer[offset + 2] = image[offset + 2];
                }
            }
        }).unwrap();
    }


    pub fn show (&mut self, canvas: &mut Canvas<Window>, image: &Vec<u8>) -> Result<(), String> {
        self.write_image_to_texture(&image);

        canvas.clear();
        canvas.copy(&self.texture, None, Some(Rect::new(0, 0, self.width, self.height)))?;
        canvas.present();

        Ok(())
    }
}

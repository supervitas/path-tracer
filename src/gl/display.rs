extern crate rand;

extern crate sdl2;
use sdl2::pixels::PixelFormatEnum;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Texture, Canvas, TextureCreator};


pub struct Display<'a> {
    width: u32,
    height: u32,
    texture: Texture<'a>,
}

impl<'a> Display <'a> {
     pub fn new(width: u32, height: u32, texture_creator: &'a TextureCreator<WindowContext>) -> Result<Display, String> {
         let texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, width, height)
             .map_err(|e| e.to_string()).unwrap();

         let display = Display {
             width,
             height,
             texture
         };

        Ok(display)
    }

    fn write_image_to_texture(&mut self, image: &Vec<f32>) {
        let width = self.width as usize;
        let height = self.height as usize;

        self.texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for h in 0..height {
                for w in 0..width {
                    let offset = h * pitch + w * 3;

                    buffer[offset] = f32::min(image[offset],255.0) as u8;
                    buffer[offset + 1] = f32::min(image[offset + 1],255.0) as u8;
                    buffer[offset + 2] = f32::min(image[offset + 2],255.0) as u8;
                }
            }
        }).unwrap();
    }

    pub fn show (&mut self, canvas: &mut Canvas<Window>, image: &Vec<f32>) -> Result<(), String> {
        self.write_image_to_texture(&image);

        canvas.clear();
        canvas.copy(&self.texture, None, None);
        canvas.present();

        Ok(())
    }
}

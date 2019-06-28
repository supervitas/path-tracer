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



pub struct Renderer<'a> {
    sdl_context: Sdl,
    width: u32,
    height: u32,
    video_subsystem: VideoSubsystem,
    canvas: Canvas<Window>,
    texture: Texture<'a>,
    texture_creator: TextureCreator<WindowContext>,
}

impl <'a> Renderer <'a>{
     pub fn new(width: u32, height: u32) -> Result<Renderer<'a>, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window("Pathtracer", width, height)
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())?;

        let texture_creator = canvas.texture_creator();

         let texture = texture_creator.create_texture_streaming(PixelFormatEnum::IYUV, width, height)
            .map_err(|e| e.to_string()).unwrap();


         let renderer = Renderer {
             width,
             height,
             texture,
             sdl_context,
             video_subsystem,
             canvas,
             texture_creator,
         };

        Ok(renderer)
    }


    pub fn run (&mut self) -> Result<(), String> {
        let mut event_pump = self.sdl_context.event_pump().map_err(|e| e.to_string())?;


//        self.canvas.clear();
//        self.canvas.copy(&texture, None, Some(Rect::new(0, 0, self.width, self.height)))?;
//        self.canvas.present();

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                    _ => {}
                }
            }
        }

        Ok(())
    }

//    fn random_texture(&mut self) -> Texture  {
//        let mut texture = self.texture_creator.create_texture_streaming(PixelFormatEnum::IYUV, self.width, self.height)
//            .map_err(|e| e.to_string()).unwrap();
//
//
//        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
//            // `pitch` is the width of the Y component
//            // The U and V components are half the width and height of Y
//
//            let w = self.width as usize;
//            let h = self.height as usize;
//
//            // Set Y (constant)
//            for y in 0..h {
//                for x in 0..w {
//                    let offset = y*pitch + x;
//                    buffer[offset] = 128;
//                }
//            }
//
//            let y_size = pitch*h;
//
//            let mut rng = rand::thread_rng();
//
//
//            // Set U and V (X and Y)
//            for y in 0..h/2 {
//                for x in 0..w/2 {
//                    let u_offset = y_size + y*pitch/2 + x;
//                    let v_offset = y_size + (pitch/2 * h/2) + y*pitch/2 + x;
//
//                    let n1: u8 = rng.gen();
//                    let n2: u8 = rng.gen();
//
//                    buffer[u_offset] = n1;
//                    buffer[v_offset] = n2;
//                }
//            }
//        }).unwrap();
//
//        texture
//    }
}

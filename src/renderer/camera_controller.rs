use crate::renderer::camera::Camera;
use sdl2::mouse::MouseButton;
use sdl2::EventPump;

pub struct CameraController {
}

impl CameraController  {
    pub fn new() -> Self {
       CameraController {
       }
    }

    pub fn update(&mut self, camera: &mut Camera, event_pump: &EventPump) {
        if event_pump.mouse_state().is_mouse_button_pressed(MouseButton::Left) {
            let state = event_pump.relative_mouse_state();
            println!("Relative - X = {:?}, Y = {:?}", state.x(), state.y());

            let mut position = camera.position().clone();
            position.x += state.x() as f32;
            position.y += state.y() as f32;

            camera.set_position(position);
        }
    }
}

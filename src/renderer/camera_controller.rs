use crate::renderer::camera::Camera;
use sdl2::mouse::MouseButton;
use sdl2::EventPump;

pub struct CameraController <'a> {
    camera: &'a Camera,

}

impl <'a> CameraController <'a> {
    pub fn new(camera: &'a Camera) -> Self {
       CameraController {
           camera
       }
    }

    pub fn update(&self, event_pump: &EventPump) {
        if event_pump.mouse_state().is_mouse_button_pressed(MouseButton::Left) {
            let state = event_pump.relative_mouse_state();
            println!("Relative - X = {:?}, Y = {:?}", state.x(), state.y());
        }
    }
}

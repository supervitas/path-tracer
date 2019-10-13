use crate::renderer::camera::Camera;
use sdl2::mouse::MouseButton;
use sdl2::EventPump;
use std::f32::consts::PI;
use crate::math::mat4::Matrix4;
use crate::math::vec3::Vector3;
use crate::math::spherical::Spherical;
use num::clamp;

pub struct CameraController {
    pub spherical: Spherical<f32>,
    last_x: f32,
    last_y: f32
}

impl CameraController  {
    pub fn new(cam: &Camera) -> Self {
       let cam_position = &cam.position;

       CameraController {
           spherical: Spherical::from_cartesian(cam_position.x, cam_position.y, cam_position.z),
           last_x: 0.,
           last_y: 0.
       }
    }

    pub fn update(&mut self, cam: &mut Camera, event_pump: &EventPump) {
        if event_pump.mouse_state().is_mouse_button_pressed(MouseButton::Left) {
            let state = event_pump.mouse_state();
            let x = state.x() as f32;
            let y = state.y() as f32;

            if self.last_x == 0.0 {
                self.last_x = x;
                self.last_y = y;
            }

            let delta_x = x - self.last_x;
            let delta_y = y - self.last_y;

            self.spherical.azimuth_angle -= delta_x * 0.01;
            self.spherical.polar_angle = clamp(self.spherical.polar_angle - (delta_y * 0.01), 0.2,  1.4);

            self.last_x = x;
            self.last_y = y;

            let new_position = self.spherical.to_cartesian();
            cam.update(&new_position);
        } else {
            self.last_x = 0.0;
            self.last_y = 0.0;
        }
    }
}

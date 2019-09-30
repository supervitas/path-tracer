use crate::renderer::camera::Camera;
use sdl2::mouse::MouseButton;
use sdl2::EventPump;
use std::f32::consts::PI;
use crate::math::mat4::Matrix4;
use crate::math::vec3::Vector3;

pub struct CameraController {
    theta: f32,
    phi: f32,
    pub min_polar_angle: f32,
    pub max_polar_angle: f32,

    last_x: f32,
    last_y: f32
}

impl CameraController  {
    pub fn new(cam: &Camera) -> Self {
       let cam_position = &cam.position;

       CameraController {
           theta: f32::atan(f32::sqrt((f32::powi(cam_position.x, 2) + f32::powi(cam_position.y, 2))) / cam_position.z),
           phi: f32::atan2(cam_position.y, cam_position.x),
           min_polar_angle: 0.2,
           max_polar_angle: PI / 2.5,
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

            self.theta = num::clamp(self.theta + delta_y, self.min_polar_angle, self.max_polar_angle ) * 0.1;
            self.phi = (self.phi - delta_x) * 0.01;

            self.last_x = x;
            self.last_y = y;

            let mut x_rot_matrix: Matrix4<f32> = Matrix4::identity();
            let mut y_rot_matrix: Matrix4<f32> = Matrix4::identity();

            x_rot_matrix.rotate_x(-self.theta);
            y_rot_matrix.rotate_y(self.phi);
            cam.update_from_rotation(&x_rot_matrix, &y_rot_matrix);
        } else {
            self.last_x = 0.0;
            self.last_y = 0.0;
        }
    }
}

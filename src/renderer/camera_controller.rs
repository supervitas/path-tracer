use crate::renderer::camera::Camera;
use sdl2::mouse::MouseButton;
use sdl2::EventPump;
use std::f32::consts::PI;
use crate::math::mat4::Matrix4;
use crate::math::vec3::Vector3;

pub struct CameraController {
    theta: f32,
    phi: f32,
    radius: f32,
    pub min_polar_angle: f32,
    pub max_polar_angle: f32,

    last_x: f32,
    last_y: f32
}

impl CameraController  {
    pub fn new(cam: &Camera) -> Self {
       let cam_position = cam.position();

       CameraController {
           theta: f32::atan(f32::sqrt((f32::powi(cam_position.x, 2) + f32::powi(cam_position.y, 2))) / cam_position.z),
           phi: f32::atan2(cam_position.y, cam_position.x),
           min_polar_angle: 0.2,
           max_polar_angle: PI / 2.5,
           radius: 10.,

           last_x: 0.,
           last_y: 0.
       }
    }

    pub fn update(&mut self, cam: &mut Camera, event_pump: &EventPump) {
        if event_pump.mouse_state().is_mouse_button_pressed(MouseButton::Left) {
            let state = event_pump.mouse_state();

            let x = state.x() as f32;
            let y = state.y() as f32;
            let sencitivity = 0.1;
//            println!("Relative - X = {:?}, Y = {:?}", state.x(), state.y());

            self.theta = num::clamp(self.theta + (y - self.last_y), self.min_polar_angle, self.max_polar_angle );
            self.phi = self.phi - (x - self.last_x);

            println!("{}, {}", self.theta, self.phi);

            self.last_x = x;
            self.last_y = y;

            let mut camera = Matrix4::identity();
            camera.translation(0., 0., self.radius);

            let mut x_rot_matrix = Matrix4::identity();
            let mut y_rot_matrix = Matrix4::identity();

            x_rot_matrix.rotate_x(-self.theta);
            y_rot_matrix.rotate_y(self.phi);


            camera.multiply(&x_rot_matrix);
            camera.multiply(&y_rot_matrix);

            let cam_focus_vector = cam.position() - cam.target();


//            let look_at = Matrix4::look_at(&cam_focus_vector, cam.target());

//            cam.set_position(Vector3::new(look_at.elements[12], look_at.elements[13], look_at.elements[14]))
            cam.update_camera_world();

        } else {
            self.last_x = 0.0;
            self.last_y = 0.0;
        }
    }
}

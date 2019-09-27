use crate::math::vec3::Vector3;
use crate::math::ray::Ray;
use crate::math::mat4::Matrix4;

pub struct Camera {
    position: Vector3<f32>,
    target: Vector3<f32>,
    fov: f32,
    z_near: f32,
    z_far: f32,
    camera_world: Matrix4<f32>
}

impl Camera {
    pub fn new(fov: f32, z_near: f32, z_far: f32, position: Vector3<f32>, target: Vector3<f32>) -> Self {
        Camera {
            fov,
            z_far,
            z_near,
            position,
            target,
            camera_world: Matrix4::look_at(&position, &target)
        }
    }

    pub fn update_camera_world(&mut self) {
        self.camera_world = Matrix4::look_at(&self.position, &self.target);
    }

    pub fn get_camera_ray(&self, x: u32, y: u32, width: u32, height: u32) -> Ray {
        let fov_adjustment = (45.0_f32.to_radians() / 2.0).tan();
        let aspect_ratio = (width as f32) / (height as f32);
        let dir_x = ((((x as f32 + 0.5) / width as f32) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let dir_y = (1.0 - ((y as f32 + 0.5) / height as f32) * 2.0) * fov_adjustment;

        let mut direction = Vector3::new(dir_x, dir_y, -1.0);


        let mut inverse = self.camera_world.clone();
        inverse.get_inverse();
//        direction.apply_matrix(&inverse);
        direction.normalize();


//        let mut from_cam_to_target = &self.target - &self.position;
//        from_cam_to_target.normalize();
//
//        let angle = from_cam_to_target.angle_between(&initial_direction);
//        let mut rotation_axis = direction.clone();
//        rotation_axis.cross(&from_cam_to_target);
//        rotation_axis.normalize();
//
//        let mat = Matrix4::from_axis_angle(&rotation_axis, angle);
//        direction.apply_matrix(&mat);


        Ray {
            origin: self.position.clone(),
            direction
        }
    }

    pub fn set_position(&mut self, position: Vector3<f32>) {
        self.position = position;
    }

    pub fn position(&self) -> &Vector3<f32> {
        &self.position
    }
    pub fn target(&self) -> &Vector3<f32> {
        &self.target
    }
}

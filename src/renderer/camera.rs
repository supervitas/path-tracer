use crate::math::vec3::Vector3;
use crate::math::ray::Ray;
use crate::math::mat4::Matrix4;

pub struct Camera {
    pub position: Vector3<f32>,
    pub target: Vector3<f32>,
    up: Vector3<f32>,
    fov: f32,
    z_near: f32,
    z_far: f32,
    camera_world: Matrix4<f32>,
}

impl Camera {
    pub fn new(fov: f32, z_near: f32, z_far: f32, position: Vector3<f32>, target: Vector3<f32>) -> Self {
        let up = Vector3::new(0., 1.,0.);
        let pos = &target + &position;
        let mut camera_world:Matrix4<f32> = Matrix4::identity();
        camera_world.look_at(&pos, &target, &up);

        Camera {
            fov,
            z_far,
            z_near,
            position: pos,
            target,
            up,
            camera_world,
        }
    }

    pub fn update(&mut self, new_position: &Vector3<f32>) {
        self.position = &self.target + &new_position;
        self.camera_world.look_at(&self.position, &self.target, &self.up);
    }

    pub fn get_camera_ray(&self, x: u32, y: u32, width: u32, height: u32) -> Ray {
        let fov_adjustment = (self.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (width as f32) / (height as f32);
        let dir_x = ((((x as f32 + 0.5) / width as f32) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let dir_y = (1.0 - ((y as f32 + 0.5) / height as f32) * 2.0) * fov_adjustment;

        let mut inverse_camera = self.camera_world.clone();
        inverse_camera.inverse();

        let mut direction = Vector3::new(dir_x, dir_y, -1.0);

        direction.apply_matrix(&self.camera_world);
        direction.normalize();

        Ray {
            origin: self.position,
            direction
        }
    }
}

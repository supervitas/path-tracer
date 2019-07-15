use crate::math::vec3::Vector3;
use crate::primitives::renderable::Renderable;
use crate::renderer::scene::Scene;
use crate::renderer::camera::Camera;
use rand::Rng;

pub fn render(scene: &Scene, camera: &Camera, width: &u32, height: &u32) -> Vec<u8> {
    let mut image: Vec<u8> = Vec::with_capacity(((*width * *height) * 3) as usize);
    let mut rng = rand::thread_rng();

    for w in 0..*width {
        for h in 0..*height {
            image.push(rng.gen_range(1, 255));
            image.push(rng.gen_range(1, 255));
            image.push(rng.gen_range(1, 255));
        }
    }

    return image;
}

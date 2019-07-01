use crate::math::vec3::Vector3;
use crate::primitives::renderable::Renderable;

pub struct Scene {
    background: Vector3<f32>,
    renderables: Vec<Box<dyn Renderable>>
}

impl Scene {
    pub fn new(background: Vector3<f32>) -> Scene {
      Scene {
          background,
          renderables: Vec::new(),
      }
    }

    pub fn add_renderable(&mut self, renderable: Box<dyn Renderable>) {
        self.renderables.push(renderable);
    }
}

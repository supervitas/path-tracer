use crate::math::vec3::Vector3;
use crate::renderables::renderable::Renderable;

pub struct Scene {
    background: [u8; 3],
    renderables: Vec<Box<dyn Renderable>>
}

impl Scene {
    pub fn new(background: [u8; 3]) -> Self {
      Scene {
          background,
          renderables: Vec::new(),
      }
    }

    pub fn get_renderables(&self) -> &Vec<Box<dyn Renderable>> {
        &self.renderables
    }

    pub fn get_background(&self) -> &[u8; 3] {
        &self.background
    }

    pub fn add_renderable(&mut self, renderable: Box<dyn Renderable>) {
        self.renderables.push(renderable);
    }
}

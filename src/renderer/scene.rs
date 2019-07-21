use crate::math::vec3::Vector3;
use crate::renderables::renderable::Renderable;

pub struct Scene {
    background: Vector3<f64>,
    renderables: Vec<Box<dyn Renderable>>
}

impl Scene {
    pub fn new(background: Vector3<f64>) -> Self {
      Scene {
          background,
          renderables: Vec::new(),
      }
    }

    pub fn get_renderables(&self) -> &Vec<Box<dyn Renderable>> {
        &self.renderables
    }

    pub fn get_background(&self) -> &Vector3<f64> {
        &self.background
    }

    pub fn add_renderable(&mut self, renderable: Box<dyn Renderable>) {
        self.renderables.push(renderable);
    }
}

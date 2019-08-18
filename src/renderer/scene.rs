use crate::math::vec3::Vector3;
use crate::renderables::renderable::Renderable;
use crate::renderer::light::Light;

pub struct Scene {
    background: [u8; 3],
    lights: Vec<Light>,
    renderables: Vec<Box<dyn Renderable>>
}

impl Scene {
    pub fn new(background: [u8; 3], lights: Vec<Light>) -> Self {
      Scene {
          background,
          lights,
          renderables: Vec::new(),
      }
    }

    pub fn get_renderables(&mut self) -> &mut Vec<Box<dyn Renderable>> {
        &mut self.renderables
    }

    pub fn get_lights(&self) -> &Vec<Light>{
        &self.lights
    }

    pub fn get_background(&self) -> &[u8; 3] {
        &self.background
    }

    pub fn add_renderable(&mut self, renderable: Box<dyn Renderable>) {
        self.renderables.push(renderable);
    }
}

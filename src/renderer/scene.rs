use crate::math::vec3::Vector3;
use crate::renderables::renderable::Renderable;
use crate::renderer::light::Light;
use crate::math::color::Color;

pub struct Scene {
    background: Color,
    lights: Vec<Light>,
    renderables: Vec<Box<dyn Renderable>>
}

impl Scene {
    pub fn new(background: Color, lights: Vec<Light>) -> Self {
      Scene {
          background,
          lights,
          renderables: Vec::new(),
      }
    }

    pub fn get_renderables(&self) -> &Vec<Box<dyn Renderable>> {
        &self.renderables
    }

    pub fn get_lights(&self) -> &Vec<Light>{
        &self.lights
    }

    pub fn get_background(&self) -> &Color {
        &self.background
    }

    pub fn add_renderable(&mut self, renderable: Box<dyn Renderable>) {
        self.renderables.push(renderable);
    }
}

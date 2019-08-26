use crate::math::vec3::Vector3;
use crate::renderables::renderable::Renderable;
use crate::renderer::light::Light;
use crate::math::color::Color;
use crate::gl::obj_loader::load_obj;
use crate::renderables::material::Material;
use crate::renderables::sphere::Sphere;
use crate::renderables::plane::Plane;

pub struct Scene {
    background: Color,
    lights: Vec<Light>,
    renderables: Vec<Box<dyn Renderable>>
}

impl Scene {
    pub fn new(background: Color) -> Self {
      Scene {
          background,
          lights: Vec::new(),
          renderables: Vec::new(),
      }
    }

    pub fn get_renderables(&self) -> &Vec<Box<dyn Renderable>> {
        &self.renderables
    }

    pub fn get_lights(&self) -> &Vec<Light>{
        &self.lights
    }

    pub fn add_light(&mut self, light: Light) {self.lights.push(light)}

    pub fn get_background(&self) -> &Color {
        &self.background
    }

    pub fn add_renderable(&mut self, renderable: Box<dyn Renderable>) {
        self.renderables.push(renderable);
    }

    pub fn load_model(&mut self, path: String) {
        let mut meshes = load_obj(&path);

        for mesh in meshes {
            self.add_renderable(Box::new(mesh));
        }
    }
}

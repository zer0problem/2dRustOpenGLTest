use crate::tpixel::registry::Registry;
use crate::tpixel::material_info::MaterialInfo;
use crate::tpixel::camera::Camera;
use crate::tpixel::color::Color;
use crate::tpixel::deferred_renderer::DeferredRenderer;
use crate::tpixel::shader_factory::ShaderFactory;

pub struct Renderer {
    deferred_renderer : DeferredRenderer,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            deferred_renderer : DeferredRenderer::new(),
        }
    }
    pub fn prepare_material(&mut self, material_info : &MaterialInfo) {
        self.deferred_renderer.prepare_material(material_info);
    }
    pub fn init(&mut self, shader_factory : &ShaderFactory) {
        self.deferred_renderer.init(shader_factory);
    }
    pub fn resize_framebuffer(&mut self, width : i32, height : i32) {
        self.deferred_renderer.resize_geo_buffer(width, height);
    }
    pub fn render(&mut self, registry : &Registry, camera : &Camera, ambient_color : &Color) {
        self.deferred_renderer.render(registry, camera, ambient_color);
    }
}

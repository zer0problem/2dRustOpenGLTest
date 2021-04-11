use crate::tpixel::registry::Registry;
use crate::tpixel::renderer::Renderer;
use crate::tpixel::sprite_factory::SpriteFactory;
use crate::tpixel::texture_factory::TextureFactory;
use crate::tpixel::material_factory::MaterialFactory;
use crate::tpixel::shader_factory::ShaderFactory;
use crate::tpixel::camera::Camera;
use crate::tpixel::color::Color;
use crate::tpixel::input_manager::InputManager;

// registry inits
use crate::tpixel::sprite::Sprite;
use crate::tpixel::point_light::PointLight;

use glfw::{Action, Key};
use std::time::{Instant};

pub struct Engine {
    pub registry : Registry,
    pub camera : Camera,
    pub ambient_color : Color,
    
    delta_time : f32,
    last_frame_instance : Instant,
    input_manager : InputManager,

    renderer : Renderer,
    sprite_factory : SpriteFactory,
    texture_factory : TextureFactory,
    material_factory : MaterialFactory,
    shader_factory : ShaderFactory,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            registry : Registry::new(),
            camera : Camera::new(),
            ambient_color : Color {
                r : 0f32,
                g : 0f32,
                b : 0f32,
                a : 0f32,
            },
            
            delta_time : 0f32,
            last_frame_instance : Instant::now(),
            input_manager : InputManager::new(),
            
            renderer : Renderer::new(),
            sprite_factory : SpriteFactory::new(),
            texture_factory : TextureFactory::new(),
            material_factory : MaterialFactory::new(),
            shader_factory : ShaderFactory::new(),
        }
    }
    pub fn init(&mut self) {
        self.registry.init_map::<Sprite>();
        self.registry.init_map::<PointLight>();

        self.renderer.init(&self.shader_factory);
    }
    pub fn start_frame(&mut self) {
        let new_now = Instant::now();
        self.delta_time = new_now.duration_since(self.last_frame_instance).as_secs_f32();
        self.last_frame_instance = new_now;
    }
    pub fn render(&mut self) {
        self.renderer.render(&self.registry, &self.camera, &self.ambient_color);
    }
    pub fn process_event(&mut self, window : &mut glfw::Window, event : &glfw::WindowEvent) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                self.renderer.resize_framebuffer(*width, *height);
                self.camera.view_size.x = (*width as f32) / 3.0;
                self.camera.view_size.y = (*height as f32) / 3.0;
                unsafe {
                    gl::Viewport(0, 0, *width, *height);
                }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true);
            },
            glfw::WindowEvent::Key(key, _scancode, action, _modifiers) => {
                self.input_manager.update_event(*key, *action);
            },
            _ => {}
        }
    }
    pub fn update_input(&mut self, window : &glfw::Window) {
        self.input_manager.update_input(window);
    }

    pub fn new_sprite(&mut self, material_id : u32) -> Sprite {
        self.sprite_factory.new_sprite(material_id)
    }
    pub fn new_material(&mut self, color_path : &str, material_path : &str, normal_path : &str) -> u32 {
        let material_data = self.material_factory.new_material(&mut self.texture_factory, color_path, material_path, normal_path);
        self.renderer.prepare_material(&material_data);
        material_data.id
    }

    pub fn get_dt(&self) -> f32 {
        return self.delta_time;
    }
    pub fn is_key_pressed(&self, key : Key) -> bool {
        self.input_manager.is_key_pressed(key)
    }
    pub fn is_key_down(&self, key : Key) -> bool {
        self.input_manager.is_key_down(key)
    }
    pub fn is_key_released(&self, key : Key) -> bool {
        self.input_manager.is_key_released(key)
    }
}

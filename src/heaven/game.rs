use crate::tpixel::engine::Engine;
use crate::tpixel::sprite::Sprite;
use crate::tpixel::vector2::Vector2;
use crate::tpixel::rect::Rect;
use crate::tpixel::color::Color;
use crate::tpixel::point_light::PointLight;
use crate::tpixel::matrix3x2::Matrix3x2;

use rand::Rng;
use glfw::Key;

pub struct Game {
    entities : Vec<u32>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            entities : Vec::new(),
        }
    }
    pub fn init(&mut self, engine : &mut Engine) {
        let material_id = engine.new_material(
            "target/debug/assets/test_color.png",
            "target/debug/assets/test_material.png",
            "target/debug/assets/test_normal.png");
        let mut rng = rand::thread_rng();
        for i in 0..1024 {
            let tile_x : f32 = (i % 32) as f32 * 128.0;
            let tile_y : f32 = (i / 32) as f32 * 128.0;
            let ent = engine.registry.create_entity();
            self.entities.push(ent);
            let mut sprite1 : Sprite = engine.new_sprite(material_id);
            sprite1.transform = Matrix3x2::new_transform(
                Vector2{ x : tile_x - 512.0f32, y : tile_y - 512.0f32 },
                Vector2{ x : 1f32 + 0f32 * rng.gen::<f32>(), y : 1f32 + 0f32 * rng.gen::<f32>() },
                rng.gen::<f32>() * std::f32::consts::PI * 0.0f32,
            );
            sprite1.pivot = Vector2 {x : 0.5f32, y : 0.5f32};
            sprite1.color = Color {r : rng.gen::<f32>(), g : rng.gen::<f32>(), b : rng.gen::<f32>(), a : 1.0f32};
            sprite1.uv_rect = Rect {begin : Vector2 {x : 0.0f32, y : 0.0f32}, end : Vector2 {x : 1.0f32, y : 1.0f32}};
            sprite1.z = rng.gen::<f32>();
            engine.registry.get_map_mut::<Sprite>().insert(ent, sprite1);
        }
        for i in 0..4 {
            let ent = self.entities[i];
            let x : f32 = (i % 2) as f32;
            let y : f32 = (i / 2) as f32;
            let light = PointLight {
                color : Color {
                    r : x,
                    g : y,
                    b : 1f32,
                    a : 1f32,
                },
                position : Vector2 {
                    x : x * 768f32 - 256f32,
                    y : y * 768f32 - 256f32,
                },
                height : 64f32,
                range : 512f32,
            };
            engine.registry.get_map_mut::<PointLight>().insert(ent, light);
        }
    }
    pub fn update(&mut self, engine : &mut Engine) {
        let dt = engine.get_dt();
        let move_speed = 128f32;
        let turn_speed = std::f32::consts::PI;
        let mut fuck : bool = false;
        if engine.is_key_down(Key::Space) {
            fuck = true;
        }
        if engine.is_key_down(Key::Q) {
            for sprite_kv in engine.registry.get_map_mut::<Sprite>().all_iter_mut() {
                sprite_kv.value.transform.translate(Vector2{x : dt * 32f32, y : dt * 32f32});
                sprite_kv.value.transform.rotate(dt);
            }
        }
        engine.registry.get_map_mut::<PointLight>().get_mut(self.entities[3]).position = engine.camera.transform.get_position();
        
        //engine.camera.transform.translate(Vector2 {x : dt * 5.0f32, y : 0.0f32});
        if engine.is_key_down(Key::Left) {
            let sprite = engine.registry.get_map_mut::<Sprite>().get_mut(self.entities[1]);
            if fuck {
                sprite.transform.rotate(dt * turn_speed);
            } else {
                engine.camera.transform.rotate(dt * turn_speed);
            }
        }
        if engine.is_key_down(Key::Right) {
            let sprite = engine.registry.get_map_mut::<Sprite>().get_mut(self.entities[1]);
            if fuck {
                sprite.transform.rotate(-dt * turn_speed);
            } else {
                engine.camera.transform.rotate(-dt * turn_speed);
            }
        }
        if engine.is_key_down(Key::A) {
            let sprite = engine.registry.get_map_mut::<Sprite>().get_mut(self.entities[1]);
            if fuck {
                sprite.transform.translate(Vector2{x : -dt * move_speed, y: 0f32});
            } else {
                engine.camera.transform.translate(Vector2{x : -dt * move_speed, y: 0f32});
            }
        }
        if engine.is_key_down(Key::D) {
            let sprite = engine.registry.get_map_mut::<Sprite>().get_mut(self.entities[1]);
            if fuck {
                sprite.transform.translate(Vector2{x : dt * move_speed, y: 0f32});
            } else {
                engine.camera.transform.translate(Vector2{x : dt * move_speed, y: 0f32});
            }
        }
        if engine.is_key_down(Key::W) {
            let sprite = engine.registry.get_map_mut::<Sprite>().get_mut(self.entities[1]);
            if fuck {
                sprite.transform.translate(Vector2{y : dt * move_speed, x: 0f32});
            } else {
                engine.camera.transform.translate(Vector2{y : dt * move_speed, x: 0f32});
            }
        }
        if engine.is_key_down(Key::S) {
            let sprite = engine.registry.get_map_mut::<Sprite>().get_mut(self.entities[1]);
            if fuck {
                sprite.transform.translate(Vector2{y : -dt * move_speed, x: 0f32});
            } else {
                engine.camera.transform.translate(Vector2{y : -dt * move_speed, x: 0f32});
            }
        }
    }
}

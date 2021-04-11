use crate::tpixel::vector2::Vector2;
use crate::tpixel::color::Color;
use crate::tpixel::matrix3x2::Matrix3x2;
use crate::tpixel::rect::Rect;
use crate::tpixel::sprite::Sprite;

pub struct SpriteFactory {}

impl SpriteFactory {
    pub fn new() -> SpriteFactory {
        SpriteFactory {}
    }
    pub fn new_sprite(&self, material_id : u32) -> Sprite {
        Sprite {
            transform : Matrix3x2::new(),
            pivot : Vector2::new(),
            color : Color::new(),
            uv_rect : Rect::new_uv(),
            z : 0f32,
            height : 0f32,
            material_id : material_id,
        }
    }
}

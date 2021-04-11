use crate::tpixel::vector2::Vector2;
use crate::tpixel::color::Color;
use crate::tpixel::matrix3x2::Matrix3x2;
use crate::tpixel::rect::Rect;

pub struct Sprite { // IF YOU ADD STUFF HERE, REMEMBER TO UPDATE THE RENDERER AND ITS LAYOUT
    pub transform : Matrix3x2,
    pub pivot : Vector2,
    pub color : Color,
    pub uv_rect : Rect,
    pub z : f32,
    pub height : f32,
    pub(crate) material_id : u32,
}

impl Sprite {
    pub(crate) fn clone(&self) -> Sprite {
        Sprite {
            transform : self.transform,
            pivot : self.pivot,
            color : self.color,
            uv_rect : self.uv_rect,
            z : self.z,
            height : self.z,
            material_id : self.material_id,
        }
    }
}

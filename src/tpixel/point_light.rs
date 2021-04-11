use crate::tpixel::color::Color;
use crate::tpixel::vector2::Vector2;

pub struct PointLight {
    pub color : Color,
    pub position : Vector2,
    pub height : f32,
    pub range : f32,
}

impl PointLight {
    pub(crate) fn clone(&self) -> PointLight {
        PointLight {
            color : self.color,
            position : self.position,
            height : self.height,
            range : self.range,
        }
    }
}

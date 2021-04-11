use crate::tpixel::vector2::Vector2;

#[derive(Copy, Clone)]
pub struct Rect {
    pub begin : Vector2,
    pub end : Vector2,
}

impl Rect {
    pub fn new_uv() -> Rect {
        Rect {
            begin : Vector2 { x : 0f32, y : 0f32 },
            end : Vector2 { x : 1f32, y : 1f32 },
        }
    }
}

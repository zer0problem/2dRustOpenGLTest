
#[derive(Copy, Clone)]
pub struct Vector2 {
    pub x : f32,
    pub y : f32,
}

impl Vector2 {
    pub fn new() -> Vector2 {
        Vector2 { x : 0f32, y : 0f32 }
    }
}

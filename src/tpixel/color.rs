
#[derive(Copy, Clone)]
pub struct Color {
    pub r : f32,
    pub g : f32,
    pub b : f32,
    pub a : f32,
}

impl Color {
    pub fn new() -> Color {
        Color {
            r : 1f32,
            g : 1f32,
            b : 1f32,
            a : 1f32,
        }
    }
}

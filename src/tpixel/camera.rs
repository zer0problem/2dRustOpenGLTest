use crate::tpixel::matrix3x2::Matrix3x2;
use crate::tpixel::vector2::Vector2;

pub struct Camera {
    pub transform : Matrix3x2,
    pub view_size : Vector2,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            transform : Matrix3x2::new_transform(Vector2 { x : 0.0f32, y : 0.0f32 },
                Vector2 {x : 1f32, y : 1f32 },
                0f32,
            ),
            view_size : Vector2 { x : 1280.0f32, y : 720.0f32 },
        }
    }
}

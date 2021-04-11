use crate::tpixel::vector2::Vector2;

#[derive(Copy, Clone)]
pub struct MaterialInfo {
    pub id : u32,
    pub color : u32,
    pub material : u32,
    pub normal : u32,
    pub size : Vector2,
}

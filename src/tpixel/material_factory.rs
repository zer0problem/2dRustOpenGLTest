use crate::tpixel::material_info::MaterialInfo;
use crate::tpixel::texture_factory::TextureFactory;

pub struct MaterialFactory {
    next_id : u32,
}

impl MaterialFactory {
    pub fn new() -> MaterialFactory {
        MaterialFactory {
            next_id : 0u32
        }
    }
    pub fn new_material(&mut self, texture_factory : &mut TextureFactory, color_path : &str, material_path : &str, normal_path : &str) -> MaterialInfo {
        let color = texture_factory.new_texture(color_path);
        let material_info = MaterialInfo {
            id : self.next_id,
            color : color.id,
            size : color.size,
            material : texture_factory.new_texture(material_path).id,
            normal : texture_factory.new_texture(normal_path).id,
        };
        self.next_id += 1u32;
        material_info
    }
}

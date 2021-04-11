use std::path::Path;
use std::os::raw::c_void;
use std::collections::HashMap;
use crate::tpixel::vector2::Vector2;
use crate::tpixel::texture_info::TextureInfo;

pub struct TextureFactory {
    textures : HashMap<String, TextureInfo>,
}

impl TextureFactory {
    pub fn new() -> TextureFactory {
        TextureFactory {
            textures : HashMap::new(),
        }
    }
    pub fn new_texture(&mut self, texture_path : &str) -> &TextureInfo {
        //let texture_wrap = self.textures.get_mut(texture_path);
        let mut texture_info : &mut TextureInfo;
        if self.textures.contains_key(texture_path) {
            texture_info = self.textures.get_mut(texture_path).unwrap();
            texture_info.life_count += 1;
        } else {
            self.textures.insert(texture_path.to_string(), TextureInfo {
                id : 0u32,
                size : Vector2::new(),
                life_count : 1u32,
            });
            texture_info = self.textures.get_mut(texture_path).unwrap();

            unsafe {
                gl::GenTextures(1, &mut texture_info.id);
                gl::BindTexture(gl::TEXTURE_2D, texture_info.id);

                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

                let img = image::open(&Path::new(texture_path)).expect("Failed to load texture");
                let buffer = img.into_rgba8();
                texture_info.size.x = buffer.width() as f32;
                texture_info.size.y = buffer.height() as f32;
                let width = buffer.width() as i32;
                let height = buffer.height() as i32;
                let data = buffer.into_raw();
                gl::TexImage2D(gl::TEXTURE_2D,
                    0,
                    gl::RGBA as i32,
                    width,
                    height,
                    0,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    &data[0] as *const u8 as *const c_void);
                gl::GenerateMipmap(gl::TEXTURE_2D);
            }
        }
        texture_info
    }
}

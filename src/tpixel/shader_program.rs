
use std::ffi::CString;

pub struct ShaderProgram {
    vertex_shader : u32,
    fragment_shader : u32,
    shader_program : u32,
}

impl ShaderProgram {
    pub fn new() -> ShaderProgram {
        ShaderProgram {
            vertex_shader : 0,
            fragment_shader : 0,
            shader_program : 0,
        }
    }
    pub fn new_init(vertex_shader : u32, fragment_shader : u32, shader_program : u32) -> ShaderProgram {
        ShaderProgram {
            vertex_shader : vertex_shader,
            fragment_shader : fragment_shader,
            shader_program : shader_program,
        }
    }
    pub fn get_uniform_location(&self, uniform_name : &str) -> i32 {
        let c_str = CString::new(uniform_name).unwrap();
        unsafe {
            gl::GetUniformLocation(self.shader_program, c_str.as_ptr())
        }
    }
    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.shader_program);
        }
    }
    pub fn drop(&mut self) {
        unsafe {
            //gl::DeleteShader(self.vertex_shader); // 0s are ignored it's fine
            //gl::DeleteShader(self.fragment_shader);
            //gl::DeleteProgram(self.shader_program);
        }
    }
}

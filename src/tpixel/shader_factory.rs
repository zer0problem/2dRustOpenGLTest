use crate::tpixel::shader_program::ShaderProgram;

use gl::types::*;
use std::ffi::CString;
use std::str;
use std::ptr;

pub struct ShaderFactory {

}

impl ShaderFactory {
    pub fn new() -> ShaderFactory {
        ShaderFactory {}
    }
    pub fn new_program(&self, vertex_shader_source : &str, fragment_shader_source : &str) -> ShaderProgram {
        let vertex_shader;
        let fragment_shader;
        let shader_program;
        unsafe {
            vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_vert = CString::new(vertex_shader_source.as_bytes()).unwrap();
            gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);
            
            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(2048);
            info_log.set_len(2048 - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(vertex_shader, 2000, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
            }


            fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_frag = CString::new(fragment_shader_source.as_bytes()).unwrap();
            gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader);
            
            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(2048);
            info_log.set_len(2048 - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(fragment_shader, 2000, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
            }


            shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);

            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(2048);
            info_log.set_len(2048 - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(shader_program, 2000, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
            }
        }
        ShaderProgram::new_init(vertex_shader, fragment_shader, shader_program)
    }
}

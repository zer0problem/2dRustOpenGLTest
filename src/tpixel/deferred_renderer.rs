use crate::tpixel::registry::Registry;
use crate::tpixel::sprite::Sprite;
use crate::tpixel::sparse_map::SparseMap;
use crate::tpixel::vector2::Vector2;
use crate::tpixel::material_info::MaterialInfo;
use crate::tpixel::shader_factory::ShaderFactory;
use crate::tpixel::shader_program::ShaderProgram;
use crate::tpixel::camera::Camera;
use crate::tpixel::color::Color;
use crate::tpixel::point_light::PointLight;
use crate::tpixel::matrix3x2::Matrix3x2;

use gl::types::*;
use std::ptr;
use std::str;
use std::os::raw::c_void;

const VERTEX_GEO_SHADER_SOURCE : &str = r#"
    #version 420 core
    // vertex
    layout (location = 0) in vec2 vertex_pos;
    // instance
    layout (location = 1) in mat3x2 instance_transform;
    layout (location = 4) in vec2 instance_pivot;
    layout (location = 5) in vec4 instance_color;
    layout (location = 6) in vec4 instance_uv;
    layout (location = 7) in float instance_z;
    layout (location = 8) in float instance_height;
    layout (location = 9) in uint instance_id;

    out vec2 UV;
    out vec4 COLOR;

    uniform mat3x2 camera_transform;
    uniform vec2 camera_view;

    uniform vec2 image_size;

    void main() {
        int uv_x = gl_VertexID % 2;
        int uv_y = gl_VertexID / 2;
        UV.x = instance_uv[uv_x*2];//[0];
        UV.y = instance_uv[uv_y*2+1];//[1];

        vec2 quad_size = instance_uv.zw - instance_uv.xy;
        vec2 instance_size = quad_size * image_size;

        vec2 world_pos = instance_transform * vec3(vertex_pos.xy * instance_size - instance_size * instance_pivot, 1.0f);
        vec2 view_pos = camera_transform * vec3(world_pos, 1.0f);
        view_pos /= camera_view;
        gl_Position = vec4(view_pos.x, view_pos.y, -instance_z, 1.0);
        COLOR = instance_color;
    }
"#;

const FRAGMENT_GEO_SHADER_SOURCE : &str = r#"
    #version 420 core
    layout(location=0) out vec4 out_color;
    layout(location=1) out vec4 out_normal;
    layout(location=2) out vec4 out_material;

    in vec2 UV;
    in vec4 COLOR;

    layout(binding=0) uniform sampler2D image_color;
    layout(binding=1) uniform sampler2D image_normal;
    layout(binding=2) uniform sampler2D image_material;

    void main() {
        vec4 color = texture(image_color, UV);
        if (color.a * COLOR.a < 0.1f) {
            //discard;
        }
        out_color = COLOR * color;
        out_normal = texture(image_normal, UV);
        out_material = texture(image_material, UV);
    }
"#;

const VERTEX_FULLSCREEN_SHADER_SOURCE : &str = r#"
    #version 420 core

    out vec2 UV;
    out vec2 WORLD_POSITION;

    uniform mat3x2 camera_transform_inverse;
    uniform vec2 camera_view;

    void main()
    {
        float x = -1.0 + float((gl_VertexID & 1) << 2);
        float y = -1.0 + float((gl_VertexID & 2) << 1);
        UV.x = (x+1.0)*0.5;
        UV.y = (y+1.0)*0.5;

        vec2 view_pos = vec2(x, y);
        WORLD_POSITION = view_pos / camera_view;
        WORLD_POSITION = camera_transform_inverse * vec3(WORLD_POSITION, 1.0);
        
        WORLD_POSITION = view_pos * camera_view;
        WORLD_POSITION = camera_transform_inverse * vec3(WORLD_POSITION, 1.0);

        gl_Position = vec4(x, y, 0, 1);
    }
"#;

const FRAGMENT_AMBIENCE_SHADER_SOURCE : &str = r#"
    #version 420 core
    out vec4 out_color;

    layout(binding=0) uniform sampler2D image_color;
    layout(binding=1) uniform sampler2D image_normal;
    layout(binding=2) uniform sampler2D image_material;

    in vec2 UV;
    in vec2 WORLD_POSITION;

    uniform vec4 world_ambience;

    void main() {
        vec4 col = texture(image_color, UV);
        vec4 mat = texture(image_material, UV);
        vec4 nor = texture(image_normal, UV);
        
        out_color.rgba = col.rgba;
        out_color.rgb *= world_ambience.rgb * world_ambience.a;
    }
"#;

const FRAGMENT_POINT_LIGHT_SHADER_SOURCE : &str = r#"
    #version 420 core
    out vec4 out_color;

    layout(binding=0) uniform sampler2D image_color;
    layout(binding=1) uniform sampler2D image_normal;
    layout(binding=2) uniform sampler2D image_material;

    in vec2 UV;
    in vec2 WORLD_POSITION;

    uniform vec4 light_color;
    uniform vec4 light_position;

    void main() {
        vec4 col = texture(image_color, UV);
        vec4 mat = texture(image_material, UV);
        vec4 nor = texture(image_normal, UV);

        float light_height = light_position.z;
        float light_range = light_position.w;
        vec3 world_pos = vec3(WORLD_POSITION.xy, 0.0); // TODO fix height
        float linear_attenuation = (light_range - length(world_pos - light_position.xyz)) / light_range; 
        float attenuation = pow(max(0.0, linear_attenuation), 2.0);
        vec3 light_color = light_color.rgb * light_color.a;

        vec3 normal = vec3(nor.x * 2.0 - 1.0, nor.y * 2.0 - 1, 0.0);
        normal.z = 1 - (normal.x * normal.x + normal.y * normal.y);
        vec3 dir = normalize(light_position.xyz - world_pos);
        float theta = max(0.0, dot(dir, normal));

        out_color.rgb = col.rgb * light_color * attenuation * theta;
        out_color.a = 1.0;
    }
"#;

struct MaterialPrepInfo {
    material : MaterialInfo,
    batch : Vec<Sprite>,
}

const MAX_GEO_INSTANCE_COUNT : usize = 1024usize;

struct GBuffer {
    renderbuffer : u32,
    framebuffer : u32,
    color : u32,
    normal : u32,
    material : u32,
}

impl GBuffer {
    pub fn new() -> GBuffer {
        GBuffer {
            renderbuffer : 0,
            framebuffer : 0,
            color : 0,
            normal : 0,
            material : 0,
        }
    }
    pub fn drop(&mut self) {
        self.free();
    }
    pub fn build(&mut self, width : i32, height : i32) {
        unsafe {
            gl::GenFramebuffers(1, &mut self.framebuffer);
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer);

            gl::GenTextures(1, &mut self.color);
            gl::BindTexture(gl::TEXTURE_2D, self.color);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA8 as GLint, width, height, 0, gl::RGBA, gl::UNSIGNED_BYTE, ptr::null());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, self.color, 0);

            gl::GenTextures(1, &mut self.normal);
            gl::BindTexture(gl::TEXTURE_2D, self.normal);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA8 as GLint, width, height, 0, gl::RGBA, gl::UNSIGNED_BYTE, ptr::null());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT1, gl::TEXTURE_2D, self.normal, 0);

            gl::GenTextures(1, &mut self.material);
            gl::BindTexture(gl::TEXTURE_2D, self.material);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA8 as GLint, width, height, 0, gl::RGBA, gl::UNSIGNED_BYTE, ptr::null());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT2, gl::TEXTURE_2D, self.material, 0);

            let attachments = [gl::COLOR_ATTACHMENT0, gl::COLOR_ATTACHMENT1, gl::COLOR_ATTACHMENT2];
            gl::DrawBuffers(3, &attachments[0]);

            gl::GenRenderbuffers(1, &mut self.renderbuffer);
            gl::BindRenderbuffer(gl::RENDERBUFFER, self.renderbuffer);
            gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT, width, height);
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, self.renderbuffer);

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                println!("Framebuffer did not complete!");
            }
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }
    pub fn rebuild(&mut self, width : i32, height : i32) {
        self.free();
        self.build(width, height);
    }
    pub fn free(&mut self) {
        unsafe {
            gl::DeleteRenderbuffers(1, &self.framebuffer);
            gl::DeleteFramebuffers(1, &self.framebuffer);
            gl::DeleteTextures(1, &self.color);
            gl::DeleteTextures(1, &self.normal);
            gl::DeleteTextures(1, &self.material);
        }
    }
}

pub struct DeferredRenderer {
    geometry_shader : ShaderProgram,
    ambience_shader : ShaderProgram,
    point_light_shader : ShaderProgram,

    gbuffer : GBuffer,

    geometry_shader_camera_transform : i32,
    geometry_shader_camera_view : i32,
    
    geometry_shader_image_size : i32,
    geometry_shader_image_color : i32,
    geometry_shader_image_material : i32,
    geometry_shader_image_normal : i32,

    ambience_shader_camera_transform_inverse : i32,
    ambience_shader_camera_view : i32,

    ambience_shader_world_ambience : i32,

    point_light_shader_camera_transform_inverse : i32,
    point_light_shader_camera_view : i32,

    point_light_shader_light_color : i32,
    point_light_shader_light_position : i32,

    instance_buffer_object : u32,
    vertex_buffer_object : u32,
    vertex_array_object : u32,
    
    material_preps : SparseMap<MaterialPrepInfo>,
}

impl DeferredRenderer {
    pub fn new() -> DeferredRenderer {
        DeferredRenderer {
            geometry_shader : ShaderProgram::new(),
            ambience_shader : ShaderProgram::new(),
            point_light_shader : ShaderProgram::new(),

            gbuffer : GBuffer::new(),

            geometry_shader_camera_transform : 0,
            geometry_shader_camera_view : 0,

            geometry_shader_image_size : 0,
            geometry_shader_image_color : 0,
            geometry_shader_image_material : 0,
            geometry_shader_image_normal : 0,

            ambience_shader_camera_transform_inverse : 0,
            ambience_shader_camera_view : 0,

            ambience_shader_world_ambience : 0,

            point_light_shader_camera_transform_inverse : 0,
            point_light_shader_camera_view : 0,
        
            point_light_shader_light_color : 0,
            point_light_shader_light_position : 0,
            
            instance_buffer_object : 0,
            vertex_buffer_object : 0,
            vertex_array_object : 0,
            
            material_preps : SparseMap::new(),
        }
    }
    pub fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.instance_buffer_object);
            gl::DeleteBuffers(1, &self.vertex_buffer_object);
            gl::DeleteVertexArrays(1, &self.vertex_array_object);
        }
    }
    pub fn prepare_material(&mut self, material_info : &MaterialInfo) {
        if !self.material_preps.contains_key(material_info.id) {
            self.material_preps.insert(material_info.id, MaterialPrepInfo{
                material : *material_info,
                batch : Vec::new(),
            });
        }
    }
    pub fn init(&mut self, shader_factory : &ShaderFactory) {
        self.geometry_shader = shader_factory.new_program(VERTEX_GEO_SHADER_SOURCE, FRAGMENT_GEO_SHADER_SOURCE);
        self.ambience_shader = shader_factory.new_program(VERTEX_FULLSCREEN_SHADER_SOURCE, FRAGMENT_AMBIENCE_SHADER_SOURCE);
        self.point_light_shader = shader_factory.new_program(VERTEX_FULLSCREEN_SHADER_SOURCE, FRAGMENT_POINT_LIGHT_SHADER_SOURCE);
        
        self.geometry_shader_camera_transform = self.geometry_shader.get_uniform_location("camera_transform");
        self.geometry_shader_camera_view = self.geometry_shader.get_uniform_location("camera_view");

        self.geometry_shader_image_size = self.geometry_shader.get_uniform_location("image_size");
        self.geometry_shader_image_color = self.geometry_shader.get_uniform_location("image_color");
        self.geometry_shader_image_material = self.geometry_shader.get_uniform_location("image_material");
        self.geometry_shader_image_normal = self.geometry_shader.get_uniform_location("image_normal");

        self.ambience_shader_camera_transform_inverse = self.ambience_shader.get_uniform_location("camera_transform_inverse");
        self.ambience_shader_camera_view = self.ambience_shader.get_uniform_location("camera_view");

        self.ambience_shader_world_ambience = self.ambience_shader.get_uniform_location("world_ambience");

        self.point_light_shader_camera_transform_inverse = self.point_light_shader.get_uniform_location("camera_transform_inverse");
        self.point_light_shader_camera_view = self.point_light_shader.get_uniform_location("camera_view");
    
        self.point_light_shader_light_color = self.point_light_shader.get_uniform_location("light_color");
        self.point_light_shader_light_position = self.point_light_shader.get_uniform_location("light_position");

        unsafe {
            // TODO encapsulate
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            
            gl::GenBuffers(1, &mut self.instance_buffer_object);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.instance_buffer_object);
            gl::BufferData(gl::ARRAY_BUFFER, (std::mem::size_of::<Sprite>() * MAX_GEO_INSTANCE_COUNT) as isize, 0 as *const f32 as *const c_void, gl::DYNAMIC_DRAW);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            
            let verts : [f32; 8] = [
                // positions
                0.0f32,  1.0f32,// 0.0f32, 
                1.0f32,  1.0f32,// 0.0f32,
                0.0f32,  0.0f32,// 0.0f32,
                1.0f32,  0.0f32,// 0.0f32, 
            ];

            gl::GenBuffers(1, &mut self.vertex_buffer_object);

            gl::GenVertexArrays(1, &mut self.vertex_array_object);
            gl::BindVertexArray(self.vertex_array_object);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer_object);
            gl::BufferData(gl::ARRAY_BUFFER,
                (std::mem::size_of::<f32>() * verts.len()) as isize,
                &verts[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, (std::mem::size_of::<f32>() * 2) as i32, 0 as *const c_void);
            // instance stuff
            gl::BindBuffer(gl::ARRAY_BUFFER, self.instance_buffer_object);
            let stride = (std::mem::size_of::<f32>() * 19) as i32;
            // transform
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (std::mem::size_of::<f32>() * 0) as *const c_void);
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, (std::mem::size_of::<f32>() * 2) as *const c_void);
            gl::EnableVertexAttribArray(3);
            gl::VertexAttribPointer(3, 2, gl::FLOAT, gl::FALSE, stride, (std::mem::size_of::<f32>() * 4) as *const c_void);
            // pivot
            gl::EnableVertexAttribArray(4);
            gl::VertexAttribPointer(4, 2, gl::FLOAT, gl::FALSE, stride, (std::mem::size_of::<f32>() * 6) as *const c_void);
            // color
            gl::EnableVertexAttribArray(5);
            gl::VertexAttribPointer(5, 4, gl::FLOAT, gl::FALSE, stride, (std::mem::size_of::<f32>() * 8) as *const c_void);
            // uv
            gl::EnableVertexAttribArray(6);
            gl::VertexAttribPointer(6, 4, gl::FLOAT, gl::FALSE, stride, (std::mem::size_of::<f32>() * 12) as *const c_void);
            // z
            gl::EnableVertexAttribArray(7);
            gl::VertexAttribPointer(7, 1, gl::FLOAT, gl::FALSE, stride, (std::mem::size_of::<f32>() * 16) as *const c_void);
            // height
            gl::EnableVertexAttribArray(8);
            gl::VertexAttribPointer(8, 1, gl::FLOAT, gl::FALSE, stride, (std::mem::size_of::<f32>() * 17) as *const c_void);
            // id
            gl::EnableVertexAttribArray(9);
            gl::VertexAttribPointer(9, 1, gl::UNSIGNED_INT, gl::FALSE, stride, (std::mem::size_of::<f32>() * 18) as *const c_void);
            
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::VertexAttribDivisor(1, 1);
            gl::VertexAttribDivisor(2, 1);
            gl::VertexAttribDivisor(3, 1);
            gl::VertexAttribDivisor(4, 1);
            gl::VertexAttribDivisor(5, 1);
            gl::VertexAttribDivisor(6, 1);
            gl::VertexAttribDivisor(7, 1);
            gl::VertexAttribDivisor(8, 1);
            gl::VertexAttribDivisor(9, 1);


            self.gbuffer.build(1280, 720);
        }
    }
    pub fn render(&mut self, registry : &Registry, camera : &Camera, ambient_color : &Color) {
        self.generate_gbuffer(registry, camera);
        self.render_gbuffer(registry, camera, ambient_color);
    }

    pub fn resize_geo_buffer(&mut self, width : i32, height : i32) {
        self.gbuffer.rebuild(width, height);
    }
    fn generate_gbuffer(&mut self, registry : &Registry, camera : &Camera) {
        unsafe {
            gl::Disable(gl::BLEND);

            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::BindFramebuffer(gl::FRAMEBUFFER, self.gbuffer.framebuffer);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            self.geometry_shader.use_program();
        }

        let camera_transform : Matrix3x2 = camera.transform.inverse();
        
        let camera_view = Vector2 { x : camera.view_size.x / 2.0f32, y : camera.view_size.y / 2.0f32 };

        let sprite_map = registry.get_map::<Sprite>();
        for sprite_kv in sprite_map.all_iter() {
            let material_object = &sprite_kv.value.material_id;
            let material_info = self.material_preps.get_mut(*material_object);
            material_info.batch.push(sprite_kv.value.clone());
        }
        
        unsafe {
            gl::UniformMatrix3x2fv(self.geometry_shader_camera_transform, 1, gl::FALSE, &camera_transform.elements[0]);
            gl::Uniform2fv(self.geometry_shader_camera_view, 1, &camera_view.x);
        }

        for material_prep in self.material_preps.all_iter_mut() {
            let material_info = &mut material_prep.value;
            let color : u32 = material_info.material.color;
            let material : u32 = material_info.material.material;
            let normal : u32 = material_info.material.normal;

            let count : i32 = material_info.batch.len() as i32;
            let size : isize = (std::mem::size_of::<Sprite>() * material_info.batch.len()) as isize;
            unsafe {
                gl::NamedBufferSubData(
                    self.instance_buffer_object,
                    0,
                    size,
                    &material_info.batch[0].transform.elements[0] as *const f32 as *const c_void);

                // bind texture
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, color);
                gl::ActiveTexture(gl::TEXTURE1);
                gl::BindTexture(gl::TEXTURE_2D, normal);
                gl::ActiveTexture(gl::TEXTURE2);
                gl::BindTexture(gl::TEXTURE_2D, material);

                gl::Uniform2fv(self.geometry_shader_image_size, 1, &material_info.material.size.x);
                
                // render
                gl::BindVertexArray(self.vertex_array_object);
                gl::DrawArraysInstanced(gl::TRIANGLE_STRIP, 0, 4, count);
                gl::BindVertexArray(0);
            }
            material_info.batch.clear();
        }
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }
    fn render_gbuffer(&self, registry : &Registry, camera : &Camera, ambient_color : &Color) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.gbuffer.color);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, self.gbuffer.normal);
            gl::ActiveTexture(gl::TEXTURE2);
            gl::BindTexture(gl::TEXTURE_2D, self.gbuffer.material);
        }
        self.render_ambience(camera, ambient_color);
        self.render_lights(registry, camera);
    }
    fn render_ambience(&self, camera : &Camera, ambient_color : &Color) {
        unsafe {
            let camera_view = Vector2 { x : camera.view_size.x / 2.0f32, y : camera.view_size.y / 2.0f32 };
            let camera_transform = camera.transform;

            gl::Disable(gl::BLEND);

            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            self.ambience_shader.use_program();
            
            gl::UniformMatrix3x2fv(self.ambience_shader_camera_transform_inverse, 1, gl::FALSE, &camera_transform.elements[0]);
            gl::Uniform2fv(self.ambience_shader_camera_view, 1, &camera_view.x);

            gl::Uniform4fv(self.ambience_shader_world_ambience, 1, &ambient_color.r);

            gl::BindVertexArray(self.vertex_array_object);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::BindVertexArray(0);
        }
    }
    fn render_lights(&self, registry : &Registry, camera : &Camera) {
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::ONE, gl::ONE);
            gl::Disable(gl::DEPTH_TEST);
        }
        
        let camera_view = Vector2 { x : camera.view_size.x / 2.0f32, y : camera.view_size.y / 2.0f32 };
        let camera_transform = camera.transform;
        
        self.point_light_shader.use_program();
        
        unsafe {
            gl::UniformMatrix3x2fv(self.point_light_shader_camera_transform_inverse, 1, gl::FALSE, &camera_transform.elements[0]);
            gl::Uniform2fv(self.point_light_shader_camera_view, 1, &camera_view.x);
        }
        
        let light_map = registry.get_map::<PointLight>();
        for light_kv in light_map.all_iter() {
            unsafe {
                gl::Uniform4fv(self.point_light_shader_light_color, 1, &light_kv.value.color.r);
                gl::Uniform4fv(self.point_light_shader_light_position, 1, &light_kv.value.position.x);
                
                gl::BindVertexArray(self.vertex_array_object);
                gl::DrawArrays(gl::TRIANGLES, 0, 3);
                gl::BindVertexArray(0);
            }
        }
    }
}

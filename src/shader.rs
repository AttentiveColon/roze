use std::collections::HashMap;

use crate::{renderer::GL, system::gl32::*};

pub struct Shader {
    shader_ids: HashMap<String, GLuint>,
}

impl Shader {
    pub fn new() -> Self {
        Self {
            shader_ids: HashMap::new(),
        }
    }
    pub fn _load(
        &mut self,
        gl: &GL,
        vertex_path: &str,
        fragment_path: &str,
        shader_name: &str,
    ) -> GLuint {
        let shader_id = _shader_from_file(gl, vertex_path, fragment_path);

        self.shader_ids.insert(String::from(shader_name), shader_id);
        #[cfg(debug_assertions)]
        println!("Inserting Shader: {:?} with id: {shader_id}", shader_name);
        shader_id
    }
    pub fn load_raw(
        &mut self,
        gl: &GL,
        raw_vertex: String,
        raw_fragment: String,
        shader_name: &str,
    ) -> GLuint {
        let shader_id = shader_from_raw(gl, raw_vertex, raw_fragment);

        self.shader_ids.insert(String::from(shader_name), shader_id);
        #[cfg(debug_assertions)]
        println!("Inserting Shader: {:?} with id: {shader_id}", shader_name);
        shader_id
    }
    pub fn get(&self, sname: &str) -> GLuint {
        let shader_id = self.shader_ids.get(sname);
        if let Some(shader_id) = shader_id {
            *shader_id
        } else {
            eprintln!("get_shader failed: '{sname}' does not exist");
            0
        }
    }
}

pub fn _shader_from_file(gl: &GL, vertex_file_path: &str, fragment_file_path: &str) -> GLuint {
    use std::fs::read_to_string;
    let vertex_shader_code =
        read_to_string(vertex_file_path).expect("Couldn't read vertex shader!");
    let fragment_shader_code =
        read_to_string(fragment_file_path).expect("Couldn't read fragment shader!");

    load_shaders(
        gl,
        vertex_shader_code,
        fragment_shader_code,
        vertex_file_path,
        fragment_file_path,
    )
}

pub fn shader_from_raw(gl: &GL, raw_vertex: String, raw_fragment: String) -> GLuint {
    load_shaders(
        gl,
        raw_vertex,
        raw_fragment,
        "internal: vertex_shader",
        "interal: fragment_shader",
    )
}

#[allow(unused_variables)]
#[rustfmt::skip]
pub fn load_shaders(gl: &GL, mut raw_vertex: String, mut raw_fragment: String, vertex_file_path: &str, fragment_file_path: &str) -> GLuint {
    let vertex_shader_id: GLuint = gl.CreateShader(GL_VERTEX_SHADER);
    let fragment_shader_id: GLuint = gl.CreateShader(GL_FRAGMENT_SHADER);


    let result: GLint = GL_FALSE as i32;
    let info_log_length = 0;

    raw_vertex.push('\0');
    let vertex_shader_ptr = raw_vertex.as_ptr();

    #[cfg(debug_assertions)]
    println!("Compiling shader: {}", vertex_file_path);
    gl.ShaderSource(vertex_shader_id, 1, &vertex_shader_ptr, std::ptr::null());
    gl.CompileShader(vertex_shader_id);
    gl.GetShaderiv(vertex_shader_id, GL_COMPILE_STATUS, &result);
    gl.GetShaderiv(vertex_shader_id, GL_INFO_LOG_LENGTH, &info_log_length);

    if info_log_length > 0 {
        println!("Compilation Failed!");
        println!("Result: {result}");
        println!("Info Log Length: {info_log_length}");

        let error_message = vec![0; info_log_length as usize];

        gl.GetShaderInfoLog(
            vertex_shader_id,
            info_log_length,
            std::ptr::null(),
            error_message.as_ptr(),
        );

        let error = String::from_utf8(error_message).unwrap();
        println!("error message: {error}");
    } else {
        #[cfg(debug_assertions)]
        println!("Compilation Successful");
    }

    raw_fragment.push('\0');
    let fragment_shader_ptr = raw_fragment.as_ptr();

    #[cfg(debug_assertions)]
    println!("Compiling shader: {}", fragment_file_path);
    gl.ShaderSource(
        fragment_shader_id,
        1,
        &fragment_shader_ptr,
        std::ptr::null(),
    );
    gl.CompileShader(fragment_shader_id);
    gl.GetShaderiv(fragment_shader_id, GL_COMPILE_STATUS, &result);
    gl.GetShaderiv(fragment_shader_id, GL_INFO_LOG_LENGTH, &info_log_length);
    if info_log_length > 0 {
        println!("Compilation Failed!");
        println!("Result: {result}");
        println!("Info Log Length: {info_log_length}");

        let error_message = vec![0; info_log_length as usize];

        gl.GetShaderInfoLog(
            fragment_shader_id,
            info_log_length,
            std::ptr::null(),
            error_message.as_ptr(),
        );

        let error = String::from_utf8(error_message).unwrap();
        println!("error message: {error}");
    } else {
        #[cfg(debug_assertions)]
        println!("Compilation Successful");
    }

    #[cfg(debug_assertions)]
    println!("Linking shader program");
    let program_id = gl.CreateProgram();
    gl.AttachShader(program_id, vertex_shader_id);
    gl.AttachShader(program_id, fragment_shader_id);
    gl.LinkProgram(program_id);

    gl.GetProgramiv(program_id, GL_LINK_STATUS, &result);
    gl.GetProgramiv(program_id, GL_INFO_LOG_LENGTH, &info_log_length);

    if info_log_length > 0 {
        println!("Shader program creation failed!");
        println!("Result: {result}");
        println!("Info Log Length: {info_log_length}");

        let error_message = vec![0; info_log_length as usize];

        gl.GetProgramInfoLog(
            program_id,
            info_log_length,
            std::ptr::null(),
            error_message.as_ptr(),
        );

        let error = String::from_utf8(error_message).unwrap();
        println!("error message: {error}");
    } else {
        #[cfg(debug_assertions)]
        println!("Shader program creation succeeded!");
    }

    gl.DetachShader(program_id, vertex_shader_id);
    gl.DetachShader(program_id, fragment_shader_id);

    gl.DeleteShader(vertex_shader_id);
    gl.DeleteShader(fragment_shader_id);

    program_id
}

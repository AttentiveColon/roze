use crate::shader::*;
use crate::sprite::ParticleSystem;
use crate::sprite::TextBuilder;
use crate::sprite::{RectBuilder, SpriteBuilder};
use crate::system::gl32::*;
use crate::system::win32::*;
use crate::texture::Texture;
use std::ffi::c_void;
use std::ffi::OsStr;
use std::mem::transmute;
use std::os::windows::prelude::OsStrExt;

pub struct Renderer {
    device_context: HDC,
    _gl_context: HGLRC,
    pub gl: Box<GL>,
    pub rect: RectBuilder,
    pub sprite: SpriteBuilder,
    pub text: TextBuilder,
    pub particle: ParticleSystem,
    pub shader: Shader,
    pub texture: Texture,
}

#[allow(non_snake_case)]
#[rustfmt::skip]
pub struct GL {
    pub glGenVertexArrays: fn(n: GLsizei, arrays: &GLuint),
    pub glBindVertexArray: fn(array: GLuint),
    pub glGenBuffers: fn(n: GLsizei, buffers: &mut GLuint),
    pub glBindBuffer: fn(target: GLenum, buffer: GLuint),
    pub glBufferData: fn(target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum),
    pub glVertexAttribPointer: fn( index: GLuint, size: GLint, types: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const c_void),
    pub glDrawArrays: fn(mode: GLenum, first: GLint, count: GLsizei),
    pub glEnableVertexAttribArray: fn(index: GLuint),
    pub glDisableVertexAttribArray: fn(index: GLuint),
    pub glCreateShader: fn(shader_type: GLenum) -> GLuint,
    pub glShaderSource: fn(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint),
    pub glCompileShader: fn(shader: GLuint),
    pub glGetShaderiv: fn(shader: GLuint, pname: GLenum, params: &GLint),
    pub glGetShaderInfoLog: fn(shader: GLuint, max_length: GLsizei, length: *const GLsizei, info_log: *const GLchar),
    pub glCreateProgram: fn() -> GLuint,
    pub glAttachShader: fn(program: GLuint, shader: GLuint),
    pub glLinkProgram: fn(program: GLuint),
    pub glGetProgramiv: fn(program: GLuint, pname: GLenum, params: &GLint),
    pub glGetProgramInfoLog: fn(program: GLuint, max_length: GLsizei, length: *const GLsizei, info_log: *const GLchar),
    pub glDetachShader: fn(program: GLuint, shader: GLuint),
    pub glDeleteShader: fn(shader: GLuint),
    pub glUseProgram: fn(program: GLuint),
    pub glGetUniformLocation: fn(program: GLuint, name: *const GLchar) -> GLint,
    pub glUniform4fv: fn(location: GLint, count: GLsizei, value: *const GLfloat),
    pub glUniformMatrix4fv: fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat),
    pub glGenTextures:fn(n: GLsizei, textures: *const GLuint),
    pub glBindTexture:fn(target: GLenum, texture: GLuint),
    pub glCompressedTexImage2D: fn(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void,),
    pub glGenerateMipmap:fn(target: GLenum),
    pub glDebugMessageCallback: fn(callback: *const c_void, userParam: *const c_void),
}

#[allow(non_snake_case, dead_code, clippy::too_many_arguments)]
impl GL {
    #[rustfmt::skip]
    pub fn new() -> Box<Self> {
        unsafe {
            let glGenVertexArrays: glGenVertexArrays = transmute(wglGetProcAddressChecked("glGenVertexArrays"));
            let glBindVertexArray: glBindVertexArray = transmute(wglGetProcAddressChecked("glBindVertexArray"));
            let glGenBuffers: glGenBuffers = transmute(wglGetProcAddressChecked("glGenBuffers"));
            let glBindBuffer: glBindBuffer = transmute(wglGetProcAddressChecked("glBindBuffer"));
            let glBufferData: glBufferData = transmute(wglGetProcAddressChecked("glBufferData"));
            let glVertexAttribPointer: glVertexAttribPointer = transmute(wglGetProcAddressChecked("glVertexAttribPointer"));
            let glDrawArrays: glDrawArrays = transmute(wglGetProcAddressChecked("glDrawArrays"));
            let glEnableVertexAttribArray: glEnableVertexAttribArray = transmute(wglGetProcAddressChecked("glEnableVertexAttribArray"));
            let glDisableVertexAttribArray: glDisableVertexAttribArray = transmute(wglGetProcAddressChecked("glDisableVertexAttribArray"));
            let glCreateShader: glCreateShader = transmute(wglGetProcAddressChecked("glCreateShader"));
            let glShaderSource: glShaderSource = transmute(wglGetProcAddressChecked("glShaderSource"));
            let glCompileShader: glCompileShader = transmute(wglGetProcAddressChecked("glCompileShader"));
            let glGetShaderiv: glGetShaderiv = transmute(wglGetProcAddressChecked("glGetShaderiv"));
            let glGetShaderInfoLog: glGetShaderInfoLog = transmute(wglGetProcAddressChecked("glGetShaderInfoLog"));
            let glCreateProgram: glCreateProgram = transmute(wglGetProcAddressChecked("glCreateProgram"));
            let glAttachShader: glAttachShader = transmute(wglGetProcAddressChecked("glAttachShader"));
            let glLinkProgram: glLinkProgram = transmute(wglGetProcAddressChecked("glLinkProgram"));
            let glGetProgramiv: glGetProgramiv = transmute(wglGetProcAddressChecked("glGetProgramiv"));
            let glGetProgramInfoLog: glGetProgramInfoLog = transmute(wglGetProcAddressChecked("glGetProgramInfoLog"));
            let glDetachShader: glDetachShader = transmute(wglGetProcAddressChecked("glDetachShader"));
            let glDeleteShader: glDeleteShader = transmute(wglGetProcAddressChecked("glDeleteShader"));
            let glUseProgram: glUseProgram = transmute(wglGetProcAddressChecked("glUseProgram"));
            let glGetUniformLocation: glGetUniformLocation = transmute(wglGetProcAddressChecked("glGetUniformLocation"));
            let glUniform4fv: glUniform4fv = transmute(wglGetProcAddressChecked("glUniform4fv"));
            let glUniformMatrix4fv: glUniformMatrix4fv = transmute(wglGetProcAddressChecked("glUniformMatrix4fv"));
            let glGenTextures: glGenTextures = transmute(wglGetProcAddressChecked("glGenTextures"));
            let glBindTexture: glBindTexture = transmute(wglGetProcAddressChecked("glBindTexture"));
            let glCompressedTexImage2D: glCompressedTexImage2D = transmute(wglGetProcAddressChecked("glCompressedTexImage2D"));
            let glGenerateMipmap: glGenerateMipmap = transmute(wglGetProcAddressChecked("glGenerateMipmap"));
            let glDebugMessageCallback: glDebugMessageCallback = transmute(wglGetProcAddressChecked("glDebugMessageCallback"));

            Box::new(GL{
                glGenVertexArrays,
                glBindVertexArray,
                glGenBuffers,
                glBindBuffer,
                glBufferData,
                glVertexAttribPointer,
                glDrawArrays,
                glEnableVertexAttribArray,
                glDisableVertexAttribArray,
                glCreateShader,
                glShaderSource,
                glCompileShader,
                glGetShaderiv,
                glGetShaderInfoLog,
                glCreateProgram,
                glAttachShader,
                glLinkProgram,
                glGetProgramiv,
                glGetProgramInfoLog,
                glDetachShader,
                glDeleteShader,
                glUseProgram,
                glGetUniformLocation,
                glUniform4fv,
                glUniformMatrix4fv,
                glGenTextures,
                glBindTexture,
                glCompressedTexImage2D,
                glGenerateMipmap,
                glDebugMessageCallback,
            })
        }
    }

    pub fn GenVertexArrays(&self, n: GLsizei, arrays: &GLuint) {
        (self.glGenVertexArrays)(n, arrays);
        open_gl_error("glGenVertexArrays");
    }
    pub fn BindVertexArray(&self, array: GLuint) {
        (self.glBindVertexArray)(array);
        open_gl_error("glBindVertexArray");
    }
    pub fn GenBuffers(&self, n: GLsizei, buffers: &mut GLuint) {
        (self.glGenBuffers)(n, buffers);
        open_gl_error("glGenBuffers");
    }
    pub fn BindBuffer(&self, target: GLenum, buffer: GLuint) {
        (self.glBindBuffer)(target, buffer);
        open_gl_error("glBindBuffer");
    }
    pub fn BufferData(&self, target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum) {
        (self.glBufferData)(target, size, data, usage);
        open_gl_error("glBufferData");
    }
    pub fn VertexAttribPointer(
        &self,
        index: GLuint,
        size: GLint,
        types: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const c_void,
    ) {
        (self.glVertexAttribPointer)(index, size, types, normalized, stride, pointer);
        open_gl_error("glVertexAttribPointer");
    }
    pub fn DrawArrays(&self, mode: GLenum, first: GLint, count: GLsizei) {
        (self.glDrawArrays)(mode, first, count);
        open_gl_error("glDrawArrays");
    }
    pub fn EnableVertexAttribArray(&self, index: GLuint) {
        (self.glEnableVertexAttribArray)(index);
        open_gl_error("glEnableVertexAttribArray");
    }
    pub fn DisableVertexAttribArray(&self, index: GLuint) {
        (self.glDisableVertexAttribArray)(index);
        open_gl_error("glDisableVertexAttribArray");
    }
    pub fn CreateShader(&self, shader_type: GLenum) -> GLuint {
        let res = (self.glCreateShader)(shader_type);
        open_gl_error("glCreateShader");
        res
    }
    pub fn ShaderSource(
        &self,
        shader: GLuint,
        count: GLsizei,
        string: *const *const GLchar,
        length: *const GLint,
    ) {
        (self.glShaderSource)(shader, count, string, length);
        open_gl_error("glShaderSource");
    }
    pub fn CompileShader(&self, shader: GLuint) {
        (self.glCompileShader)(shader);
        open_gl_error("glCompileShader");
    }
    pub fn GetShaderiv(&self, shader: GLuint, pname: GLenum, params: &GLint) {
        (self.glGetShaderiv)(shader, pname, params);
        open_gl_error("glGetShaderiv");
    }
    pub fn GetShaderInfoLog(
        &self,
        shader: GLuint,
        max_length: GLsizei,
        length: *const GLsizei,
        info_log: *const GLchar,
    ) {
        (self.glGetShaderInfoLog)(shader, max_length, length, info_log);
        open_gl_error("glGetShaderInfoLog");
    }
    pub fn CreateProgram(&self) -> GLuint {
        let res = (self.glCreateProgram)();
        open_gl_error("glCreateProgram");
        res
    }
    pub fn AttachShader(&self, program: GLuint, shader: GLuint) {
        (self.glAttachShader)(program, shader);
        open_gl_error("glAttachShader");
    }
    pub fn LinkProgram(&self, program: GLuint) {
        (self.glLinkProgram)(program);
        open_gl_error("glLinkProgram");
    }
    pub fn GetProgramiv(&self, program: GLuint, pname: GLenum, params: &GLint) {
        (self.glGetProgramiv)(program, pname, params);
        open_gl_error("glGetProgramiv");
    }
    pub fn GetProgramInfoLog(
        &self,
        program: GLuint,
        max_length: GLsizei,
        length: *const GLsizei,
        info_log: *const GLchar,
    ) {
        (self.glGetProgramInfoLog)(program, max_length, length, info_log);
        open_gl_error("glGetProgramInfoLog");
    }
    pub fn DetachShader(&self, program: GLuint, shader: GLuint) {
        (self.glDetachShader)(program, shader);
        open_gl_error("glDetachShader");
    }
    pub fn DeleteShader(&self, shader: GLuint) {
        (self.glDeleteShader)(shader);
        open_gl_error("glDeleteShader");
    }
    pub fn UseProgram(&self, program: GLuint) {
        (self.glUseProgram)(program);
        open_gl_error("glUseProgram");
    }
    pub fn GetUniformLocation(&self, program: GLuint, name: *const GLchar) -> GLint {
        let res = (self.glGetUniformLocation)(program, name);
        open_gl_error("glGetUniformLocation");
        res
    }
    pub fn Uniform4fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) {
        (self.glUniform4fv)(location, count, value);
        open_gl_error("glUniform4fv");
    }
    pub fn UniformMatrix4fv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        (self.glUniformMatrix4fv)(location, count, transpose, value);
        open_gl_error("glUniformMatrix4fv");
    }
    pub fn GenTextures(&self, n: GLsizei, textures: *const GLuint) {
        (self.glGenTextures)(n, textures);
        open_gl_error("glGenTextures");
    }
    pub fn BindTexture(&self, target: GLenum, texture: GLuint) {
        (self.glBindTexture)(target, texture);
        open_gl_error("glBindTextures");
    }
    pub fn CompressedTexImage2D(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        imageSize: GLsizei,
        data: *const c_void,
    ) {
        (self.glCompressedTexImage2D)(
            target,
            level,
            internalformat,
            width,
            height,
            border,
            imageSize,
            data,
        );
        open_gl_error("glCompressedTexImage2D");
    }
    pub fn GenerateMipmap(&self, target: GLenum) {
        (self.glGenerateMipmap)(target);
        open_gl_error("glGenerateMipmap");
    }
    pub fn DebugMessageCallback(&self, callback: *const c_void, userParam: *const c_void) {
        (self.glDebugMessageCallback)(callback, userParam);
        open_gl_error("glDebugMessageCallback");
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    //Non-import Functions
    ///////////////////////////////////////////////////////////////////////////////////////////////////

    pub fn GetString(&self, name: u32) -> *mut u8 {
        unsafe {
            let res = glGetString(name);
            open_gl_error("glGetString");
            res
        }
    }
    pub fn Enable(&self, cap: u32) {
        unsafe {
            glEnable(cap);
            open_gl_error("glEnable");
        }
    }
    pub fn Hint(&self, target: u32, mode: u32) {
        unsafe {
            glHint(target, mode);
            open_gl_error("glHint");
        }
    }
    pub fn ClearColor(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe {
            glClearColor(red, green, blue, alpha);
            open_gl_error("glClearColor");
        }
    }
    pub fn Clear(&self, mask: u32) {
        unsafe {
            glClear(mask);
            open_gl_error("glClear");
        }
    }
    pub fn DepthFunc(&self, cap: GLenum) {
        unsafe {
            glDepthFunc(cap);
            open_gl_error("glDepthFunc");
        }
    }
    pub fn TexImage2D(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        data: *const c_void,
    ) {
        unsafe {
            glTexImage2D(
                target,
                level,
                internalformat as i32,
                width,
                height,
                border,
                format,
                type_,
                data,
            );
            open_gl_error("glTexImage2D");
        }
    }
    pub fn TexParameteri(&self, target: GLenum, pname: GLenum, param: GLint) {
        unsafe {
            glTexParameteri(target, pname, param);
            open_gl_error("glTexParameteri");
        }
    }
    pub fn BlendFunc(&self, sfactor: GLenum, dfactor: GLenum) {
        unsafe {
            glBlendFunc(sfactor, dfactor);
            open_gl_error("glBlendFunc");
        }
    }
    pub fn Disable(&self, cap: GLenum) {
        unsafe {
            glDisable(cap);
            open_gl_error("glDisable");
        }
    }
}

#[allow(dead_code)]
impl Renderer {
    pub fn new(device_context: HDC, pool_size: usize) -> Box<Self> {
        unsafe {
            let pfd = crate::system::gl32::PFD;
            let pixel_format = ChoosePixelFormat(device_context, &pfd);
            assert!(
                SetPixelFormat(device_context, pixel_format, &pfd),
                "Failed to set pixel format"
            );

            let _gl_context = wglCreateContext(device_context);
            wglMakeCurrent(device_context, _gl_context);

            let gl = GL::new();

            let vertex_array_id: GLuint = 0;
            gl.GenVertexArrays(1, &vertex_array_id);
            gl.BindVertexArray(vertex_array_id);

            #[allow(non_snake_case)]
            let wglSwapInterval: wglSwapInterval =
                transmute(wglGetProcAddressChecked("wglSwapIntervalEXT"));
            wglSwapInterval(0);

            gl.Enable(GL_BLEND);
            //gl.Enable(GL_ALPHA_TEST);
            gl.BlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

            gl.Enable(GL_TEXTURE_2D);
            gl.Hint(GL_PERSPECTIVE_CORRECTION_HINT, GL_NICEST);
            gl.Enable(GL_DEPTH_TEST);

            gl.DepthFunc(GL_LEQUAL);

            #[cfg(debug_assertions)]
            gl.Enable(GL_DEBUG_OUTPUT);
            gl.DebugMessageCallback(
                crate::system::gl32::MessageCallback as *const c_void,
                std::ptr::null(),
            );

            let mut shader = Shader::new();
            shader.load_raw(
                &gl,
                SPRITE_SHADER_VERT.to_owned(),
                SPRITE_SHADER_FRAG.to_owned(),
                "sprite_shader",
            );
            shader.load_raw(
                &gl,
                LINE_SHADER_VERT.to_owned(),
                LINE_SHADER_FRAG.to_owned(),
                "line_shader",
            );
            shader.load_raw(
                &gl,
                PARTICLE_SHADER_VERT.to_owned(),
                PARTICLE_SHADER_FRAG.to_owned(),
                "particle_shader",
            );

            let texture = Texture::new();

            let rect = RectBuilder::new(shader.get("line_shader"), &gl);
            let sprite = SpriteBuilder::new(shader.get("sprite_shader"), &gl);
            let text = TextBuilder::new();
            let particle = ParticleSystem::new(pool_size, shader.get("particle_shader"), &gl);

            Box::new(Self {
                device_context,
                _gl_context,
                gl,
                rect,
                sprite,
                text,
                particle,
                shader,
                texture,
            })
        }
    }

    pub fn delete_context(&self) -> bool {
        unsafe {
            wglDeleteContext(self._gl_context);
        }
        true
    }

    pub fn display_clear(&self, color: (f32, f32, f32, f32)) -> bool {
        self.gl.ClearColor(color.0, color.1, color.2, color.3);
        self.gl.Clear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

        true
    }

    pub fn display_frame(&self) -> bool {
        unsafe {
            SwapBuffers(self.device_context);
            DwmFlush();
        }
        true
    }

    pub fn begin_drawing(&self) -> bool {
        true
    }
}

pub fn open_gl_error(op: &str) {
    unsafe {
        let error = glGetError();
        if error > 0 {
            match error {
                GL_INVALID_ENUM => eprintln!("Error: Invalid Enum: {op}"),
                GL_INVALID_VALUE => eprintln!("Error: Invalid Value: {op}"),
                GL_INVALID_OPERATION => eprintln!("Error: Invalid Operation: {op}"),
                GL_STACK_OVERFLOW => eprintln!("Error: Stack Overflow: {op}"),
                GL_STACK_UNDERFLOW => eprintln!("Error: Stack Underflow: {op}"),
                GL_OUT_OF_MEMORY => eprintln!("Error: Out Of Memory: {op}"),
                GL_INVALID_FRAMEBUFFER_OPERATION => {
                    eprintln!("Error: Invalid Framebuffer Operation: {op}")
                }
                _ => eprintln!("UNKNOWN ERROR!: {op}"),
            }
            panic!("OpenGL Error");
        }
    }
}

pub fn pcstr(string: &str) -> PCSTR {
    (String::from(string) + "\0").as_ptr()
}

fn _to_wchar(str: &str) -> *const u16 {
    let v: Vec<u16> = OsStr::new(str)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect();
    v.as_ptr()
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//Shader Text
///////////////////////////////////////////////////////////////////////////////////////////////////

pub const SPRITE_SHADER_VERT: &str = r#"
#version 330 core
layout(location = 0) in vec3 vertexPosition_modelspace;
layout(location = 1) in vec2 vertexUV;
out vec2 UV;
out vec4 tint;
uniform mat4 MVP;
uniform vec4 COLOR;

void main(){
  gl_Position = MVP * vec4(vertexPosition_modelspace, 1);
  UV = vertexUV;
  tint = COLOR;
}
"#;

pub const SPRITE_SHADER_FRAG: &str = r#"
#version 330 core
in vec2 UV;
in vec4 tint;
out vec4 color;
uniform sampler2D myTextureSampler;
void main(){
  color = texture( myTextureSampler, UV).xyza * tint;
}
"#;

pub const LINE_SHADER_VERT: &str = r#"
#version 330 core
layout(location = 0) in vec3 vertexPos;

uniform mat4 MVP;
uniform vec4 COLOR;

out vec4 tint;

void main() {
    gl_Position = MVP * vec4(vertexPos, 1);
    tint = COLOR;
}
"#;

pub const LINE_SHADER_FRAG: &str = r#"
#version 330 core
in vec4 tint;
out vec4 color;

void main() {
    color = tint;
}
"#;

pub const PARTICLE_SHADER_VERT: &str = r#"
#version 330 core
layout(location = 0) in vec3 vertexPos;
layout(location = 1) in vec2 vertexUV;

out vec2 UV;
out vec4 tint;

uniform mat4 MVP;
uniform vec4 COLOR;

void main() {
    gl_Position = MVP * vec4(vertexPos, 1);
    UV = vertexUV;
    tint = COLOR;
}
"#;

pub const PARTICLE_SHADER_FRAG: &str = r#"
#version 330 core
in vec2 UV;
in vec4 tint;

out vec4 color;
uniform sampler2D textureSampler;

void main() {
    color = texture(textureSampler, UV).xyza * tint;
}

"#;

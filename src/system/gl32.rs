#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::style)]

use crate::{renderer::pcstr, system::win32::*};
use std::ffi::c_void;

///////////////////////////////////////////////////////////////////////////////////////////////////
// WIN32 STATICS
///////////////////////////////////////////////////////////////////////////////////////////////////

pub static PFD: PIXELFORMATDESCRIPTOR = PIXELFORMATDESCRIPTOR {
    nSize: std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u16,
    nVersion: 1,
    dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
    iPixelType: PFD_TYPE_RGBA,
    cColorBits: 32,
    cRedBits: 0,
    cRedShift: 0,
    cGreenBits: 0,
    cGreenShift: 0,
    cBlueBits: 0,
    cBlueShift: 0,
    cAlphaBits: 0,
    cAlphaShift: 0,
    cAccumBits: 0,
    cAccumRedBits: 0,
    cAccumGreenBits: 0,
    cAccumBlueBits: 0,
    cAccumAlphaBits: 0,
    cDepthBits: 24,
    cStencilBits: 8,
    cAuxBuffers: 0,
    iLayerType: PFD_MAIN_PLANE,
    bReserved: 0,
    dwLayerMask: 0,
    dwVisibleMask: 0,
    dwDamageMask: 0,
};

///////////////////////////////////////////////////////////////////////////////////////////////////
// WIN32 TYPES
///////////////////////////////////////////////////////////////////////////////////////////////////

pub type HGLRC = isize;
pub type PFD_FLAGS = u32;
pub type PFD_PIXEL_TYPE = i8;
pub type PFD_LAYER_TYPE = i8;
pub type PROC = *mut c_void;

pub type GLuint = u32;
pub type GLint = i32;
pub type GLubyte = u8;
pub type GLenum = u32;
pub type GLboolean = u8;
pub type GLbitfield = u32;
pub type GLfloat = f32;
pub type GLsizei = i32;
pub type GLsizeiptr = *const i32;
pub type GLchar = u8;
pub type GLvoid = c_void;

///////////////////////////////////////////////////////////////////////////////////////////////////
// WIN32 CONSTANTS
///////////////////////////////////////////////////////////////////////////////////////////////////

pub const PFD_DRAW_TO_WINDOW: PFD_FLAGS = 4u32;
pub const PFD_SUPPORT_OPENGL: PFD_FLAGS = 32u32;
pub const PFD_DOUBLEBUFFER: PFD_FLAGS = 1u32;
pub const PFD_TYPE_RGBA: PFD_PIXEL_TYPE = 0i8;
pub const PFD_MAIN_PLANE: PFD_LAYER_TYPE = 0i8;

pub const GL_COLOR_BUFFER_BIT: GLbitfield = 16384;
pub const GL_NICEST: GLenum = 4354;
pub const GL_PERSPECTIVE_CORRECTION_HINT: GLenum = 3152;
pub const GL_TEXTURE_2D: GLenum = 3553;
pub const GL_VERSION: GLenum = 7938;
pub const GLU_VERSION: GLenum = 100800;

pub const GL_ARRAY_BUFFER: GLenum = 34962;
pub const GL_STATIC_DRAW: GLenum = 35044;
pub const GL_FLOAT: GLenum = 5126;
pub const GL_FALSE: GLboolean = 0;
pub const GL_TRUE: GLboolean = 1;
pub const GL_TRIANGLES: GLenum = 4;

pub const GL_VERTEX_SHADER: GLenum = 35633;
pub const GL_FRAGMENT_SHADER: GLenum = 35632;
pub const GL_COMPILE_STATUS: GLenum = 35713;
pub const GL_INFO_LOG_LENGTH: GLenum = 35716;
pub const GL_LINK_STATUS: GLenum = 35714;
pub const GL_DEPTH_BUFFER_BIT: GLbitfield = 256;

pub const GL_RGB: GLenum = 6407u32;
pub const GL_RGBA: GLenum = 6408u32;
pub const GL_BGR: GLenum = 32992u32;
pub const GL_BGRA: GLenum = 32993u32;
pub const GL_UNSIGNED_BYTE: GLenum = 5121u32;
pub const GL_TEXTURE_WRAP_S: GLenum = 10242u32;
pub const GL_TEXTURE_WRAP_T: GLenum = 10243u32;
pub const GL_REPEAT: GLenum = 10497u32;
pub const GL_TEXTURE_MAG_FILTER: GLenum = 10240u32;
pub const GL_NEAREST: GLenum = 9728u32;
pub const GL_LINEAR: GLenum = 9729u32;
pub const GL_TEXTURE_MIN_FILER: GLenum = 10241u32;
pub const GL_LINEAR_MIPMAP_LINEAR: GLenum = 9987u32;
pub const GL_DEPTH_TEST: GLenum = 2929u32;
pub const GL_LESS: GLenum = 513u32;

pub const GL_NO_ERROR: GLenum = 0u32;
pub const GL_INVALID_ENUM: GLenum = 1280u32;
pub const GL_INVALID_VALUE: GLenum = 1281u32;
pub const GL_INVALID_OPERATION: GLenum = 1282u32;
pub const GL_STACK_OVERFLOW: GLenum = 1283u32;
pub const GL_STACK_UNDERFLOW: GLenum = 1284u32;
pub const GL_OUT_OF_MEMORY: GLenum = 1285u32;
pub const GL_INVALID_FRAMEBUFFER_OPERATION: GLenum = 1286u32;

pub const GL_DEBUG_OUTPUT: GLenum = 37600u32;

pub const GL_DEBUG_TYPE_ERROR: GLenum = 33356u32;
pub const GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR: GLenum = 33357u32;
pub const GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR: GLenum = 33358u32;
pub const GL_DEBUG_TYPE_PORTABILITY: GLenum = 33359u32;
pub const GL_DEBUG_TYPE_PERFORMANCE: GLenum = 33360u32;
pub const GL_DEBUG_TYPE_OTHER: GLenum = 33361u32;

pub const GL_DEBUG_TYPE_MARKER: GLenum = 33384u32;
pub const GL_DEBUG_TYPE_PUSH_GROUP: GLenum = 33385u32;
pub const GL_DEBUG_TYPE_POP_GROUP: GLenum = 33386u32;

pub const GL_DEBUG_SEVERITY_HIGH: GLenum = 37190u32;
pub const GL_DEBUG_SEVERITY_MEDIUM: GLenum = 37191u32;
pub const GL_DEBUG_SEVERITY_LOW: GLenum = 37192u32;
pub const GL_DEBUG_SEVERITY_NOTIFICATION: GLenum = 33387u32;

pub const GL_DEBUG_SOURCE_API: GLenum = 33350u32;
pub const GL_DEBUG_SOURCE_WINDOW_SYSTEM: GLenum = 33351u32;
pub const GL_DEBUG_SOURCE_SHADER_COMPILER: GLenum = 33352u32;
pub const GL_DEBUG_SOURCE_THIRD_PARTY: GLenum = 33353u32;
pub const GL_DEBUG_SOURCE_APPLICATION: GLenum = 33354u32;
pub const GL_DEBUG_SOURCE_OTHER: GLenum = 33355u32;

pub const GL_COMPRESSED_RGBA_S3TC_DXT1_EXT: GLenum = 33777u32;
pub const GL_COMPRESSED_RGBA_S3TC_DXT3_EXT: GLenum = 33778u32;
pub const GL_COMPRESSED_RGBA_S3TC_DXT5_EXT: GLenum = 33779u32;

pub const GL_TEXTURE_COMPRESSED: GLenum = 34465u32;
pub const GL_BLEND: GLenum = 3042u32;
pub const GL_SRC_ALPHA: GLenum = 770u32;
pub const GL_ONE_MINUS_SRC_ALPHA: GLenum = 771u32;
pub const GL_TEXTURE_BASE_LEVEL: GLenum = 33084u32;
pub const GL_TEXTURE_MAX_LEVEL: GLenum = 33085u32;
pub const GL_TEXTURE_MIN_FILTER: GLenum = 10241u32;
pub const GL_ALPHA_TEST: GLenum = 3008u32;

pub const GL_CLAMP_TO_EDGE: GLenum = 33071u32;
pub const GL_RGBA8: GLenum = 32856u32;
pub const GL_GREATER: GLenum = 516u32;
pub const GL_LEQUAL: GLenum = 515u32;

///////////////////////////////////////////////////////////////////////////////////////////////////
// WIN32 STRUCTURES
///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PIXELFORMATDESCRIPTOR {
    pub nSize: u16,
    pub nVersion: u16,
    pub dwFlags: PFD_FLAGS,
    pub iPixelType: PFD_PIXEL_TYPE,
    pub cColorBits: u8,
    pub cRedBits: u8,
    pub cRedShift: u8,
    pub cGreenBits: u8,
    pub cGreenShift: u8,
    pub cBlueBits: u8,
    pub cBlueShift: u8,
    pub cAlphaBits: u8,
    pub cAlphaShift: u8,
    pub cAccumBits: u8,
    pub cAccumRedBits: u8,
    pub cAccumGreenBits: u8,
    pub cAccumBlueBits: u8,
    pub cAccumAlphaBits: u8,
    pub cDepthBits: u8,
    pub cStencilBits: u8,
    pub cAuxBuffers: u8,
    pub iLayerType: PFD_LAYER_TYPE,
    pub bReserved: u8,
    pub dwLayerMask: u32,
    pub dwVisibleMask: u32,
    pub dwDamageMask: u32,
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// WIN32 FUNCTION POINTERS
///////////////////////////////////////////////////////////////////////////////////////////////////

pub type wglSwapInterval = fn(i32) -> bool;

pub type glGenVertexArrays = fn(n: GLsizei, arrays: &GLuint);
pub type glBindVertexArray = fn(array: GLuint);
pub type glGenBuffers = fn(n: GLsizei, buffers: &mut GLuint);
pub type glBindBuffer = fn(target: GLenum, buffer: GLuint);
pub type glBufferData = fn(target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum);
pub type glVertexAttribPointer = fn(
    index: GLuint,
    size: GLint,
    types: GLenum,
    normalized: GLboolean,
    stride: GLsizei,
    pointer: *const c_void,
);
pub type glDrawArrays = fn(mode: GLenum, first: GLint, count: GLsizei);
pub type glEnableVertexAttribArray = fn(index: GLuint);
pub type glDisableVertexAttribArray = fn(index: GLuint);

pub type glCreateShader = fn(shader_type: GLenum) -> GLuint;
pub type glShaderSource =
    fn(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint);
pub type glCompileShader = fn(shader: GLuint);
pub type glGetShaderiv = fn(shader: GLuint, pname: GLenum, params: &GLint);
pub type glGetShaderInfoLog =
    fn(shader: GLuint, max_length: GLsizei, length: *const GLsizei, info_log: *const GLchar);
pub type glCreateProgram = fn() -> GLuint;
pub type glAttachShader = fn(program: GLuint, shader: GLuint);
pub type glLinkProgram = fn(program: GLuint);
pub type glGetProgramiv = fn(program: GLuint, pname: GLenum, params: &GLint);
pub type glGetProgramInfoLog =
    fn(program: GLuint, max_length: GLsizei, length: *const GLsizei, info_log: *const GLchar);
pub type glDetachShader = fn(program: GLuint, shader: GLuint);
pub type glDeleteShader = fn(shader: GLuint);
pub type glUseProgram = fn(program: GLuint);
pub type glGetUniformLocation = fn(program: GLuint, name: *const GLchar) -> GLint;
pub type glUniform4fv = fn(location: GLint, count: GLsizei, value: *const GLfloat);
pub type glUniformMatrix4fv =
    fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glGenTextures = fn(n: GLsizei, textures: *const GLuint);
pub type glBindTexture = fn(target: GLenum, texture: GLuint);

pub type glCompressedTexImage2D = fn(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
    imageSize: GLsizei,
    data: *const c_void,
);
pub type glGenerateMipmap = fn(target: GLenum);

pub type glDebugMessageCallback = fn(callback: *const c_void, userParam: *const c_void);

///////////////////////////////////////////////////////////////////////////////////////////////////
// WIN32 FUNCTIONS
///////////////////////////////////////////////////////////////////////////////////////////////////

#[link(name = "Gdi32")]
extern "system" {
    pub fn ChoosePixelFormat(hdc: HDC, ppfd: *const PIXELFORMATDESCRIPTOR) -> i32;
    pub fn SetPixelFormat(hdc: HDC, format: i32, ppfd: *const PIXELFORMATDESCRIPTOR) -> bool;
    pub fn SwapBuffers(hdc: HDC) -> bool;
}

#[link(name = "Dwmapi")]
extern "system" {
    pub fn DwmFlush() -> HRESULT;
}

#[link(name = "Opengl32")]
extern "system" {
    pub fn wglCreateContext(hdc: HDC) -> HGLRC;
    pub fn wglMakeCurrent(hdc: HDC, hglrc: HGLRC) -> bool;
    pub fn wglDeleteContext(hglrc: HGLRC) -> bool;
    pub fn wglGetProcAddress(identifier: PCSTR) -> PROC;
    pub fn wglUseFontBitmapsA(hdc: HDC, param1: DWORD, param2: DWORD, param3: DWORD) -> bool;
    pub fn glGetString(name: u32) -> *mut u8;
    pub fn glEnable(cap: u32) -> ();
    pub fn glHint(target: u32, mode: u32) -> ();
    pub fn glClearColor(red: f32, green: f32, blue: f32, alpha: f32) -> ();
    pub fn glClear(mask: u32) -> ();
    pub fn glDepthFunc(cap: GLenum);
    pub fn glGetError() -> GLenum;
    pub fn glTexImage2D(
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        data: *const c_void,
    );
    pub fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint);
    pub fn glBlendFunc(sfactor: GLenum, dfactor: GLenum);
    pub fn glDisable(cap: GLenum);
    pub fn glRasterPos2f(x: GLfloat, y: GLfloat);
    pub fn glRasterPos3f(x: GLfloat, y: GLfloat, z: GLfloat);
    pub fn glListBase(base: GLuint);
    pub fn glCallLists(n: GLsizei, type_: GLenum, lists: *const GLvoid);
    pub fn glFlush();
    pub fn glGenLists(range: GLsizei) -> GLuint;
    pub fn glColor4f(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
}

#[link(name = "Glu32")]
extern "system" {
    pub fn gluGetString(name: GLenum) -> *const GLubyte;
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Helpers
///////////////////////////////////////////////////////////////////////////////////////////////////

pub fn wglGetProcAddressChecked(identifier: &str) -> PROC {
    unsafe {
        let proc = wglGetProcAddress(pcstr(identifier));
        if proc == std::ptr::null_mut() {
            eprintln!("wglGetProcAddress failed to load: {identifier}!");
        }
        proc
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Callbacks
///////////////////////////////////////////////////////////////////////////////////////////////////

pub fn MessageCallback(
    source: GLenum,
    type_: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    _userParam: *const c_void,
) {
    let mut myType = "";
    if type_ == GL_DEBUG_TYPE_ERROR {
        myType = "** GL ERROR **"
    }

    let slice = unsafe { std::slice::from_raw_parts(message, length as usize) };
    let myMessage = String::from_utf8(slice.to_vec()).unwrap();

    let source_specific = match source {
        GL_DEBUG_SOURCE_API => String::from("GL_DEBUG_SOURCE_API"),
        GL_DEBUG_SOURCE_WINDOW_SYSTEM => String::from("GL_DEBUG_SOURCE_WINDOW_SYSTEM"),
        GL_DEBUG_SOURCE_SHADER_COMPILER => String::from("GL_DEBUG_SOURCE_SHADER_COMPILER"),
        GL_DEBUG_SOURCE_THIRD_PARTY => String::from("GL_DEBUG_SOURCE_THIRD_PARTY"),
        GL_DEBUG_SOURCE_APPLICATION => String::from("GL_DEBUG_SOURCE_APPLICATION"),
        GL_DEBUG_SOURCE_OTHER => String::from("GL_DEBUG_SOURCE_OTHER"),
        _ => String::from("UNDEFINED SOURCE"),
    };

    let type_specific = match type_ {
        GL_DEBUG_TYPE_ERROR => String::from("GL_DEBUG_TYPE_ERROR"),
        GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR => String::from("GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR"),
        GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR => String::from("GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR"),
        GL_DEBUG_TYPE_PORTABILITY => String::from("GL_DEBUG_TYPE_PORTABILITY"),
        GL_DEBUG_TYPE_PERFORMANCE => String::from("GL_DEBUG_TYPE_PERFORMANCE"),
        GL_DEBUG_TYPE_OTHER => String::from("GL_DEBUG_TYPE_OTHER"),
        GL_DEBUG_TYPE_MARKER => String::from("GL_DEBUG_TYPE_MARKER"),
        GL_DEBUG_TYPE_PUSH_GROUP => String::from("GL_DEBUG_TYPE_PUSH_GROUP"),
        GL_DEBUG_TYPE_POP_GROUP => String::from("GL_DEBUG_TYPE_POP_GROUP"),
        _ => String::from("UNDEFINED TYPE"),
    };

    let alert;
    let severity_specific = match severity {
        GL_DEBUG_SEVERITY_HIGH => {
            alert = "\x1b[37;41m"; //Red Background - White Text
            String::from("GL_DEBUG_SEVERITY_HIGH")
        }
        GL_DEBUG_SEVERITY_MEDIUM => {
            alert = "\x1b[31m"; //Red Text
            String::from("GL_DEBUG_SEVERITY_MEDIUM")
        }
        GL_DEBUG_SEVERITY_LOW => {
            alert = "\x1b[33m"; //Yellow Text
            String::from("GL_DEBUG_SEVERITY_LOW")
        }
        GL_DEBUG_SEVERITY_NOTIFICATION => {
            alert = "\x1b[32m"; //Green Text
            String::from("GL_DEBUG_SEVERITY_NOTIFICATION")
        }
        _ => {
            alert = "\x1b[31;43m"; //Yellow Background - Red Text
            String::from("UNDEFINED SEVERITY")
        }
    };

    eprintln!(
        "\n\n{}GL CALLBACK: \x1b[37;40m{}\nsource = {}\nid = {}\ntype = {}\nseverity = {}\nmessage = {}\n\n",
        alert,
        myType,
        source_specific,
        id,
        type_specific,
        severity_specific,
        myMessage
    );
}

#![allow(clippy::too_many_arguments)]

use std::{collections::HashMap, ffi::c_void, mem::size_of, time::Duration};

use crate::{
    math::{float, Matrix, Vector},
    rand::RngEngine,
    renderer::{pcstr, GL},
    system::gl32::*,
    system::win32::*,
};

#[rustfmt::skip]
pub static QUAD: [GLfloat; 18] = [
    //First Triangle
    -0.5,  0.5,  0.0,
     0.5,  0.5,  0.0,
    -0.5, -0.5,  0.0,
    //Second Triangle
     0.5,  0.5,  0.0,
     0.5, -0.5,  0.0,
    -0.5, -0.5,  0.0,
];

#[rustfmt::skip]
pub static QUAD_UV: [GLfloat; 12] = [
    //First Triangle
    0.0, 1.0,
    1.0, 1.0,
    0.0, 0.0,
    //Second Triangle
    1.0, 1.0,
    1.0, 0.0,
    0.0, 0.0,
];

pub struct RectBuilder {
    pub shader_id: GLuint,
    pub matrix_id: GLint,
    pub color_id: GLint,
    pub vertex_id: GLuint,
}

impl RectBuilder {
    pub fn new(shader_id: GLuint, gl: &GL) -> Self {
        let matrix_id = gl.GetUniformLocation(shader_id, pcstr("MVP"));
        let color_id = gl.GetUniformLocation(shader_id, pcstr("COLOR"));

        let mut vertex_id = 0;
        gl.GenBuffers(1, &mut vertex_id);
        gl.BindBuffer(GL_ARRAY_BUFFER, vertex_id);
        gl.BufferData(
            GL_ARRAY_BUFFER,
            size_of::<[GLfloat; 18]>() as *const i32,
            &QUAD as *const _ as *const c_void,
            GL_STATIC_DRAW,
        );

        Self {
            shader_id,
            matrix_id,
            color_id,
            vertex_id,
        }
    }

    pub fn draw_rect(
        &self,
        x: f32,
        y: f32,
        z: f32,
        w: f32,
        h: f32,
        rot: f32,
        color: Vector,
        proj: Matrix,
        view: Matrix,
        gl: &GL,
    ) {
        let translation = Matrix::create_translation_matrix((x, y, -z).into());
        let rotation = Matrix::create_rotation_matrix(rot);
        let scale = Matrix::create_scaling_matrix((w, h, 1.0).into());
        let model = Matrix::create_trs(translation, rotation, scale);
        let mvp = Matrix::create_mvp(proj, view, model);

        gl.UseProgram(self.shader_id);
        gl.UniformMatrix4fv(self.matrix_id, 1, GL_FALSE, &mvp.c0.x as *const f32);
        gl.Uniform4fv(self.color_id, 1, &color.x as *const f32);

        gl.EnableVertexAttribArray(0);
        gl.BindBuffer(GL_ARRAY_BUFFER, self.vertex_id);
        gl.VertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 0, std::ptr::null());

        gl.DrawArrays(GL_TRIANGLES, 0, 6);
        gl.DisableVertexAttribArray(0);
        gl.UseProgram(0);
    }

    pub fn draw_line(
        &self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        z: f32,
        width: f32,
        color: Vector,
        proj: Matrix,
        view: Matrix,
        gl: &GL,
    ) {
        let position = (Vector::new(x1, y1, 0.0, 0.0) + (x2, y2, 0.0).into()) / 2.0;
        let length_vec = Vector::new(x1, y1, 0.0, 0.0) - (x2, y2, 0.0).into();
        let length = Vector::mag(length_vec);
        let angle = f32::atan2(length_vec.x, length_vec.y).to_degrees();

        let translation = Matrix::create_translation_matrix((position.x, position.y, -z).into());
        let rotation = Matrix::create_rotation_matrix(angle);
        let scale = Matrix::create_scaling_matrix((width, length, 1.0).into());
        let model = Matrix::create_trs(translation, rotation, scale);
        let mvp = Matrix::create_mvp(proj, view, model);

        gl.UseProgram(self.shader_id);
        gl.UniformMatrix4fv(self.matrix_id, 1, GL_FALSE, &mvp.c0.x as *const f32);
        gl.Uniform4fv(self.color_id, 1, &color.x as *const f32);

        gl.EnableVertexAttribArray(0);
        gl.BindBuffer(GL_ARRAY_BUFFER, self.vertex_id);
        gl.VertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 0, std::ptr::null());

        gl.DrawArrays(GL_TRIANGLES, 0, 6);
        gl.DisableVertexAttribArray(0);
        gl.UseProgram(0);
    }
}

pub struct Sprite {
    mvp: Matrix,
    color: Vector,
    texture_id: GLuint,
}

pub struct SpriteBuilder {
    pub shader_id: GLuint,
    pub matrix_id: GLint,
    pub color_id: GLint,
    pub vertex_id: GLuint,
    pub uv_id: GLuint,

    sprites: Vec<Sprite>,
}

impl SpriteBuilder {
    pub fn new(shader_id: GLuint, gl: &GL) -> Self {
        let matrix_id = gl.GetUniformLocation(shader_id, pcstr("MVP"));
        let color_id = gl.GetUniformLocation(shader_id, pcstr("COLOR"));

        let mut vertex_id = 0;
        gl.GenBuffers(1, &mut vertex_id);
        gl.BindBuffer(GL_ARRAY_BUFFER, vertex_id);
        gl.BufferData(
            GL_ARRAY_BUFFER,
            size_of::<[GLfloat; 18]>() as *const i32,
            &QUAD as *const _ as *const c_void,
            GL_STATIC_DRAW,
        );

        let mut uv_id = 0;
        gl.GenBuffers(1, &mut uv_id);
        gl.BindBuffer(GL_ARRAY_BUFFER, uv_id);
        gl.BufferData(
            GL_ARRAY_BUFFER,
            size_of::<[GLfloat; 12]>() as *const i32,
            &QUAD_UV as *const _ as *const c_void,
            GL_STATIC_DRAW,
        );

        Self {
            shader_id,
            matrix_id,
            color_id,
            vertex_id,
            uv_id,
            sprites: Vec::new(),
        }
    }

    pub fn add_sprite(
        &mut self,
        x: f32,
        y: f32,
        z: f32,
        w: f32,
        h: f32,
        rot: f32,
        texture_id: GLuint,
        color: Vector,
        proj: Matrix,
        view: Matrix,
    ) {
        let translation = Matrix::create_translation_matrix((x, y, -z).into());
        let rotation = Matrix::create_rotation_matrix(rot);
        let scale = Matrix::create_scaling_matrix((w, h, 1.0).into());
        let model = Matrix::create_trs(translation, rotation, scale);
        let mvp = Matrix::create_mvp(proj, view, model);

        self.sprites.push(Sprite {
            mvp,
            color,
            texture_id,
        })
    }

    pub fn draw_sprites(&mut self, gl: &GL) {
        gl.UseProgram(self.shader_id);

        gl.EnableVertexAttribArray(0);
        gl.BindBuffer(GL_ARRAY_BUFFER, self.vertex_id);
        gl.VertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 0, std::ptr::null());

        gl.EnableVertexAttribArray(1);
        gl.BindBuffer(GL_ARRAY_BUFFER, self.uv_id);
        gl.VertexAttribPointer(1, 2, GL_FLOAT, GL_FALSE, 0, std::ptr::null());

        for sprite in &self.sprites {
            gl.UniformMatrix4fv(self.matrix_id, 1, GL_FALSE, &sprite.mvp.c0.x as *const f32);
            gl.Uniform4fv(self.color_id, 1, &sprite.color.x as *const f32);
            gl.BindTexture(GL_TEXTURE_2D, sprite.texture_id);
            gl.DrawArrays(GL_TRIANGLES, 0, 6);
        }

        gl.DisableVertexAttribArray(0);
        gl.DisableVertexAttribArray(1);
        gl.BindTexture(GL_TEXTURE_2D, 0);
        gl.UseProgram(0);
        self.sprites.clear();
    }
}

pub enum Weight {
    DONTCARE,
    THIN,
    EXTRALIGHT,
    LIGHT,
    NORMAL,
    MEDIUM,
    SEMIBOLD,
    BOLD,
    EXTRABOLD,
    HEAVY,
}

pub struct TextBuilder {
    fonts: HashMap<String, usize>,
    index: usize,
}

impl TextBuilder {
    pub fn new() -> Self {
        Self {
            fonts: HashMap::new(),
            index: 1,
        }
    }
    pub fn define_font(
        &mut self,
        name: &str,
        size: i32,
        weight: i32,
        italic: bool,
        underline: bool,
        strikeout: bool,
        font: &str,
        hdc: HDC,
    ) {
        unsafe {
            let font = CreateFontA(
                size,
                0,
                0,
                0,
                weight,
                italic as u32,
                underline as u32,
                strikeout as u32,
                DEFAULT_CHARSET as u32,
                OUT_OUTLINE_PRECIS as u32,
                CLIP_DEFAULT_PRECIS as u32,
                CLEARTYPE_QUALITY as u32,
                VARIABLE_PITCH as u32,
                pcstr(font),
            );

            SelectObject(hdc, font);
            wglUseFontBitmapsA(hdc, 0, 255, self.index as u32 * 1000);
            DeleteObject(font);
        }

        self.fonts.insert(String::from(name), self.index);
        self.index += 1;
    }
    pub fn draw_text(
        &self,
        text: &str,
        x: f32,
        y: f32,
        color: (f32, f32, f32, f32),
        font: &str,
        gl: &GL,
    ) {
        gl.UseProgram(0);
        let len = text.len();

        let font_index = match self.fonts.get(font) {
            Some(x) => x,
            None => {
                eprintln!("Invalid font: {font}");
                return;
            }
        };

        unsafe {
            glRasterPos3f(x, y, -1.0);
            glColor4f(color.0, color.1, color.2, color.3);
            glListBase(*font_index as u32 * 1000);
            glCallLists(len as i32, GL_UNSIGNED_BYTE, text.as_ptr() as *const c_void);
            glFlush();
        }
        gl.UseProgram(0);
    }
}

pub struct ParticleProperties {
    pub pos: Vector,
    pub velocity: Vector,
    pub velocity_var: Vector,
    pub color_begin: Vector,
    pub color_end: Vector,
    pub size_begin: f32,
    pub size_end: f32,
    pub size_var: f32,
    pub lifetime: f32,
    pub tex_id: Option<GLuint>,
}

#[derive(Copy, Clone)]
pub struct Particle {
    pub pos: Vector,
    pub velocity: Vector,
    pub color_begin: Vector,
    pub color_end: Vector,
    pub rotation: f32,
    pub size_begin: f32,
    pub size_end: f32,
    pub lifetime: f32,
    pub life_remaining: f32,
    pub active: bool,
    pub tex_id: Option<GLuint>,
}

impl Default for Particle {
    fn default() -> Self {
        let zeroed = Vector::new(0.0, 0.0, 0.0, 0.0);
        Self {
            pos: zeroed,
            velocity: zeroed,
            color_begin: zeroed,
            color_end: zeroed,
            rotation: 0.0,
            size_begin: 0.0,
            size_end: 0.0,
            lifetime: 0.0,
            life_remaining: 0.0,
            active: false,
            tex_id: None,
        }
    }
}

pub struct ParticleSystem {
    particle_pool: Vec<Particle>,
    pool_index: usize,

    vertex_id: GLuint,
    uv_id: GLuint,
    shader_id: GLuint,
    matrix_id: GLint,
    color_id: GLint,

    texture_white_id: GLuint,

    pub active: bool,
}

impl ParticleSystem {
    pub fn new(pool_size: usize, shader_id: GLuint, gl: &GL) -> Self {
        let particle_pool = vec![Particle::default(); pool_size];
        let pool_index = pool_size - 1;

        let matrix_id = gl.GetUniformLocation(shader_id, pcstr("MVP"));
        let color_id = gl.GetUniformLocation(shader_id, pcstr("COLOR"));

        let mut vertex_id = 0;
        gl.GenBuffers(1, &mut vertex_id);
        gl.BindBuffer(GL_ARRAY_BUFFER, vertex_id);
        gl.BufferData(
            GL_ARRAY_BUFFER,
            size_of::<[GLfloat; 18]>() as *const i32,
            &QUAD as *const _ as *const c_void,
            GL_STATIC_DRAW,
        );

        let mut uv_id = 0;
        gl.GenBuffers(1, &mut uv_id);
        gl.BindBuffer(GL_ARRAY_BUFFER, uv_id);
        gl.BufferData(
            GL_ARRAY_BUFFER,
            size_of::<[GLfloat; 12]>() as *const i32,
            &QUAD_UV as *const _ as *const c_void,
            GL_STATIC_DRAW,
        );

        let texture_white_id = 0;
        gl.GenTextures(1, &texture_white_id);
        gl.BindTexture(GL_TEXTURE_2D, texture_white_id);
        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILER, GL_LINEAR as i32);
        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR as i32);
        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE as i32);
        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE as i32);
        let color: u32 = 0xffffffff;
        gl.TexImage2D(
            GL_TEXTURE_2D,
            0,
            GL_RGBA,
            1,
            1,
            0,
            GL_BGRA,
            GL_UNSIGNED_BYTE,
            &color as *const _ as *const c_void,
        );

        Self {
            particle_pool,
            pool_index,

            vertex_id,
            uv_id,
            shader_id,
            matrix_id,
            color_id,

            texture_white_id,
            active: false,
        }
    }

    pub fn update(&mut self, dt: Duration) {
        let mut inactive = true;
        for mut particle in &mut self.particle_pool {
            if !particle.active {
                continue;
            }

            if particle.life_remaining <= 0.0 {
                particle.active = false;
                continue;
            }

            inactive = false;

            particle.life_remaining -= dt.as_secs_f32();
            particle.pos += particle.velocity * dt.as_secs_f32();
            particle.rotation += 0.5 * dt.as_secs_f32();
        }

        if inactive {
            self.active = false;
        }
    }

    pub fn draw(&mut self, proj: Matrix, view: Matrix, gl: &GL) {
        gl.UseProgram(self.shader_id);

        gl.EnableVertexAttribArray(0);
        gl.BindBuffer(GL_ARRAY_BUFFER, self.vertex_id);
        gl.VertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 0, std::ptr::null());

        gl.EnableVertexAttribArray(1);
        gl.BindBuffer(GL_ARRAY_BUFFER, self.uv_id);
        gl.VertexAttribPointer(1, 2, GL_FLOAT, GL_FALSE, 0, std::ptr::null());

        for particle in &self.particle_pool {
            if !particle.active {
                continue;
            }

            let life = particle.life_remaining / particle.lifetime;
            let color = Vector::lerp(particle.color_end, particle.color_begin, life);
            let size = float::lerp(particle.size_end, particle.size_begin, life);

            let translation = Matrix::create_translation_matrix(
                (particle.pos.x, particle.pos.y, particle.pos.z).into(),
            );
            let rotation = Matrix::create_rotation_matrix(particle.rotation);
            let scale = Matrix::create_scaling_matrix((size, size, 0.0).into());
            let model = Matrix::create_trs(translation, rotation, scale);
            let mvp = Matrix::create_mvp(proj, view, model);

            gl.UniformMatrix4fv(self.matrix_id, 1, GL_FALSE, &mvp.c0.x as *const f32);
            gl.Uniform4fv(self.color_id, 1, &color.x as *const f32);

            if particle.tex_id.is_none() {
                gl.BindTexture(GL_TEXTURE_2D, self.texture_white_id);
            } else {
                gl.BindTexture(GL_TEXTURE_2D, particle.tex_id.unwrap());
            }

            gl.DrawArrays(GL_TRIANGLES, 0, 6);
            gl.BindTexture(GL_TEXTURE_2D, 0);
        }

        gl.DisableVertexAttribArray(0);
        gl.DisableVertexAttribArray(1);
        gl.BindTexture(GL_TEXTURE_2D, 0);
        gl.UseProgram(0);
    }

    pub fn emit(&mut self, properties: &ParticleProperties, rng: &mut RngEngine) {
        self.active = true;
        let particle = &mut self.particle_pool[self.pool_index];

        particle.active = true;
        particle.pos = properties.pos;
        particle.rotation =
            (rng.get_float_range(-1.0, 1.0) as f32 * 2.0 * std::f32::consts::PI).to_degrees();

        //velocity
        particle.velocity = properties.velocity;
        particle.velocity.x += properties.velocity_var.x * rng.get_float_range(-1.0, 1.0) as f32;
        particle.velocity.y += properties.velocity_var.y * rng.get_float_range(-1.0, 1.0) as f32;

        //color
        particle.color_begin = properties.color_begin;
        particle.color_end = properties.color_end;

        particle.lifetime = properties.lifetime;
        particle.life_remaining = properties.lifetime;
        particle.size_begin =
            properties.size_begin + properties.size_var * rng.get_float_range(-1.0, 1.0) as f32;
        particle.size_end = properties.size_end;

        particle.tex_id = properties.tex_id;

        if self.pool_index == 0 {
            self.pool_index = self.particle_pool.len() - 1;
        } else {
            self.pool_index -= 1;
        }
    }
}

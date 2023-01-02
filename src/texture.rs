use std::collections::HashMap;
use std::ffi::c_void;
use std::fs;
use std::str::from_utf8;

use crate::renderer::GL;
use crate::system::gl32::*;

pub struct Texture {
    texture_ids: HashMap<String, GLuint>,
}

impl Texture {
    pub fn new() -> Self {
        Self {
            texture_ids: HashMap::new(),
        }
    }
    pub fn load(&mut self, gl: &GL, path: &str) -> GLuint {
        let Some(extension) = std::path::Path::new(&path).extension() else {
                eprintln!("Loading texture '{path}' failed: Couldn't parse extension!");
                return 0;
        };

        if extension.to_ascii_lowercase() == "dds" {
            let Some(filename) = std::path::Path::new(&path).file_stem() else {
                eprintln!("Loading texture '{path}' failed: Couldn't resolve texture name!");
                return 0;
            };

            let tex_id = load_dds(gl, path);

            if tex_id == 0 {
                return 0;
            }

            self.texture_ids
                .insert(String::from(filename.to_str().unwrap()), tex_id);
            #[cfg(debug_assertions)]
            println!("Inserting texture: {:?} with id: {tex_id}", filename);
            tex_id
        } else if extension.to_ascii_lowercase() == "bmp" {
            let Some(filename) = std::path::Path::new(&path).file_stem() else {
                eprintln!("Loading texture '{path}' failed: Couldn't resolve texture name!");
                return 0;
            };

            let tex_id = load_bmp(gl, path);

            if tex_id == 0 {
                return 0;
            }

            self.texture_ids
                .insert(String::from(filename.to_str().unwrap()), tex_id);
            #[cfg(debug_assertions)]
            println!("Inserting texture: {:?} with id: {tex_id}", filename);
            tex_id
        } else {
            eprintln!("Loading texture '{path}' failed: Unrecognzied format!");
            0
        }
    }
    pub fn load_directory(&mut self, gl: &GL, path: &str) {
        let directory = std::path::Path::new(path);

        if let Ok(entries) = fs::read_dir(directory) {
            for entry in entries {
                let entry = entry.unwrap();
                self.load(gl, entry.path().to_str().unwrap());
            }
        } else {
            eprintln!("Couldn't find texture directory: '{path}'");
        }
    }
    pub fn get(&self, tname: &str) -> GLuint {
        let tex_id = self.texture_ids.get(tname);
        if let Some(tex_id) = tex_id {
            *tex_id
        } else {
            eprintln!("get_texture failed: '{tname}' does not exist");
            0
        }
    }
}

const BMP_HEADER_SIZE: usize = 54;

pub fn load_bmp(gl: &GL, path: &str) -> GLuint {
    let Ok(file) = fs::read(path) else {
        eprintln!("Loading texture '{path}' failed: File not found!");
        return 0;
    };

    if file.len() < BMP_HEADER_SIZE || file[0] != b'B' || file[1] != b'M' {
        eprintln!("Loading texture '{path}' failed: Malformed header data!");
        return 0;
    }

    let mut data_position = i32::from_le_bytes([file[10], file[11], file[12], file[13]]);
    let mut _image_size = i32::from_le_bytes([file[34], file[35], file[36], file[37]]);
    let width = i32::from_le_bytes([file[18], file[19], file[20], file[21]]);
    let height = i32::from_le_bytes([file[22], file[23], file[24], file[25]]);

    if _image_size == 0 {
        _image_size = width * height * 3;
    }

    if data_position == 0 {
        data_position = BMP_HEADER_SIZE as i32;
    }

    let texture = file[data_position as usize..file.len()].to_vec();

    let texture_id: GLuint = 0;
    gl.GenTextures(1, &texture_id);
    gl.BindTexture(GL_TEXTURE_2D, texture_id);

    gl.TexImage2D(
        GL_TEXTURE_2D,
        0,
        GL_RGBA,
        width,
        height,
        0,
        GL_BGRA,
        GL_UNSIGNED_BYTE,
        &texture[0] as *const _ as *const c_void,
    );
    gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR as i32);
    gl.TexParameteri(
        GL_TEXTURE_2D,
        GL_TEXTURE_MIN_FILER,
        GL_LINEAR_MIPMAP_LINEAR as i32,
    );
    gl.GenerateMipmap(GL_TEXTURE_2D);
    gl.BindTexture(GL_TEXTURE_2D, 0);

    texture_id
}

const DDS_BUFFER_START: usize = 128;
const DDS_DXT1: u32 = 0x31545844;
const DDS_DXT3: u32 = 0x33545844;
const DDS_DXT5: u32 = 0x35545844;

pub fn load_dds(gl: &GL, path: &str) -> GLuint {
    let Ok(file) = fs::read(path) else {
        eprintln!("Loading texture '{path}' failed: File not found!");
        return 0;
    };

    let filecode = if file.len() > 4 {
        from_utf8(&file[0..4]).unwrap()
    } else {
        eprintln!("Loading texture '{path}' failed: Malformed data!");
        return 0;
    };

    if filecode != "DDS " {
        eprintln!("Loading texture '{path}' failed: Malformed header data!");
        return 0;
    }

    let height = u32::from_le_bytes([file[12], file[13], file[14], file[15]]);
    let width = u32::from_le_bytes([file[16], file[17], file[18], file[19]]);
    let linear_size = u32::from_le_bytes([file[20], file[21], file[22], file[23]]);
    let mut mipmap_count = u32::from_le_bytes([file[28], file[29], file[30], file[31]]);
    let four_cc = u32::from_le_bytes([file[84], file[85], file[86], file[87]]);

    let _buf_size = if mipmap_count > 1 {
        linear_size * 2
    } else {
        linear_size
    };

    let texture = file[DDS_BUFFER_START..file.len()].to_vec();

    let format = match four_cc {
        DDS_DXT1 => GL_COMPRESSED_RGBA_S3TC_DXT1_EXT,
        DDS_DXT3 => GL_COMPRESSED_RGBA_S3TC_DXT3_EXT,
        DDS_DXT5 => GL_COMPRESSED_RGBA_S3TC_DXT5_EXT,
        _ => {
            eprintln!("Loading texture '{path}' failed: Unsupported DDS format!");
            return 0;
        }
    };

    let texture_id: GLuint = 0;
    gl.GenTextures(1, &texture_id);

    gl.BindTexture(GL_TEXTURE_2D, texture_id);
    gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_BASE_LEVEL, 0);
    gl.TexParameteri(
        GL_TEXTURE_2D,
        GL_TEXTURE_MAX_LEVEL,
        (mipmap_count - 1) as i32,
    );
    gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR as i32);
    gl.TexParameteri(
        GL_TEXTURE_2D,
        GL_TEXTURE_MIN_FILTER,
        GL_LINEAR_MIPMAP_LINEAR as i32,
    );
    gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT as i32);
    gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT as i32);

    let block_size = if format == GL_COMPRESSED_RGBA_S3TC_DXT1_EXT {
        8
    } else {
        16
    };

    let mut offset = 0;
    let mut level = 0;
    let mut size;
    let mut w = width;
    let mut h = height;

    while level < mipmap_count {
        if w == 0 || h == 0 {
            mipmap_count -= 1;
            continue;
        }
        size = ((w + 3) / 4) * ((h + 3) / 4) * block_size;
        gl.CompressedTexImage2D(
            GL_TEXTURE_2D,
            level as i32,
            format,
            w as i32,
            h as i32,
            0,
            size as i32,
            &texture[offset] as *const _ as *const c_void,
        );
        offset += size as usize;
        w /= 2;
        h /= 2;
        level += 1;
    }
    gl.TexParameteri(
        GL_TEXTURE_2D,
        GL_TEXTURE_MAX_LEVEL,
        (mipmap_count - 1) as i32,
    );
    gl.BindTexture(GL_TEXTURE_2D, 0);

    texture_id
}

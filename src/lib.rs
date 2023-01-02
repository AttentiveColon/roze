//! # ROZE - The zero dependency game engine.
//!
//! ROZE is a barebones, zero dependency Windows game engine using win32 and OpenGL.
//! This is the 0.1 release and is subject to major api changes.
//!
//! #### Current Features:
//! * Texture loading system that supports 32bit rgba BMP and DDS files
//! * Audio loading system that supports WAV files
//! * Audio playback system for playing one-shots and loops
//! * Text rendering
//! * Input system for Mouse, Keyboard and Xinput Gamepads
//! * Random number generation system
//! * Textured Sprite and Primitives system
//! * Basic Orthographic Camera system
//! * Barebones Math library
//! * Textured particle system
//! * Basic fps, frametime and performance statistics
//!
//! ### Entry point:
//! * The [System] struct defines most of the important functions.
//!     * Check [SystemConf] if you want more project setup options.
//!
//! * [EventFunctions] must be implemented to start a project.
//!
//! * Check out [ParticleProperties] to define a particle.
//!
//! #### Getting started:
//! Displays a 600x600 window with red background.
//!
//! ```
//! pub struct Game {}
//!
//! impl EventFunctions for Game {
//!     fn init(&mut self, system: &mut System) {}
//!     fn update(&mut self, system: &mut System, dt: Duration) {}
//!     fn draw(&mut self, system: &mut System) {
//!         system.display_clear(1.0, 0.0, 0.0, 1.0);
//!     }
//! }
//!
//! fn main() {
//!     let system = System::new(600, 600, "Game Title");
//!     let game = Game {};
//!     Event::run(game, system);
//! }
//! ```
//!
//! #### Using sprite, texture and audio systems:
//! Displays a window with a spinning textured sprite. An audio clip plays when the left mouse
//! button is clicked.
//!
//! ```
//! pub struct Game {
//!     rotation: f32,
//! }
//!
//! impl EventFunctions for Game {
//!     fn init(&mut self, system: &mut System) {
//!         system.load_texture("textures/SpriteA.bmp");
//!         system.load_audio("audio/sound.wav");
//!     }
//!     fn update(&mut self, system: &mut System, dt: Duration) {
//!         self.rotation += 90.0 * dt.as_secs_f32();
//!
//!         if system.mouse_pressed(MOUSECODE::L) {
//!             system.play_one_shot("sound", 1.0);
//!         }
//!
//!         if system.keyboard_pressed(KEYCODE::ESC) {
//!             system.quit();
//!         }
//!     }
//!     fn draw(&mut self, system: &mut System) {
//!         system.display_clear(0.05, 0.08, 0.05, 0.0);
//!
//!         system.sprite(
//!             system.screen_width() / 2.0,
//!             system.screen_height() / 2.0,
//!             0.0,
//!             100.0,
//!             100.0,
//!             self.rotation,
//!             system.get_texture("SpriteA"),
//!             (1.0, 1.0, 1.0, 1.0).into(),
//!         );
//!     }
//! }
//!
//! fn main() {
//!     let system = System::new(800, 800, "Sprites, Textures and Audio");
//!     let game = Game { rotation: 0.0 };
//!     Event::run(game, system);
//! }
//! ```
//!
//! #### Using particle and text systems:
//! Define an untextured colored particle effect that moves out from the center of the screen. Also
//! define a font and display the current fps in the top right of the screen.
//!
//! ```
//! pub struct Game {
//!     particle_prop: ParticleProperties,
//! }
//!
//! impl Game {
//!     pub fn new() -> Self {
//!         let particle_prop = ParticleProperties {
//!             pos: (0.0, 0.0, 0.0).into(),
//!             velocity: (0.0, 0.0, 0.0).into(),
//!             velocity_var: (150.0, 150.0, 0.0).into(),
//!             color_begin: (1.0, 0.0, 0.0, 1.0).into(),
//!             color_end: (0.0, 0.0, 1.0, 0.0).into(),
//!             size_begin: 15.0,
//!             size_end: 25.0,
//!             size_var: 10.0,
//!             lifetime: 5.0,
//!             tex_id: None,
//!         };
//!         Self { particle_prop }
//!     }
//! }
//!
//! impl EventFunctions for Game {
//!     fn init(&mut self, system: &mut System) {
//!         self.particle_prop.pos = (
//!             system.screen_width() / 2.0,
//!             system.screen_height() / 2.0,
//!             0.0,
//!         )
//!             .into();
//!
//!         system.define_font(
//!             "myFont",
//!             22,
//!             Weight::NORMAL,
//!             false,
//!             false,
//!             false,
//!             "consolas",
//!         );
//!     }
//!     fn update(&mut self, system: &mut System, _dt: Duration) {
//!         system.particle_emit(&self.particle_prop);
//!
//!         if system.keyboard_pressed(KEYCODE::ESC) {
//!             system.quit();
//!         }
//!     }
//!     fn draw(&mut self, system: &mut System) {
//!         system.display_clear(0.05, 0.08, 0.05, 0.0);
//!
//!         system.text(
//!             &format!("fps: {}", system.get_fps()),
//!             system.screen_width() - 100.0,
//!             20.0,
//!             (1.0, 1.0, 1.0, 1.0),
//!             "myFont",
//!         );
//!     }
//! }
//!
//! fn main() {
//!     let config = SystemConf {
//!         window_title: "Particles".into(),
//!         screen_width: 1200,
//!         screen_height: 1200,
//!         audio_one_shot_channels: 10,
//!         audio_loop_channels: 5,
//!         particle_pool_size: 5000,
//!         rng_seed: 1,
//!         rng_pool_size: 1_000_000,
//!         lock_fps: false,
//!     };
//!     let system = System::new_ex(config);
//!     let game = Game::new();
//!     Event::run(game, system);
//! }
//! ```
//!
//!

#![allow(clippy::too_many_arguments)]

mod audio;
mod camera;
mod input;
mod math;
mod rand;
mod renderer;
mod shader;
mod sprite;
mod statistics;
mod system;
mod texture;
mod window;

use audio::Audio;
use camera::OrthoCam;
use input::{Input, GAMEPAD, KEYCODE, MOUSECODE};
use math::*;
use rand::RngEngine;
use renderer::Renderer;
use sprite::{ParticleProperties, Weight};
use statistics::Stats;
use system::gl32::GLuint;
use system::win32::*;
use window::{Window, WindowResources};

use std::time::{Duration, Instant, UNIX_EPOCH};

pub mod prelude {
    pub use crate::input::{GAMEPAD, KEYCODE, MOUSECODE};
    pub use crate::math::*;
    pub use crate::sprite::{ParticleProperties, Weight};
    pub use crate::{Event, EventFunctions, System, SystemConf};
    pub use std::time::Duration;
}

pub struct System {
    window: Box<window::Window>,
    audio: Box<audio::Audio>,
    input: Box<input::Input>,
    resources: &'static mut WindowResources,
    renderer: Box<Renderer>,
    rng: Box<RngEngine>,
    cam: Box<OrthoCam>,
    stats: Box<Stats>,
    lock_fps: bool,
}

pub struct SystemConf {
    pub window_title: String,
    pub screen_width: i32,
    pub screen_height: i32,
    pub audio_one_shot_channels: u8,
    pub audio_loop_channels: u8,
    pub particle_pool_size: usize,
    pub rng_seed: u64,
    pub rng_pool_size: usize,
    pub lock_fps: bool,
}

impl System {
    ///Create new instance of system with default settings.
    pub fn new(width: i32, height: i32, window_title: &str) -> Self {
        let window = Box::new(Window::create_window(window_title, width, height));
        let audio = Audio::new(8, 8);
        let mut input = Box::new(Input::new());
        let mut renderer = Renderer::new(window.device_context, 1000);

        let seed = std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let rng = Box::new(RngEngine::new(seed, 1_000_000));

        let cam = Box::new(OrthoCam::new(
            0.0,
            width as f32,
            height as f32,
            0.0,
            0.1,
            100.0,
        ));

        let stats = Box::new(Stats::new());

        unsafe {
            let mut resources = &mut *window.window_resources;
            resources.input = &mut *input;
            resources.renderer = &mut *renderer;

            System {
                window,
                audio,
                input,
                resources,
                renderer,
                rng,
                cam,
                stats,
                lock_fps: true,
            }
        }
    }
    ///Create new instance of System using SystemConf struct.
    pub fn new_ex(config: SystemConf) -> Self {
        let window = Box::new(Window::create_window(
            &config.window_title,
            config.screen_width,
            config.screen_height,
        ));
        let audio = Audio::new(config.audio_one_shot_channels, config.audio_loop_channels);
        let mut input = Box::new(Input::new());
        let mut renderer = Renderer::new(window.device_context, config.particle_pool_size);
        let rng = Box::new(RngEngine::new(config.rng_seed, config.rng_pool_size));
        let cam = Box::new(OrthoCam::new(
            0.0,
            config.screen_width as f32,
            config.screen_height as f32,
            0.0,
            0.1,
            100.0,
        ));
        let stats = Box::new(Stats::new());
        let lock_fps = config.lock_fps;

        unsafe {
            let mut resources = &mut *window.window_resources;
            resources.input = &mut *input;
            resources.renderer = &mut *renderer;

            System {
                window,
                audio,
                input,
                resources,
                renderer,
                rng,
                cam,
                stats,
                lock_fps,
            }
        }
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // Textures
    ///////////////////////////////////////////////////////////////////////////////////////////////////

    ///Load all supported textures in provided directory, keys to textures are their file stem names
    ///without the extension.
    pub fn load_texture_dir(&mut self, path: &str) {
        self.renderer
            .texture
            .load_directory(&self.renderer.gl, path);
    }

    ///Load single texture from path, key to texture is the file stem name without extension.
    pub fn load_texture(&mut self, texture_path: &str) -> GLuint {
        self.renderer.texture.load(&self.renderer.gl, texture_path)
    }

    ///Get texture id using texture key.
    pub fn get_texture(&self, tname: &str) -> GLuint {
        self.renderer.texture.get(tname)
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // Audio
    ///////////////////////////////////////////////////////////////////////////////////////////////////

    ///Load all supported audio files in provided directory, keys to audio files are their file stem
    ///names without extension.
    pub fn load_audio_dir(&mut self, path: &str) {
        self.audio.load_assets(path);
    }

    ///Load single audio file from path, key to audio file is the file stem name without extension.
    pub fn load_audio(&mut self, audio_path: &str) {
        self.audio.load_audio(audio_path);
    }

    ///Play audio file using its key. File will be played as a one shot sample.
    pub fn play_one_shot(&mut self, audio: &str, level: f32) {
        self.audio.one_shot(audio, level);
    }

    ///Play audio file using its key. File will be played as a looping sample on selected channel
    ///until stop_loop() is called on that channel.
    pub fn play_loop(&mut self, audio: &str, channel: u8, level: f32) {
        self.audio.submit_loop(audio, channel, level);
    }

    ///Stop loop on selected channel.
    pub fn stop_loop(&mut self, channel: u8) {
        self.audio.stop_loop(channel);
    }

    ///Set master volume of audio system.
    pub fn set_master_volume(&mut self, level: f32) {
        self.audio.set_master_volume(level);
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // Renderer
    ///////////////////////////////////////////////////////////////////////////////////////////////////

    ///Clear screen to selected color.
    pub fn display_clear(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.renderer.display_clear((r, g, b, a));
    }

    ///Draw simple colored rectangle primitive at given coordinates.
    pub fn rect(&mut self, x: f32, y: f32, z: f32, w: f32, h: f32, rot: f32, color: Vector) {
        let (proj, view) = self.cam.get_proj_view();
        self.renderer
            .rect
            .draw_rect(x, y, z, w, h, rot, color, proj, view, &self.renderer.gl)
    }

    ///Draw line primitive between points (x1,y1) and (x2,y2) with a given thickness of w.
    pub fn line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, z: f32, w: f32, color: Vector) {
        let (proj, view) = self.cam.get_proj_view();
        self.renderer
            .rect
            .draw_line(x1, y1, x2, y2, z, w, color, proj, view, &self.renderer.gl)
    }

    ///Draw textured sprite. Use get_texture() to assign appropriate texture value.
    pub fn sprite(
        &mut self,
        x: f32,
        y: f32,
        z: f32,
        w: f32,
        h: f32,
        rot: f32,
        texture_id: GLuint,
        color: Vector,
    ) {
        let (proj, view) = self.cam.get_proj_view();
        self.renderer
            .sprite
            .add_sprite(x, y, z, w, h, rot, texture_id, color, proj, view);
    }

    fn draw_sprites(&mut self) {
        self.renderer.sprite.draw_sprites(&self.renderer.gl);
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // Text
    ///////////////////////////////////////////////////////////////////////////////////////////////////

    ///Draw text string at coordinate. A font must be defined first! Pass defined font name to
    ///font parameter.
    pub fn text(&mut self, text: &str, x: f32, y: f32, color: Vector, font: &str) {
        let ndc_x = (x / self.screen_width()) * 2.0 - 1.0;
        let ndc_y = (y / self.screen_height()) * 2.0 - 1.0;
        self.renderer.text.draw_text(
            text,
            ndc_x,
            -ndc_y,
            (color.x, color.y, color.z, color.w),
            font,
            &self.renderer.gl,
        );
    }

    ///Define a font to be draw with text() later. Will find closest match if system can't find
    ///font that matches exact specification.
    pub fn define_font(
        &mut self,
        name: &str,
        size: i32,
        weight: Weight,
        italic: bool,
        underline: bool,
        strikeout: bool,
        font: &str,
    ) {
        let weight_value = match weight {
            Weight::DONTCARE => 0,
            Weight::THIN => 100,
            Weight::EXTRALIGHT => 200,
            Weight::LIGHT => 300,
            Weight::NORMAL => 400,
            Weight::MEDIUM => 500,
            Weight::SEMIBOLD => 600,
            Weight::BOLD => 700,
            Weight::EXTRABOLD => 800,
            Weight::HEAVY => 900,
        };
        self.renderer.text.define_font(
            name,
            size,
            weight_value,
            italic,
            underline,
            strikeout,
            font,
            self.window.device_context,
        );
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // Particles
    ///////////////////////////////////////////////////////////////////////////////////////////////////

    ///After defining your ParticleProperties struct call to render your particle.
    pub fn particle_emit(&mut self, properties: &ParticleProperties) {
        self.renderer.particle.emit(properties, &mut self.rng);
    }

    fn particle_update(&mut self, dt: Duration) {
        self.renderer.particle.update(dt);
    }

    fn particle_draw(&mut self) {
        let (proj, view) = self.cam.get_proj_view();

        self.renderer.particle.draw(proj, view, &self.renderer.gl);
    }

    fn particle_active(&self) -> bool {
        self.renderer.particle.active
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // Rng Engine
    ///////////////////////////////////////////////////////////////////////////////////////////////////

    ///Get random int in range from low to exclusive high.
    pub fn get_int_range(&mut self, low: i32, high: i32) -> i32 {
        self.rng.get_int_range(low as usize, high as usize) as i32
    }

    ///Get random float value in range from low to exclusive high.
    pub fn get_float_range(&mut self, low: f32, high: f32) -> f32 {
        self.rng.get_float_range(low as f64, high as f64) as f32
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // Camera
    ///////////////////////////////////////////////////////////////////////////////////////////////////

    pub fn cam_translate(&mut self, direction: Vector, amount: f32) {
        self.cam.translate(direction, amount);
    }

    pub fn cam_zoom(&mut self, amount: f32) {
        self.cam.zoom(amount);
    }

    pub fn cam_position(&self) -> (f32, f32) {
        (self.cam.position.x, self.cam.position.y)
    }

    ///Change aspect of camera view.
    pub fn cam_size(&self) -> (f32, f32) {
        (
            self.cam.right - self.cam.left,
            self.cam.bottom - self.cam.top,
        )
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // Input
    ///////////////////////////////////////////////////////////////////////////////////////////////////

    pub fn get_mouse_x(&self) -> f32 {
        self.input.mouse.get_x() as f32
    }

    pub fn get_mouse_y(&self) -> f32 {
        self.input.mouse.get_y() as f32
    }

    pub fn get_mouse_wheel(&self) -> f32 {
        self.input.mouse.get_wheel_delta() as f32
    }

    pub fn get_mouse_wheel_h(&self) -> f32 {
        self.input.mouse.get_wheel_h_delta() as f32
    }

    pub fn mouse_pressed(&self, code: MOUSECODE) -> bool {
        self.input.mouse.pressed(code)
    }

    pub fn mouse_held(&self, code: MOUSECODE) -> bool {
        self.input.mouse.held(code)
    }

    pub fn mouse_released(&self, code: MOUSECODE) -> bool {
        self.input.mouse.released(code)
    }

    pub fn keyboard_pressed(&self, code: KEYCODE) -> bool {
        self.input.keyboard.pressed(code)
    }

    pub fn keyboard_held(&self, code: KEYCODE) -> bool {
        self.input.keyboard.held(code)
    }

    pub fn keyboard_released(&self, code: KEYCODE) -> bool {
        self.input.keyboard.released(code)
    }

    pub fn gamepad_pressed(&self, code: GAMEPAD) -> bool {
        self.input.controllers[0].pressed(code)
    }

    pub fn gamepad_held(&self, code: GAMEPAD) -> bool {
        self.input.controllers[0].held(code)
    }

    pub fn gamepad_released(&self, code: GAMEPAD) -> bool {
        self.input.controllers[0].released(code)
    }

    pub fn gamepad_left_trigger_pressed(&self, threshold: u8) -> bool {
        self.input.controllers[0].left_trigger_pressed(threshold)
    }

    pub fn gamepad_left_trigger_held(&self, threshold: u8) -> bool {
        self.input.controllers[0].left_trigger_held(threshold)
    }

    pub fn gamepad_left_trigger_released(&self, threshold: u8) -> bool {
        self.input.controllers[0].left_trigger_released(threshold)
    }

    pub fn gamepad_left_trigger_state(&self) -> u8 {
        self.input.controllers[0].get_left_trigger()
    }

    pub fn gamepad_right_trigger_pressed(&self, threshold: u8) -> bool {
        self.input.controllers[0].right_trigger_pressed(threshold)
    }

    pub fn gamepad_right_trigger_held(&self, threshold: u8) -> bool {
        self.input.controllers[0].right_trigger_held(threshold)
    }

    pub fn gamepad_right_trigger_released(&self, threshold: u8) -> bool {
        self.input.controllers[0].right_trigger_released(threshold)
    }

    pub fn gamepad_right_trigger_state(&self) -> u8 {
        self.input.controllers[0].get_right_trigger()
    }

    pub fn gamepad_left_stick_x(&self) -> i16 {
        self.input.controllers[0].get_left_stick_x()
    }

    pub fn gamepad_left_stick_y(&self) -> i16 {
        self.input.controllers[0].get_left_stick_y()
    }

    pub fn gamepad_right_stick_x(&self) -> i16 {
        self.input.controllers[0].get_right_stick_x()
    }

    pub fn gamepad_right_stick_y(&self) -> i16 {
        self.input.controllers[0].get_right_stick_y()
    }

    pub fn gamepad_set_vibration(&mut self, left_speed: u16, right_speed: u16) {
        self.input.controllers[0].set_vibration(left_speed, right_speed);
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // System Helpers
    ///////////////////////////////////////////////////////////////////////////////////////////////////

    ///End system event loop.
    pub fn quit(&mut self) {
        self.resources.running = false;
    }

    pub fn screen_width(&self) -> f32 {
        self.window.dimensions.width as f32
    }

    pub fn screen_height(&self) -> f32 {
        self.window.dimensions.height as f32
    }

    pub fn show_cursor(&mut self) {
        self.resources.show_cursor = true;
    }

    pub fn hide_cursor(&mut self) {
        self.resources.hide_cursor = true;
    }

    fn set_cursor_state(&mut self) {
        if self.resources.show_cursor {
            unsafe { while ShowCursor(1) < 0 {} }
            self.resources.show_cursor = false;
        }
        if self.resources.hide_cursor {
            unsafe { while ShowCursor(0) >= 0 {} }
            self.resources.hide_cursor = false;
        }
    }

    ///Get screen mouse position in world space.
    pub fn screen_to_world_point(&self) -> (f32, f32) {
        let x = ((self.input.mouse.get_x() as f32) / self.screen_width()) * 2.0 - 1.0;
        let y = -((self.input.mouse.get_y() as f32) / self.screen_height()) * 2.0 + 1.0;
        let z = -1.0;

        let position = Vector::new(x, y, z, 1.0);

        let (view, proj) = self.cam.get_proj_view();
        let Some(inv_viewproj) = (view * proj).invert() else {
            return (0.0, 0.0);
        };
        let new_pos = inv_viewproj * position;

        (new_pos.x, new_pos.y)
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // Statistics
    ///////////////////////////////////////////////////////////////////////////////////////////////////

    pub fn get_frametime(&self) -> f32 {
        self.stats.get_average_frametime()
    }

    pub fn get_fps(&self) -> usize {
        self.stats.get_average_fps()
    }

    pub fn get_update_time(&self) -> usize {
        self.stats.get_update_time()
    }

    pub fn get_draw_time(&self) -> usize {
        self.stats.get_draw_time()
    }

    pub fn get_particle_time(&self) -> usize {
        self.stats.get_particle_time()
    }
}

pub trait EventFunctions {
    fn init(&mut self, system: &mut System);
    fn update(&mut self, system: &mut System, dt: Duration);
    fn draw(&mut self, system: &mut System);
}

pub struct Event {}

impl Event {
    pub fn run(mut game: impl EventFunctions, mut system: System) {
        unsafe {
            let mut message = MSG::default();
            let mut ticks = Instant::now();
            let frametime = if system.lock_fps { 16u64 } else { 0u64 };

            game.init(&mut system);

            while system.resources.running {
                while PeekMessageA(&mut message, 0, 0, 0, PM_REMOVE) > 0 {
                    if message.message == WM_QUIT {
                        system.resources.running = false;
                    }

                    TranslateMessage(&message);
                    DispatchMessageA(&message);
                }

                while (Instant::now() - ticks) < Duration::from_millis(frametime) {}
                let dt = Instant::now() - ticks;
                ticks = Instant::now();

                system.stats.calculate_frametime(dt);

                let update_time = Instant::now();
                system.set_cursor_state();
                system.input.poll(system.window.window_handle);
                game.update(&mut system, dt);
                system.stats.update_time(update_time);

                let draw_time = Instant::now();
                game.draw(&mut system);
                system.draw_sprites();
                system.stats.draw_time(draw_time);

                let particle_time = Instant::now();
                if system.particle_active() {
                    system.particle_update(dt);
                    system.particle_draw();
                }
                system.stats.particle_time(particle_time);

                system.input.advance();
                system.renderer.display_frame();
            }
        }
    }
}

 # ROZE - The zero dependency game engine.

 ROZE is a barebones, zero dependency Windows game engine using win32 and OpenGL.
 This is the 0.1 release and is subject to major api changes.

 #### Current Features:
 * Texture loading system that supports 32bit rgba BMP and DDS files
 * Audio loading system that supports WAV files
 * Audio playback system for playing one-shots and loops
 * Text rendering
 * Input system for Mouse, Keyboard and Xinput Gamepads
 * Random number generation system
 * Textured Sprite and Primitives system
 * Basic Orthographic Camera system
 * Barebones Math library
 * Textured particle system
 * Basic fps, frametime and performance statistics

 ### Entry point:
 * The System struct defines most of the important functions.
     * Check SystemConf if you want more project setup options.

 * EventFunctions must be implemented to start a project.

 * Check out ParticleProperties to define a particle.

 #### Getting started:
 Displays a 600x600 window with red background.

 ```rs
 pub struct Game {}

 impl EventFunctions for Game {
     fn init(&mut self, system: &mut System) {}
     fn update(&mut self, system: &mut System, dt: Duration) {}
     fn draw(&mut self, system: &mut System) {
         system.display_clear(1.0, 0.0, 0.0, 1.0);
     }
 }

 fn main() {
     let system = System::new(600, 600, "Game Title");
     let game = Game {};
     Event::run(game, system);
 }
 ```

 #### Using sprite, texture and audio systems:
 Displays a window with a spinning textured sprite. An audio clip plays when the left mouse
 button is clicked.

 ```rs
 pub struct Game {
     rotation: f32,
 }

 impl EventFunctions for Game {
     fn init(&mut self, system: &mut System) {
         system.load_texture("textures/SpriteA.bmp");
         system.load_audio("audio/sound.wav");
     }
     fn update(&mut self, system: &mut System, dt: Duration) {
         self.rotation += 90.0 * dt.as_secs_f32();

         if system.mouse_pressed(MOUSECODE::L) {
             system.play_one_shot("sound", 1.0);
         }

         if system.keyboard_pressed(KEYCODE::ESC) {
             system.quit();
         }
     }
     fn draw(&mut self, system: &mut System) {
         system.display_clear(0.05, 0.08, 0.05, 0.0);

         system.sprite(
             system.screen_width() / 2.0,
             system.screen_height() / 2.0,
             0.0,
             100.0,
             100.0,
             self.rotation,
             system.get_texture("SpriteA"),
             (1.0, 1.0, 1.0, 1.0).into(),
         );
     }
 }

 fn main() {
     let system = System::new(800, 800, "Sprites, Textures and Audio");
     let game = Game { rotation: 0.0 };
     Event::run(game, system);
 }
 ```

 #### Using particle and text systems:
 Define an untextured colored particle effect that moves out from the center of the screen. Also
 define a font and display the current fps in the top right of the screen.

 ```rs
 pub struct Game {
     particle_prop: ParticleProperties,
 }

 impl Game {
     pub fn new() -> Self {
         let particle_prop = ParticleProperties {
             pos: (0.0, 0.0, 0.0).into(),
             velocity: (0.0, 0.0, 0.0).into(),
             velocity_var: (150.0, 150.0, 0.0).into(),
             color_begin: (1.0, 0.0, 0.0, 1.0).into(),
             color_end: (0.0, 0.0, 1.0, 0.0).into(),
             size_begin: 15.0,
             size_end: 25.0,
             size_var: 10.0,
             lifetime: 5.0,
             tex_id: None,
         };
         Self { particle_prop }
     }
 }

 impl EventFunctions for Game {
     fn init(&mut self, system: &mut System) {
         self.particle_prop.pos = (
             system.screen_width() / 2.0,
             system.screen_height() / 2.0,
             0.0,
         )
             .into();

         system.define_font(
             "myFont",
             22,
             Weight::NORMAL,
             false,
             false,
             false,
             "consolas",
         );
     }
     fn update(&mut self, system: &mut System, _dt: Duration) {
         system.particle_emit(&self.particle_prop);

         if system.keyboard_pressed(KEYCODE::ESC) {
             system.quit();
         }
     }
     fn draw(&mut self, system: &mut System) {
         system.display_clear(0.05, 0.08, 0.05, 0.0);

         system.text(
             &format!("fps: {}", system.get_fps()),
             system.screen_width() - 100.0,
             20.0,
             (1.0, 1.0, 1.0, 1.0),
             "myFont",
         );
     }
 }

 fn main() {
     let config = SystemConf {
         window_title: "Particles".into(),
         screen_width: 1200,
         screen_height: 1200,
         audio_one_shot_channels: 10,
         audio_loop_channels: 5,
         particle_pool_size: 5000,
         rng_seed: 1,
         rng_pool_size: 1_000_000,
         lock_fps: false,
     };
     let system = System::new_ex(config);
     let game = Game::new();
     Event::run(game, system);
 }
 ```


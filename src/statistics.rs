use std::time::{Duration, Instant};

pub struct Stats {
    tick_index: usize,
    tick_sum: usize,
    tick_list: [usize; 100],

    update_time: usize,
    draw_time: usize,
    particle_time: usize,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            tick_index: 0,
            tick_sum: 0,
            tick_list: [0; 100],

            update_time: 0,
            draw_time: 0,
            particle_time: 0,
        }
    }

    pub fn calculate_frametime(&mut self, dt: Duration) {
        self.tick_sum -= self.tick_list[self.tick_index];
        self.tick_sum += dt.as_millis() as usize;

        self.tick_list[self.tick_index] = dt.as_millis() as usize;

        self.tick_index += 1;
        if self.tick_index >= self.tick_list.len() {
            self.tick_index = 0;
        }
    }

    pub fn get_average_frametime(&self) -> f32 {
        self.tick_sum as f32 / self.tick_list.len() as f32
    }

    pub fn get_average_fps(&self) -> usize {
        (1000.0 / self.get_average_frametime()) as usize
    }

    pub fn update_time(&mut self, update_time: Instant) {
        self.update_time = (Instant::now() - update_time).as_micros() as usize;
    }

    pub fn get_update_time(&self) -> usize {
        self.update_time
    }

    pub fn draw_time(&mut self, draw_time: Instant) {
        self.draw_time = (Instant::now() - draw_time).as_micros() as usize;
    }

    pub fn get_draw_time(&self) -> usize {
        self.draw_time
    }

    pub fn particle_time(&mut self, particle_time: Instant) {
        self.particle_time = (Instant::now() - particle_time).as_micros() as usize;
    }

    pub fn get_particle_time(&self) -> usize {
        self.particle_time
    }
}

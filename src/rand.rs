pub struct RngEngine {
    table: Vec<f64>,
    position: usize,
    seed: u64,
}

impl RngEngine {
    pub const MULT: f64 = 16807.0;
    pub const MODULO: f64 = 2147483647.0;

    pub fn new(seed: u64, size: usize) -> Self {
        let table = Self::pseudo_uniform(seed as f64, size as f64);
        Self {
            table,
            position: 0,
            seed,
        }
    }

    pub fn get_int_range(&mut self, low: usize, high: usize) -> usize {
        let low = low as f64;
        let high = high as f64;
        if self.position >= self.table.len() {
            self.seed += 1;
            self.regenerate_uniform(self.seed, self.table.len());
        }
        let num = self.table[self.position];
        self.position += 1;
        (low + (high - low) * num).floor() as usize
    }

    pub fn get_float_range(&mut self, low: f64, high: f64) -> f64 {
        if self.position >= self.table.len() {
            self.seed += 1;
            self.regenerate_uniform(self.seed, self.table.len())
        }
        let num = self.table[self.position];
        self.position += 1;
        low + (high - low) * num
    }

    fn regenerate_uniform(&mut self, seed: u64, size: usize) {
        self.table = Self::pseudo_uniform(seed as f64, size as f64);
        self.position = 0;
        self.seed = seed;
    }

    fn pseudo_uniform(seed: f64, size: f64) -> Vec<f64> {
        let mut uniform = vec![0.0; size as usize];
        let mut x = (seed * Self::MULT + 1.0) % Self::MODULO;
        uniform[0] = x / Self::MODULO;
        for value in uniform.iter_mut().take(size as usize).skip(1) {
            x = (x * Self::MULT + 1.0) % Self::MODULO;
            *value = x / Self::MODULO;
        }
        uniform
    }
}

#[derive(Debug, Copy, Clone)]
pub struct McScheduler {
    pub max_step: u32,
    pub max_temperature: f64,
    pub min_temperature: f64,
}

impl McScheduler {
    pub fn new(max_step: u32, max_temperature: f64, min_temperature: f64) -> McScheduler {
        McScheduler {
            max_step,
            max_temperature,
            min_temperature,
        }
    }

    pub fn temperature(&self, step: u32) -> f64 {
        let t = step as f64 / self.max_step as f64;
        self.max_temperature.powf(t) * self.min_temperature.powf(1.0 - t)
    }
}

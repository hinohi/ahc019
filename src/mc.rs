#[derive(Debug, Copy, Clone)]
pub struct McParams {
    pub max_step: u32,
    pub max_temperature: f64,
    pub min_temperature: f64,
    pub erase_small_th: usize,
    pub cut_off: f64,
}

impl McParams {
    pub fn temperature(&self, step: u32) -> f64 {
        let t = step as f64 / self.max_step as f64;
        self.max_temperature.powf(t) * self.min_temperature.powf(1.0 - t)
    }
}

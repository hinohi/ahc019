use std::time::Duration;

#[derive(Debug, Copy, Clone)]
pub struct McParams {
    pub mc_run: u64,
    pub max_temperature: f64,
    pub min_temperature: f64,
    pub erase_small_th: usize,
    pub erase_shared_p: f64,
    pub cut_off: f64,
}

impl McParams {
    pub fn temperature(&self, limit: Duration, elapsed: Duration) -> f64 {
        let t = elapsed.as_secs_f64() / limit.as_secs_f64();
        self.max_temperature.powf(1.0 - t) * self.min_temperature.powf(t)
    }
}

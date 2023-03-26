use std::time::Duration;

#[derive(Debug, Copy, Clone, Default)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temperature() {
        let params = McParams {
            max_temperature: 10.0,
            min_temperature: 1.0,
            ..Default::default()
        };
        let limit = Duration::from_millis(1000);
        assert_eq!(params.temperature(limit, Duration::from_millis(0)), 10.0);
        assert_eq!(params.temperature(limit, Duration::from_millis(1000)), 1.0);
    }
}

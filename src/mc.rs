#[derive(Debug, Copy, Clone, Default)]
pub struct McParams {
    pub mc_run: u64,
    pub erase_small_th: usize,
    pub erase_shared_p: f64,
}

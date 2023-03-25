use ahc019::{mc_solve, McParams};
use proconio::{input, marker::Bytes};
use rand_pcg::Mcg128Xsl64;
use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();
    input! {
        d: u8,
        front1: [Bytes; d],
        right1: [Bytes; d],
        front2: [Bytes; d],
        right2: [Bytes; d],
    }
    let params = McParams {
        cut_off: 3.0,
        erase_small_th: d as usize,
        max_temperature: 0.5,
        mc_run: 1,
        min_temperature: 1e-8,
    };
    let mut rng = Mcg128Xsl64::new(3456);
    let result = mc_solve(
        start,
        Duration::from_millis(200),
        &mut rng,
        d,
        &front1,
        &right1,
        &front2,
        &right2,
        params,
    );
    println!("{}", result.run_count);
}

use ahc019::{mc_solve, McParams, SolveInput};
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
        erase_shared_p: 0.0,
    };
    let input = SolveInput {
        start,
        limit: Duration::from_millis(200),
        front1,
        right1,
        front2,
        right2,
        params,
    };
    let mut rng = Mcg128Xsl64::new(3456);
    let result = mc_solve(&mut rng, &input, d);
    println!("{}\t{}", (result.run_count as f64).ln(), result.score.ln());
}

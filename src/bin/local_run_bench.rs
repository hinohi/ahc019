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
    let mut params = McParams::opt(d);
    params.mc_run = 1;
    let input = SolveInput {
        start,
        limit: Duration::from_millis(200),
        front1,
        right1,
        front2,
        right2,
        params: McParams::opt(d),
    };
    let mut rng = Mcg128Xsl64::new(3456);
    let result = mc_solve(&mut rng, &input, d);
    println!("{}\t{}", (result.run_count as f64).ln(), result.score.ln());
}

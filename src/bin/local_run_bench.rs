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
        erase_small_th: match d {
            5 => 5,
            6 => 5,
            7 => 8,
            8 => 14,
            9 => 17,
            10 => 20,
            11 => 25,
            12 => 30,
            13 => 35,
            14 => 40,
            _ => unreachable!(),
        },
        erase_big_p: 0.3,
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
    eprintln!("{}", result.best_update_count);
    println!("{}\t{}", (result.run_count as f64).ln(), result.score.ln());
}

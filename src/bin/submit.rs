use ahc019::{mc_solve, McParams, SolveInput};
use proconio::{input, marker::Bytes};
use rand_pcg::Mcg128Xsl64;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

fn print_v(v: &[u16], block_id_map: &HashMap<u16, usize>) {
    for (i, g) in v.iter().enumerate() {
        if i != 0 {
            print!(" ");
        }
        print!("{}", block_id_map.get(g).unwrap_or(&0));
    }
    println!();
}

fn print_ans(g1: &[u16], g2: &[u16]) {
    let mut block_id_map = HashMap::new();
    for &g in g1.iter().chain(g2.iter()) {
        if g == 0 || g == !0 {
            continue;
        }
        let id = block_id_map.len() + 1;
        block_id_map.entry(g).or_insert(id);
    }
    println!("{}", block_id_map.len());
    print_v(g1, &block_id_map);
    print_v(g2, &block_id_map);
}

fn main() {
    let start = Instant::now();
    input! {
        d: u8,
        front1: [Bytes; d],
        right1: [Bytes; d],
        front2: [Bytes; d],
        right2: [Bytes; d],
    }
    let params = McParams::opt(d);
    let input = SolveInput {
        start,
        limit: Duration::from_millis(5800),
        front1,
        right1,
        front2,
        right2,
        params,
    };
    let mut rng = Mcg128Xsl64::new(9085);
    let result = mc_solve(&mut rng, &input, d);
    eprintln!("{} {}", result.run_count, result.score);
    print_ans(&result.g1, &result.g2);
}

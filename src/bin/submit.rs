use ahc019::{solve, McScheduler};
use proconio::{input, marker::Bytes};
use rand_pcg::Mcg128Xsl64;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

fn print_ans(v: &[u16], block_id_map: &HashMap<u16, usize>) {
    for (i, g) in v.iter().enumerate() {
        if i != 0 {
            print!(" ");
        }
        print!("{}", block_id_map.get(g).unwrap_or(&0));
    }
    println!();
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
    let scheduler = McScheduler::new(1000, 20.0, 1e-4);
    let mut rng = Mcg128Xsl64::new(9085);
    let mut best_score = 1e300;
    let mut best = (Vec::new(), Vec::new());
    let mut mc_run = 0;
    while start.elapsed() < Duration::from_millis(5500) {
        mc_run += 1;
        let (g1, g2, score) = solve(&mut rng, d, &front1, &right1, &front2, &right2, scheduler);
        if score < best_score {
            best_score = score;
            best = (g1, g2);
        }
    }
    let (g1, g2) = best;
    let mut block_id_map = HashMap::new();
    for &g in g1.iter().chain(g2.iter()) {
        if g == 0 || g == !0 {
            continue;
        }
        let id = block_id_map.len() + 1;
        block_id_map.entry(g).or_insert(id);
    }
    println!("{}", block_id_map.len());
    print_ans(&g1, &block_id_map);
    print_ans(&g2, &block_id_map);
    eprintln!("{} {}", mc_run, best_score);
}

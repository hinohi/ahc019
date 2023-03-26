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

fn get_params(d: u8) -> McParams {
    match d {
        5 => McParams {
            erase_small_th: 5,
            erase_big_p: 0.3,
        },
        6 => McParams {
            erase_small_th: 5,
            erase_big_p: 0.3,
        },
        7 => McParams {
            erase_small_th: 7,
            erase_big_p: 0.3,
        },
        8 => McParams {
            erase_small_th: 14,
            erase_big_p: 0.3,
        },
        9 => McParams {
            erase_small_th: 17,
            erase_big_p: 0.3,
        },
        10 => McParams {
            erase_small_th: 20,
            erase_big_p: 0.3,
        },
        11 => McParams {
            erase_small_th: 24,
            erase_big_p: 0.3,
        },
        12 => McParams {
            erase_small_th: 27,
            erase_big_p: 0.3,
        },
        13 => McParams {
            erase_small_th: 35,
            erase_big_p: 0.3,
        },
        14 => McParams {
            erase_small_th: 43,
            erase_big_p: 0.3,
        },
        _ => unreachable!(),
    }
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
    let params = get_params(d);
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
    eprintln!(
        "{} {} {}",
        result.run_count, result.best_update_count, result.score
    );
    print_ans(&result.g1, &result.g2);
}

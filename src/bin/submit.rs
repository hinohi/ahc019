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
        // -0.8665073454427983
        5 => McParams {
            erase_shared_p: 0.7641974640494824,
            erase_small_th: 5,
            mc_run: 45,
        },
        // -1.0465880873030173
        6 => McParams {
            erase_shared_p: 0.5271543071699281,
            erase_small_th: 8,
            mc_run: 100,
        },
        // -1.0383037291956507
        7 => McParams {
            erase_shared_p: 0.5276321643573454,
            erase_small_th: 11,
            mc_run: 65,
        },
        // -1.0700763969294835
        8 => McParams {
            erase_shared_p: 0.8262167169942501,
            erase_small_th: 12,
            mc_run: 86,
        },
        // -1.1036122300059443
        9 => McParams {
            erase_shared_p: 0.6772038451250363,
            erase_small_th: 17,
            mc_run: 31,
        },
        // -1.1916639012730843
        10 => McParams {
            erase_shared_p: 0.5750391690408527,
            erase_small_th: 21,
            mc_run: 56,
        },
        // -1.0348616264530257
        11 => McParams {
            erase_shared_p: 0.6873535191800519,
            erase_small_th: 30,
            mc_run: 50,
        },
        // -1.4303001266271784
        12 => McParams {
            erase_shared_p: 0.8043228784487291,
            erase_small_th: 37,
            mc_run: 56,
        },
        // -1.407338859035549
        13 => McParams {
            erase_shared_p: 0.5669638180845599,
            erase_small_th: 47,
            mc_run: 23,
        },
        // -1.5757117753234124
        14 => McParams {
            erase_shared_p: 0.7236849754072749,
            erase_small_th: 52,
            mc_run: 46,
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
    eprintln!("{} {}", result.run_count, result.score);
    print_ans(&result.g1, &result.g2);
}

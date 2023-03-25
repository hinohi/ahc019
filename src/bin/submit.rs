use ahc019::{mc_solve, McParams};
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
    print_v(&g1, &block_id_map);
    print_v(&g2, &block_id_map);
}

fn get_params(d: u8) -> McParams {
    match d {
        5 => McParams {
            mc_run: 71,
            max_temperature: 2.424978521005016,
            min_temperature: 1e-8,
            erase_small_th: 5,
            cut_off: 3.7165636563013154,
        },
        6 => McParams {
            mc_run: 50,
            max_temperature: 1.424978521005016,
            min_temperature: 1e-8,
            erase_small_th: 8,
            cut_off: 3.7165636563013154,
        },
        7 => McParams {
            mc_run: 40,
            max_temperature: 0.7,
            min_temperature: 1e-8,
            erase_small_th: 9,
            cut_off: 3.7165636563013154,
        },
        8 => McParams {
            mc_run: 28,
            max_temperature: 0.10302552733534334,
            min_temperature: 1e-8,
            erase_small_th: 10,
            cut_off: 3.1732947086447654,
        },
        9 => McParams {
            mc_run: 24,
            max_temperature: 0.09,
            min_temperature: 1e-8,
            erase_small_th: 14,
            cut_off: 3.2732947086447654,
        },
        10 => McParams {
            mc_run: 20,
            max_temperature: 0.08,
            min_temperature: 1e-8,
            erase_small_th: 18,
            cut_off: 3.333955750814579,
        },
        11 => McParams {
            mc_run: 18,
            max_temperature: 0.08,
            min_temperature: 1e-8,
            erase_small_th: 20,
            cut_off: 3.333955750814579,
        },
        12 => McParams {
            mc_run: 16,
            max_temperature: 0.07667314618950373,
            min_temperature: 1e-8,
            erase_small_th: 22,
            cut_off: 3.971231018940054,
        },
        13 => McParams {
            mc_run: 12,
            max_temperature: 0.05667314618950373,
            min_temperature: 1e-8,
            erase_small_th: 26,
            cut_off: 3.971231018940054,
        },
        14 => McParams {
            mc_run: 8,
            max_temperature: 0.03667314618950373,
            min_temperature: 1e-8,
            erase_small_th: 30,
            cut_off: 3.971231018940054,
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
    let mut rng = Mcg128Xsl64::new(9085);
    let result = mc_solve(
        start,
        Duration::from_millis(5800),
        &mut rng,
        d,
        &front1,
        &right1,
        &front2,
        &right2,
        params,
    );
    eprintln!("{} {}", result.run_count, result.score);
    print_ans(&result.g1, &result.g2);
}

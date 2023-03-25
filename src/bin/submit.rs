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
            cut_off: 3.7165636563013154,
            erase_small_th: 5,
            max_temperature: 2.424978521005016,
            mc_run: 71,
            min_temperature: 1e-8,
        },
        6 => McParams {
            cut_off: 8.255809235452936,
            erase_small_th: 5,
            max_temperature: 0.12783905726356284,
            mc_run: 89,
            min_temperature: 1e-8,
        },
        7 => McParams {
            cut_off: 8.498373594249625,
            erase_small_th: 7,
            max_temperature: 14.592163756296676,
            mc_run: 50,
            min_temperature: 1e-8,
        },
        8 => McParams {
            cut_off: 3.1732947086447654,
            erase_small_th: 10,
            max_temperature: 0.10302552733534334,
            mc_run: 28,
            min_temperature: 1e-8,
        },
        9 => McParams {
            cut_off: 0.50137301000144,
            erase_small_th: 11,
            max_temperature: 22.064939273483084,
            mc_run: 14,
            min_temperature: 1e-8,
        },
        10 => McParams {
            cut_off: 2.4754740521591696,
            erase_small_th: 13,
            max_temperature: 1.5483427427216714,
            mc_run: 6,
            min_temperature: 1e-8,
        },
        11 => McParams {
            cut_off: 7.02960251174804,
            erase_small_th: 17,
            max_temperature: 0.051489579282792755,
            mc_run: 73,
            min_temperature: 1e-8,
        },
        12 => McParams {
            cut_off: 3.0213848130509273,
            erase_small_th: 18,
            max_temperature: 0.2017942454490659,
            mc_run: 3,
            min_temperature: 1e-8,
        },
        13 => McParams {
            cut_off: 5.166818776552572,
            erase_small_th: 28,
            max_temperature: 6.334108581065736,
            mc_run: 33,
            min_temperature: 1e-8,
        },
        14 => McParams {
            cut_off: 2.520969159388388,
            erase_small_th: 29,
            max_temperature: 0.2005563105195312,
            mc_run: 6,
            min_temperature: 1e-8,
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

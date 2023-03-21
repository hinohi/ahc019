use ahc019::{mc_run, McParams};
use proconio::{input, marker::Bytes, source::auto::AutoSource};
use rand_pcg::Mcg128Xsl64;

fn main() {
    let data = [
        &include_bytes!("../../tools/in/0000.txt")[..],
        &include_bytes!("../../tools/in/0001.txt")[..],
        &include_bytes!("../../tools/in/0002.txt")[..],
        &include_bytes!("../../tools/in/0003.txt")[..],
        &include_bytes!("../../tools/in/0004.txt")[..],
        &include_bytes!("../../tools/in/0005.txt")[..],
        &include_bytes!("../../tools/in/0006.txt")[..],
        &include_bytes!("../../tools/in/0007.txt")[..],
        &include_bytes!("../../tools/in/0008.txt")[..],
        &include_bytes!("../../tools/in/0009.txt")[..],
        &include_bytes!("../../tools/in/0010.txt")[..],
    ];
    let mut rng = Mcg128Xsl64::new(1);
    for &data in data.iter() {
        let src = AutoSource::new(data);
        input! {
            from src,
            d: u8,
            front1: [Bytes; d],
            right1: [Bytes; d],
            front2: [Bytes; d],
            right2: [Bytes; d],
        }
        let params = McParams {
            max_step: 1000,
            max_temperature: 20.0,
            min_temperature: 1e-4,
            erase_small_th: 2,
            cut_off: 3.0,
        };
        let (g1, g2, score) = mc_run(&mut rng, d, &front1, &right1, &front2, &right2, params);
        assert!(g1.len() < 10000);
        assert!(g2.len() < 10000);
        assert!(score > 0.0);
    }
}
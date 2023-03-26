use ahc019::{mc_solve, McParams, SolveInput};
use proconio::{input, marker::Bytes, source::auto::AutoSource};
use rand_pcg::Mcg128Xsl64;
use std::time::{Duration, Instant};

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
        let mut params = McParams::opt(d);
        params.mc_run /= 6;
        if params.mc_run == 0 {
            params.mc_run = 1;
        }
        let intput = SolveInput {
            start: Instant::now(),
            limit: Duration::from_millis(1000),
            front1,
            right1,
            front2,
            right2,
            params,
        };
        let r = mc_solve(&mut rng, &intput, d);
        assert!(r.g1.len() < 10000);
        assert!(r.g2.len() < 10000);
        assert!(r.score > 0.0);
    }
}

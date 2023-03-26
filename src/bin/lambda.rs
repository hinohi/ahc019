use ahc019::{mc_solve, McParams, SolveInput};
use lambda_runtime::{service_fn, Error, LambdaEvent};
use rand_pcg::Mcg128Xsl64;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

#[derive(Deserialize, Serialize)]
struct Request {
    seed: u64,
    d: usize,
    mc_run: u64,
    erase_small_th: usize,
    erase_shared_p: f64,
}

#[derive(Serialize)]
struct Response {
    request: Request,
    score: f64,
    run_count: u32,
}

fn face_conv(input: &[Vec<i32>]) -> Vec<Vec<u8>> {
    let mut output = Vec::with_capacity(input.len());
    for row in input {
        output.push(
            row.iter()
                .map(|&i| if i == 1 { b'1' } else { b'0' })
                .collect(),
        )
    }
    output
}

async fn func(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let input = tools::gen(event.payload.seed, Some(event.payload.d));
    // 近似的にここで測る
    let start = Instant::now();
    let params = McParams {
        mc_run: event.payload.mc_run,
        erase_small_th: event.payload.erase_small_th,
        erase_shared_p: event.payload.erase_shared_p,
    };
    let input = SolveInput {
        start,
        limit: Duration::from_millis(5800),
        front1: face_conv(&input.f[0]),
        right1: face_conv(&input.r[0]),
        front2: face_conv(&input.f[1]),
        right2: face_conv(&input.r[1]),
        params,
    };
    let mut rng = Mcg128Xsl64::new(32343);
    let best = mc_solve(&mut rng, &input, event.payload.d as u8);

    Ok(Response {
        request: event.payload,
        score: best.score,
        run_count: best.run_count,
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

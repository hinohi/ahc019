use ahc019::{mc_solve, McParams};
use lambda_runtime::{service_fn, Error, LambdaEvent};
use rand_pcg::Mcg128Xsl64;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

#[derive(Deserialize, Serialize)]
struct Request {
    seed: u64,
    d: usize,
    max_step: u32,
    max_temperature: f64,
    min_temperature: f64,
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

    let front1 = face_conv(&input.f[0]);
    let right1 = face_conv(&input.r[0]);
    let front2 = face_conv(&input.f[1]);
    let right2 = face_conv(&input.r[1]);
    let mut rng = Mcg128Xsl64::new(32343);
    let scheduler = McParams::new(
        event.payload.max_step,
        event.payload.max_temperature,
        event.payload.min_temperature,
    );
    let best = mc_solve(
        start,
        Duration::from_millis(5800),
        &mut rng,
        event.payload.d as u8,
        &front1,
        &right1,
        &front2,
        &right2,
        scheduler,
    );

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

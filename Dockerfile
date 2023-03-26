###############
# Build
###############
# lambda_runtime 0.7 が edition=2021 であり、2021 の最初のバージョンは 1.56
FROM rust:1.56 as build

WORKDIR /app

COPY tools tools
COPY Cargo.* .
COPY src src

RUN cargo build --release --bin=lambda

###############
# Run
###############
FROM rust:1.56-slim

COPY --from=build /app/target/release/lambda /app/target/release/lambda
CMD ["/app/target/release/lambda"]

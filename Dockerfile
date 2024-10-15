FROM rust:1.72 as builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

RUN cargo test --verbose

FROM debian:buster-slim

COPY --from=builder /usr/src/app/target/release/rusticle /usr/local/bin/rusticle

CMD ["rusticle"]
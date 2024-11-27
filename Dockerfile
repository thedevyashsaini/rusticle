FROM rust:1.72 as builder

WORKDIR /usr/src/app

COPY . .

WORKDIR /usr/src/app/rusticle_store

RUN cargo build --release

RUN cargo test --verbose

FROM debian:buster-slim

COPY --from=builder /usr/src/app/rusticle_store/target/release/rusticle_store /usr/local/bin/rusticle_store

CMD ["rusticle_store"]
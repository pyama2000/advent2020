FROM rust:1 as builder
WORKDIR /usr/src/advent2020_api
COPY Cargo.toml Cargo.toml
RUN mkdir src && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN cargo build --release
RUN rm -rf target/release/deps/advent2020_api*
COPY . .
RUN cargo build --release
RUN cargo install --path .

FROM debian:buster-slim
EXPOSE 8000 8000
RUN apt-get update && apt-get install -y libssl-dev libpq-dev ca-certificates
COPY --from=builder /usr/local/cargo/bin/advent2020_api /usr/local/bin/advent2020_api

CMD ["advent2020_api"]

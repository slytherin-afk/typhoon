FROM rust:latest

WORKDIR /app

COPY . .

RUN cargo build --release

ENV RUST_BACKTRACE=1

ENTRYPOINT ["cargo", "run", "--release"]

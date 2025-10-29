FROM rust:1.82-slim

WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["./target/release/wallet_cli"]
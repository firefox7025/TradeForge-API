FROM rust:1-bullseye as cargo-build
WORKDIR /app
COPY ./src /app/src/
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
RUN cargo build --release
CMD ["/app/target/release/trade_forge_api"]

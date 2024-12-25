FROM rust:1-bullseye as cargo-build
WORKDIR /app
COPY ./src /app/src/
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
RUN /root/.cargo/bin/cargo build --release
FROM alpine:3
COPY --from=cargo-build /app/target/release/trade_forge_api .
CMD ["./trade_forge_api"]

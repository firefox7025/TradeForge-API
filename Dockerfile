FROM rust:1-bullseye as cargo-build
RUN apt-get update
RUN apt-get install musl-tools -y
RUN /root/.cargo/bin/rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY ./src /app/src/
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
RUN /root/.cargo/bin/cargo build --target x86_64-unknown-linux-musl --release
FROM alpine:3
COPY --from=cargo-build /app/target/x86_64-unknown-linux-musl/release/search_and_sip .
CMD ["./trade_forge_api"]

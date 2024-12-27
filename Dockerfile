
FROM rust:1-bullseye AS builder
RUN mkdir /build
COPY ["Cargo.toml", "Cargo.lock", "/build/"]
ADD src /build/src
WORKDIR /build
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install postgresql -y
RUN mkdir /app
COPY --from=builder /build/target/release/trade_forge_api /app/trade_forge_api
EXPOSE 8080
CMD "/app/trade_forge_api"
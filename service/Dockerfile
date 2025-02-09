FROM rust AS builder
    RUN apt-get update && apt-get install -y musl-tools
    WORKDIR /app
    RUN rustup target add x86_64-unknown-linux-musl
    COPY . .
    RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
    WORKDIR /usr/local/bin
    COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/telemetry /usr/local/bin/telemetry
    EXPOSE 80
    CMD ["/usr/local/bin/telemetry"]


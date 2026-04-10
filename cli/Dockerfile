# Build stage
FROM rust:1.75-slim as builder
WORKDIR /usr/src/idpctl
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
COPY --from=builder /usr/src/idpctl/target/release/idpctl /usr/local/bin/idpctl
ENTRYPOINT ["idpctl"]

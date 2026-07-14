# Build stage
FROM rust:1.96 as builder
WORKDIR /app
ADD . /app
RUN cargo build --release

# Prod stage
FROM gcr.io/distroless/cc:debian13
COPY --from=builder /app/target/release/tiny-ip /

ENTRYPOINT ["/tiny-ip"]

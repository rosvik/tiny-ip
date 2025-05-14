# Build stage
FROM rust:1.85 as builder
WORKDIR /app
ADD . /app
RUN cargo build --release

# Prod stage
FROM gcr.io/distroless/cc
COPY --from=builder /app/target/release/tiny-ip /

EXPOSE 8000

CMD ["./tiny-ip"]

# All of the paths below are in the linux container

# Build command using this Dockerfile:
# % docker build -t learn_rust .

# Create, run the container and delete after it exits
# % docker run --rm learn_rust

# Stage 1: Build the Rust binary
FROM rust:1-slim-bullseye AS builder
WORKDIR /app      
COPY . .
RUN cargo build --release

# Stage 2: Minimal runtime image
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/learn_rust /usr/local/bin/learn_rust
CMD ["learn_rust"]


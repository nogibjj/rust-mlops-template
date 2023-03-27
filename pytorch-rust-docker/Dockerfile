# First stage: builder
FROM rust:latest as builder

# Set environment variables
ENV APP pytorch-rust-docker

# Set working directory
WORKDIR /usr/src/$APP

# Copy the content of the current directory into the working directory
COPY . .

# Install required dependencies
RUN apt-get update && \
    apt-get install -y libtorch-dev && \
    rm -rf /var/lib/apt/lists/*

# Build the application
RUN cargo build --release

# Second stage: distroless
FROM gcr.io/distroless/cc

# Copy the binary from the builder stage
COPY --from=builder /usr/src/pytorch-rust-docker/target/release/pytorch_rust_docker /app/pytorch_rust_docker

# Set the entrypoint for the container
ENTRYPOINT ["/app/pytorch_rust_docker"]
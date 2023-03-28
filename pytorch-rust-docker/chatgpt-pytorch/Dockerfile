# Use the official Rust image as the base image
FROM rust:1.67

# Set the working directory
WORKDIR /app

# Install system dependencies required by PyTorch and tch-rs
RUN apt-get update && \
    apt-get install -y build-essential python3-dev python3-pip libopenblas-dev libopencv-dev

# Copy the Rust project into the container
COPY . .

# Build the Rust project
RUN cargo build --release

# Set the command to run the binary
CMD ["./target/release/chatgpt_pytorch"]

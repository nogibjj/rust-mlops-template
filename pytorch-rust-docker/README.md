### Notes

* This is a simple example of using `rust-bert` to load a pytorch model and run inference on an image.
* Using `rust-bert` to install pytorch here:  https://crates.io/crates/tch 
* To run: `cargo run -- resnet18.ot Walking_tiger_female.jpg`

### Docker Examples

The size is HUGE: `6.06GB`

You can also run inside a docker container:

* `docker build -t pytorch-rust-docker .`
* `docker run -it pytorch-rust-docker`
* Next inside the container run: `cargo run -- resnet18.ot Walking_tiger_female.jpg`

One possibility is to try this, but a key issue is the `libtorch_cpu.so`

```bash
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
```

** Warning Broken for Now **

To run this, you could do something like this:  
`docker run -it --rm 61503cf1e0b3 ./app/pytorch_rust_docker resnet18.ot Walking_tiger_female.jpg`



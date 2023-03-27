FROM rust:1.67.1 as build-env
ENV RUSTFLAGS="-C link-arg=-s -C lto -C codegen-units=1"
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/distcli /
CMD ["./distcli"]
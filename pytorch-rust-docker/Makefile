install:
	cargo clean &&\
		cargo build -j 1

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

build-docker:
	docker build -t pytorch-rust-docker .

run:
	cargo run 

all: format lint test run
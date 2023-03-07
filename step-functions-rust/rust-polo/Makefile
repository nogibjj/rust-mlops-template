install:
	#unfortunately python dependencies needed
	pip3 install cargo-lambda
format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run 

release:
	#cargo build --release --target x86_64-unknown-linux-musl
	cargo lambda build --release

release-arm:
	cargo lambda build --release --arm64

deploy:
	cargo lambda deploy

invoke:
	cargo lambda invoke --remote \
  		--data-ascii '{"payload": "Polo"}' \
  		--output-format json \
  		rust-polo

all: format lint test run
format:
	cargo fmt --quiet

lint:
	@rustup component add clippy 2> /dev/null
	cargo clippy --quiet

test:
	cargo test --quiet

build-docker:
	docker build -t distroless-cli .

run:
	cargo run -- greet --name bob

all: format lint test run
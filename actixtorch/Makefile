format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

build:
	docker build -t torch .

rundocker:
	docker run -it --rm -p 8080:8080 torch

run:
	cargo run 

all: format lint test run
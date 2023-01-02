install:
	pip install --upgrade pip &&\
		pip install -r requirements.txt
	cargo clean &&\
		cargo build -j 1


format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run 

all: format lint test run
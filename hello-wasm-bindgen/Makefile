install:
	curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
build-wasm:
	wasm-pack build --target web

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run 

all: format lint test run
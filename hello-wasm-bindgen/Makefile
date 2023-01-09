install:
	echo "Installing rustwasm and wasm3"
	curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
	## Install wasm3 if you want
	#brew install wasm3
	#arch -arm64 brew install wasm3 #if you are on M1

build-wasm:
	wasm-pack build --target web

serve:
	python3 -m http.server

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run 

all: format lint test run
build-py:
	cd python && virtualenv venv && source ./venv/bin/activate && cd ../lib && maturin develop --features python

run-py: build-py
	cd python && source ./venv/bin/activate && python main.py

build-wasm:
	cd lib && wasm-pack build --target nodejs --out-dir ../node/lib/core --out-name index -- --features wasm

run-wasm: build-wasm
	cd node && node main.mjs

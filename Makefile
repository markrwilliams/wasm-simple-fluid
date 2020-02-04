wasm = target/wasm32-unknown-unknown/release/wasm_simple_fluid.wasm

run: $(wasm)
	python serve.py

$(wasm):
	cargo build --target wasm32-unknown-unknown --release

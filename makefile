all: build-prereq build
.PHONY: all

build:
	cargo run exec -- 'echo test'

build-prereq:
	cd wasm-test; cargo build --target wasm32-wasi
	pwd
	mv wasm-test/target/wasm32-wasi/debug/wasm-test.wasm rules/wasm-test.wasm
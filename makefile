all: build-prereq build
.PHONY: all

build:
	cargo run exec -- 'echo test'

build-prereq:
	cd examples/hello-world; cargo build --target wasm32-wasi
	mv examples/hello-world/target/wasm32-wasi/debug/hello-world.wasm rules/wasm-test.wasm
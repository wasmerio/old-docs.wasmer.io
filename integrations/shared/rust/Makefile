%.wasm: %.rs
	rustc $^ -o $@ --target=wasm32-unknown-unknown

all: add.wasm passing-data.wasm host-functions.wasm handling-errors.wasm exit-early.wasm

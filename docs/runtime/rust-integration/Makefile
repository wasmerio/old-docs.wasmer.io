.PHONY: wasm examples run-all

# generate wasm
wasm:
	cargo build --release --target=wasm32-unknown-unknown -p hello-world-guest
	cargo build --release --target=wasm32-unknown-unknown -p passing-data-guest
	cargo build --release --target=wasm32-unknown-unknown -p host-functions-guest
	cargo build --release --target=wasm32-unknown-unknown -p handling-errors-guest
	cargo build --release --target=wasm32-unknown-unknown -p early-exit-guest

# run examples
examples:
	cargo run --release --example hello_world
	cargo run --release --example passing_data
	cargo run --release --example host_functions
	cargo run --release --example handling_errors
	cargo run --release --example early_exit

run-all: wasm examples

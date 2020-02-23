%.wasm: %.wat
	wat2wasm $^ -o $@

all: wasi/helloworld.wasm wasi/clocktimeget.wasm

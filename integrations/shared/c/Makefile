all:
	# This compilation requires Wasienv
	# You can install it here:
	# https://github.com/wasienv/wasienv

	wasmcc ./add.c -o add.wasm
	wasmcc ./passing-data.c -o passing-data.wasm
	wasmcc ./handling-errors.c -o handling-errors.wasm

	# Allow external undefined functions (For our import functions)
	# with "-Wl,--allow-undefined"
	# https://lld.llvm.org/WebAssembly.html

	wasmcc -Wl,--allow-undefined ./host-functions.c -o host-functions.wasm
	wasmcc -Wl,--allow-undefined ./exit-early.c -o exit-early.wasm

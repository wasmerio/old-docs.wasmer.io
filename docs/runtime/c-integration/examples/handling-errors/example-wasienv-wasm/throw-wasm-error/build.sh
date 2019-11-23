#/bin/bash

# Allow external undefined functions (For our import functions)
# https://lld.llvm.org/WebAssembly.html
wasmcc -Wl,--allow-undefined throw-wasm-error.c -o throw-wasm-error

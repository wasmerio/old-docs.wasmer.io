#/bin/bash

# Allow external undefined functions (For our import functions)
# https://lld.llvm.org/WebAssembly.html
wasmcc -Wl,--allow-undefined exit-early-import.c -o exit-early-import

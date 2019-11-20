#/bin/bash

# Allow external undefined functions (For our import functions)
# https://lld.llvm.org/WebAssembly.html
wasmcc -Wl,--allow-undefined host-counter.c -o host-counter

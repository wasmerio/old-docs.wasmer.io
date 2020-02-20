gcc handling-errors.c -I${WASMER_C_API}/include -L${WASMER_C_API}/lib -lwasmer -o handling-errors -rpath ${WASMER_C_API}/lib
# Note: add `-rpath ${WASMER_C_API}/lib` if you are in macOS

gcc exit-early.c -I${WASMER_C_API}/include -L${WASMER_C_API}/lib -lwasmer -o exit-early
# Note: add `-rpath ${WASMER_C_API}/lib` if you are in macOS

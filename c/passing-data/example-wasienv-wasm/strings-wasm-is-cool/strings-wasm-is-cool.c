
// Define a fixed size bbyte array
char byteBufferPointer[1000] = {0};

char* getBufferPointer() {
  return byteBufferPointer;
}

// Function that will add the string "Wasm is cool" to the pointer for the original string
char addWasmIsCool() {
  // Get the current string in the buffer
  byteBufferPointer[3] = 24;
  return byteBufferPointer[0];
}

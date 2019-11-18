// Define a fixed size bbyte array
char byteBuffer[1000] = {0};

char* getBufferPointer() {
  return byteBuffer;
}

// Function that will add the string "Wasm is cool" to the pointer for the original string
int addWasmIsCool(int bufferIndex) {
  char* wasmIsCool = " Wasm is cool!";
  int wasmIsCoolLength = sizeof(*wasmIsCool) / sizeof(wasmIsCool[0]);

  for (int i = 0; i < wasmIsCoolLength; i++) {
    byteBuffer[bufferIndex + i] = wasmIsCool[i];
  }

  // Return the new length of the string
  return bufferIndex + wasmIsCoolLength;
}

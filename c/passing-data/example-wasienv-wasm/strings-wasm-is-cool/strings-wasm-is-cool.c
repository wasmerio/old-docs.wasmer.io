// Define a fixed size bbyte array
unsigned char byteBuffer[100];

unsigned char*  getBufferPointer() {
  return byteBuffer;
}

// Function that will add the string "Wasm is cool" to the pointer for the original string
int addWasmIsCool() {

  // Define our total string
  char totalString[100];

  // Declare the end of a string character
  char endOfString = '\0';

  // Read in the string we passed in:
  int passedStringLength = 0;
  for(int i = 0; i < 100; i++) {
    if (byteBuffer[i] == endOfString) {
      // Break our of the loop
      i = 100;
    } else {
      totalString[i] = byteBuffer[i];
      passedStringLength++;
    }
  }

  char wasmIsCool[15] = "Wasm is cool!";
  int wasmIsCoolLength = sizeof(wasmIsCool) / sizeof(wasmIsCool[0]);

  for (int i = 0; i < wasmIsCoolLength; i++) {
    totalString[passedStringLength + i] = wasmIsCool[i];
  }

  int totalStringLength = passedStringLength + wasmIsCoolLength;

  // Add the trailing end of string character
  totalString[totalStringLength] = endOfString;

  // Write the string back to memory
  for (int i = 0; i < totalStringLength; i++) {
    byteBuffer[i] = totalString[i];
  }

  // Return the new length of the string
  return totalStringLength;
}

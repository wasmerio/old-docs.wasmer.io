// Define the maximum size of our buffer
const int MAX_BUFFER_SIZE = 100;

// Define the end of a string in C
// This will be used to find where a string ends in the byte buffer.
const char END_OF_STRING = '\0';

// Define a fixed size byte array
// That we can use as a buffer for passing data between
// The host application, and this guest WASM module
unsigned char byte_buffer[MAX_BUFFER_SIZE];

// Function to return a pointer to our buffer
unsigned char*  get_buffer_pointer() {
  return byte_buffer;
}

// Function that will fetch the data in the byteBuffer as a string,
// And then concatenate the string "WASM is Cool",
// And write the concatednated string back to the byteBuffer.
int add_wasm_is_cool() {

  // Define our total concatenated string
  char total_string[MAX_BUFFER_SIZE];

  // Read in the string that was passed in by the
  // host application into the byteBuffer

  // Record the length of the passed string
  int passed_string_length = 0;

  // Loop over the buffer, break out if we
  // Find the end of the string,
  // Otherwise add the characters to the total_tring.
  for(int i = 0; i < MAX_BUFFER_SIZE; i++) {
    if (byte_buffer[i] == END_OF_STRING) {
      // Break our of the loop
      i = MAX_BUFFER_SIZE;
    } else {
      // Add the character in the buffer to our total string
      total_string[i] = byte_buffer[i];
      passed_string_length++;
    }
  }

  // Define our WASM is cool string, and get its length
  char wasm_is_cool[15] = "WASM is cool!";
  int wasm_is_cool_length = sizeof(wasm_is_cool) / sizeof(wasm_is_cool[0]);

  // Loop over the WASM is cool string, concatenate it to the total string
  for (int i = 0; i < wasm_is_cool_length; i++) {
    total_string[passed_string_length + i] = wasm_is_cool[i];
  }

  // Get the length of the total string for conveinience.
  int total_string_length = passed_string_length + wasm_is_cool_length;

  // Add the trailing end of string character
  total_string[total_string_length] = END_OF_STRING;

  // Write the string back to our buffer
  for (int i = 0; i < total_string_length; i++) {
    byte_buffer[i] = total_string[i];
  }

  // Return the length of the total string
  return total_string_length;
}

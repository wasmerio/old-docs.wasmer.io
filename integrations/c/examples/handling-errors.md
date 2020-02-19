# Handling Errors

{% hint style="success" %}
**Note**: The final code for this example can be found on [GitHub](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/runtime/c-integration/examples/handling-errors)
{% endhint %}

There will come a time where running a WebAssembly module will not work, and trying to figure out why it does not work can be a difficult task! In the current MVP of WebAssembly, debugging is quite vauge, in runtimes for both the browser and the server. But errors can still be handled and debugged gracefully.

In this example, we will load a WebAssembly module that purposely divide by zero \(and arithmetic error\) on its exported function call. The host \(our C application\) will check for the error `call_result` and output the error message returned from Wasmer:

```c
#include <stdio.h>
#include "wasmer.h"
#include <assert.h>
#include <string.h>

// Function to print the most recent error string from Wasmer if we have them
void print_wasmer_error()
{
  int error_len = wasmer_last_error_length();
  printf("Error len: `%d`\n", error_len);
  char *error_str = malloc(error_len);
  wasmer_last_error_message(error_str, error_len);
  printf("Error str: `%s`\n", error_str);
}

// Function to create a Wasmer Instance
wasmer_instance_t *create_wasmer_instance() {

  // Create module name for our imports

  // Create a UTF-8 string as bytes for our module name. 
  // And, place the string into the wasmer_byte_array type so it can be used by our guest wasm instance.
  const char *module_name = "env";
  wasmer_byte_array module_name_bytes = { .bytes = (const uint8_t *) module_name,
    .bytes_len = strlen(module_name) };

  // Define an array containing our imports
  wasmer_import_t imports[] = {};

  // Read the wasm file bytes
  FILE *file = fopen("example-wasienv-wasm/throw-wasm-error/throw-wasm-error.wasm", "r");
  assert(file != NULL);
  fseek(file, 0, SEEK_END);
  long len = ftell(file);
  uint8_t *bytes = malloc(len);
  fseek(file, 0, SEEK_SET);
  fread(bytes, 1, len, file);
  fclose(file);

  // Instantiate a WebAssembly Instance from wasm bytes and imports
  wasmer_instance_t *instance = NULL;
  wasmer_result_t compile_result = wasmer_instantiate(
      &instance, // Our reference to our wasm instance 
      bytes, // The bytes of the WebAssembly modules
      len, // The length of the bytes of the WebAssembly module
      imports, // The Imports array the will be used as our importObject
      0 // The number of imports in the imports array
      );

  // Ensure the compilation was successful.
  if (compile_result != WASMER_OK)
  {
    print_wasmer_error();
  }

  // Assert the wasm instantion completed
  assert(compile_result == WASMER_OK);

  // Return the Wasmer Instance
  return instance;
}

int main() {

  // Initialize our Wasmer Instance
  wasmer_instance_t *instance = create_wasmer_instance();

  // Let's call the our throw_wasm_error Function in the guest wasm module

  // Define our results. Results are created with { 0 } to avoid null issues,
  // And will be filled with the proper result after calling the guest wasm function.
  wasmer_value_t result_one = { 0 };
  wasmer_value_t results[] = {result_one};

  // Define our parameters (none) we are passing into the guest wasm function call.
  wasmer_value_t params[] = {0};


  // Call the wasm function
  wasmer_result_t call_result = wasmer_instance_call(
      instance, // Our Wasm Instance
      "throw_wasm_error", // the name of the exported function we want to call on the guest wasm module
      params, // Our array of parameters
      0, // The number of parameters
      results, // Our array of results
      1 // The number of results
      );

  // Assert that the guest wasm function call Error'd
  // Checking both WASMER_OK and WASMER_ERROR just for
  // demonstrating the two statuses exposed by the header file.
  assert(call_result != WASMER_OK);
  assert(call_result == WASMER_ERROR);

  // Print out what happened (So it is not confusing to see an error).
  printf("The Guest Wasm Function \"throw_wasm_error\" threw and error like we expected!\n");
  printf("Printing the error ...\n\n");

  // Print out the error
  print_wasmer_error();

  // Confirming everything ran as expected!
  printf("\nTest ran sucessfully, ending execution ...\n");

  // Destroy the instances we created for our wasmer
  wasmer_instance_destroy(instance);
  return 0;
}
```

The expected output of this program would be:

```text
The Guest Wasm Function "throw_wasm_error" threw and error like we expected!
Printing the error ...

Error len: `83`
Error str: `Call error: WebAssembly trap occurred during runtime: illegal arithmetic operation`

Test ran sucessfully, ending execution ...
```

Meaning we were able to cause an error in our guest wasm module, and then get a hint on what caused the error!

Next, let's take a look at interrupting the execution of a wasm module.


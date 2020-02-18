# Interrupting Execution

{% hint style="success" %}
**Note**: The final code for this example can be found on [GitHub](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/runtime/c-integration/examples/exit-early)

> Please take a look at the [setup steps for the C/C++ integration](../setup.md).
{% endhint %}

WebAssembly in its current state is currently run synchronously. Thus, once WebAssembly starts executing, you have to wait for the execution to complete to continue running code on the host \(your C application\).

However, there are cases where you may want to interrupt this synchronous execution while the guest WebAssembly module is calling a host function. This can be useful for saving resources, and not returning back to the guest WebAssembly for execution, when you already know the Wasm execution will fail, or no longer be needed.

In this example, we will run a Wasm module that calls the imported host function, "interrupt\_execution". This host function will immediately stop executing the WebAssembly module:

```c
#include <stdio.h>
#include "dist/wasmer-build/src/wasmer-runtime-c-api/lib/runtime-c-api/wasmer.h"
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

// The function to interrupt the execution of our guest wasm module.
// Will be passed as in import function in the importObject
// For our Guest Wasm Module.
// NOTE: The first parameter of an import function for the Wasmer C API
// must always be a pointer to the context, followed by other function parameters.
void interrupt_execution(wasmer_instance_context_t *ctx) {
  printf("Interrupting execution of the guest wasm module, from the imported host function ...\n");

  // Divide by zero here to throw an error
  // Since Execptions are not in C, only in C++
  int value_one = 24;
  int value_two = 0;
  int error = value_one / value_two;
}

// Function and Variable that should not be called
// that will be passed as in import functio in the importObject
int was_called = 0;
void should_not_be_called(wasmer_instance_context_t *ctx) {
  was_called = 1;
}

// Function to create a function import to pass to our wasmer instance
wasmer_import_func_t *create_wasmer_import_function(
    void (*function_pointer)(void *), // A Pointer to the host functiono
    wasmer_value_tag params_signature[],  // Function signature for the function params
    int num_params,  // Number of params
    wasmer_value_tag returns_signature[], // Function signature for the function returns 
    int num_returns // Number of Returns
    ) {

  // Create a new func to hold the parameter and signature
  // of our `print_str` host function
  wasmer_import_func_t *func = wasmer_import_func_new(
      function_pointer, 
      params_signature, 
      num_params, 
      returns_signature, 
      num_returns
      );

  return func;
}

// Function to create a Wasmer Instance
wasmer_instance_t *create_wasmer_instance(
    wasmer_import_func_t *interrupt_execution_import_function,  
    char *interrupt_execution_import_function_name,
    wasmer_import_func_t *should_not_be_called_import_function,  
    char *should_not_be_called_import_function_name
    ) {

  // Create module name for our imports

  // Create a UTF-8 string as bytes for our module name. 
  // And, place the string into the wasmer_byte_array type so it can be used by our guest wasm instance.
  const char *module_name = "env";
  wasmer_byte_array module_name_bytes = { .bytes = (const uint8_t *) module_name,
    .bytes_len = strlen(module_name) };

  // Define our interrupt_execution import
  // See the "Hello World" example for more context
  // On the Key/Value Pairs of the declarations below:
  wasmer_byte_array interrupt_execution_import_function_name_bytes = { .bytes = (const uint8_t *) interrupt_execution_import_function_name,
    .bytes_len = strlen(interrupt_execution_import_function_name) };
  wasmer_import_t interrupt_execution_import = { .module_name = module_name_bytes,
    .import_name = interrupt_execution_import_function_name_bytes,
    .tag = WASM_FUNCTION,
    .value.func = interrupt_execution_import_function };

  // Define our should_not_be_called import
  // See the "Hello World" example for more context
  // On the Key/Value Pairs of the declarations below:
  wasmer_byte_array should_not_be_called_import_function_name_bytes = { .bytes = (const uint8_t *) should_not_be_called_import_function_name,
    .bytes_len = strlen(should_not_be_called_import_function_name) };
  wasmer_import_t should_not_be_called_import = { .module_name = module_name_bytes,
    .import_name = should_not_be_called_import_function_name_bytes,
    .tag = WASM_FUNCTION,
    .value.func = should_not_be_called_import_function };

  // Define an array containing our imports
  wasmer_import_t imports[] = {interrupt_execution_import, should_not_be_called_import};

  // Read the wasm file bytes
  FILE *file = fopen("example-wasienv-wasm/exit-early-import/exit-early-import.wasm", "r");
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
      2 // The number of imports in the imports array
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

  // Create our function imports

  // Create the interrupt_execution function import
  wasmer_value_tag interrupt_execution_params_sig[] = {};
  wasmer_value_tag interrupt_execution_returns_sig[] = {};
  wasmer_import_func_t *interrupt_execution_import_function = create_wasmer_import_function(
      (void (*)(void *)) interrupt_execution, // Function Pointer
      interrupt_execution_params_sig, // Params Signature
      0, // Number of Params
      interrupt_execution_returns_sig, // Returns Signature
      0 // Number of Returns
      );

  // Create the should_not_be_called function import
  wasmer_value_tag should_not_be_called_params_sig[] = {};
  wasmer_value_tag should_not_be_called_returns_sig[] = {};
  wasmer_import_func_t *should_not_be_called_import_function = create_wasmer_import_function(
      (void (*)(void *)) should_not_be_called, // Function Pointer
      should_not_be_called_params_sig, // Params Signature
      0, // Number of Params
      should_not_be_called_returns_sig, // Returns Signature
      0 // Number of Returns
      );


  // Initialize our Wasmer Memory and Instance
  wasmer_instance_t *instance = create_wasmer_instance(
      interrupt_execution_import_function,
      "interrupt_execution",
      should_not_be_called_import_function,
      "should_not_be_called"
      );

  // Define our results. Results are created with { 0 } to avoid null issues,
  // And will be filled with the proper result after calling the guest wasm function.
  wasmer_value_t result_one = { 0 };
  wasmer_value_t results[] = {result_one};

  // Define our parameters (none) we are passing into the guest wasm function call.
  wasmer_value_t params[] = {0};

  // Call the wasm function
  wasmer_result_t call_result = wasmer_instance_call(
      instance, // Our Wasm Instance
      "exit_early", // the name of the exported function we want to call on the guest wasm module
      params, // Our array of parameters
      0, // The number of parameters
      results, // Our array of results
      1 // The number of results
      );

  // Assert the call error'd (Because we error'd from the host import function)
  assert(call_result == WASMER_ERROR);
  assert(was_called == 0);

  printf("should_not_be_called was never called becasue of the interruption of execution!\nSuccess!\n");

  // Destroy the instances we created for our wasmer
  wasmer_import_func_destroy(interrupt_execution_import_function);
  wasmer_import_func_destroy(should_not_be_called_import_function);
  wasmer_instance_destroy(instance);

  return 0;
}
```


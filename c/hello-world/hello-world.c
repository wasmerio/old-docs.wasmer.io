#include <stdio.h>
#include "dist/wasmer-build/src/wasmer-runtime-c-api/lib/runtime-c-api/wasmer.h"
#include <assert.h>
#include <string.h>

// Use the last_error API to retrieve error messages
void print_wasmer_error()
{
    int error_len = wasmer_last_error_length();
    printf("Error len: `%d`\n", error_len);
    char *error_str = malloc(error_len);
    wasmer_last_error_message(error_str, error_len);
    printf("Error str: `%s`\n", error_str);
}

int main() {
  printf("Running the Wasmer C API example...\n");

  // Create module name for our imports
  // represented in bytes for UTF-8 compatability
  const char *module_name = "env";
  wasmer_byte_array module_name_bytes = { .bytes = (const uint8_t *) module_name,
                                          .bytes_len = strlen(module_name) };

  // Define a memory import
  const char *import_memory_name = "memory";
  wasmer_byte_array import_memory_name_bytes = { .bytes = (const uint8_t *) import_memory_name,
                                                 .bytes_len = strlen(import_memory_name) };
  wasmer_import_t memory_import = { .module_name = module_name_bytes,
                                    .import_name = import_memory_name_bytes,
                                    .tag = WASM_MEMORY };
  wasmer_memory_t *memory = NULL;
  wasmer_limit_option_t max = { .has_some = true,
                                .some = 256 };
  wasmer_limits_t descriptor = { .min = 256,
                                 .max = max };

  wasmer_result_t memory_result = wasmer_memory_new(&memory, descriptor);
  if (memory_result != WASMER_OK)
  {
      print_wasmer_error();
  }
  memory_import.value.memory = memory;

  // Define a global import
  const char *import_global_name = "__memory_base";
  wasmer_byte_array import_global_name_bytes = { .bytes = (const uint8_t *) import_global_name,
                                                 .bytes_len = strlen(import_global_name) };
  wasmer_import_t global_import = { .module_name = module_name_bytes,
                                    .import_name = import_global_name_bytes,
                                    .tag = WASM_GLOBAL };

  wasmer_value_t val = { .tag = WASM_I32,
                         .value.I32 = 1024 };
  wasmer_global_t *global = wasmer_global_new(val, false);
  global_import.value.global = global;

  // Define an array containing our imports
  wasmer_import_t imports[] = {global_import, memory_import};

  // Read the wasm file bytes
  FILE *file = fopen("example-wasienv-wasm/add-one/add-one.wasm", "r");
  fseek(file, 0, SEEK_END);
  long len = ftell(file);
  uint8_t *bytes = malloc(len);
  fseek(file, 0, SEEK_SET);
  fread(bytes, 1, len, file);
  fclose(file);

  // Creates a WebAssembly Instance from wasm bytes and imports
  wasmer_instance_t *instance = NULL;
  wasmer_result_t compile_result = wasmer_instantiate(&instance, bytes, len, imports, 2);
  printf("Compile result:  %d\n", compile_result);

  if (compile_result != WASMER_OK)
  {
      print_wasmer_error();
  }

  // Assert the wasm instantion completed
  assert(compile_result == WASMER_OK);

   // Call the exported "hello_wasm" function of our instance
  wasmer_value_t params[] = { { .tag = WASM_I32, .value.I32 = 24 } };
  wasmer_value_t result_one = { 0 };
  wasmer_value_t results[] = {result_one};
  wasmer_result_t call_result = wasmer_instance_call(instance, "addOne", params, 1, results, 1);

  // Get our response, we know the function is an i32, thus we assign the value to an int
  int response_tag = results[0].tag;
  int response_value = results[0].value.I32;

  // Print out the call result
  printf("Call result:  %d\n", call_result);
  printf("Result tag:  %d\n", results[0].tag);
  printf("Result value:  %d\n", results[0].value.I32);

  // Assert the call succeded
  assert(call_result == WASMER_OK);
  // Asset the value is correct to our assumptions
  assert(response_value == 25);

  // Use *_destroy methods to cleanup as specified in the header documentation
  wasmer_global_destroy(global);
  wasmer_memory_destroy(memory);
  wasmer_instance_destroy(instance);
  return 0;
}

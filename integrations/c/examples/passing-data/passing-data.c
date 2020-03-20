#include <stdio.h>
#include "wasmer.h"
#include <assert.h>
#include <string.h>

// Function to print the most recent error string from Wasmer if we have them
void print_wasmer_error()
{
  int error_len = wasmer_last_error_length();
  char *error_str = malloc(error_len);
  wasmer_last_error_message(error_str, error_len);
  printf("Error: `%s`\n", error_str);
}

// Function to create a wasmer memory instance, so we can import
// memory into a wasmer instance.
wasmer_memory_t *create_wasmer_memory() {
  // Create our initial size of the memory 
  wasmer_memory_t *memory = NULL;
  // Create our maximum memory size.
  // .has_some represents wether or not the memory has a maximum size
  // .some is the value of the maxiumum size
  wasmer_limit_option_t max = { .has_some = true,
    .some = 256 };
  // Create our memory descriptor, to set our minimum and maximum memory size
  // .min is the minimum size of the memory
  // .max is the maximuum size of the memory
  wasmer_limits_t descriptor = { .min = 256,
    .max = max };

  // Create our memory instance, using our memory and descriptor,
  wasmer_result_t memory_result = wasmer_memory_new(&memory, descriptor);
  // Ensure the memory was instantiated successfully.
  if (memory_result != WASMER_OK)
  {
    print_wasmer_error();
  }

  // Return the Wasmer Memory Instance
  return memory;
}

// Function to create a Wasmer Instance
wasmer_instance_t *create_wasmer_instance(wasmer_memory_t *memory) {

  // Create module name for our imports

  // Create a UTF-8 string as bytes for our module name. 
  // And, place the string into the wasmer_byte_array type so it can be used by our guest Wasm instance.
  const char *module_name = "env";
  wasmer_byte_array module_name_bytes = { .bytes = (const uint8_t *) module_name,
    .bytes_len = strlen(module_name) };

  // Define a memory import

  // Create a UTF-8 string as bytes for our module name. 
  // And, place the string into the wasmer_byte_array type so it can be used by our guest Wasm instance.
  const char *import_memory_name = "memory";
  wasmer_byte_array import_memory_name_bytes = { .bytes = (const uint8_t *) import_memory_name,
    .bytes_len = strlen(import_memory_name) };

  // Create our memory import object, from our passed memory,
  // that will be used as shared Wasm memory between the host (this application),
  // and the guest Wasm module.
  // The .module_name is the key of the importObject that this memory is associated with.
  // The .import_name is the key of the module that is within the importObject
  // The .tag is the type of import being added to the import object
  wasmer_import_t memory_import = { .module_name = module_name_bytes,
    .import_name = import_memory_name_bytes,
    .tag = WASM_MEMORY };

  // Set the memory to our import object
  memory_import.value.memory = memory;

  // Define an array containing our imports
  wasmer_import_t imports[] = {memory_import};

  // Read the Wasm file bytes
  FILE *file = fopen("../../../shared/c/passing-data.wasm", "r");
  assert(file != NULL);
  fseek(file, 0, SEEK_END);
  long len = ftell(file);
  uint8_t *bytes = malloc(len);
  fseek(file, 0, SEEK_SET);
  fread(bytes, 1, len, file);
  fclose(file);

  // Instantiate a WebAssembly Instance from Wasm bytes and imports
  wasmer_instance_t *instance = NULL;
  wasmer_result_t compile_result = wasmer_instantiate(
      &instance, // Our reference to our Wasm instance 
      bytes, // The bytes of the WebAssembly modules
      len, // The length of the bytes of the WebAssembly module
      imports, // The Imports array the will be used as our importObject
      1 // The number of imports in the imports array
      );

  // Ensure the compilation was successful.
  if (compile_result != WASMER_OK)
  {
    print_wasmer_error();
  }

  // Assert the Wasm instantion completed
  assert(compile_result == WASMER_OK);

  // Return the Wasmer Instance
  return instance;
}

// Function to get a pointer to the guest Wasm linear memory.
uint8_t *get_pointer_to_memory(wasmer_instance_t *instance) {
  // Get the Wasmer Context from the instance.
  // NOTE: To get the memory from the Wasmer Instance, it MUST be
  // from the instance context, and NOT the imported memory.
  const wasmer_instance_context_t *ctx = wasmer_instance_context_get(instance);
  const wasmer_memory_t *memory = wasmer_instance_context_memory(ctx, 0);

  // Return the uint8_t representation of the guest Wasm linear memory.
  return wasmer_memory_data(memory);
}

// Function to get the length of the guest Wasm linear memory.
uint32_t get_length_of_memory(wasmer_instance_t *instance) {
  // Get the Wasmer Context from the instance.
  // NOTE: To get the memory from the Wasmer Instance, it MUST be
  // from the instance context, and NOT the imported memory.
  const wasmer_instance_context_t *ctx = wasmer_instance_context_get(instance);
  wasmer_memory_t *memory = wasmer_instance_context_memory(ctx, 0);

  // Return the length (as in number of uint8 bytes) of the guest Wasm linear memory
  return wasmer_memory_data_length(memory);
}

// Function to call a function on the guest Wasm module, and return an i32 result
int call_wasm_function_and_return_i32(wasmer_instance_t *instance, char* functionName, wasmer_value_t params[], int num_params) {
  // Define our results. Results are created with { 0 } to avoid null issues,
  // And will be filled with the proper result after calling the guest Wasm function.
  wasmer_value_t result_one = { .tag  = WASM_I32,
                                .value = { .I32 = 0 } };
  wasmer_value_t results[] = {result_one};


  // Call the Wasm function
  wasmer_result_t call_result = wasmer_instance_call(
      instance, // Our Wasm Instance
      functionName, // the name of the exported function we want to call on the guest Wasm module
      params, // Our array of parameters
      num_params, // The number of parameters
      &results[0], // Our array of results
      1 // The number of results
      );

  if (call_result != WASMER_OK)
  {
    print_wasmer_error();
  }

  // Get our response, we know the function is an i32, thus we assign the value to an int
  assert(results[0].tag == WASM_I32);
  int response_value = results[0].value.I32; 

  // Return the i32 (int) result.
  return response_value;
}

int main() {

  // Initialize our Wasmer Memory and Instance
  wasmer_memory_t *memory = create_wasmer_memory();
  wasmer_instance_t *instance = create_wasmer_instance(memory);

  // Get the Wasm Memory and it's length from the wasmer instance
  uint8_t *memory_data = get_pointer_to_memory(instance);
  uint32_t memory_length = get_length_of_memory(instance);

  // Let's get the pointer to the buffer exposed by our Guest Wasm Module
  wasmer_value_t get_buffer_pointer_params[] = { 0 };
  int buffer_pointer = call_wasm_function_and_return_i32(instance, "get_wasm_memory_buffer_pointer", get_buffer_pointer_params, 0);
  printf("Found pointer at 0x%x\n", buffer_pointer);

  // Define and print our original string
  char original_string[] = "Did you know";
  int og_str_len = sizeof(original_string);
  printf("original_string: \"%s\"\n", original_string);

  // Get the length of the original string, and write it to the guest wasm's exposed buffer.
  memcpy(&memory_data[buffer_pointer], original_string, sizeof(original_string));

  // Call the exported "add_wasm_is_cool" function of our instance
  wasmer_value_t add_wasm_is_cool_params[] = { { .tag = WASM_I32,
                                                 .value = { .I32 = og_str_len - 1 }, } };
  int new_string_length = call_wasm_function_and_return_i32(instance, "add_wasm_is_cool", add_wasm_is_cool_params, 1);
  printf("Return value: %d\n", new_string_length);
  assert(new_string_length > og_str_len);

  buffer_pointer = call_wasm_function_and_return_i32(instance, "get_wasm_memory_buffer_pointer", get_buffer_pointer_params, 0);

  memory_data = get_pointer_to_memory(instance);
  memory_length = get_length_of_memory(instance);

  // Get the new string from the guest wasm's exposed buffer
  char new_string[101];
  assert(new_string_length < sizeof(new_string));

  memcpy(new_string, &memory_data[buffer_pointer], new_string_length);
  new_string[new_string_length] = 0;

  // Print and assert the new string
  printf("new_string: \"%s\"\n", new_string);
  assert(strcmp(new_string, "Did you know Wasm is cool!") == 0);

  // Destroy the instances we created for our wasmer
  wasmer_memory_destroy(memory);
  wasmer_instance_destroy(instance);

  return 0;
}

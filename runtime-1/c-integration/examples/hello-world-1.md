# Hello World

[Full Example Project Source Code](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/runtime/c-integration/examples/hello-world)

Please take a look at the installation steps for the C/C++ integration.

In this example we will be building a "Hello World"-like project. WebAssembly only supports passing integers and floats in the current MVP, thus we will be writing a host application that calls the "addOne" function of a guest wasm module, which adds 1 to the value passed as a parameter, and returns the result.

First we are going to want to initialize a new project. To do this we can navigate to our project folder, or create one. In this example, we will create a new project named `hello-world`. Thus, lets create the directory for it, and navigate to it:

```bash
mkdir hello-world
cd hello-world
```

Then, let's create our CMake file. This will handle how the Wasmer Runtime is installed and linked to our Host C application.

```text
# Minimum cmake version required
cmake_minimum_required (VERSION 2.6)
# Name of our project
project (WasmerCHelloWorld)

# Output executable, and Entrypoint into our project
add_executable(hello-world hello-world.c)

# Added the Wasmer Runtime C API As a dependency
# Will be downloaded and compiled with rust
include(ExternalProject)
# set_directory_properties(PROPERTIES EP_PREFIX ${CMAKE_BINARY_DIR}/wasmer-build)
ExternalProject_Add(
        wasmer-runtime-c-api
        DOWNLOAD_DIR ${CMAKE_CURRENT_SOURCE_DIR}/cmake-build
        GIT_REPOSITORY https://github.com/wasmerio/wasmer.git
        GIT_TAG origin/master
        CONFIGURE_COMMAND ""
        BUILD_COMMAND cargo build -p wasmer-runtime-c-api
        COMMAND cargo build -p wasmer-runtime-c-api
        BINARY_DIR "${CMAKE_SOURCE_DIR}/dist/wasmer-build/src/wasmer-runtime-c-api/"
        INSTALL_COMMAND ""
        LOG_BUILD ON)

# Add the dependency to our hello-world executable we are ourputing
add_dependencies(hello-world wasmer-runtime-c-api)

# Handled some windows specific building
if(WIN32)
    set(WASMER_LIB "${CMAKE_SOURCE_DIR}/dist/wasmer-build/src/wasmer-runtime-c-api/target/debug/wasmer_runtime_c_api.dll")
else()
    set(WASMER_LIB "${CMAKE_SOURCE_DIR}/dist/wasmer-build/src/wasmer-runtime-c-api/target/debug/libwasmer_runtime_c_api${CMAKE_SHARED_LIBRARY_SUFFIX}")
endif()

# Error if Wasmer is not found
if(NOT WASMER_LIB)
    message(FATAL_ERROR "wasmer library not found")
endif()

# Link our hello world executable to the Wasmer runtime.
target_link_libraries(hello-world general ${WASMER_LIB})
```

Please see the comments in the CMake file for a detailed explanation of how everything is described and run.

Now that we have our CMake file, let's create the entrypoint file that the CMake config refers to, `hello-world.c`.

```c
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

  // Print the beginning of the example message.
  printf("Running the Wasmer C API example...\n");

  // Create module name for our imports
  // Create a UTF-8 string as bytes for our module name.
  // And, place the string into the wasmer_byte_array type so it can be used by our guest wasm instance.
  const char *module_name = "env";
  wasmer_byte_array module_name_bytes = { .bytes = (const uint8_t *) module_name,
                                          .bytes_len = strlen(module_name) };

  // Define an array containing our imports
  wasmer_import_t imports[] = {};

  // Read the wasm file bytes from "example-wasienv-wasm/add-one/add-one.wasm"
  // NOTE: You should replace this file path for where your guest wasm module is.
  FILE *file = fopen("example-wasienv-wasm/add-one/add-one.wasm", "r");
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

  // Print our the result of our compilation,
  // Ensure the compilation was successful.
  printf("Compile result:  %d\n", compile_result);
  if (compile_result != WASMER_OK)
  {
      print_wasmer_error();
  }

  // Assert the wasm instantion completed
  assert(compile_result == WASMER_OK);

   // Call the exported "hello_wasm" function of our instance

  // Define our parameters we are passing into the guest wasm function call.
  // Params are created with the following properties
  // .tag is the tag of the type of the param being passed to the guest wasm function
  // .value.I32 is the value being passed to the guest wasm function
  wasmer_value_t param_one = { .tag = WASM_I32, .value.I32 = 24 };

  // Create our array of our params
  wasmer_value_t params[] = { param_one };

  // Define our results. Results are created with { 0 } to avoid null issues,
  // And will be filled with the proper result after calling the guest wasm function.
  wasmer_value_t result_one = { 0 };
  wasmer_value_t results[] = {result_one};

  // Call the wasm function
  wasmer_result_t call_result = wasmer_instance_call(
      instance, // Our Wasm Instance
      "addOne", // the name of the exported function we want to call on the guest wasm module
      params, // Our array of parameters
      1, // The number of parameters
      results, // Our array of results
      1 // The number of results
  );

  // Get our response, we know the function is an i32, thus we assign the value to an int
  int response_tag = results[0].tag;
  int response_value = results[0].value.I32;

  // Print out our results esult
  printf("Call result:  %d\n", call_result);
  printf("Result tag:  %d\n", results[0].tag);
  printf("Result value:  %d\n", results[0].value.I32);

  // Assert the call succeded
  assert(call_result == WASMER_OK);
  // Asset the value is correct to our assumptions
  assert(response_value == 25);

  // Use *_destroy methods to cleanup as specified in the header documentation
  wasmer_instance_destroy(instance);
  return 0;
}
```

Please take a look at the comments of the `hello-world.c` file to see how everything is working. In particular, notice that the guest wasm module is loading a file from "example-wasienv-wasm/add-one/add-one.wasm". This file path should be replaced by wherever your add-one.wasm is located. [To get the add-one.wasm, please take a look at the example guest module in our host application example source code](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/runtime/c-integration/examples/hello-world/example-wasienv-wasm/add-one).

Now, we can build our host application! Building and application tends to generate a lot of files, so let's create an output directory and navigate to it:

```bash
mkdir -p dist
cd dist
```

Then, let's use `cmake` on the root of our project to generate the necessary build files in our current directory:

`cmake ..`

Then, with the build output generated by cmake, we can build our executable! Let's run `make` with the `-j` flag to build the executable, while using multiple threads for a faster build time:

`make -j`

If all of this works, you should now have a `hello-world` executable. If you run:

`./hello-world`

Your output should look something like:

```bash
Running the Wasmer C API example...
Compile result:  1
Call result:  1
Result tag:  0
Result value:  25
```

Which means we have our first application!

Next, let's take a look at how we can pass data between our host application, and our guest wasm module.


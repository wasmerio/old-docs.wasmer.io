# Hello World

{% hint style="success" %}
**Note**: The final code for this example can be found on [GitHub](https://github.com/wasmerio/docs.wasmer.io/tree/master/integrations/c/examples/hello-world).
{% endhint %}

In this example we will be building a "Hello World"-like project. WebAssembly only supports passing integers and floats directly right now, thus to keep it simple we will be writing a host application that calls the `add_one` function of a guest Wasm module, which adds `1` to the value passed as a parameter, and returns the result.

First we are going to want to initialize a new project. To do this we can navigate to our project folder, or create one. In this example, we will create a new project named `hello-world`. Thus, lets create the directory for it, and navigate to it:

```bash
mkdir hello-world
cd hello-world
```

Let's create the entrypoint file `hello-world.c`.

```c
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

int main() {
  // Define an array containing our imports
  // Our guest module does not use any import functions, or memory, so we omit them.
  wasmer_import_t imports[] = {};

  // Read the Wasm file bytes from "../../../shared/c/add.wasm"
  // NOTE: You should replace this file path for where your guest Wasm module is.
  FILE *file = fopen("../../../shared/c/add.wasm", "r");
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
      0 // The number of imports in the imports array
  );

  // Print our the result of our compilation,
  // Ensure the compilation was successful.
  printf("Compile result:  %d\n", compile_result);
  if (compile_result != WASMER_OK)
  {
      print_wasmer_error();
  }

  // Assert the Wasm instantion completed
  assert(compile_result == WASMER_OK);

   // Call the exported "hello_wasm" function of our instance

  // Define our parameters we are passing into the guest Wasm function call.
  // Params are created with the following properties
  // .tag is the tag of the type of the param being passed to the guest Wasm function
  // .value.I32 is the value being passed to the guest Wasm function
  wasmer_value_t param_one = { .tag = WASM_I32, .value.I32 = 24 };

  // Create our array of our params
  wasmer_value_t params[] = { param_one };

  // Define our results. Results are created with { 0 } to avoid null issues,
  // And will be filled with the proper result after calling the guest Wasm function.
  wasmer_value_t result_one = { 0 };
  wasmer_value_t results[] = {result_one};

  // Call the Wasm function
  wasmer_result_t call_result = wasmer_instance_call(
      instance, // Our Wasm Instance
      "add_one", // the name of the exported function we want to call on the guest Wasm module
      params, // Our array of parameters
      1, // The number of parameters
      results, // Our array of results
      1 // The number of results
  );

  // Get our response, we know the function is an i32, thus we assign the value to an int
  int response_value = results[0].value.I32;

  // Print out our results esult
  printf("Call result:  %d\n", call_result);
  printf("Result value:  %d\n", response_value);

  // Assert the call succeded
  assert(call_result == WASMER_OK);
  // Asset the value is correct to our assumptions
  assert(response_value == 25);

  wasmer_instance_destroy(instance);
  return 0;
}
```

Please take a look at the comments of the `hello-world.c` file to see how everything is working.

{% hint style="info" %}
You can download the `add.wasm` WebAssembly module here:  
[integrations/shared/c/add.wasm](https://github.com/wasmerio/docs.wasmer.io/raw/master/integrations/shared/c/add.wasm)

Note: You can [find the implementation of it here](https://github.com/wasmerio/docs.wasmer.io/blob/master/integrations/shared/c/add.c)
{% endhint %}

Now, we should be ready to compile it with our favorite C compiler \(`gcc` or `clang`\):

```bash
gcc hello-world.c -I${WASMER_C_API}/include -L${WASMER_C_API}/lib -lwasmer -o hello-world
# Add -rpath ${WASMER_C_API}/lib if you are on macOS
```

{% hint style="warning" %}
Make sure the `WASMER_C_API` environment var is properly set before runnning the command.  
You can follow the [Setup C/C++ environment instructions](../setup.md).
{% endhint %}

If all of this works, you should now have a `hello-world` executable. If you run:

`./hello-world`

Your output should look something like:

```bash
Compile result:  1
Call result:  1
Result value:  25
```

Which means we have our first application!

{% hint style="info" %}
If you want to run the examples from the docs codebase directly, you can also do:

```bash
git clone https://github.com/wasmerio/docs.wasmer.io.git
cd docs.wasmer.io/integrations/c/hello-world
```
{% endhint %}

Next, let's take a look at how we can pass data between our host application, and our guest Wasm module.


# Passing Data Between Rust and Wasm

{% hint style="success" %}
**Note**: The final code for this example can be found on [GitHub](https://github.com/wasmerio/docs.wasmer.io/blob/master/docs/runtime/rust-integration/examples/passing_data.rs)

> Please take a look at the [setup steps for the Rust integration](../installation.md).
{% endhint %}
Linear memory is one of the major concepts in WebAssembly:

{% hint style="info" %}
Because WebAssembly is sandboxed, memory must be copied between the host \(your Rust application\) and the Wasm module. Upcoming proposals like WebAssembly Interface types will make this process much easier, but it is still a work in progress.
{% endhint %}

The way that this memory is allocated, freed, passed, organized, etc... can vary depending on the ABI of the Wasm module.

For example, some ABIs will provide functions, either as imports or exports, for allocation and freeing of memory from the host or guest. Some Wasm ABIs may want to control their memory implictly, for example the ABI may say that the host can reserve memory addresses 0 to 1000 for a special purpose and simply write there directly. You will want to take a look at the documentation of the ABI of your Wasm module, to see what conventions are used for memory allocation.

In this example, let's say we have a Wasm module that can perform transformations on a string passed into the module's memory. This module exports a function that returns a pointer to a fixed size static buffer. This Wasm module will take in a string, and append the string " Wasm is cool!" to the end. This example shows how we can read and write memory from the host \(your Rust application\), and how the Wasm module can also read and write to the same memory.

So if we generate a new project, we can modify our `src/main.rs` to be the following:

```rust
// Import the wasmer runtime so we can use it
use wasmer_runtime::{error, imports, instantiate, Array, Func, WasmPtr};

// Our entry point to our application
fn main() -> error::Result<()> {
    // Let's get the .wasm file as bytes
    let wasm_bytes = include_bytes!("passing-data.wasm");

    // Now that we have the wasm file as bytes, let's run it with the wasmer runtime
    // Our import object, that allows exposing functions to our wasm module.
    // We're not importing anything, so make an empty import object.
    let import_object = imports! {};
    // Let's create an instance of wasm module running in the wasmer-runtime
    let instance = instantiate(wasm_bytes, &import_object)?;

    // Lets get the context and memory of our Wasm Instance
    let wasm_instance_context = instance.context();
    let wasm_instance_memory = wasm_instance_context.memory(0);

    // Let's get the pointer to the buffer defined by the wasm module in the wasm memory.
    // We use the type system and the power of generics to get a function we can call
    // directly with a type signature of no arguments and returning a WasmPtr<u8, Array>
    let get_wasm_memory_buffer_pointer: Func<(), WasmPtr<u8, Array>> = instance
        .func("get_wasm_memory_buffer_pointer")
        .expect("get_wasm_memory_buffer_pointer");
    let wasm_buffer_pointer = get_wasm_memory_buffer_pointer.call().unwrap();
    // Let's write a string to the wasm memory
    let original_string = "Did you know";
    println!("The original string is: {}", original_string);

    // We deref our WasmPtr to get a &[Cell<u8>]
    let memory_writer = wasm_buffer_pointer
        .deref(wasm_instance_memory, 0, original_string.len() as u32)
        .unwrap();
    for (i, b) in original_string.bytes().enumerate() {
        memory_writer[i].set(b);
    }

    // Let's call the exported function that concatenates a phrase to our string.
    let add_wasm_is_cool: Func<u32, u32> = instance
        .func("add_wasm_is_cool")
        .expect("Wasm is cool export");
    let new_string_length = add_wasm_is_cool.call(original_string.len() as u32).unwrap();

    // Get our pointer again, since memory may have shifted around
    let new_wasm_buffer_pointer = get_wasm_memory_buffer_pointer.call().unwrap();
    // Read the string from that new pointer.
    let new_string = new_wasm_buffer_pointer
        .get_utf8_string(wasm_instance_memory, new_string_length)
        .unwrap();

    println!("The new string is: {}", new_string);
    // Asserting that the returned value from the function is our expected value.
    assert_eq!(new_string, "Did you know Wasm is cool!");

    // Log a success message
    println!("Success!");
    // Return OK since everything executed successfully!
    Ok(())
}
```

{% hint style="info" %}
You can download the `passing-data.wasm` WebAssembly module here:  
[https://github.com/wasmerio/docs.wasmer.io/raw/master/integrations/shared/passing-data.wasm](https://github.com/wasmerio/docs.wasmer.io/raw/master/integrations/shared/passing-data.wasm)
{% endhint %}


Taking a look at the `src/main.rs` above, we see that we:

1. Load our wasm module from a file
2. Instantiate this wasm module
3. Create a `WasmPtr` from the pointer returned from the exported function
4. Write the bytes of our string to the `WasmPtr` location
5. Call the exported transformation function
6. Get a new `WasmPtr`, in case any memory has moved around in Wasm module
7. Retrieve the transformed string from the Wasm module

Now, we should be ready to run it!

```text
cargo run
```

{% hint style="info" %}
If you want to run the examples from the docs codebase directly, you can also do:

```bash
git clone https://github.com/wasmerio/docs.wasmer.io.git
cd docs.wasmer.io/integrations/rust/passing-data
```
{% endhint %}

Now that we have a general idea of how we can pass data back and forth between the Host and a Wasm module using its linear memory, let's take a look at how we can expose Host functions to the Wasm module.

---
id: runtime-rust-integration-examples-passing-data
title: Runtime Rust Integration: Passing Data Between Rust and Wasm
sidebar_label: Passing Data Between Rust and Wasm
---

[Full Example Project Source Code](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/runtime/rust-integration/examples/passing-data)

Linear memory is one of the major concepts in WebAssembly. 

Because WebAssembly is sandboxed and WebAssembly is still young, memory must be copied between the host (your rust application) the Wasm module. Upcoming proposals like the WebAssembly Interface types will make this process much easier, but it is still a work in progress.

The way that this memory is allocated, freed, passed, organized, etc... can vary depending on the API exposed by the Wasm module. 

For example, some ABIs will provide explicit function for allocation and freeing of memory from the host. And some Wasm modules may want to control their memory themself, and the host may only need to modify that memory in place. You will want to take a look at the documentation of your wasm module, to see how it wants to interact with its memory from a Host.

In this example, let's say we have a wasm module than can perform transformations on a string passed into the module's memory. This module exports a function that returns a pointer to a fixed size static buffer, which allows one transformation at a time. This Wasm module will take in a string, and concatenate the string " Wasm is cool!". This example shows how we can read and write memory from the host (your rust application), and the Wasm module can also read and write to the same memory.

So if we generate a new project, we can modify our `src/main.rs` to be the following:

```rust
// Import the Filesystem so we can read our .wasm file
use std::io::prelude::*;
use std::fs::File;
    
// Import the wasmer runtime so we can use it
use wasmer_runtime::{
    instantiate,
    imports,
    error,
    Func
};

// Import some helpers for handling Pointers into Wasm Memory
use wasmer_runtime_core::memory::ptr::{WasmPtr, Array};

// Our entry point to our application
fn main() -> error::Result<()> {

    // Let's read in our .wasm file as bytes

    // Let's open the file.
    // The file path may be different depending where you run `cargo run`, and where you place the file.
    let mut file = File::open("./example-rust-wasm-crate/strings-wasm-is-cool/pkg/strings_wasm_is_cool_bg.wasm").expect("Incorrect file path to wasm module.");

    // Let's read the file into a Vec
    let mut wasm_vec = Vec::new();
    file.read_to_end(&mut wasm_vec).expect("Error reading the wasm file");

    // Let's get our byte slice ( [u8] ) from our wasm_vec.
    let wasm_bytes = wasm_vec.as_slice();

    // Now that we have the wasm file as bytes, let's run it with the wasmer runtime

    // Our import object, that allows exposing functions to our wasm module.
    // We're not importing anything, so make an empty import object.
    let import_object = imports!{};

    // Let's create an instance of wasm module running in the wasmer-runtime
    let instance = instantiate(wasm_bytes, &import_object)?;

    // Lets get the context and memory of our Wasm Instance
    let wasm_instance_context = instance.context();
    let wasm_instance_memory = wasm_instance_context.memory(0);

    // Let's get the pointer to the buffer defined by the wasm module in the wasm memory
    let get_wasm_memory_buffer_pointer: Func<(), i32> = 
        instance
        .func("get_wasm_memory_buffer_pointer")
        .expect("get_wasm_memory_buffer_pointer");
    let response = get_wasm_memory_buffer_pointer.call().unwrap() as u32;
    let wasm_buffer_pointer: WasmPtr<u8, Array> = WasmPtr::new(response);

    // Let's write a string to the wasm memory
    let original_string = "Did you know";
    println!("The original string is: {}", original_string);
    let memory_writer = wasm_buffer_pointer.deref(wasm_instance_memory, 0, original_string.len() as u32).unwrap();
    for (i, b) in original_string.bytes().enumerate() {
        memory_writer[i].set(b);
    }

    // Let's call the exported function that concatenates a phrase to our string.
    let add_wasm_is_cool: Func<u32, i32> = instance.func("add_wasm_is_cool").expect("Wasm is cool export");
    let new_string_length = add_wasm_is_cool.call(original_string.len() as u32).unwrap();

     // Get our pointer again, since memory may have shifted around
    let new_pointer_response = get_wasm_memory_buffer_pointer.call().unwrap() as u32;
    let new_wasm_buffer_pointer: WasmPtr<u8, Array> = WasmPtr::new(new_pointer_response);

    // Read the string from that new pointer.
    let new_string = new_wasm_buffer_pointer.get_utf8_string(wasm_instance_memory, new_string_length as u32).unwrap();
    println!("The new string is: {}", new_string);
    
    // Asserting that the returned value from the function is our expected value.
    assert_eq!(new_string, "Did you know Wasm is cool!");

    // Log a success message
    println!("Success!");

    // Return OK since everything executed successfully!
    Ok(())
}
```

Taking a look at the `src/main.rs` above, we see that we:

1. Load our wasm module from a file
2. Instantiate this wasm module
3. Create a `WasmPtr` from the pointer returned from the exported function
4. Write the bytes of our string to the `WasmPtr` location
5. Call the exported transformation function
6. Get a new `WasmPtr`, in case any memory has moved around in Wasm module
7. Retrieve the transformed string from the Wasm module

Now that we have a general idea of how we can pass data back and forth between wasm module using it's linear memory, let's take a look on how we can expose our host functions to the wasm module.

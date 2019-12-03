---
id: runtime-rust-integration-examples-host-functions
title: Runtime Rust Integration: Exposing Host Functions to WebAssembly
sidebar_label: Exposing Host Functions to WebAssembly
---

[Full Example Project Source Code](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/runtime/rust-integration/examples/host-functions)

Importing function into a WebAssembly object is another great feature about WebAssembly. Using the `importObject` we can expose functions in the host (our rust application) for the WebAssembly module to call, and interact with host from within the WebAssembly modules.

In this example, let's assume we have a WebAssemblly module, that expects some "counter" functions from the host. The idea of the functions being: 

1. There will be a `get_counter` function that will return an `i32` of the current global counter.
2. There will be an `add_to_counter` function will add the passed `i32` value to a global counter, and will return an `i32` of the current global counter.

Let's generate a new project, and our `src/main.rs` would look something like this:

```rust
// Import the Filesystem so we can read our .wasm file
use std::io::prelude::*;
use std::fs::File;

// Import the wasmer runtime so we can use it
use wasmer_runtime::{
    instantiate,
    Value,
    imports,
    error,
    // Include the function macro
    func,
    // Include the Context for our Wasm Instance for passing imported host functions
    Ctx
};

static mut COUNTER: i32 = 0;

// Our entry point to our application
fn main() -> error::Result<()> {

    // Let's read in our .wasm file as bytes

    // Let's open the file. 
    // The file path may be different depending where you run `cargo run`, and where you place the file.
    let mut file = File::open("./example-rust-wasm-crate/host-counter/pkg/host_counter_bg.wasm").expect("Incorrect file path to wasm module.");

    // Let's read the file into a Vec
    let mut wasm_vec = Vec::new();
    file.read_to_end(&mut wasm_vec).expect("Error reading the wasm file");

    // Let's get our byte slice ( [u8] ) from ouw wasm_vec.
    let wasm_bytes = wasm_vec.as_slice();

    // Now that we have the wasm file as bytes, let's run it with the wasmer runtime

    // Let's define the import object used to import our function
    // into our webassembly sample application.
    //
    // Make sure to check your function signature (parameter and return types) carefully!
    let import_object = imports! {
        // Define the "env" namespace that was implicitly used
        // by our example rust wasm crate.
        "env" => {
            // Key should be the name of the imported function
            // Value should be the func! macro, with the function passed in.
            "get_counter" => func!(get_counter),
            "add_to_counter" => func!(add_to_counter),
        },
    };

    // Let's create an instance of wasm module running in the wasmer-runtime
    let instance = instantiate(wasm_bytes, &import_object)?;

    // Define the number of times we want to loop our increment
    let number_of_times_to_loop: i32 = 5;

    // Let's call the exported "increment_counter_loop" function ont the wasm module.
    let values = instance
        .dyn_func("increment_counter_loop")?
        .call(&[Value::I32(number_of_times_to_loop)])?;

    unsafe {
        // Assert our counter is the expected value
        assert_eq!(number_of_times_to_loop, COUNTER);

        // Asserting that the returned value from the function is our expected value.
        assert_eq!(values[0], Value::I32(COUNTER));

        // Log the new value
        println!("New Counter Value: {}", COUNTER);
    }

    // Log a success message.
    println!("Success!");

    // Return OK since everything executed successfully!
    Ok(())
}

// Define a Host function that will be imported to the wasm module
// Note, the first parameter must be the Wasm Instance Wasmer Context
// Following parameter types, and return types would be as usual.
//
// This function returns our global counter.
fn get_counter(_ctx: &mut Ctx) -> i32 {
    unsafe {
        COUNTER
    }
}

// Define a Host function that will be imported to the wasm module
// Note, the first parameter must be the Wasm Instance Wasmer Context
// Following parameter types, and return types would be as usual.
//
// This function adds the value to our global counter, and then returns the counter
fn add_to_counter(_ctx: &mut Ctx, value_to_add: i32) -> i32 {
    unsafe {
        COUNTER += value_to_add;
        COUNTER
    }
}
```

The main idea here, is that we want to assign our "get_counter" function to the "get_counter" key in our importObject. And since we are using the default "env" namespace, these functions should be nested under the "env" object in our `importObject`. 

Depending on the wasm module, the function may need to be nested differently. You will want to take a look at the module's documentation, or the module's source language documentation to see how the import object should be nested to expose the function to the module.

Next, we will take a look at handling errors from a WebAssembly module!

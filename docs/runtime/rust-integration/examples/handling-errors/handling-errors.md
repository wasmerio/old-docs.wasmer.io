---
id: runtime-rust-integration-examples-handling-errors
title: Runtime Rust Integration: Handling Errors
sidebar_label: Handling Errors
---

There will come a time where running a WebAssembly module will not work, and trying to figure out why it does not work can be a difficult task! In the current MVP of WebAssembly, debugging is quite vauge, in runtimes for both the browser and the server. But errors can still be handled and debugged gracefully.

In this example, we will load a WebAssembly module that purposely `panic!()`'s on its exported function call. The host (our rust application) will pattern match for the error and output the error message returned from Wasmer:

```rust
// Import the Filesystem so we can read our .wasm file
use std::io::prelude::*;
use std::fs::File;

// Import the wasmer runtime so we can use it
use wasmer_runtime::{
    instantiate,
    Func,
    imports,
    error,
};

// Our entry point to our application
fn main() -> error::Result<()> {

// Let's read in our .wasm file as bytes

// Let's open the file. 
// The file path may be different depending where you run `cargo run`, and where you place the file.
let mut file = File::open("./example-rust-wasm-crate/throw-wasm-error/pkg/throw_wasm_error_bg.wasm").expect("Incorrect file path to wasm module.");

// Let's read the file into a Vec
let mut wasm_vec = Vec::new();
file.read_to_end(&mut wasm_vec).expect("Error reading the wasm file");

// Let's get our byte slice ( [u8] ) from ouw wasm_vec.
let wasm_bytes = wasm_vec.as_slice();

// Now that we have the wasm file as bytes, let's run it with the wasmer runtime

// Our import object, that allows exposing functions to our wasm module.
// We're not importing anything, so make an empty import object.
let import_object = imports!{};

// Let's create an instance of wasm module running in the wasmer-runtime
let instance = instantiate(wasm_bytes, &import_object)?;

// Let's call the exported "throw_error" function ont the wasm module.
let throw_error_func: Func<(), ()> = instance
    .func("throw_wasm_error")
    .expect("throw_wasm_error function was not found");

let response = throw_error_func.call();

    match response {
       Ok(_) => {
            // The wasm modules should have thrown an error.
            panic!("throw_wasm_error did not error");
       },
       Err(e) => {
           // Log the error
           println!("Error from throw_wasm_error: {}", e);
       },
    }

    // Log a success message.
    println!("Success!");

    // Return OK since everything executed successfully!
    Ok(())
}
```

If we run the following code with `cargo run`, we would see a result like:

[Cargo Run Terminal Output. Error from throw_wasm_error: WebAssemblytrap occured during runtime: unkown](/img/docs/rust-handling-errors-1.png)

However, let's make this a little more interesting. Let's unwrap the error, as you would during the development process and not in production. This will cause the program to error at the WebAssembly module function call, and will give a different output to our console:

```rust
// Import the Filesystem so we can read our .wasm file
use std::io::prelude::*;
use std::fs::File;

// Import the wasmer runtime so we can use it
use wasmer_runtime::{
    instantiate,
    Func,
    imports,
    error,
};

// Our entry point to our application
fn main() -> error::Result<()> {

    // Let's read in our .wasm file as bytes

    // Let's open the file. 
    // The file path may be different depending where you run `cargo run`, and where you place the file.
    let mut file = File::open("./example-rust-wasm-crate/throw-wasm-error/pkg/throw_wasm_error_bg.wasm").expect("Incorrect file path to wasm module.");

    // Let's read the file into a Vec
    let mut wasm_vec = Vec::new();
    file.read_to_end(&mut wasm_vec).expect("Error reading the wasm file");

    // Let's get our byte slice ( [u8] ) from ouw wasm_vec.
    let wasm_bytes = wasm_vec.as_slice();

    // Now that we have the wasm file as bytes, let's run it with the wasmer runtime

    // Our import object, that allows exposing functions to our wasm module.
    // We're not importing anything, so make an empty import object.
    let import_object = imports!{};

    // Let's create an instance of wasm module running in the wasmer-runtime
    let instance = instantiate(wasm_bytes, &import_object)?;

    // Let's call the exported "throw_error" function ont the wasm module.
    let throw_error_func: Func<(), ()> = instance
        .func("throw_wasm_error")
        .expect("throw_wasm_error function was not found");

    // Unwrapping here, so that the error is thrown here
    let _response = throw_error_func.call().unwrap();

    /*
    
    Commenting the pattern matching, to show the unwrapped error above. 

    match response {
       Ok(_) => {
            // This should have thrown an error, return an error
            panic!("throw_wasm_error did not error");
       },
       Err(e) => {
           // Log the error
           println!("Error from throw_wasm_error: {}", e);
       },
    }

    */

    // Log a success message.
    println!("Success!");

    // Return OK since everything executed successfully!
    Ok(())
}
```

If we run the code with `cargo run`, we would see a result like:

[Cargo Run Terminal Output. Main thread panics on unwrap without a line number, and suggests rust backtrace](/img/docs/rust-handling-errors-2.png)


As you can tell, this error doesn't give us much insight into why this had an error. Such as the line number in the host application. This can be extremely fustrating, especially if you are making multiple calls to a wasm module in a complex rust application. What we can do to get some more insight is use what the error itself suggests, which is the `RUST_BACKTRACE=1` environment variable.  **It is HIGHLY RECCOMENDED that you use the `RUST_BACKTRACE=1` environment variable for debugging you rust application that embeds the Wasmer runtime.**

So let's run the code with **`RUST_BACKTRACE=1 cargo run`** instead, we would see a result like:

[Cargo Run Terminal Output. Shows rust back trace, which you can see where things started to break, as explained below](/img/docs/rust-handling-errors-3.png)

If we look our for our file name (`src/main.rs`), we will see at step 10, there was an error on line 44. Which is the line number for where we call and unwrap the `throw_wasm_error` function. This is great, as now we can start to investigate the particular function call, and why it may be returning and error.

Next, let's take a look at how we can interrupt an executing Wasm module.

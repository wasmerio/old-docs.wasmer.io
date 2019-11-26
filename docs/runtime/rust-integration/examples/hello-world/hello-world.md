---
id: runtime-rust-integration-examples-hello-world
title: Runtime Rust Integration: Hello World
sidebar_label: Hello World
---

Please take a look at the installation steps for the Rust integration.

In this example we will be building a "Hello World"-like project. WebAssembly only supports passing integers and floats in the current MVP, thus we will be writing a host application that calls the "add_one" function of a guest wasm module, which adds 1 to the value passed as a parameter, and returns the result.

First we are going to want to initialize a new project. To do this we can navigate to our project folder, or create one. In this example, we will create a new project named `hello-world`. Thus, lets create the directory for it, and navigate to it:

```bash
mkdir hello-world
cd hello-world
```

Then, we will initialize the project as a new rust project, by running:

`cargo init`

This should generate two files, `Cargo.toml` and `src/main.rs`. The `Cargo.toml` is a file that describes your project and its dependencies. The `src/main.rs` is the entry point for your project, and contains the `fn main() { .. }` that is run when the project is executed.

Let's modify our `Cargo.toml` to add the [`wasmer-runtime` crate](https://crates.io/crates/wasmer-runtime/0.1.4) to our project. At the time of this writing, the crate is at version `0.9.0`. So we change the `Cargo.toml` to the following:

```toml
[package]
name = "hello-world"
version = "0.1.0"
authors = ["Aaron Turner <aaron@aaronthedev.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
# Add the wasmer-runtime as a dependency
wasmer-runtime = "0.11.0"
```

Now that we have the Wasmer runtime added as a dependency, let's go ahead and try it out! For our hello world, what we will do is use the Wasmer runtime to execute an exported function on a WebAssembly module, that adds one the the integer passed to the function. To do this, we will modify our `src/main.rs` to the following:

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
};

    // Our entry point to our application
    fn main() -> error::Result<()> {
    
    // Let's read in our .wasm file as bytes

    // Let's open the file. 
    // The file path may be different depending where you run `cargo run`, and where you place the file.
    let mut file = File::open("./example-rust-wasm-crate/add-one/pkg/add_one_bg.wasm").expect("Incorrect file path to wasm module.");

    // Let's read the file into a Vec
    let mut wasm_vec = Vec::new();
    file.read_to_end(&mut wasm_vec).expect("Error reading the wasm file");

    // Let's get our byte slice ( &[u8] ) from our wasm_vec.
    let wasm_bytes = wasm_vec.as_slice();

    // Now that we have the wasm file as bytes, let's run it with the wasmer runtime

    // Our import object, that allows exposing functions to our wasm module.
    // We're not importing anything, so make an empty import object.
    let import_object = imports!{};

    // Let's create an instance of wasm module running in the wasmer-runtime
    let instance = instantiate(wasm_bytes, &import_object)?;

    // Let's get a number we want to add one to
    let value_to_add = 42;
    println!("Original Value: {}", value_to_add);

    // Let's call the exported "add_one" function ont the wasm module.
    let values = instance
        .dyn_func("add_one")?
        .call(&[Value::I32(value_to_add)])?;

    // Asserting that the returned value from the function is our expected value.
    assert_eq!(values[0], Value::I32(43));

    // Log the new value
    println!("New Value: {}", 43);

    // Log a success message.
    println!("Success!");
    
    // Return OK since everything executed successfully!
    Ok(())
}
```

This should execute the "add_one" function, which we pass the i32 value of 42 to it, and returns the integer value of 43! Hooray! We got our first rust application running WebAssembly working!

Next, let's take a look at passing data between the host (our rust application), and the web assembly module.

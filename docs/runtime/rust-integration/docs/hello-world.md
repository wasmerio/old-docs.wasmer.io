---
id: runtime-rust-integration-examples-hello-world
title: Rust Integration: Hello World
sidebar_label: Hello World
---

In this example we will be building a "Hello World"-like project.

> ***Important***
>
> At the moment, you can only pass single integers or floats across the interface between WebAssembly and a host language.  Therefore, to keep this program as simple as possible, our Rust host application will call the `add_one` function of a guest WASM module, which unsurprisingly, adds 1 to the supplied integer and returns the result.

## Development Steps

1. ***Create a New Rust Project***  
    Change into some development directory and then use the Rust Package Manager `cargo` to create and initialise a new project. Use the following command to create a new project called `hello-world`:

    ```bash
    $ cd <some_development_directory>
    $ cargo new hello-world
    $ cd hello-world
    ```

    The `cargo` command creates the `hello-world` directory and into it, places two important files: `Cargo.toml` and `src/main.rs`.
    
    * `Cargo.toml`  
        This file contains a description of your project and its dependencies.  It is divided into named sections identified by a name in square brackets such as `[package]` or `[dependencies]`.
    
    * `src/main.rs`  
        This file contains a function called `main()` that acts as the default entry point for any Rust application.

1. ***Prepare the Guest WASM Module***  
    Follow the steps for [preparing WASM modules](./runtime-rust-integration-prepare-wasm-modules) in order to create the required `hello-world-guest` WASM module that will be called below.

1. ***Add a Dependency for the Wasmer Runtime***  
    In order for a Rust application to execute a WASM module, we need to make use of the functionality found in the [`wasmer-runtime` crate](https://crates.io/crates/wasmer-runtime/0.13.1). At the time of writing (Feb 2020), this crate is at version `0.13.1`.

    Insert the following line into the `[dependencies]` section of the `Cargo.toml` file:

    `wasmer-runtime = "0.13.1"`

    So the file now looks like this:

    ```toml
    [package]
    name = "hello-world"
    version = "0.1.0"
    authors = ["The Wasmer Engineering Team <engineering@wasmer.io>"]
    edition = "2018"
    
    [dependencies]
    # Add the wasmer-runtime as a dependency
    wasmer-runtime = "0.13.1"
    ```

1. ***Write the Rust Code to Invoke the WASM Module***  
    Now that the Rust compiler has been informed of our dependency on the Wasmer runtime functionality, we can write some Rust code that does the following:

    * Read the WASM module file into a Rust `Vector` and then instantiate it
    * Create an instance of the `add_one` function exposed by the WASM module
    * Invoke the `add_one` WASM function and pass it an integer such as `42`
    * Asserting that the returned value is the answer we expect
    
    To do this, we need to modify our `src/main.rs` to the following ([hello_world.rs](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/runtime/rust-integration/examples/hello_world.rs)):

    ```rust
    // Import the Filesystem so we can read our .wasm file
    use std::fs::File;
    use std::io::prelude::*;
    
    // Import the wasmer runtime so we can use it
    use wasmer_runtime::{error, imports, instantiate, Func};
    
    const WASM_FILE_PATH: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/target/wasm32-unknown-unknown/release/hello_world_guest.wasm"
    );
    
    // Our entry point to our application
    fn main() -> error::Result<()> {
        // Let's read in our .wasm file as bytes
    
        // Let's open the file.
        let mut file = File::open(WASM_FILE_PATH).expect(&format!("WASM file at {}", WASM_FILE_PATH));
    
        // Let's read the file into a Vec
        let mut wasm_vec = Vec::new();
        file.read_to_end(&mut wasm_vec)
            .expect("Error reading the WASM file");
    
        // Now that we have the WASM file as bytes, let's run it with the wasmer runtime
    
        // Our import object, that allows exposing functions to our WASM module.
        // We're not importing anything, so make an empty import object.
        let import_object = imports! {};
    
        // Let's create an instance of WASM module running in the wasmer-runtime
        let instance = instantiate(&wasm_vec, &import_object)?;
    
        // Let's get a number we want to add one to
        let value_to_add = 42;
        println!("Original Value: {}", value_to_add);
    
        // Let's get `add_one` as a function which takes one `u32` and returns one `u32`
        let add_one: Func<u32, u32> = instance.func("add_one")?;
        let result = add_one.call(value_to_add)?;
    
        // Log the new value
        println!("New Value: {}", 43);
    
        // Asserting that the returned value from the function is our expected value.
        assert_eq!(result, 43);
    
        // Log a success message.
        println!("Success!");
    
        // Return OK since everything executed successfully!
        Ok(())
    }
    ```

1. ***Execute the Rust Host Application***  
    The Rust host application can be compiled and executed using the command `cargo run`.
    
    The first time you run this command for a project, the Rust compiler must download and compile all your project's library dependencies.  This will take a minute or two and you will see lots of additional library compilation messages.

    ```bash
    $ cargo run
       Compiling semver-parser v0.7.0
       Compiling cfg-if v0.1.10
    # Snip lots of library compilation messages...
       Compiling wasmer-runtime v0.13.1
       Compiling hello-world v0.1.0
        Finished dev [unoptimized + debuginfo] target(s) in 41.74s
         Running `target/debug/hello-world`
    Original Value: 42
    New Value: 43
    Success!
    ```

    As you can see, we has successfully passed the `i32` value `42` to the `add_one` function in our guest WASM module.  This added one to the value and gave us back another `i32` containing the integer value `43`
    
Fantabulous! We've succeessfully built our first Rust application that calls a WebAssembly module!

Next, let's take a look at passing more data than just a single integer between the Rust host application and the WebAssembly module.

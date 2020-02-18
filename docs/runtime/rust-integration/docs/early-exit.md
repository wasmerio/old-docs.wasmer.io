---
id: runtime-rust-integration-examples-exit-early
title: Rust Integration: Interrupting Execution
sidebar_label: Interrupting Execution
---

Currently, WebAssembly programs are always run synchronously. Thus, once a WebAssembly programs starts executing, the code in the host application  must wait until it either completes normally or crashes. 

However, there are cases where the host application may want to interrupt the synchronous execution of a guest WebAssembly module. This can be useful
for saving resources; for example, in situations where you already know that WASM execution will fail, or is no longer be needed.

In this example, we will do the following:

1. Create a Rust application that supplies a host function to WASM called `interrupt_execution`
1. Invoke a WASM module that calls this inported host function
1. The host function then terminates the WebAssembly module

## Development Steps

1. ***Create a New Rust Project***  
    Create a new Rust project called `early-exit` and change into the newly created directory.

    ```bash
    $ cargo new early-exit
    $ cd early-exit
    ```

1. ***Prepare the Guest WASM Module***  
    Follow the steps for [preparing WASM modules](./runtime-rust-integration-prepare-wasm-modules) in order to create the required `early-exit-guest` WASM module that will be called below.

1. ***Add a Dependency for the Wasmer Runtime***  
    Insert the following line into the `[dependencies]` section of the `Cargo.toml` file:

    `wasmer-runtime = "0.13.1"`

1. ***Write the Rust Code to Invoke the WASM Module***  
    Now that the Rust compiler has been informed of our dependency on the Wasmer runtime functionality, we can write some Rust code that calls the WASM module.

    To do this, we need to modify our `src/main.rs` to the following ([early_exit.rs](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/runtime/rust-integration/examples/early_exit.rs))

    ```rust
    // Import the Filesystem so we can read our .wasm file
    use std::fs::File;
    use std::io::prelude::*;
    
    // Import the wasmer runtime so we can use it
    use wasmer_runtime::{
        error,
        // Include the function macro
        func,
        imports,
        instantiate,
        // Include the Context for our WASM Instance for passing imported host functions
        Ctx,
        Func,
    };
    
    const WASM_FILE_PATH: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/target/wasm32-unknown-unknown/release/early_exit_guest.wasm"
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
    
        // Let's define the import object used to import our function
        // into our webassembly sample application.
        //
        // Make sure to check your function signature (parameter and return types) carefully!
        let import_object = imports! {
            // Define the "env" namespace that was implicitly used
            // by our example rust WASM crate.
            "env" => {
                // Key should be the name of the imported function
                // Value should be the func! macro, with the function passed in.
                "interrupt_execution" => func!(interrupt_execution),
            },
        };
    
        // Let's create an instance of WASM module running in the wasmer-runtime
        let instance = instantiate(&wasm_vec, &import_object)?;
    
        // Let's call the exported "exit_early" function on the WASM module.
        let exit_early_func: Func<(), i32> = instance
            .func("exit_early")
            .expect("exit_early function not found");
        let response = exit_early_func.call();
    
        match response {
            Ok(value) => {
                // This should have thrown an error, return an error
                panic!("exit_early did not error. Returned the value: {}", value);
            }
            Err(e) => {
                // Log the error
                println!("Error from exit_early: {}", e);
            }
        }
    
        // Log a success message.
        println!("Success!");
    
        // Return OK since everything executed successfully!
        Ok(())
    }
    
    // Function that is imported into the guest WASM module, that will immediately stop execution
    fn interrupt_execution(_ctx: &mut Ctx) -> Result<(), ()> {
        // Log that we were called
        println!("interrupt_execution called!");
    
        // Return an error, which will immediately stop execution of the WASM module
        Err(())
    }
    ```
1. ***Execute the Rust Host Application***  
    The Rust host application can be compiled and executed using the command `cargo run`.
    
    ```bash
    $ cargo run
       Compiling semver-parser v0.7.0
       Compiling cfg-if v0.1.10
    # Snip lots of library compilation messages...
       Compiling wasmer-runtime v0.13.1
       Compiling early-exit v0.1.0
        Finished dev [unoptimized + debuginfo] target(s) in 43.44s
         Running `target/debug/early-exit`
    interrupt_execution called!
    Error from exit_early: unknown error
    Success!
    ```

## Postscript

In addition to exiting in host calls, Wasmer also offers a metering API for allowing you to exit a host function after a pre-defined amount of time; however, the docs for the Metering API have not been yet written -- stay tuned for more!

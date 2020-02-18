---
id: runtime-rust-integration-examples-host-functions
title: Rust Integration: Exposing Host Functions to WebAssembly
sidebar_label: Exposing Host Functions to WebAssembly
---

## WASM and Host Functions

Up until now, our WebAssembly program has only been able to do pure computation; that is, the ability simply to receive arguments and return values.  However, most interesting use cases require more than just computation.

In this section we'll demonstrate how to give WASM modules access to host functionality by means of an `ImportObject`.

> ***Reminder***
>
> Remember that the term *"host function"* refers to functionality provide by the host environment within which the WASM module is running &mdash; and this could be either the runtime environment of some language such a Rust, C/C++ or Python, or it could be the actual operating system.
>
> Once a WASM module is granted permission to access host functions, the WASM module takes on the same capabilities as native code running in the host environment.

In this example, we'll create a system for getting and adjusting a counter value.

1. The `get_counter` function returns the current global counter as an `i32`.
1. The `add_to_counter` function receives an `i32` value which is adds to the value of the global counter.  
    The new global counter value is then returned as an `i32`.

## Development Steps

1. ***Create a New Rust Project***  
    Create a new Rust project called `host-functions` and change into the newly created directory.

    ```bash
    $ cargo new host-functions
    $ cd host-functions
    ```

1. ***Prepare the Guest WASM Module***  
    Follow the steps for [preparing WASM modules](./runtime-rust-integration-prepare-wasm-modules) in order to create the required `host-functions-guest` WASM module that will be called below.

1. ***Add a Dependency for the Wasmer Runtime***  
    Insert the following line into the `[dependencies]` section of the `Cargo.toml` file:

    `wasmer-runtime = "0.13.1"`

1. ***Write the Rust Code to Provide the WASM Module with "Host Functions"***  
    Now that the Rust compiler has been informed of our dependency on the Wasmer runtime functionality, we can write some Rust code that provides the WASM module with some "host functions" it can execute.

    To do this, we need to modify our `src/main.rs` to the following ([host_functions.rs](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/runtime/rust-integration/examples/host_functions.rs)):

    In this case, notice that both of the "host functions" provided by Rust (`get_counter` and `add_to_counter`) are closures, but they don't have to be.  Host functions can take an optional `&mut Ctx` value as their first argument, which is how host functions get access to WASM memory and other WASM-related data.

    ```rust
    // Import the Filesystem so we can read our .wasm file
    use std::cell::RefCell;
    use std::fs::File;
    use std::io::prelude::*;
    use std::sync::Arc;
    
    // Import the wasmer runtime so we can use it
    use wasmer_runtime::{
        error,
        // Include the function macro
        func,
        imports,
        instantiate,
        Func,
    };
    
    const WASM_FILE_PATH: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/example-rust-wasm-crate/host-counter/target/wasm32-unknown-unknown/release/host_counter.wasm"
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
    
        // We create some shared data here, [`Arc`] is required because we may
        // move our WebAssembly instance to another thread to run it.  RefCell
        // lets us get shared mutabilty which is fine because we know we won't
        // run hostcalls concurrently.  If concurrency is a possibilty, we'd have to
        // use a `Mutex`.
        let shared_counter: Arc<RefCell<i32>> = Arc::new(RefCell::new(0));
    
        // Clone the [`Arc`] for our closure and pass it into the host function
        let counter = Arc::clone(&shared_counter);
        let get_counter = move || -> i32 { *counter.borrow() };
    
        // Clone the [`Arc`] for our closure and pass it into the host function
        let counter = Arc::clone(&shared_counter);
        let add_to_counter = move |value_to_add: i32| -> i32 {
            let mut counter_ref = counter.borrow_mut();
            *counter_ref += value_to_add;
            *counter_ref
        };
    
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
                "get_counter" => func!(get_counter),
                "add_to_counter" => func!(add_to_counter),
            },
        };
    
        // Let's create an instance of WASM module running in the wasmer-runtime
        let instance = instantiate(&wasm_vec, &import_object)?;
    
        // Define the number of times we want to loop our increment
        let number_of_times_to_loop: i32 = 5;
    
        // Let's get `increment_counter_loop` as a function which takes one `i32` and returns one `i32`
        let increment_counter_loop: Func<i32, i32> = instance.func("increment_counter_loop")?;
        let result = increment_counter_loop.call(number_of_times_to_loop)?;
    
        let counter_value: i32 = *shared_counter.borrow();
    
        // Assert our counter is the expected value
        assert_eq!(number_of_times_to_loop, counter_value);
    
        // Asserting that the returned value from the function is our expected value.
        assert_eq!(result, counter_value);
    
        // Log the new value
        println!("New Counter Value: {}", counter_value);
    
        // Log a success message.
        println!("Success!");
    
        // Return OK since everything executed successfully!
        Ok(())
    }
    ```

    In the example above, the Rust program exposes some "host functions" to the guest WASM module.  These "host functions" names are declared within a namespace and supplied to WASM using the `imports!` macro.  Here, we use the default namespace of `env` and list `get_counter` and `add_to_counter` there.

    Depending on the ABI of the WASM module, we may need to expose functions under a different namespace.  On the guest side, a non-default import namespace looks like:

    ```rust
    extern "C" {
       #[link_name = "namespace"]
       fn import_name(arg: u32);
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
       Compiling host-functinos v0.1.0
        Finished dev [unoptimized + debuginfo] target(s) in 36.35s
         Running `target/debug/host-functions`
    New Counter Value: 5
    Success!
    ```


Next, we will take a look at handling errors from a WebAssembly module!

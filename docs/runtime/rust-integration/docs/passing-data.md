---
id: runtime-rust-integration-examples-passing-data
title: Rust Integration: Passing Data Between Rust and WASM
sidebar_label: Passing Data Between Rust and WASM
---

## Linear Memory: How WebAssembly Stores Runtime Data

All runtime memory used by a WASM module is allocated in 64Kb units known as "pages".  A single WASM memory page is therefore nothing more than a contiguous block of 65,536 bytes of storage that can be used in any way the WASM module requires.

If a WASM module needs more than 64Kb of storage, it can allocate additional pages as needed at runtime.

It would become very tedious to have to repeatedly use the phrase "*a contiguous, 64kb block of storage*"; so instead, we simply refer to "linear memory".

For more details, see the description of [linear memory](https://webassembly.org/docs/semantics/#linear-memory) on the <http://webassembly.org> website.

## The WebAssembly Runtime is Sand-Boxed By Design

The designers of WebAssembly made the deliberate design decision to ensure that at runtime, a WASM module's execution is completely isolated (or "sand-boxed") from the host runtime environment.  Therefore, any memory you wish to share between the host environment (in this case, your Rust application) and the WASM module must currently be copied.  Upcoming proposals like 
[WebAssembly Interface Types](https://hacks.mozilla.org/2019/08/webassembly-interface-types/) will make this process much easier, but this is still a work in progress.

## Application Binary Interface: The Bridge Between Two Worlds

In order to have a host environment interact with WebAssembly, an interface layer called an Application Binary Interface (ABI) is needed.  The ABI acts as the bridge between your host language's runtime environment and the sand-boxed world of WebAssembly.

In its current stage of development, the way that WASM memory is allocated, freed, passed, and organized, etc. varies depending on your choice of ABI.  For example, some ABIs provide functions (either as imports or exports) for the allocation and freeing of memory from the host or guest. Alternatively, some WASM ABIs may want to control their memory implictly; for example, the ABI may reserve memory addresses 0 to 1000 for special use by the host environment and simply write there directly.

You will need to take a look at the documentation of the ABI of your WASM module to see what conventions are used for memory allocation.

## WASM Module Functionality

In this example, we will demonstrate how to read and write WASM memory from the host langauge environment (your Rust application).  In this case, the WASM module exposes some memory to the host environment.  The host program then writes to that memory, WASM modifies the data and returns a pointer to the modified data.

Here, we have a WASM module that transforms a string passed into it's memory by appending `" WASM is cool!"`.  It does this by exporting two functions:  
* `get_wasm_memory_buffer_pointer` that returns a pointer to a fixed size static buffer
* `add_wasm_is_cool` that modifies the data at the location returned by `get_wasm_memory_buffer_pointer`

## Development Steps

1. ***Create a New Rust Project***  
    Create a new Rust project called `passing-data` and change into the newly created directory.

    ```bash
    $ cargo new passing-data
    $ cd passing-data
    ```

1. ***Prepare the Guest WASM Module***  
    Follow the steps for [preparing WASM modules](./runtime-rust-integration-prepare-wasm-modules) in order to create the required `passing-data-guest` WASM module that will be called below.

1. ***Add a Dependency for the Wasmer Runtime***  
    Insert the following line into the `[dependencies]` section of the `Cargo.toml` file:

    `wasmer-runtime = "0.13.1"`

1. ***Write the Rust Code to Invoke the WASM Module***  
    Now that the Rust compiler has been informed of our dependency on the Wasmer runtime functionality, we can write some Rust code that does the following:

    * Load our WASM module from a file and instantiate it
    * Create a `WasmPtr` from the pointer returned from the exported function.  This points to the location in WASM memory where we should write our string value
    * Write the bytes of our string to the `WasmPtr` location
    * Call the exported transformation function
    * Get a new `WasmPtr`, in case any memory has moved around in WASM module
    * Retrieve the transformed string from the WASM module
    
    To do this, we need to modify our `src/main.rs` to the following ([passing_data.rs](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/runtime/rust-integration/examples/passing_data.rs)):

    ```rust
    // Import the Filesystem so we can read our .wasm file
    use std::fs::File;
    use std::io::prelude::*;
    
    // Import the wasmer runtime so we can use it
    use wasmer_runtime::{error, imports, instantiate, Array, Func, WasmPtr};
    
    // Create an absolute path to the WASM file
    const WASM_FILE_PATH: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/target/wasm32-unknown-unknown/release/passing_data_guest.wasm"
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
    
        // Lets get the context and memory of our WASM Instance
        let wasm_instance_context = instance.context();
        let wasm_instance_memory = wasm_instance_context.memory(0);
    
        // Let's get the pointer to the buffer defined by the WASM module in the WASM memory.
        // We use the type system and the power of generics to get a function we can call
        // directly with a type signature of no arguments and returning a WasmPtr<u8, Array>
        let get_wasm_memory_buffer_pointer: Func<(), WasmPtr<u8, Array>> = instance
            .func("get_wasm_memory_buffer_pointer")
            .expect("get_wasm_memory_buffer_pointer");
        let wasm_buffer_pointer = get_wasm_memory_buffer_pointer.call().unwrap();
    
        // Let's write a string to the WASM memory
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
            .expect("WASM is cool export");
        let new_string_length = add_wasm_is_cool.call(original_string.len() as u32).unwrap();
    
        // Get our pointer again, since memory may have shifted around
        let new_wasm_buffer_pointer = get_wasm_memory_buffer_pointer.call().unwrap();
    
        // Read the string from that new pointer.
        let new_string = new_wasm_buffer_pointer
            .get_utf8_string(wasm_instance_memory, new_string_length)
            .unwrap();
        println!("The new string is: {}", new_string);
    
        // Asserting that the returned value from the function is our expected value.
        assert_eq!(new_string, "Did you know WASM is cool!");
    
        // Log a success message
        println!("Success!");
    
        // Return OK since everything executed successfully!
        Ok(())
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
       Compiling passing-data v0.1.0
        Finished dev [unoptimized + debuginfo] target(s) in 40.97s
         Running `target/debug/passing-data`
    The original string is: Did you know
    The new string is: Did you know WASM is cool!
    Success!
    ```

Now that we have a general idea of how we can pass data back and forth between the Host and a WASM module using its linear memory, let's take a look at how we can expose Host functions to the WASM module.

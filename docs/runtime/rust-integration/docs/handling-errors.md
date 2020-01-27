---
id: runtime-rust-integration-examples-handling-errors
title: Runtime Rust Integration: Handling Errors
sidebar_label: Handling Errors
---

[Full Example Project Source Code](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/runtime/rust-integration/examples/handling_errors.rs)

There will come a time where running a WebAssembly module will not work, 
and trying to figure out why it does not work can be a difficult task!
In the current MVP of WebAssembly, debugging isn't explicitly defined for
runtimes both in and out of the browser. So we'll have to write some error
handling code ourselves.

In this example, we will load a WebAssembly module that purposely 
`panic!()`s in its exported function call. The Host (our Rust application)
will pattern match for the error and output the error message returned from Wasmer:

```rust
// Import the Filesystem so we can read our .wasm file
use std::fs::File;
use std::io::prelude::*;

// Import the wasmer runtime so we can use it
use wasmer_runtime::{error, imports, instantiate, Func, error::{RuntimeError}};

const WASM_FILE_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/target/wasm32-unknown-unknown/release/handling_errors_guest.wasm"
);

// Our entry point to our application
fn main() -> error::Result<()> {
    // Let's read in our .wasm file as bytes

    // Let's open the file.
    let mut file = File::open(WASM_FILE_PATH).expect(&format!("wasm file at {}", WASM_FILE_PATH));

    // Let's read the file into a Vec
    let mut wasm_vec = Vec::new();
    file.read_to_end(&mut wasm_vec)
        .expect("Error reading the wasm file");

    // Now that we have the wasm file as bytes, let's run it with the wasmer runtime

    // Our import object, that allows exposing functions to our wasm module.
    // We're not importing anything, so make an empty import object.
    let import_object = imports! {};

    // Let's create an instance of wasm module running in the wasmer-runtime
    let instance = instantiate(&wasm_vec, &import_object)?;

    // Let's call the exported "throw_error" function ont the wasm module.
    let throw_error_func: Func<(), ()> = instance
        .func("throw_wasm_error")
        .expect("throw_wasm_error function was not found");

    let response = throw_error_func.call();

    match response {
       Ok(_) => {
            // This should have thrown an error, return an error
            panic!("throw_wasm_error did not error");
       },
        Err(RuntimeError::Trap { msg }) => {
           // Log the error
           println!("Trap caught from `throw_wasm_error`: {}", msg);
       },
        Err(RuntimeError::Error { .. }) => {
            panic!("Expected Trap, found Error with unknown data!");
        },
    }

    // Log a success message.
    println!("Success!");

    // Return OK since everything executed successfully!
    Ok(())
}

```

If we run the following code with `cargo run`, we would see a result like:

![Cargo Run Terminal Output. Error from throw_wasm_error: WebAssemblytrap occured during runtime: unkown](/img/docs/rust-handling-errors-1.png)

A common occurrence during development is to pass the errors up to main or
`unwrap` them. We'll go over the error output for that as well:

```rust
// Import the Filesystem so we can read our .wasm file
use std::fs::File;
use std::io::prelude::*;

// Import the wasmer runtime so we can use it
use wasmer_runtime::{error, imports, instantiate, Func, error::{RuntimeError}};

const WASM_FILE_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/target/wasm32-unknown-unknown/release/handling_errors_guest.wasm"
);

// Our entry point to our application
fn main() -> error::Result<()> {
    // Let's read in our .wasm file as bytes

    // Let's open the file.
    let mut file = File::open(WASM_FILE_PATH).expect(&format!("wasm file at {}", WASM_FILE_PATH));

    // Let's read the file into a Vec
    let mut wasm_vec = Vec::new();
    file.read_to_end(&mut wasm_vec)
        .expect("Error reading the wasm file");

    // Now that we have the wasm file as bytes, let's run it with the wasmer runtime

    // Our import object, that allows exposing functions to our wasm module.
    // We're not importing anything, so make an empty import object.
    let import_object = imports! {};

    // Let's create an instance of wasm module running in the wasmer-runtime
    let instance = instantiate(&wasm_vec, &import_object)?;

    // Let's call the exported "throw_error" function ont the wasm module.
    let throw_error_func: Func<(), ()> = instance
        .func("throw_wasm_error")
        .expect("throw_wasm_error function was not found");

    let _response = throw_error_func.call()?;

    // Log a success message.
    println!("Success!");

    // Return OK since everything executed successfully!
    Ok(())
}
```

If we run the code with `cargo run`, we would see a result like:

![Cargo Run Terminal Output. Main thread panics on unwrap without a line number, and suggests rust backtrace](/img/docs/rust-handling-errors-2.png)


As you can tell, this error doesn't give us much insight into why we had an error.
However it helpfully suggests setting the `RUST_BACKTRACE` environment variable
and running it again.  When we again run, `RUST_BACKTRACE=1 cargo run`, we see
output like:

![Cargo Run Terminal Output. Shows rust back trace, which you can see where things started to break, as explained below](/img/docs/rust-handling-errors-3.png)

If we look our for our file name (`src/main.rs`), we will see at step 10, there was
an error on line 44. Which is the line number for where we call and unwrap the
`throw_wasm_error` function. This is great, as now we can start to investigate
the particular function call, and why it may be returning and error.

It's important to keep in mind that that compiling in `release` mode reduces the
amount of debug information available by default. Debug information can be enabled
with the `[profile.release]` section in the `Cargo.toml`, simple add `debug = true`
to this section and your release builds will include debug information.

Next, let's take a look at how we can interrupt an executing Wasm module.

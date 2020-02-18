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

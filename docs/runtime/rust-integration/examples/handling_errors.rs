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

    // Let's call the exported "throw_error" function ont the WASM module.
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

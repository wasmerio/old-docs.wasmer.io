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

    // Let's get our byte slice ( [u8] ) from ouw wasm_vec.
    let wasm_bytes = wasm_vec.as_slice();

    // Now that we have the wasm file as bytes, let's run it with the wasmer runtime

    // Our import object, that allows exposing functions to our wasm module.
    // We're not importing anything, so make an empty import object.
    let import_object = imports! {};

    // Let's create an instance of wasm module running in the wasmer-runtime
    let instance = instantiate(wasm_bytes, &import_object)?;

    // Let's call the exported "add_one" function ont the wasm module.
    let values = instance
        .dyn_func("add_one")?
        .call(&[Value::I32(42)])?;

    // Asserting that the returned value from the function is our expected value.
    assert_eq!(values[0], Value::I32(43));

    // Log a success message.
    println!("add_one executed successfully!");
    
    // Return OK since everything executed successfully!
    Ok(())
}

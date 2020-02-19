// Import the wasmer runtime so we can use it
use wasmer_runtime::{error, imports, instantiate, Func, error::{RuntimeError}};

// Our entry point to our application
fn main() -> error::Result<()> {
    // Let's read in our .wasm file as bytes
    let wasm_bytes = include_bytes!("../../../../shared/handling-errors.wasm");

    // Our import object, that allows exposing functions to our wasm module.
    // We're not importing anything, so make an empty import object.
    let import_object = imports! {};

    // Let's create an instance of wasm module running in the wasmer-runtime
    let instance = instantiate(wasm_bytes, &import_object)?;

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

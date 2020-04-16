// Import the wasmer runtime so we can use it
use wasmer_runtime::{error, imports, instantiate, Func};

fn main() -> error::Result<()> {
    // Let's get the .wasm file as bytes
    let wasm_bytes = include_bytes!("../../../../shared/rust/add.wasm");

    // Our import object, that allows exposing functions to our Wasm module.
    // We're not importing anything, so make an empty import object.
    let import_object = imports! {};

    // Let's create an instance of Wasm module running in the wasmer-runtime
    let instance = instantiate(wasm_bytes, &import_object)?;

    // Let's get `add_one` as a function which takes one `u32` and returns one `u32`
    let add_one: Func<u32, u32> = instance.func("add_one")?;
    let result = add_one.call(42)?;

    // Log the new value
    println!("Result: {}", result);

    // Asserting that the returned value from the function is our expected value.
    assert_eq!(result, 43);  // 42 + 1

    // Return from main
    Ok(())
}
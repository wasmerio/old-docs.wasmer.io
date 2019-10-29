// Import the Filesystem so we can read our .wasm file
use std::io::prelude::*;
use std::fs::File;

// Import the wasmer runtime so we can use it
use wasmer_runtime::{
    instantiate,
    Value,
    imports,
    error,
    // Incllude the function macro, and context for wasm
    func,
    Ctx
};

static mut COUNTER: i32 = 0;

// Our entry point to our application
fn main() -> error::Result<()> {

    // Let's read in our .wasm file as bytes

    // Let's open the file. 
    // The file path may be different depending where you run `cargo run`, and where you place the file.
    let mut file = File::open("./example-rust-wasm-crate/host-counter/pkg/host_counter_bg.wasm").expect("Incorrect file path to wasm module.");

    // Let's read the file into a Vec
    let mut wasm_vec = Vec::new();
    file.read_to_end(&mut wasm_vec).expect("Error reading the wasm file");

    // Let's get our byte slice ( [u8] ) from ouw wasm_vec.
    let wasm_bytes = wasm_vec.as_slice();

    // Now that we have the wasm file as bytes, let's run it with the wasmer runtime

    // Let's define the import object used to import our function
    // into our webassembly sample application.
    //
    // Make sure to check your function signature (parameter and return types) carefully!
    let import_object = imports! {
        // Define the "env" namespace that was implicitly used
        // by our example rust wasm crate.
        "env" => {
            // Key should be the name of the imported function
            // Value should be the func! macro, with the function passed in.
            "get_counter" => func!(get_counter),
            "add_to_counter" => func!(add_to_counter),
        },
    };

    // Let's create an instance of wasm module running in the wasmer-runtime
    let instance = instantiate(wasm_bytes, &import_object)?;

    // Define the number of times we want to loop our increment
    let number_of_times_to_loop: i32 = 5;

    // Let's call the exported "add_one" function ont the wasm module.
    let values = instance
        .dyn_func("increment_counter_loop")?
        .call(&[Value::I32(number_of_times_to_loop)])?;

    unsafe {
        // Assert our counter is the expected value
        assert_eq!(number_of_times_to_loop, COUNTER);

        // Asserting that the returned value from the function is our expected value.
        assert_eq!(values[0], Value::I32(COUNTER));

        // Log the new value
        println!("New Counter Value: {}", COUNTER);
    }

    // Log a success message.
    println!("Success!");

    // Return OK since everything executed successfully!
    Ok(())
}

fn get_counter(_ctx: &mut Ctx) -> i32 {
    unsafe {
        COUNTER
    }
}

fn add_to_counter(_ctx: &mut Ctx, value_to_add: i32) -> i32 {
    unsafe {
        COUNTER += value_to_add;
        COUNTER
    }
}

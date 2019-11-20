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
    // Include the Context for our Wasm Instance for passing imported host functions
    Ctx,
    Func,
};

const WASM_FILE_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/example-rust-wasm-crate/host-counter/pkg/host_counter_bg.wasm"
);

static mut COUNTER: i32 = 0;

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
    let instance = instantiate(&wasm_vec, &import_object)?;

    // Define the number of times we want to loop our increment
    let number_of_times_to_loop: i32 = 5;

    // Let's get `increment_counter_loop` as a function which takes one `i32` and returns one `i32`
    let increment_counter_loop: Func<i32, i32> = instance.func("increment_counter_loop")?;
    let result = increment_counter_loop.call(number_of_times_to_loop)?;

    // Let's get a copy of the value in `COUNTER`
    let counter_value = unsafe { COUNTER };

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

// Define a Host function that will be imported to the wasm module
// Note, the first parameter must be the Wasm Instance Wasmer Context
// Following parameter types, and return types would be as usual.
//
// This function returns our global counter.
fn get_counter(_ctx: &mut Ctx) -> i32 {
    unsafe { COUNTER }
}

// Define a Host function that will be imported to the wasm module
// Note, the first parameter must be the Wasm Instance Wasmer Context
// Following parameter types, and return types would be as usual.
//
// This function adds the value to our global counter, and then returns the counter
fn add_to_counter(_ctx: &mut Ctx, value_to_add: i32) -> i32 {
    unsafe {
        COUNTER += value_to_add;
        COUNTER
    }
}

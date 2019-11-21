// Import the Filesystem so we can read our .wasm file
use std::fs::File;
use std::io::prelude::*;

// Import the wasmer runtime so we can use it
use wasmer_runtime::{error, imports, instantiate, Array, Func, WasmPtr};

// Create an absolute path to the Wasm file
const WASM_FILE_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/example-rust-wasm-crate/strings-wasm-is-cool/pkg/strings_wasm_is_cool_bg.wasm"
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

    // Lets get the context and memory of our Wasm Instance
    let wasm_instance_context = instance.context();
    let wasm_instance_memory = wasm_instance_context.memory(0);

    // Let's get the pointer to the buffer defined by the wasm module in the wasm memory.
    // We use the type system and the power of generics to get a function we can call
    // directly with a type signature of no arguments and returning a WasmPtr<u8, Array>
    let get_wasm_memory_buffer_pointer: Func<(), WasmPtr<u8, Array>> = instance
        .func("get_wasm_memory_buffer_pointer")
        .expect("get_wasm_memory_buffer_pointer");
    let wasm_buffer_pointer = get_wasm_memory_buffer_pointer.call().unwrap();

    // Let's write a string to the wasm memory
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
        .expect("Wasm is cool export");
    let new_string_length = add_wasm_is_cool.call(original_string.len() as u32).unwrap();

    // Get our pointer again, since memory may have shifted around
    let new_wasm_buffer_pointer = get_wasm_memory_buffer_pointer.call().unwrap();

    // Read the string from that new pointer.
    let new_string = new_wasm_buffer_pointer
        .get_utf8_string(wasm_instance_memory, new_string_length)
        .unwrap();
    println!("The new string is: {}", new_string);

    // Asserting that the returned value from the function is our expected value.
    assert_eq!(new_string, "Did you know Wasm is cool!");

    // Log a success message
    println!("Success!");

    // Return OK since everything executed successfully!
    Ok(())
}

// Import the Filesystem so we can read our .wasm file
use std::io::prelude::*;
use std::fs::File;

// Import the wasmer runtime so we can use it
use wasmer_runtime::{
    instantiate,
    Value,
    imports,
    error,
    Func
};

// Import some helpers for handling Pointers into Wasm Memory
use wasmer_runtime_core::memory::ptr::{WasmPtr, Array};

// Our entry point to our application
fn main() -> error::Result<()> {

    // Let's read in our .wasm file as bytes

    // Let's open the file.
    // The file path may be different depending where you run `cargo run`, and where you place the file.
    let mut file = File::open("./example_rust_wasm_crate/strings-wasm-is-cool/pkg/strings_wasm_is_cool_bg.wasm").expect("Incorrect file path to wasm module.");

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

    // Lets get the contextr and memory of our Wasm Instance
    let wasm_instance_context = instance.context();
    let wasm_instance_memory = wasm_instance_context.memory(0);

    // Let's get the pointer to to our buffer in the wasm memory
    let get_wasm_memory_buffer_pointer: Func<(), i32> = 
        instance
        .func("get_wasm_memory_buffer_pointer")
        .expect("get_wasm_memory_buffer_pointer");
    let response = get_wasm_memory_buffer_pointer.call().unwrap() as u32;
    let wasm_buffer_pointer: WasmPtr<u8, Array> = WasmPtr::new(response);

    // Let's write a string to the wasm memory
    let original_string = "Did you know";
    let memory_writer = wasm_buffer_pointer.deref(wasm_instance_memory, 0, original_string.len() as u32).unwrap();
    for (i, b) in original_string.bytes().enumerate() {
        memory_writer[i].set(b);
    }

    let add_wasm_is_cool: Func<u32, i32> = instance.func("add_wasm_is_cool").expect("Wasm is cool export");
    let new_string_length = add_wasm_is_cool.call(original_string.len() as u32).unwrap();

     // Get our pointer again, since memory may have shifted around
    let new_pointer_response = get_wasm_memory_buffer_pointer.call().unwrap() as u32;
    let new_wasm_buffer_pointer: WasmPtr<u8, Array> = WasmPtr::new(new_pointer_response);

    // Read the string from that pointer.
    if let Some(my_str) = new_wasm_buffer_pointer.get_utf8_string(wasm_instance_memory, new_string_length as u32) {
        println!("yooo {}", my_str);
    }
    
    // Asserting that the returned value from the function is our expected value.
    // assert_eq!(values[0], Value::I32(43));

    // Log a success message.
    println!("Success!");

    // Return OK since everything executed successfully!
    Ok(())
}

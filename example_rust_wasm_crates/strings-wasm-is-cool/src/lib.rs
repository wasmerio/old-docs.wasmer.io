
// Import the standard string library
use std::str;

// Import wasm bindgen
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Create a static mutable byte buffer.
// We will use for passing memory between our host and wasm.
// NOTE: global `static mut` means we will have "unsafe" code
// but for passing memory between a host and wasm should be fine.
const WASM_MEMORY_BUFFER_SIZE: usize = 1024;
static mut WASM_MEMORY_BUFFER: [u8; WASM_MEMORY_BUFFER_SIZE] = [0; WASM_MEMORY_BUFFER_SIZE];

// Function to return a pointer to our buffer
// in wasm memory
#[wasm_bindgen]
pub fn get_wasm_memory_buffer_pointer() -> *const u8 {
  let pointer: *const u8;
  unsafe {
    pointer = WASM_MEMORY_BUFFER.as_ptr();
  }

  return pointer;
}

// Function to get the string from the buffer and add the text to it
#[wasm_bindgen]
pub fn add_wasm_is_cool(passed_string_length: usize) -> usize {
    // Let's copy our our bytes from the slcie for the entire UTF8 String
    let mut string_bytes = Vec::with_capacity(passed_string_length);
    unsafe {
        string_bytes.copy_from_slice(&WASM_MEMORY_BUFFER[0..passed_string_length]);
    }

    // Let's get the passed string from our passed bytes
    let passed_string = str::from_utf8(&string_bytes).unwrap();

    // Let's add our phrase to the passed string
    let new_string = format!("{} Wasm is cool!", passed_string);

    // Let's write the new string back to our buffer
    let new_string_bytes = new_string.as_bytes();
    for i in 0..new_string_bytes.len() {
        unsafe {
            WASM_MEMORY_BUFFER[i] = new_string_bytes[i];
        }
    }

    return new_string.len();
}

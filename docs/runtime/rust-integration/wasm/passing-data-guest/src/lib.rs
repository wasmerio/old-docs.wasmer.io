// Import the standard string library
use std::str;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Create a static mutable byte buffer.
// We will use for passing memory between our host and wasm.
// NOTE: global `static mut` means we have to access it with unsafe
// and manually ensure that only one mutable reference exists to it at a time
// but for passing memory between a host and wasm should be fine if we know the
// host won't share this Wasm's linear memory with another instance.
const WASM_MEMORY_BUFFER_SIZE: usize = 1024;
static mut WASM_MEMORY_BUFFER: [u8; WASM_MEMORY_BUFFER_SIZE] = [0; WASM_MEMORY_BUFFER_SIZE];

// Function to return a pointer to our buffer
// in wasm memory
#[no_mangle]
pub fn get_wasm_memory_buffer_pointer() -> *const u8 {
    unsafe { WASM_MEMORY_BUFFER.as_ptr() }
}

// Function to get the string from the buffer and add the text to it
#[no_mangle]
pub fn add_wasm_is_cool(passed_string_length: usize) -> usize {
    // Let's get the passed string from our passed bytes
    let passed_string =
        unsafe { str::from_utf8(&WASM_MEMORY_BUFFER[..passed_string_length]).unwrap() };

    // Let's add our phrase to the passed string
    let new_string = format!("{} Wasm is cool!", passed_string);

    // Let's write the new string back to our buffer
    unsafe {
        WASM_MEMORY_BUFFER[..new_string.len()].copy_from_slice(new_string.as_bytes());
    }

    // Return the length of the new string for the host to fetch it out of memory
    new_string.len()
}

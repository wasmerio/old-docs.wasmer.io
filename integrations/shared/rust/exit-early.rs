#![no_main]

// Define a function that is imported into the module.
// By default, the "env" namespace is used.
//
// We aren't using wasm-bindgen here, as we want to
// handle the imports ourselves with our host Wasmer app.
extern "C" {
    fn interrupt_execution();
}

#[no_mangle]
pub fn exit_early() -> i32 {

    // Interrupt the execution of this function
    unsafe {
        interrupt_execution();
    }

    // This will never get returned
    return 24;
}

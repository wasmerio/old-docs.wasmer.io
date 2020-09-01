use std::{error, fmt};
use wasmer::{imports, Function, Instance, Module, NativeFunc, RuntimeError, Store};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_jit::JIT;

// Our entry point to our application
fn main() {
    // We set up the Store with a JIT using the Cranelift compiler.
    let store = Store::new(&JIT::new(&Cranelift::default()).engine());

    // We then read in the Wasm bytes.
    let wasm_bytes = include_bytes!("../../../../shared/rust/exit-early.wasm");

    // We define the import object used to import our function
    // into our WebAssembly sample application.
    //
    // Make sure to check your function signature (parameter and return types) carefully!
    let import_object = imports! {
        // Define the "env" namespace that was implicitly used
        // by our example Rust Wasm crate.
        "env" => {
            // Key should be the name of the imported function
            "interrupt_execution" => Function::new_native(&store, interrupt_execution),
        },
    };

    // We create a module from the Wasm bytes.
    let module = Module::new(&store, &wasm_bytes[..]).expect("create module");

    // We then create an instance of Wasm module with our imports.
    let instance = Instance::new(&module, &import_object).expect("instantiate module");

    // We then call the exported "exit_early" function on the Wasm module.
    let exit_early_func: NativeFunc<(), i32> = instance
        .exports
        .get_native_function("exit_early")
        .expect("exit_early function not found");
    let response = exit_early_func.call();

    match response {
        Ok(value) => {
            // This should have thrown an error, return an error
            panic!("exit_early did not error. Returned the value: {}", value);
        }
        Err(e) => {
            // Log the error
            println!("Error from exit_early: {}", e);
        }
    }

    // Log a success message.
    println!("Success!");
}

// We must create an error type to use to exit early
#[derive(Debug)]
struct ErrorType {
    message: String,
}

// The type must implement `std::error::Error` which means we must also implement
// `std::fmt::Display` for our type.
impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

// Only types implementing `std::error::Error` can be used to interrupt execution.
impl error::Error for ErrorType {}

// Function that is imported into the guest Wasm module, that will immediately stop execution
fn interrupt_execution() {
    // Log that we were called
    println!("interrupt_execution called!");

    // Return an error, which will immediately stop execution of the Wasm module
    RuntimeError::raise(Box::new(ErrorType {
        message: "interrupt_execution is interrupting execution!".to_string(),
    }));
}

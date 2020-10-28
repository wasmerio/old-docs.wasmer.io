// Import the wasmer runtime so we can use it
use wasmer::{imports, Instance, Module, Store};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_jit::JIT;

fn main() {
    // We start by creating the base data structure of Wasm: the Store.
    // To create the store we need to create an Engine, we chose Wasmer's JIT
    // engine for this example which will generate native machine code at runtime
    // and then execute it.
    // In order to generate this native machine code, we must choose a compiler.
    // Wasmer offers 3 compilers to choose from; we're using the Cranelift compiler
    // in this example because of its balance between compile-time speed and runtime speed.
    let store = Store::new(&JIT::new(&Cranelift::default()).engine());

    // Now let's get the .wasm file as bytes
    let wasm_bytes = include_bytes!("../../../../shared/rust/handling-errors.wasm");

    // With the Store and the wasm bytes we can create a wasm Module which is
    // a non-runnable representation of the contents of the wasm file.
    let module = Module::new(&store, &wasm_bytes[..]).expect("create module");

    // Our import object, that allows exposing functions to our Wasm module.
    // We're not importing anything, so make an empty import object.
    let import_object = imports! {};

    // With our Module and our ImportObject we can create an Instance, which is the runnable
    // representation of the Wasm file.
    let instance = Instance::new(&module, &import_object).expect("instantiate module");

    // Let's call the exported "throw_error" function ont the Wasm module.
    let throw_error_func = instance
        .exports
        .get_function("throw_wasm_error")
        .expect("throw_wasm_error function was not found");

    let response = throw_error_func.call(&[]);

    match response {
        Ok(_) => {
            // This should have thrown an error, return an error
            panic!("throw_wasm_error did not error");
        }
        Err(e) => {
            // Log the error
            println!("Trap caught from `throw_wasm_error`: {}", e.message());
        }
    }
}

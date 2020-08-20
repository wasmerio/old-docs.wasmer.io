// Import the wasmer runtime so we can use it
use wasmer::{imports, Instance, Module, NativeFunc, Store};
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
    let wasm_bytes = include_bytes!("../../../../shared/rust/add.wasm");

    // With the Store and the wasm bytes we can create a wasm Module which is
    // a non-runnable representation of the contents of the wasm file.
    let module = Module::new(&store, &wasm_bytes[..]).expect("create module");

    // We create an empty ImportObject for the next step because we don't need to
    // import anything into `add.wasm`.
    let import_object = imports! {};

    // With our Module and our ImportObject we can create an Instance, which is the runnable
    // representation of the Wasm file.
    let instance = Instance::new(&module, &import_object).expect("instantiate module");

    // We can get functions from our Instance and execute them.
    // We get the add_one function as a NativeFunc that takes one u32 argument
    // and returns one u32 value.
    let add_one: NativeFunc<u32, u32> = instance
        .exports
        .get_native_function("add_one")
        .expect("add_one function in Wasm module");
    let result = add_one.call(42).unwrap();

    // Log the result
    println!("Result: {}", result);

    // Assert that the returned value from the function is what we expect.
    assert_eq!(result, 43); // 42 + 1
}

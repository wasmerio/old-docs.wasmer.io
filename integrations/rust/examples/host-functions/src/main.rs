use std::{cell::RefCell, sync::Arc};

// Import the wasmer runtime so we can use it
use wasmer::{imports, Cranelift, Function, Instance, JIT, Module, NativeFunc, Store};

// Our entry point to our application
fn main() {
    // We start by creating the base data structure of Wasm: the Store.
    // To create the store we need to create an Engine, we chose Wasmer's JIT
    // engine for this example which will generate native machine code at runtime
    // and then execute it.
    // In order to generate this native machine code, we must choose a compiler.
    // Wasmer offers 3 compilers to choose from; we're using the Cranelift compiler
    // in this example because of its balance between compile-time speed and runtime speed.
    let store = Store::new(&JIT::new(&Cranelift::default()).engine());

    // Let's get the .wasm file as bytes
    let wasm_bytes = include_bytes!("../../../../shared/rust/host-functions.wasm");

    // With the Store and the wasm bytes we can create a wasm Module which is
    // a non-runnable representation of the contents of the wasm file.
    let module = Module::new(&store, &wasm_bytes[..]).expect("create module");

    // We create some shared data here, [`Arc`] is required because we may
    // move our WebAssembly instance to another thread to run it.  RefCell
    // lets us get shared mutabilty which is fine because we know we won't
    // run hostcalls concurrently.  If concurrency is a possibilty, we'd have to
    // use a `Mutex`.
    let shared_counter: Arc<RefCell<i32>> = Arc::new(RefCell::new(0));

    struct Env { counter: Arc<RefCell<i32>> }
    fn get_counter(env: &mut Env) -> i32 { *env.counter.borrow() }
    fn add_to_counter(env: &mut Env, add: i32) -> i32 {
        let mut counter_ref = env.counter.borrow_mut();
        *counter_ref += add;
        *counter_ref
    }

    // Let's define the import object used to import our function
    // into our webassembly sample application.
    //
    // Make sure to check your function signature (parameter and return types) carefully!
    let import_object = imports! {
        // Define the "host" namespace that was used
        // by our example rust Wasm crate.
        "host" => {
            // Key should be the name of the imported function
            // Value should be the func! macro, with the function passed in.
            "get_counter" => Function::new_native_with_env(&store, Env { counter: shared_counter.clone() }, get_counter),
            "add_to_counter" => Function::new_native_with_env(&store, Env { counter: shared_counter.clone() }, add_to_counter),
        },
    };

    // With our Module and our ImportObject we can create an Instance, which is the runnable
    // representation of the Wasm file.
    let instance = Instance::new(&module, &import_object).expect("instantiate module");

    // Define the number of times we want to loop our increment
    let number_of_times_to_loop: i32 = 5;

    // Let's get `increment_counter_loop` as a function which takes one `i32` and returns one `i32`
    let increment_counter_loop: NativeFunc<i32, i32> = instance
        .exports
        .get_native_function("increment_counter_loop")
        .expect("increment_counter_loop in Wasm module");
    let result = increment_counter_loop.call(number_of_times_to_loop).unwrap();

    let counter_value: i32 = *shared_counter.borrow();

    // Log the new value
    println!("New Counter Value: {}", counter_value);

    // Assert our counter is the expected value
    assert_eq!(number_of_times_to_loop, counter_value);

    // Asserting that the returned value from the function is our expected value.
    assert_eq!(result, counter_value);
}

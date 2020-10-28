// Import the wasmer runtime so we can use it
use wasmer::{imports, Array, Instance, Module, NativeFunc, Store, WasmPtr};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_jit::JIT;

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
    let wasm_bytes = include_bytes!("../../../../shared/rust/passing-data.wasm");

    // With the Store and the wasm bytes we can create a wasm Module which is
    // a non-runnable representation of the contents of the wasm file.
    let module = Module::new(&store, &wasm_bytes[..]).expect("create module");

    // We create an empty ImportObject for the next step because we don't need to
    // import anything into `add.wasm`.
    let import_object = imports! {};

    // With our Module and our ImportObject we can create an Instance, which is the runnable
    // representation of the Wasm file.
    let instance = Instance::new(&module, &import_object).expect("instantiate module");

    // Lets get the context and memory of our Wasm Instance
    let wasm_instance_memory = instance.exports.get_memory("memory").expect("instance memory");

    // Let's get the pointer to the buffer defined by the Wasm module in the Wasm memory.
    // We use the type system and the power of generics to get a function we can call
    // directly with a type signature of no arguments and returning a WasmPtr<u8, Array>
    let get_wasm_memory_buffer_pointer: NativeFunc<(), WasmPtr<u8, Array>> = instance
        .exports
        .get_native_function("get_wasm_memory_buffer_pointer")
        .expect("get_wasm_memory_buffer_pointer in Wasm module");
    let wasm_buffer_pointer = get_wasm_memory_buffer_pointer.call().unwrap();
    dbg!(wasm_buffer_pointer);
    // Let's write a string to the Wasm memory
    let original_string = "Did you know";
    println!("The original string is: {}", original_string);

    // We deref our WasmPtr to get a &[Cell<u8>]
    let memory_writer = wasm_buffer_pointer
        .deref(&wasm_instance_memory, 0, original_string.len() as u32)
        .unwrap();
    for (i, b) in original_string.bytes().enumerate() {
        memory_writer[i].set(b);
    }

    // Let's call the exported function that concatenates a phrase to our string.
    let add_wasm_is_cool: NativeFunc<u32, u32> = instance
        .exports
        .get_native_function("add_wasm_is_cool")
        .expect("Wasm is cool export");
    let new_string_length = add_wasm_is_cool.call(original_string.len() as u32).unwrap();

    // Get our pointer again, since memory may have shifted around
    let new_wasm_buffer_pointer = get_wasm_memory_buffer_pointer.call().unwrap();
    // Read the string from that new pointer.
    let new_string = new_wasm_buffer_pointer
        .get_utf8_string(&wasm_instance_memory, new_string_length)
        .unwrap();

    // Log the new string
    println!("The new string is: {}", new_string);

    // Asserting that the returned value from the function is our expected value.
    assert_eq!(new_string, "Did you know Wasm is cool!");
}

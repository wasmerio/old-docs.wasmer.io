# Interrupting Execution

{% hint style="success" %}
**Note**: The final code for this example can be found on [GitHub](https://github.com/wasmerio/docs.wasmer.io/tree/master/integrations/rust/examples/exit-early).
{% endhint %}

WebAssembly is currently always run in the same process synchronously. Thus, once WebAssembly starts executing, you have to wait for the execution to complete to continue running code on the host \(your Rust application\).

However, there are cases where you may want to interrupt this synchronous execution while the guest WebAssembly module is calling a host function. This can be useful for saving resources, and not returning back to the guest WebAssembly for execution, when you already know the Wasm execution will fail, or no longer be needed.

In this example, we will run a Wasm module that calls the imported host function `interrupt_execution`. This host function will immediately stop executing the WebAssembly module:

```rust
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
```

The key piece here is that we use `RuntimeError::raise` to raise a `RuntimeError` during execution. The error that we return must be in a `Box` and must implement [`std::error::Error`](https://doc.rust-lang.org/std/error/trait.Error.html) which requires that our type also implement [`std::fmt::Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html), which we derive, and [`std::fmt::Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html), which we manually implement.

{% hint style="info" %}
You can download the `exit-early.wasm` WebAssembly module here:  
[integrations/shared/rust/exit-early.wasm](https://github.com/wasmerio/docs.wasmer.io/raw/master/integrations/shared/rust/exit-early.wasm)
{% endhint %}

In addition to exiting in host calls, Wasmer also offers a metering API for allowing a pre-defined amount of execution before interrupting. The docs for metering are not yet written -- stay tuned for more!


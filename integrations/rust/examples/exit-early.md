# Interrupting Execution

{% hint style="success" %}
**Note**: The final code for this example can be found on [GitHub](https://github.com/wasmerio/docs.wasmer.io/tree/master/integrations/rust/examples/exit-early).
{% endhint %}

WebAssembly is currently always run in the same process synchronously. Thus, once WebAssembly starts executing, you have to wait for the execution to complete to continue running code on the host \(your Rust application\).

However, there are cases where you may want to interrupt this synchronous execution while the guest WebAssembly module is calling a host function. This can be useful for saving resources, and not returning back to the guest WebAssembly for execution, when you already know the WASM execution will fail, or no longer be needed.

In this example, we will run a WASM module that calls the imported host function `interrupt_execution`. This host function will immediately stop executing the WebAssembly module:

```rust
// Import the wasmer runtime so we can use it
use wasmer_runtime::{
    error,
    func,
    imports,
    instantiate,
    // Include the Context for our WASM Instance for passing imported host functions
    Ctx,
    Func,
};

// Our entry point to our application
fn main() -> error::Result<()> {
    // Let's read in our .wasm file as bytes
    let wasm_bytes = include_bytes!("exit-early.wasm");

    // Let's define the import object used to import our function
    // into our webassembly sample application.
    //
    // Make sure to check your function signature (parameter and return types) carefully!
    let import_object = imports! {
        // Define the "env" namespace that was implicitly used
        // by our example rust WASM crate.
        "env" => {
            // Key should be the name of the imported function
            // Value should be the func! macro, with the function passed in.
            "interrupt_execution" => func!(interrupt_execution),
        },
    };

    // Let's create an instance of WASM module running in the wasmer-runtime
    let instance = instantiate(wasm_bytes, &import_object)?;

    // Let's call the exported "exit_early" function on the WASM module.
    let exit_early_func: Func<(), i32> = instance
        .func("exit_early")
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

    // Return OK since everything executed successfully!
    Ok(())
}

// Function that is imported into the guest WASM module, that will immediately stop execution
fn interrupt_execution(_ctx: &mut Ctx) -> Result<(), ()> {
    // Log that we were called
    println!("interrupt_execution called!");

    // Return an error, which will immediately stop execution of the WASM module
    Err(())
}
```

{% hint style="info" %}
You can download the `exit-early.wasm` WebAssembly module here:  
[integrations/shared/rust/exit-early.wasm](https://github.com/wasmerio/docs.wasmer.io/raw/master/integrations/shared/rust/exit-early.wasm)
{% endhint %}

In addition to exiting in host calls, Wasmer also offers a metering API for allowing a pre-defined amount of execution before interrupting. The docs for metering are not yet written -- stay tuned for more!


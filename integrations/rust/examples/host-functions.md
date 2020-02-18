# Exposing Host Functions to WebAssembly

{% hint style="success" %}
**Note**: The final code for this example can be found on [GitHub](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/runtime/rust-integration/examples/host_functions.rs)

> Please take a look at the [setup steps for the Rust integration](../setup.md).
{% endhint %}

Up until now, our WebAssembly program has only been able to do pure computation, that is, take arguments and return values. Most interesting use cases require more than just computation though. In this section we'll go over how to give the Wasm modules we run extra abilties in the form of host functions in an `ImportObject`.

In this example, we'll create a system for getting and adjusting a counter value. However host functions are not limited to storing data outside of Wasm memory, they're normal Rust functions and can do anything that Rust can do.

1. There will be a `get_counter` function that will return an `i32` of

   the current global counter.

2. There will be an `add_to_counter` function will add the passed

   `i32` value to the counter, and return an `i32` of the current

   global counter.

Let's generate a new project, and update our `src/main.rs` to look something like this:

```rust
// Import the Filesystem so we can read our .wasm file
use std::cell::RefCell;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;

// Import the wasmer runtime so we can use it
use wasmer_runtime::{
    error,
    // Include the function macro
    func,
    imports,
    instantiate,
    Func,
};

const WASM_FILE_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/example-rust-wasm-crate/host-counter/target/wasm32-unknown-unknown/release/host_counter.wasm"
);

// Our entry point to our application
fn main() -> error::Result<()> {
    // Let's read in our .wasm file as bytes

    // Let's open the file.
    let mut file = File::open(WASM_FILE_PATH).expect(&format!("wasm file at {}", WASM_FILE_PATH));

    // Let's read the file into a Vec
    let mut wasm_vec = Vec::new();
    file.read_to_end(&mut wasm_vec)
        .expect("Error reading the wasm file");

    // We create some shared data here, [`Arc`] is required because we may
    // move our WebAssembly instance to another thread to run it.  RefCell
    // lets us get shared mutabilty which is fine because we know we won't
    // run hostcalls concurrently.  If concurrency is a possibilty, we'd have to
    // use a `Mutex`.
    let shared_counter: Arc<RefCell<i32>> = Arc::new(RefCell::new(0));

    // Clone the [`Arc`] for our closure and pass it into the host function
    let counter = Arc::clone(&shared_counter);
    let get_counter = move || -> i32 { *counter.borrow() };

    // Clone the [`Arc`] for our closure and pass it into the host function
    let counter = Arc::clone(&shared_counter);
    let add_to_counter = move |value_to_add: i32| -> i32 {
        let mut counter_ref = counter.borrow_mut();
        *counter_ref += value_to_add;
        *counter_ref
    };

    // Now that we have the wasm file as bytes, let's run it with the wasmer runtime

    // Let's define the import object used to import our function
    // into our webassembly sample application.
    //
    // Make sure to check your function signature (parameter and return types) carefully!
    let import_object = imports! {
        // Define the "env" namespace that was implicitly used
        // by our example rust wasm crate.
        "env" => {
            // Key should be the name of the imported function
            // Value should be the func! macro, with the function passed in.
            "get_counter" => func!(get_counter),
            "add_to_counter" => func!(add_to_counter),
        },
    };

    // Let's create an instance of wasm module running in the wasmer-runtime
    let instance = instantiate(&wasm_vec, &import_object)?;

    // Define the number of times we want to loop our increment
    let number_of_times_to_loop: i32 = 5;

    // Let's get `increment_counter_loop` as a function which takes one `i32` and returns one `i32`
    let increment_counter_loop: Func<i32, i32> = instance.func("increment_counter_loop")?;
    let result = increment_counter_loop.call(number_of_times_to_loop)?;

    let counter_value: i32 = *shared_counter.borrow();

    // Assert our counter is the expected value
    assert_eq!(number_of_times_to_loop, counter_value);

    // Asserting that the returned value from the function is our expected value.
    assert_eq!(result, counter_value);

    // Log the new value
    println!("New Counter Value: {}", counter_value);

    // Log a success message.
    println!("Success!");

    // Return OK since everything executed successfully!
    Ok(())
}
```

Both of the functions in this case are closures, but they don't have to be. Host functions can take an optional `&mut Ctx` argument as their first argument, which is how host functions get access to Wasm memory and other Wasm-related data.

In the above example we exposed host functions to the guest Wasm module with the namespace and name given in the `imports!` macro. The default namespace is `env` so we list `get_counter` and `add_to_counter` there.

Depending on the ABI of the Wasm module, we may need to expose functions under a different namespace. On the guest side, a non-default import namespace looks like:

```rust
extern "C" {
   #[link_name = "namespace"]
   fn import_name(arg: u32);
}
```

Next, we will take a look at handling errors from a WebAssembly module!


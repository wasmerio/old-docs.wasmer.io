# Exposing Host Functions

{% hint style="success" %}
**Note**: The final code for this example can be found on [GitHub](https://github.com/wasmerio/docs.wasmer.io/tree/master/integrations/rust/examples/host-functions).
{% endhint %}

Up until now, our WebAssembly program has only been able to do pure computation, that is, take arguments and return values. Most interesting use cases require more than just computation though. In this section we'll go over how to give the Wasm modules we run extra abilities in the form of host functions in an `ImportObject`.

In this example, we'll create a system for getting and adjusting a counter value. However host functions are not limited to storing data outside of Wasm memory, they're normal Rust functions and can do anything that Rust can do.

1. There will be a `get_counter` function that will return an `i32` of

   the current global counter.

2. There will be an `add_to_counter` function will add the passed

   `i32` value to the counter, and return an `i32` of the current

   global counter.

Let's generate a new project, and update our `src/main.rs` to look something like this:

```rust
use std::{cell::RefCell, sync::Arc};

// Import the wasmer runtime so we can use it
use wasmer_runtime::{error, imports, instantiate, func, Func};


// Our entry point to our application
fn main() -> error::Result<()> {
    // Let's get the .wasm file as bytes
    let wasm_bytes = include_bytes!("host-functions.wasm");

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

    // Now that we have the Wasm file as bytes, let's run it with the wasmer runtime

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
            "get_counter" => func!(get_counter),
            "add_to_counter" => func!(add_to_counter),
        },
    };

    // Let's create an instance of Wasm module running in the wasmer-runtime
    let instance = instantiate(wasm_bytes, &import_object)?;

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

{% hint style="info" %}
You can download the `host-functions.wasm` WebAssembly module here:  
[integrations/shared/rust/host-functions.wasm](https://github.com/wasmerio/docs.wasmer.io/raw/master/integrations/shared/rust/host-functions.wasm)
{% endhint %}

Now we should be ready to run it!

```bash
cargo run
```

{% hint style="info" %}
If you want to run the examples from the docs codebase directly, you can also do:

```bash
git clone https://github.com/wasmerio/docs.wasmer.io.git
cd docs.wasmer.io/integrations/rust/host-functions
```
{% endhint %}

Both of the functions in this case are closures, but they don't have to be. Host functions can take an optional `&mut Ctx` argument as their first argument, which is how host functions get access to Wasm memory and other WASM-related data.

In the above example we exposed host functions to the guest Wasm module with the namespace and name given in the `imports!` macro. The used namespace is `host` so we list `get_counter` and `add_to_counter` there.

{% hint style="info" %}
Depending on the ABI of the Wasm module, we may need to expose functions under a different namespace. On the guest side, a non-default import namespace looks like:

```rust
#[link(wasm_import_module = "namespace")]
extern "C" {
   fn import_name(arg: u32);
}
```
{% endhint %}

Next, we will take a look at handling errors from a WebAssembly module!


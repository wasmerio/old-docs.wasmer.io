use std::{cell::RefCell, sync::Arc};

// Import the wasmer runtime so we can use it
use wasmer_runtime::{
    error,
    // Include the function macro
    func,
    imports,
    instantiate,
    Func,
};

// Our entry point to our application
fn main() -> error::Result<()> {
    // Let's get the .wasm file as bytes
    let wasm_bytes = include_bytes!("../../../../shared/rust/host-functions.wasm");

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

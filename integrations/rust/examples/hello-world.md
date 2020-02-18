# Hello World

{% hint style="success" %}
**Note**: The final code for this example can be found on [GitHub](https://github.com/wasmerio/docs.wasmer.io/blob/master/docs/runtime/rust-integration/examples/hello_world.rs)

> Please take a look at the [setup steps for the Rust integration](../setup.md).
{% endhint %}

In this example we will be building a "Hello World"-like project. WebAssembly only supports passing integers and floats directly right now, thus to keep it simple we will be writing a host application that calls the "add\_one" function of a guest wasm module, which adds 1 to the value passed as a parameter, and returns the result.

First we are going to want to initialize a new project. To do this we can navigate to our project folder, or create one. In this example, we will create a new project named `hello-world`. Thus, lets create it with cargo and navigate to it:

```bash
cargo new hello-world
cd hello-world
```

This should generate two important files for us, `Cargo.toml` and `src/main.rs`. The `Cargo.toml` is a file that describes your project and its dependencies. The `src/main.rs` is the entry point for your project, and contains the `fn main() { .. }` that is run when the project is executed.

Let's modify our `Cargo.toml` to add the [`wasmer-runtime` crate](https://crates.io/crates/wasmer-runtime/0.13.1) to our project. At the time of this writing, the crate is at version `0.13.1`. So we change the `Cargo.toml` to the following:

```yaml
[package]
name = "hello-world"
version = "0.1.0"
authors = ["The Wasmer Engineering Team <engineering@wasmer.io>"]
edition = "2018"

[dependencies]
# Add the wasmer-runtime as a dependency
wasmer-runtime = "0.13.1"
```

Now that we have the Wasmer runtime added as a dependency, let's go ahead and try it out! For our hello world, what we will do is use the Wasmer runtime to execute an exported function on a WebAssembly module, that adds one the the integer passed to the function. To do this, we will modify our `src/main.rs` to the following:

```rust
// Import the Filesystem so we can read our .wasm file
use std::fs::File;
use std::io::prelude::*;

// Import the wasmer runtime so we can use it
use wasmer_runtime::{error, imports, instantiate, Func};

// Get the path of compiled webassembly
const WASM_FILE_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/target/wasm32-unknown-unknown/release/hello_world_guest.wasm"
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

    // Now that we have the wasm file as bytes, let's run it with the wasmer runtime

    // Our import object, that allows exposing functions to our wasm module.
    // We're not importing anything, so make an empty import object.
    let import_object = imports! {};

    // Let's create an instance of wasm module running in the wasmer-runtime
    let instance = instantiate(&wasm_vec, &import_object)?;

    // Let's get a number we want to add one to
    let value_to_add = 42;
    println!("Original Value: {}", value_to_add);

    // Let's get `add_one` as a function which takes one `u32` and returns one `u32`
    let add_one: Func<u32, u32> = instance.func("add_one")?;
    let result = add_one.call(value_to_add)?;

    // Log the new value
    println!("New Value: {}", 43);

    // Asserting that the returned value from the function is our expected value.
    assert_eq!(result, 43);

    // Log a success message.
    println!("Success!");

    // Return OK since everything executed successfully!
    Ok(())
}
```

Now, we should be ready to run it!

```text
cd integrations/rust/hello-world
cargo run
```

{% hint style="success" %}
This should execute the `add_one` function, which we pass the `i32` value of `42` to it, and returns the integer value of `43`!
{% endhint %}

Hooray! We got our first Rust application running WebAssembly working!

Next, let's take a look at passing data between the host \(our Rust application\), and the WebAssembly module.


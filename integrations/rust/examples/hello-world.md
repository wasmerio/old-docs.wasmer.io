# Hello World

{% hint style="success" %}
**Note**: The final code for this example can be found on [GitHub](https://github.com/wasmerio/docs.wasmer.io/tree/master/integrations/rust/examples/hello-world).

_Please take a look at the_ [_setup steps for Rust_](https://github.com/wasmerio/docs.wasmer.io/tree/f2ebe6a08e0ac5f6bd58ababffa793df6ab4424d/integrations/rust/examples/setup.md)_._
{% endhint %}

In this example we will be building a "Hello World"-like project. WebAssembly only supports passing integers and floats directly right now, thus to keep it simple we will be writing a host application that calls the `add_one` function of a guest Wasm module, which adds `1` to the value passed as a parameter, and returns the result.

First we are going to want to initialize a new project. To do this we can navigate to our project folder, or create one. In this example, we will create a new project named `hello-world`. Thus, lets create it with cargo and navigate to it:

```bash
cargo new hello-world
cd hello-world
```

This should generate two important files for us, `Cargo.toml` and `src/main.rs`. The `Cargo.toml` is a file that describes your project and its dependencies. The `src/main.rs` is the entry point for your project, and contains the `fn main() { .. }` that is run when the project is executed.

Let's modify our `Cargo.toml` to add the [`wasmer-runtime` crate](https://crates.io/crates/wasmer-runtime/0.13.1) to our project. At the time of this writing, the crate is at version `0.13.1`. So we change the `Cargo.toml` to the following:

{% code title="Cargo.toml" %}
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
{% endcode %}

Now that we have the Wasmer runtime added as a dependency, let's go ahead and try it out! For our hello world, what we will do is use the Wasmer runtime to execute an exported function on a WebAssembly module, that adds one the the integer passed to the function.

To do this, we will create a new `src/main.rs` file.

Let's start with the imports.

```rust
// Import the wasmer runtime so we can use it
use wasmer_runtime::{error, imports, instantiate, Func};
```

Now that we have access to the `instantiate` and imports macro, we should be able to create our first WebAssembly Instance!

```rust
fn main() -> error::Result<()> {
    // Let's get the .wasm file as bytes
    let wasm_bytes = include_bytes!("add.wasm");

    // Our import object, that allows exposing functions to our wasm module.
    // We're not importing anything, so make an empty import object.
    let import_object = imports! {};

    // Let's create an instance of wasm module running in the wasmer-runtime
    let instance = instantiate(wasm_bytes, &import_object)?;
}
```

{% hint style="info" %}
You can download the `add.wasm` WebAssembly module here:  
[https://github.com/wasmerio/docs.wasmer.io/raw/master/integrations/shared/add.wasm](https://github.com/wasmerio/docs.wasmer.io/raw/master/integrations/shared/add.wasm)
{% endhint %}

And now we can just call the `add_one` function \(remember to use it inside the `main()` function\).

```rust
    // Let's get `add_one` as a function which takes one `u32` and returns one `u32`
    let add_one: Func<u32, u32> = instance.func("add_one")?;
    let result = add_one.call(42)?;

    // Log the new value
    println!("Result: {}", result);

    // Asserting that the returned value from the function is our expected value.
    assert_eq!(result, 43);  // 42 + 1
```

{% hint style="success" %}
This should execute the `add_one` function, which we pass the `i32` value of `42` to it, and returns the integer value of `43`!
{% endhint %}

Our resulting `src/main.rs` should look like the following:

{% code title="src/main.rs" %}
```rust
// Import the wasmer runtime so we can use it
use wasmer_runtime::{error, imports, instantiate, Func};

// Our entry point to our application
fn main() -> error::Result<()> {
    // Let's get the .wasm file as bytes
    let wasm_bytes = include_bytes!("add.wasm");

    // Our import object, that allows exposing functions to our wasm module.
    // We're not importing anything, so make an empty import object.
    let import_object = imports! {};

    // Let's create an instance of wasm module running in the wasmer-runtime
    let instance = instantiate(wasm_bytes, &import_object)?;

    // Let's get `add_one` as a function which takes one `u32` and returns one `u32`
    let add_one: Func<u32, u32> = instance.func("add_one")?;
    let result = add_one.call(42)?;

    // Log the new value
    println!("Result: {}", result);

    // Asserting that the returned value from the function is our expected value.
    assert_eq!(result, 43);

    // Return OK since everything executed successfully!
    Ok(())
}
```
{% endcode %}

Now, we should be ready to run it!

```bash
cargo run
```

Hooray! We got our first Rust application running WebAssembly working! 🎉

{% hint style="info" %}
If you want to run the examples from the docs codebase directly, you can also do:

```bash
git clone https://github.com/wasmerio/docs.wasmer.io.git
cd docs.wasmer.io/integrations/rust/hello-world
```
{% endhint %}

Next, let's take a look at passing data between the host \(our Rust application\), and the WebAssembly module.


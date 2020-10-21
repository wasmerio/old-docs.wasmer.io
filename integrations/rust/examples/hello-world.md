# Hello World

{% hint style="success" %}
**Note**: The final code for this example can be found on [GitHub](https://github.com/wasmerio/docs.wasmer.io/tree/master/integrations/rust/examples/hello-world).
{% endhint %}

In this example we will be building a "Hello World"-like project. WebAssembly only supports passing integers and floats directly right now, thus to keep it simple we will be writing a host application that calls the `add_one` function of a guest Wasm module, which adds `1` to the value passed as a parameter, and returns the result.

First we are going to want to initialize a new project. To do this we can navigate to our project folder, or create one. In this example, we will create a new project named `hello-world`. Thus, lets create it with cargo and navigate to it:

```bash
cargo new hello-world
cd hello-world
```

This should generate two important files for us, `Cargo.toml` and `src/main.rs`. The `Cargo.toml` is a file that describes your project and its dependencies. The `src/main.rs` is the entry point for your project, and contains the `fn main() { .. }` that is run when the project is executed.

We then modify the `Cargo.toml` to add the Wasmer dependencies shown below:

{% code title="Cargo.toml" %}
```yaml
[package]
name = "hello-world"
version = "0.1.0"
authors = ["The Wasmer Engineering Team <engineering@wasmer.io>"]
edition = "2018"

[dependencies]
# The Wasmer API
wasmer = "1.0.0-alpha4"
# The Cranelift compiler used by the JIT engine
wasmer-compiler-cranelift = "1.0.0-alpha01.0"
# The engine we'll use in the API
wasmer-engine-jit = "1.0.0-alpha01.0"
```
{% endcode %}

Now that we have the Wasmer API added as a dependency, let's go ahead and try it out! For our hello world, what we will do is use the Wasmer runtime to execute an exported function in a WebAssembly module, that adds one to the integer passed to the function.

To do this, we will create a new `src/main.rs` file.

Let's start with the imports.

```rust
// Import the wasmer runtime so we can use it
use wasmer::{imports, Instance, Module, NativeFunc, Store};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_jit::JIT;
```

Now that we have access to what we need, we can create our first WebAssembly Instance!

```rust
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
```

{% hint style="info" %}
You can download the `add.wasm` WebAssembly module here:  
[integrations/shared/rust/add.wasm](https://github.com/wasmerio/docs.wasmer.io/raw/master/integrations/shared/rust/add.wasm)

Note: You can [find the implementation of it here](https://github.com/wasmerio/docs.wasmer.io/blob/master/integrations/shared/rust/add.rs)
{% endhint %}

And now we can get the `add_one` function from the instance and call it!

```rust
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
```

{% hint style="success" %}
This should execute the `add_one` function, which we pass the `i32` value of `42` to it, and returns the integer value of `43`!
{% endhint %}

Our resulting `src/main.rs` should look like the following:

{% code title="src/main.rs" %}
```rust
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


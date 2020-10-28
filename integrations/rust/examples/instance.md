---
description: >-
  This example illustrates the basics of using Wasmer through a "Hello World"-like project.
---

# Instantiating a Wasm module

{% hint style="success" %}
**Note**: The final code for this example can be found on 
[GitHub](https://github.com/wasmerio/wasmer/blob/master/examples/instance.rs).

_Please take a look at the_ [_setup steps for Rust_](../setup.md)_._
{% endhint %}

In this example we will be building a "Hello World"-like project. WebAssembly only supports passing integers and floats 
directly right now, thus to keep it simple we will be writing a host application that calls the `add_one` function of 
a guest Wasm module, which adds `1` to the value passed as a parameter, and returns the result.

The goal here is to show you the basics of using Wasmer, we'll focus on the steps required to get an instance out of a
Wasm module.

First we are going to want to initialize a new project. To do this we can navigate to our project folder, or create one. 
In this example, we will create a new project named `hello-world`. Thus, lets create it with cargo and navigate to it:

```bash
cargo new instance
cd instance
```

This should generate two important files for us, `Cargo.toml` and `src/main.rs`. The `Cargo.toml` is a file that 
describes your project and its dependencies. The `src/main.rs` is the entry point for your project, and contains 
the `fn main() { .. }` that is run when the project is executed.

We then modify the `Cargo.toml` to add the Wasmer dependencies as shown below:

{% code title="Cargo.toml" %}
```yaml
[package]
name = "instance"
version = "0.1.0"
authors = ["The Wasmer Engineering Team <engineering@wasmer.io>"]
edition = "2018"

[dependencies]
# The Wasmer API
wasmer = "1.0.0-alpha4"
```
{% endcode %}

Now that we have the Wasmer crate added as a dependency, let's go ahead and try it out!

## Loading the Wasm module

The first step will be to load the Wasm module we want to use. This is done by having its contents loaded as bytes:

{% code title="src/main.rs" %}
```rust
let wasm_bytes = wat2wasm(br#"
(module
  (type $add_one_t (func (param i32) (result i32)))
  (func $add_one_f (type $add_one_t) (param $value i32) (result i32)
    local.get $value
    i32.const 1
    i32.add)
  (export "add_one" (func $add_one_f)))
"#)?;
```
{% endcode %}

Here we are using the text representation of the Wasm module. Wasmer wants to have a binary representation of the module
so we have to use `wat2wasm` to do the translation.

Let's assume we have the binary version of the module (i.e the `.wasm` file), here is how we would have loaded it:

{% code title="src/main.rs" %}
```rust
let wasm_bytes = include_bytes!("./path/to/module.wasm");
```
{% endcode %}

## Compiling the Wasm module

The next step will be to compile the module. To do this, we'll need two things: the Wasm module as bytes and a `Store`.

The `Store` is a representation of the actual state of the module: it represents the state of every entities in the 
module during its lifecycle. It also holds the engine which is what will be used to actually compile the module.

Here is how we can create the store and compile the module:

{% code title="src/main.rs" %}
```rust
let store = Store::new(&JIT::new(&Cranelift::default()).engine());
let module = Module::new(&store, wasm_bytes)?;
```
{% endcode %}

As you can see, we created a store with the JIT engine and the Cranelift compiler with its default configuration. These 
are good defaults but it will be a good thing to adapt this configuration to your needs.

{% hint style="success" %}
**Note**: Wasmer provides others compilers each one having its own tradeoffs and features.

{% page-ref page="ecosystem/wasmer/wasmer-features.md" %}
{% endhint %}

## Creating an instance of the module

We are now close to having the module run in our Rust host.

The last step will be to create an `Instance` out of the Wasm module. As for the previous step, here we need more than 
just the compiled module: we also need to define imports.

In fact, Wasm modules can define entities they need to work properly. These are called imports. In this example we 
don't need any of them but we still need to define an empty set and use it to instantiate the module:

{% code title="src/main.rs" %}
```rust
let import_object = imports! {};
let instance = Instance::new(&module, &import_object)?;
```
{% endcode %}

## Running

We now have everything we need to run the Wasm module, let's do it!

You should be able to run it using the `cargo run` command. The output should look like this:

```text
Calling `add_one` function...
Results of `add_one`: 2
```

{% hint style="info" %}
If you want to run the examples from the Wasmer [repository](https://github.com/wasmerio/wasmer/) codebase directly, 
you can also do:

```bash
git clone https://github.com/wasmerio/wasmer.git
cd wasmer
cargo run --example instance --release --features "cranelift"
```
{% endhint %}

Next, let's take a look at passing data between the host \(our Rust application\), and the WebAssembly module.

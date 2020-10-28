---
description: >-
  In this example we'll focus on how to call the exported functions.
---

# Calling guest functions

{% hint style="success" %}
**Note**: The final code for this example can be found on 
[GitHub](https://github.com/wasmerio/wasmer/blob/master/examples/exports_function.rs).

_Please take a look at the_ [_setup steps for Rust_](../setup.md)_._
{% endhint %}

Now that we know how to instantiate a Wasm module, let's see how to do basic interactions with it. 

A Wasm module can export entities like functions, memories globals and table. In this example we'll focus on
how to call the exported functions as this will likely be the first thing you will want to do now.

Exported functions come into two flavors:
* Dynamic functions, where parameters and results are of a slice of `Value`;
* Native function, where parameters and results are statically typed Rust values.

We'll cover both and see how they differ from each other.

First we are going to want to initialize a new project. To do this we can navigate to our project folder, or create one. 
In this example, we will create a new project named `hello-world`. Thus, lets create it with cargo and navigate to it:

```bash
cargo new exports-function
cd exports-function
```

This should generate two important files for us, `Cargo.toml` and `src/main.rs`. The `Cargo.toml` is a file that 
describes your project and its dependencies. The `src/main.rs` is the entry point for your project, and contains 
the `fn main() { .. }` that is run when the project is executed.

We then modify the `Cargo.toml` to add the Wasmer dependencies as shown below:

{% code title="Cargo.toml" %}
```toml
[package]
name = "exports-function"
version = "0.1.0"
authors = ["The Wasmer Engineering Team <engineering@wasmer.io>"]
edition = "2018"

[dependencies]
# The Wasmer API
wasmer = "1.0.0-alpha4"
```
{% endcode %}

Now that we have the Wasmer crate added as a dependency, let's go ahead and try it out!

## Fetching exported functions

Before starting, we'll have to gather some information about the module and what it exports. To fetch an exported 
function we need three things:
* What are the exported functions names?
* For each function, what are the expected parameters types?
* For each function, what is the result type?

Let's have a look at our example Wasm module:

```text
(module
  (type $sum_t (func (param i32 i32) (result i32)))
  (func $sum_f (type $sum_t) (param $x i32) (param $y i32) (result i32)
    local.get $x
    local.get $y
    i32.add)
  (export "sum" (func $sum_f)))
```

Here we can see our module exports a single `sum` functions which expects two `i32` parameters and returns a single 
`i32` value. This is fine, let's fetch the exported function:

{% code title="src/main.rs" %}
```rust
let sum = instance.exports.get_function("sum")?;
```
{% endcode %}

As we said earlier, the instance is our entry point to the modules so we use its `exports` to fetch the `sum` function.

Here we have a dynamic version of the function. Let's see how to fetch the exact same function but this time, as a 
native one: 

{% code title="src/main.rs" %}
```rust
let sum_native = instance.exports.get_native_function::<(i32, i32), i32>("sum")?;
```
{% endcode %}

{% hint style="info" %}
Note that once you have the dynamic form a function, you can turn it into its native form:

```rust
let sum_native = sum.native::<(i32, i32), i32>()?;
```
{% endhint %}

Fetching a function as a native one is a little bit more verbose but we'll see that when calling it, it may be simpler
than with the dynamic one.

## Calling exported functions

Now that we have the exported function on the host side, let's see how to call them. We'll compare the two flavors. 
Let's start with the dynamic one:

{% code title="src/main.rs" %}
```rust
let results = sum.call(&[Value::I32(1), Value::I32(2)])?;
```
{% endcode %}

Calling a dynamic function requires passing the arguments as a slice of `Value`s.  The result will be a boxed
slice of `Value`s.

What about the native one?

{% code title="src/main.rs" %}
```rust
let result: i32 = sum_native.call(1, 2)?;
```
{% endcode %}

Calling the native function feels more like regular Rust: the arguments and result are supplied as
statically typed Rust values. This makes the calls easier to write and the results easier to use whereas with dynamic 
functions where fetching is easier but calling them and using the result is less intuitive. 

## Running

We now have everything we need to run the Wasm module, let's do it!

You should be able to run it using the `cargo run` command. The output should look like this:

```text
Compiling module...
Instantiating module...
Calling `sum` function...
Results: [I32(3)]
Calling `sum` function (natively)...
Results: 3
```

{% hint style="info" %}
If you want to run the examples from the Wasmer [repository](https://github.com/wasmerio/wasmer/) codebase directly, 
you can also do:

```bash
git clone https://github.com/wasmerio/wasmer.git
cd wasmer
cargo run --example exported-function --release --features "cranelift"
```
{% endhint %}

Next, we will take a look at handling errors from a WebAssembly module!


---
description: >-
  In this example we'll cover how to import function from the host into the Wasm module.
---

# Exposing host functions

{% hint style="success" %}
**Note**: The final code for this example can be found on 
[GitHub](https://github.com/wasmerio/wasmer/blob/master/examples/imports_function.rs).

_Please take a look at the_ [_setup steps for Rust_](../setup.md)_._
{% endhint %}

Up until now, our WebAssembly program has only been able to do pure computation, that is, take arguments and return 
values. Most interesting use cases require more than just computation though. In this section we'll go over how to give 
the Wasm modules we run extra abilities in the form of host functions in an `ImportObject`.

In this example we'll cover how to import function from the host into the Wasm module. To do so, we'll create a system
for doing computation that use logic provided by the host.

The Wasm module exports a [`sum`](https://github.com/wasmerio/wasmer/blob/master/examples/imports_function.rs#L32-L37) 
function which sums the results of two imported function:
* [`multiply_dynamic`](https://github.com/wasmerio/wasmer/blob/master/examples/imports_function.rs#L29) 
* [`multiply_native`](https://github.com/wasmerio/wasmer/blob/master/examples/imports_function.rs#L30) 

These two functions will help us cover the two type of imported function Wasmer lets us create: dynamic functions and
native functions.

Let's generate a new project, and start with the `Cargo.toml` file:

{% code title="Cargo.toml" %}
```toml
[package]
name = "imports-function"
version = "0.1.0"
authors = ["The Wasmer Engineering Team <engineering@wasmer.io>"]
edition = "2018"

[dependencies]
# The Wasmer API
wasmer = "1.0.0-alpha4"
```
{% endcode %}

Now that we have the Wasmer crate added as a dependency, let's go ahead and try it out!

## Declaring functions on the host

As we said earlier, our module needs two imported functions, let's declare them. We'll start with the first one, 
`multiply_dynamic` which, as its name implies, will be a dynamic function:

{% code title="src/main.rs" %}
```rust
let multiply_dynamic_signature = FunctionType::new(vec![Type::I32], vec![Type::I32]);
let multiply_dynamic = Function::new(&store, &multiply_dynamic_signature, |args| {
    println!("Calling `multiply_dynamic`...");

    let result = args[0].unwrap_i32() * 2;

    println!("Result of `multiply_dynamic`: {:?}", result);

    Ok(vec![Value::I32(result)])
});
```
{% endcode %}

We first have to define the function's signature using a `FunctionType`: it takes a vector of the arguments types and 
a vector of the results type. 

Then we define the function itself using `Function` providing the signature and a closure with the function's code. The
closure **must** a single argument being a slice of the parameters `Value`s.

The second function, `multiply_native` will be easier. We'll define a native function which means we only have to define
a regular Rust function and wrap it into a `Function`:

{% code title="src/main.rs" %}
```rust
fn multiply(a: i32) -> i32 {
    println!("Calling `multiply_native`...");
    let result = a * 3;

    println!("Result of `multiply_native`: {:?}", result);

    result
}
let multiply_native = Function::new_native(&store, multiply);
```
{% endcode %}

Easy, right?

Keep in mind that when choosing which type of function to use, you have to take into account more than just the 
verbosity: with native function, parameters and results are statically typed Rust values whereas dynamic functions 
could bring more flexibility.

## Importing functions into the Wasm module

Now that we have our functions ready, we'll have to wrap them in an `ImportObject` to make them available to the 
Wasm module. With Wasmer, this is an easy task:

{% code title="src/main.rs" %}
```rust
let import_object = imports! {
    "env" => {
        "multiply_dynamic" => multiply_dynamic,
        "multiply_native" => multiply_native,
    }
};
```
{% endcode %}

Note that our functions are defined in the `env` namespace here. This is a requirement of the Wasm module we are using 
but this can be anything else, even an empty string.

## Running

We now have everything we need to run the Wasm module, let's do it!

You should be able to run it using the `cargo run` command. The output should look like this:

```text
Compiling module...
Instantiating module...
Calling `sum` function...
Calling `multiply_dynamic`...
Result of `multiply_dynamic`: 2
Calling `multiply_native`...
Result of `multiply_native`: 6
Results of `sum`: 8
```

{% hint style="info" %}
If you want to run the examples from the Wasmer [repository](https://github.com/wasmerio/wasmer/) codebase directly, 
you can also do:

```bash
git clone https://github.com/wasmerio/wasmer.git
cd wasmer
cargo run --example imported-function --release --features "cranelift"
```
{% endhint %}

Next, we will take a look at handling errors from a WebAssembly module!


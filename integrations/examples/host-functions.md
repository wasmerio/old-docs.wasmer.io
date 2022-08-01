---
description: >-
  A Wasm module can import entities, like functions, memories, globals and
  tables. This example illustrates how to expose functions from the host.
---

# ↩️ Exposing host \(imported\) functions

Up until now, our WebAssembly program has only been able to do pure computation, that is, take arguments and return values. Most interesting use cases require more than just computation though. In this section we'll go over how to give the Wasm modules we run extra abilities in the form of host functions.

In this example, we'll create a system for getting and adjusting a counter value. However, host functions are not limited to storing data outside of Wasm, they're normal host functions and can do anything that the host can do.

1. There will be a `get_counter` function that will return an `i32` of

   the current global counter.

2. There will be an `add_to_counter` function that will add the passed

   `i32` value to the counter, and return an `i32` of the current

   global counter.

First we are going to want to initialize a new project. To do this we can navigate to our project folder, or create one. In this example, we will create a new project. Lets create it and navigate to it:

{% tabs %}
{% tab title="Rust" %}
{% hint style="info" %}
The final **Rust** code for this example can be found on Github: [imports_function_env.rs](https://github.com/wasmerio/wasmer/blob/master/examples/imports_function_env.rs).

_Please take a look at the_ [_setup steps for Rust_](../rust/setup.md)_._
{% endhint %}

```bash
cargo new imported-function-env
cd imported-function-env
```

We have to modify `Cargo.toml` to add the Wasmer dependencies as shown below:

```yaml
[dependencies]
# The Wasmer API
wasmer = "3.0"
```
{% endtab %}
{% endtabs %}

Now that we have everything set up, let's go ahead and try it out!

## Declaring the data

Because we want to store data outside of the Wasm module and have host functions use this data, we need to do some preparation. We'll need to declare the data we want to use and the container to hold it.

{% tabs %}
{% tab title="Rust" %}
```rust
let shared_counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

#[derive(Clone)]
struct Env {
    counter: Arc<Mutex<i32>>,
}
```

{% hint style="info" %}
Here we use a combination of `Arc` and `Mutex` to guarantee thread safety while allowing mutability.
{% endhint %}
{% endtab %}
{% endtabs %}

## Declaring functions and imports

Now that our data is available we'll declare the functions.

{% tabs %}
{% tab title="Rust" %}
```rust
fn get_counter(env: FunctionEnvMut<Env>) -> i32 {
    *env.data().counter.lock().unwrap()
}

fn add_to_counter(env: &FunctionEnvMut<Env>, add: i32) -> i32 {
    let mut counter_ref = env.data().counter.lock().unwrap();

    *counter_ref += add;
    *counter_ref
}
```

As you can see here, both functions take an extra parameter in the form of a mutable reference to an `Env` which is the container we created to hold our data.
{% endtab %}
{% endtabs %}

The last thing we need to do now is to imports the function in the Wasm module.

{% tabs %}
{% tab title="Rust" %}
```rust
let get_counter_func = Function::new_typed_with_env(
    &mut store, 
    Env { counter: shared_counter.clone() }, 
    get_counter
);

let add_to_counter_func = Function::new_typed_with_env(
    &mut store, 
    Env { counter: shared_counter.clone() }, 
    add_to_counter
);

let import_object = imports! {
    "env" => {
        "get_counter" => get_counter_func,
        "add_to_counter" => add_to_counter_func,
    }
};
```

We use `Function::new_typed_with_env` here to tell Wasmer our host functions need our `Env` to be passed in addition to other arguments.

If the host function does not need external data \(it is pure\) we use the `new_typed` function instead, which has the same signature except that it doesn't take the `env` parameter.
{% endtab %}
{% endtabs %}

Now each time the `add_to_counter` will be run from the Wasm module it will alter the data on the host side.

## Running

We now have everything we need to run the Wasm module, let's do it!

{% tabs %}
{% tab title="Rust" %}
You should be able to run it using the `cargo run` command. The output should look like this:

```text
Compiling module...
Instantiating module...
Initial ounter value: 0
Calling `increment_counter_loop` function...
New counter value (host): 5
New counter value (guest): 5
```

{% hint style="info" %}
If you want to run the examples from the Wasmer [repository](https://github.com/wasmerio/wasmer/) codebase directly, you can also do:

```bash
git clone https://github.com/wasmerio/wasmer.git
cd wasmer
cargo run --example imported-function-env --release --features "cranelift"
```
{% endhint %}
{% endtab %}
{% endtabs %}


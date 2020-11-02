# Interrupting Execution

{% hint style="warning" %}
TODO: Write this section
{% endhint %}

## Interrupting Execution

{% hint style="info" %}
**Note**: The final code for this example can be found on [GitHub](https://github.com/wasmerio/wasmer/blob/master/examples/early_exit.rs).

_Please take a look at the_ [_setup steps for Rust_](../rust/setup.md)_._
{% endhint %}

WebAssembly is currently always run in the same process synchronously. Thus, once WebAssembly starts executing, you have to wait for the execution to complete to continue running code on the host \(your Rust application\).

However, there are cases where you may want to interrupt this synchronous execution while the guest WASM module is calling a host function. This can be useful for saving resources, and not returning back to the guest WASM for execution, when you already know the WASM execution will fail, or no longer be needed.

In this example, we will run a WASM module that calls the imported host function `interrupt_execution`. This host function will immediately stop executing the WebAssembly module:

First we are going to want to initialize a new project. To do this we can navigate to our project folder, or create one. In this example, we will create a new project named `hello-world`. Thus, lets create it with cargo and navigate to it:

```bash
cargo new early-exit
cd early-exit
```

This should generate two important files for us, `Cargo.toml` and `src/main.rs`. The `Cargo.toml` is a file that describes your project and its dependencies. The `src/main.rs` is the entry point for your project, and contains the `fn main() { .. }` that is run when the project is executed.

We then modify the `Cargo.toml` to add the Wasmer dependencies as shown below:

{% code title="Cargo.toml" %}
```yaml
[package]
name = "early-exit"
version = "0.1.0"
authors = ["The Wasmer Engineering Team <engineering@wasmer.io>"]
edition = "2018"

[dependencies]
# The Wasmer API
wasmer = "1.0.0-alpha4"
```
{% endcode %}

Now that we have the Wasmer crate added as a dependency, let's go ahead and try it out!

### Setting up

Before we start with the WASM part we'll have to declare the error we'll use to terminate the execution of the guest module:

```rust
#[derive(Debug, Clone, Copy)]
struct ExitCode(u32);

impl fmt::Display for ExitCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ExitCode {}
```

There is nothing special or Wasmer specific here but it will be required later in the example.

### Defining and importing the host function

To terminate the execution of the WASM module we'll have to define a function on the host which will then be imported in the guest and called whenever execution is not required to continue. Let's do that:

{% code title="src/main.rs" %}
```rust
fn early_exit() {
    RuntimeError::raise(Box::new(ExitCode(1)));
}

let import_object = imports! {
    "env" => {
        "early_exit" => Function::new_native(&store, early_exit),
    }
};
```
{% endcode %}

As we saw in previous examples we defined a Rust function, wrap it in a native function definition and import it in the guest module, in the `env` namespace, using the `ImportObject`.

### Handling the error

Our module will call the `early_exit` function once we call its `run` function \(which is an exported function\). Let's get the function, call it and see how we can handle the error:

{% code title="src/main.rs" %}
```rust
let run_func: NativeFunc<(i32, i32), i32> = instance.exports.get_native_function("run").unwrap();

match run_func.call(1, 7) {
    Ok(result) => {
        bail!("Expected early termination with `ExitCode`, found: {}", result);
    }   
    Err(e) => match e.downcast::<ExitCode>() {
        // We found the exit code used to terminate execution.
        Ok(exit_code) => {
            println!("Exited early with exit code: {}", exit_code);
            Ok(())
        }
        Err(e) => {
            bail!("Unknown error `{}` found. expected `ErrorCode`", e);
        }
    },
}
```
{% endcode %}

We expect to get an error when calling the `run` function so what we do here is look at the result and:

* if we get a success, our test will fail;
* if we get an error, we try to downcast to our `ExitCode` error.

If downcasting succeeds it means we actually got the expected error so we make the test pass. If it fails, it means the WASM module reported an error but it wasn't the one we expected so we make the test fail.

### Running

We now have everything we need to run the WASM module, let's do it!

You should be able to run it using the `cargo run` command. The output should look like this:

```text
Exited early with exit code: 1
```

{% hint style="info" %}
If you want to run the examples from the Wasmer [repository](https://github.com/wasmerio/wasmer/) codebase directly, you can also do:

```bash
git clone https://github.com/wasmerio/wasmer.git
cd wasmer
cargo run --example early-exit --release --features "cranelift"
```
{% endhint %}


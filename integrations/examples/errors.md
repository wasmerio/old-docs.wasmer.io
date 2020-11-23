---
description: >-
  In this example we'll see how to pattern match for the error and output the
  error message returned from Wasmer.
---

# Handling Errors

There will come a time when running a WebAssembly module will not work, and trying to figure out why it does not work can be a difficult task! In the current MVP of WebAssembly, debugging isn't explicitly defined for runtimes both in and out of the browser. So we'll have to write some error handling code ourselves.

In this example, we will load a WebAssembly module that purposely produces an error in its exported function call. The Host \(our Rust application\) will pattern match for the error and output the error message returned from Wasmer.

First we are going to want to initialize a new project. To do this we can navigate to our project folder, or create one. In this example, we will create a new project. Lets create it and navigate to it:

{% tabs %}
{% tab title="Rust" %}
{% hint style="info" %}
The final code for this example can be found on [GitHub](https://github.com/wasmerio/wasmer/blob/master/examples/errors.rs).

_Please take a look at the_ [_setup steps for Rust_](../rust/setup.md)_._
{% endhint %}

```bash
cargo new errors
cd errors
```

This should generate two important files for us, `Cargo.toml` and `src/main.rs`. The `Cargo.toml` is a file that describes your project and its dependencies. The `src/main.rs` is the entry point for your project, and contains the `fn main() { .. }` that is run when the project is executed.

We then modify the `Cargo.toml` to add the Wasmer dependencies as shown below:

{% code title="Cargo.toml" %}
```yaml
[package]
name = "errors"
version = "0.1.0"
authors = ["The Wasmer Engineering Team <engineering@wasmer.io>"]
edition = "2018"

[dependencies]
# The Wasmer API
wasmer = "1.0.0-alpha4"
```
{% endcode %}
{% endtab %}

{% tab title="Go" %}
{% hint style="info" %}
The final code for this example can be found on [GitHub](https://github.com/wasmerio/wasmer-go/blob/master/examples/example_errors_test.go).

_Please take a look at the_ [_setup steps for Go_](../go/setup.md)_._
{% endhint %}

```text
mkdir wasmer-example-errors
cd wasmer-example-errors
go mod init github.com/$USER/wasmer-example-errors
```
{% endtab %}
{% endtabs %}

Now that we have everything set up, let's go ahead and try it out!

## Handling the error

There is nothing special about the WASM module or the way we'll set it up.

The only things we'll need to do are:

* Getting the exported function
* Calling the function;
* Handling the error.

Here is the easy part, getting and calling the function:

{% tabs %}
{% tab title="Rust" %}
```rust
let div_by_zero = instance.exports.get_function("div_by_zero")?.native::<(), i32>()?;
let result = div_by_zero.call();
```
{% endtab %}

{% tab title="Go" %}
```go
divByZero, err := instance.Exports.GetFunction("div_by_zero")

if err != nil {
    panic(fmt.Sprintln("Failed to get the `div_by_zero` function:", err))
}

_, err = divByZero()
```
{% endtab %}
{% endtabs %}

And here is the interesting part, handling the error:

{% tabs %}
{% tab title="Rust" %}
```rust
match result {
    Ok(_) => {
        panic!("throw_wasm_error did not error");
    },
    Err(e) => {
        println!("Error caught from `div_by_zero`: {}", e.message());

        let frames = e.trace();
        let frames_len = frames.len();

        for i in 0..frames_len {
            println!(
                "  Frame #{}: {:?}::{:?}",
                frames_len - i,
                frames[i].module_name(),
                frames[i].function_name().or(Some("<func>")).unwrap()
            );
        }
    }
}
```
{% endtab %}

{% tab title="Go" %}
```go
if err == nil {
    panic(fmt.Sprintln("`div_by_zero` did not error"))
}

fmt.Println("Error caught from `div_by_zero`:", err)

trap, ok := err.(*wasmer.TrapError)

if !ok {
    panic(fmt.Sprintln("Error was not of the expected type"))
}

frames := trap.Trace()
framesLen := len(frames)

for index, frame := range frames {
    fmt.Printf(
        "  Frame #%d: function index: %d\n", 
        framesLen - index, 
        frame.FunctionIndex()
    )
}
```
{% endtab %}
{% endtabs %}

Here we verify the result of calling the function to see if we actually got an error.

If we got an error we format a nice message containing information to help debug the problem:

* The error message.
* The error trace.

## Running

We now have everything we need to run the WASM module, let's do it!

{% tabs %}
{% tab title="Rust" %}
You should be able to run it using the `cargo run` command. The output should look like this:

```text
Compiling module...
Instantiating module...
Calling `div_by_zero` function...
Error caught from `div_by_zero`: integer divide by zero
  Frame #2: "<module>"::"do_div_by_zero_f"
  Frame #1: "<module>"::"div_by_zero_f"
```

{% hint style="info" %}
If you want to run the examples from the Wasmer [repository](https://github.com/wasmerio/wasmer/) codebase directly, you can also do:

```bash
git clone https://github.com/wasmerio/wasmer.git
cd wasmer
cargo run --example errors --release --features "cranelift"
```
{% endhint %}
{% endtab %}

{% tab title="Go" %}
You should be able to run it using the `go run main.go` command. The output should look like this:

```text
Compiling module...
Instantiating module...
Calling `div_by_zero` function...
Error caught from `div_by_zero`: integer divide by zero
  Frame #2: function index: 0
  Frame #1: function index: 50
```

{% hint style="info" %}
If you want to run the examples from the Wasmer [repository](https://github.com/wasmerio/wasmer-go) codebase directly, you can also do:

```text
git clone https://github.com/wasmerio/wasmer-go.git
cd wasmer-go
go test examples/example_errors_test.go
```
{% endhint %}
{% endtab %}
{% endtabs %}


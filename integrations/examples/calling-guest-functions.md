---
description: >-
  A Wasm module can export entities, like functions, memories, globals and
  tables. This example illustrates how to call exported functions.
---

# Calling guest functions

In this example we'll see how to use exported functions.

Exported function are the entities you will probably use the most: they will be your entrypoint to calling Wasm module logic.

Exported function come in two flavors:

* Dynamic functions;
* Native functions.

We'll cover both flavors in this example.

First we are going to want to initialize a new project. To do this we can navigate to our project folder, or create one. In this example, we will create a new project. Lets create it and navigate to it:

{% tabs %}
{% tab title="Rust" %}
{% hint style="info" %}
The final code for this example can be found on [GitHub](https://github.com/wasmerio/wasmer/blob/master/examples/exports_function.rs).

_Please take a look at the_ [_setup steps for Rust_](../rust/setup.md)_._
{% endhint %}

```bash
cargo new exports-function
cd exports-function
```

This should generate two important files for us, `Cargo.toml` and `src/main.rs`. The `Cargo.toml` is a file that describes your project and its dependencies. The `src/main.rs` is the entry point for your project, and contains the `fn main() { .. }` that is run when the project is executed.

We then modify the `Cargo.toml` to add the Wasmer dependencies as shown below:

```rust
[package]
name = "exports-function"
version = "0.1.0"
authors = ["The Wasmer Engineering Team <engineering@wasmer.io>"]
edition = "2018"

[dependencies]
# The Wasmer API
wasmer = "1.0.0-alpha5"
```
{% endtab %}

{% tab title="Go" %}
{% hint style="info" %}
The final code for this example can be found on [GitHub](https://github.com/wasmerio/wasmer-go/blob/master/examples/example_exports_function_test.go).

_Please take a look at the_ [_setup steps for Go_](../go/setup.md)_._
{% endhint %}

```bash
mkdir wasmer-example-exports-function
cd wasmer-example-exports-function
go mod init github.com/$USER/wasmer-example-exports-function
```
{% endtab %}

{% tab title="C/C++" %}
{% hint style="info" %}
The final code for this example can be found on [GitHub](https://github.com/wasmerio/wasmer/blob/master/lib/c-api/examples/instance.c).

_Please take a look at the_ [_setup steps for C/C++_](../c/setup.md)_._
{% endhint %}

```bash
mkdir wasmer-example-exports-function
cd wasmer-example-exports-function
vim Makefile
```

Let's create a simple `Makefile`:

```bash
CFLAGS = -g -I$(WASMER_C_API)/include
LDFLAGS = -L$(WASMER_C_API)/lib -Wl,-rpath,$(WASMER_C_API)/lib
LDLIBS = -lwasmer

.SILENT: exports-function exports-function.o
exports-function: exports-function.o

.PHONY: clean
.SILENT: clean
clean:
	rm -f exports-function.o exports-function
```
{% endtab %}
{% endtabs %}

Now that we have everything set up, let's go ahead and try it out!

## Using the dynamic flavor

We'll start by fetching the guest function and see how to call it using the dynamic flavor. Our Wasm module exports a `sum` function, let's get and call it:

{% tabs %}
{% tab title="Rust" %}
```rust
let sum = instance.exports.get_function("sum")?;
let args = [Value::I32(1), Value::I32(2)];
let result: Box<[Val]> = sum.call(&args)?;
```
{% endtab %}

{% tab title="Go" %}
```go
sum, err := instance.Exports.GetRawFunction("sum")

if err != nil {
  panic(fmt.Sprintf("Failed to get the `%s` function: %s\n", name, err))
}

result, err := sum.Call(1, 2)
```
{% endtab %}

{% tab title="C/C++" %}
```c
wasm_val_t args_val[2] = { WASM_I32_VAL(3), WASM_I32_VAL(4) };
wasm_val_t results_val[1] = { WASM_INIT_VAL };
wasm_val_vec_t args = WASM_ARRAY_VEC(args_val);
wasm_val_vec_t results = WASM_ARRAY_VEC(results_val);

if (wasm_func_call(sum_func, &args, &results)) {
    printf("> Error calling the `sum` function!\n");

    return 1;
}
```
{% endtab %}
{% endtabs %}

Easy right?

Both example look nice but it does not seem like we are using standard functions. In fact, we are calling an external entity. With the native flavor we can get something that feels more like we are using functions as if they were provided by the host directly.

Let's have a look at this.

## Using the native flavor

Let's continue with our previous `sum` function and see how we can make interacting with it better. To do so, we'll be using the native flavor. With this flavor, passing arguments and getting result will feel more natural.

To use this flavor, we have the choice of fetching the function again or transforming the one we already have into a native function:

{% tabs %}
{% tab title="Rust" %}
```rust
let sum = sum.native::<(i32, i32), i32>()?;
let result: i32 = sum.call(3, 4)?;
```

{% hint style="info" %}
Here we reused the previously fetched function and turned it into a native one. We could have directly fetched it as a native function:

```rust
let sum = instance
    .exports
    .get_native_function::<(i32, i32), i32>("sum")?;
```
{% endhint %}
{% endtab %}

{% tab title="Go" %}
```go
sumNative := sum.Native()
result, err = sumNative(3, 4)
```

{% hint style="info" %}
Here we reused the previously fetched function and turned it into a native one. We could have directly fetched it as a native function:

```go
sum, err := instance.Exports.GetFunction("sum")

if err != nil {
  panic(fmt.Sprintf(
    "Failed to get the `%s` function: %s\n", 
    name, 
    err
  ))
}
```
{% endhint %}
{% endtab %}
{% endtabs %}

## Running

We now have everything we need to run the Wasm module, let's do it!

{% tabs %}
{% tab title="Rust" %}
You should be able to run it using the `cargo run` command. The output should look like this:

```text
Compiling module...
Instantiating module...
Calling `sum` function...
Results: [I32(3)]
Calling `sum` function (natively)...
Results: 7
```

{% hint style="info" %}
If you want to run the examples from the Wasmer [repository](https://github.com/wasmerio/wasmer/) codebase directly, you can also do:

```bash
git clone https://github.com/wasmerio/wasmer.git
cd wasmer
cargo run --example exported-function --release --features "cranelift"
```
{% endhint %}
{% endtab %}

{% tab title="Go" %}
You should be able to run it using the `go run main.go` command. The output should look like this:

```text
Compiling module...
Instantiating module...
Calling `sum` function...
Result of the `sum` function: 3
Calling `sum` function (natively)...
Result of the `sum` function: 7
```

{% hint style="info" %}
If you want to run the examples from the Wasmer [repository](https://github.com/wasmerio/wasmer-go) codebase directly, you can also do:

```bash
git clone https://github.com/wasmerio/wasmer-go.git
cd wasmer-go
go test examples/example_exports_function_test.go
```
{% endhint %}
{% endtab %}

{% tab title="C/C++" %}
You should be able to run it using the `make clean exports-function && ./exports-function` command. The output should look like this:

```text
Creating the store...
Compiling module...
Creating imports...
Instantiating module...
Retrieving exports...
Retrieving the `sum` function...
Calling `sum` function...
Results of `sum`: 7
```

{% hint style="info" %}
If you want to run the examples from the Wasmer [repository](https://github.com/wasmerio/wasmer/) codebase directly, you can also do:

```bash
git clone https://github.com/wasmerio/wasmer.git
cd wasmer/lib/c-api/examples/exports-function.c
make clean exports-function
./exports-function
```
{% endhint %}
{% endtab %}
{% endtabs %}


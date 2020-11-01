---
description: >-
  A WASM module can export entities, like functions, memories, globals and
  tables. This example illustrates how to use exported globals.
---

# Using exported globals

In this example we'll be using a simple WASM module which exports some globals.

Globals are probably the simplest entity we'll encounter in WASM modules but there is still some interesting things to talk about. For example, globals come in two flavors:

* Immutable globals \(`const`\)
* Mutable globals \(`var`\)

We will cover both in this example.

First we are going to want to initialize a new project. To do this we can navigate to our project folder, or create one. In this example, we will create a new project. Lets create it and navigate to it:

{% tabs %}
{% tab title="Rust" %}
{% hint style="info" %}
The final code for this example can be found on [GitHub](https://github.com/wasmerio/wasmer/blob/master/examples/instance.rs).

_Please take a look at the_ [_setup steps for Rust_](../rust/setup.md)_._
{% endhint %}

```bash
cargo new exports-global
cd exports-global
```

This should generate two important files for us, `Cargo.toml` and `src/main.rs`. The `Cargo.toml` is a file that describes your project and its dependencies. The `src/main.rs` is the entry point for your project, and contains the `fn main() { .. }` that is run when the project is executed.

We then modify the `Cargo.toml` to add the Wasmer dependencies as shown below:

{% code title="Cargo.toml" %}
```rust
[package]
name = "exports-global"
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
The final code for this example can be found on [GitHub](https://github.com/wasmerio/wasmer/blob/master/examples/instance.rs).

_Please take a look at the_ _setup steps for Go._
{% endhint %}

```text
mkdir wasmer-example-imports-exports
cd wasmer-example-imports-exports
go mod init github.com/$USER/wasmer-example-imports-exports
```
{% endtab %}
{% endtabs %}

Now that we have everything set up, let's go ahead and try it out!

### Querying types information

The first interesting thing to do is to query their type information in order to know if they are mutable or not. Our module exports two globals, `one` and `some` Which one is mutable and which one is not?

{% tabs %}
{% tab title="Rust" %}
```rust
let one = instance.exports.get_global("one")?;
let some = instance.exports.get_global("some")?;

let one_type = one.ty();
let some_type = some.ty();

println!("one type: {:?} {:?}", one_type.mutability, one_type.ty);
println!("some type: {:?} {:?}", some_type.mutability, some_type.ty);
```
{% endtab %}

{% tab title="Go" %}
```go
one := utils.GetGlobal(instance, "one")
some := utils.GetGlobal(instance, "some")

oneType := one.Type()
someType := some.Type()
	
fmt.Printf(
	"`one` type: %s %s\n", 
	oneType.Mutability(), 
	oneType.ValueType().Kind().String()
)
fmt.Printf(
	"`some` type: %s %s\n", 
	someType.Mutability(), 
	someType.ValueType().Kind().String()
)
```

{% hint style="warning" %}
Note that here we used an helper function: utils.GetGlobals. This is just to avoid repeating the boilerplate code required to handle errors.

**This helper function is not part of the Wasmer API.**

If you want to know how to fetch exported globals, have a look at the following example:

{% page-ref page="imports-and-exports.md" %}
{% endhint %}
{% endtab %}
{% endtabs %}

### Getting globals values

The global API is straightforward: it provides a dedicated method to get the value of a given global. Look how easy it is:

{% tabs %}
{% tab title="Rust" %}
```rust
let one_value = one.get();
let some_value = some.get();
```
{% endtab %}

{% tab title="Go" %}
```go
oneValue, err := one.Get()
if err != nil {
  panic(fmt.Sprintln("Failed to get the `one` global value:", err))
}

someValue, err = some.Get()
if err != nil {
  panic(fmt.Sprintln("Failed to get the `some` global value:", err))
}
```
{% endtab %}
{% endtabs %}

### Setting globals 

As we said before, globals come in two flavor. Immutable globals, for which we can only set a value once and mutable ones.

First we'll try to set the value of a immutable global and see what happens:

{% tabs %}
{% tab title="Rust" %}
```rust
let result = one.set(Value::F32(42.0));
assert_eq!(
    result.expect_err("Expected an error").message(),
    "Attempted to set an immutable global"
);
```
{% endtab %}

{% tab title="Go" %}
```go
err = one.Set(float32(42.0), wasmer.F32)

if err == nil {
	panic(fmt.Sprintln("Setting value to `one` did not error"))
}
```
{% endtab %}
{% endtabs %}

As you can see here, trying to set a value on a immutable global will always lead to an error.

Now let's see how to correctly set a value on a mutable global:

{% tabs %}
{% tab title="Rust" %}
```rust
some.set(Value::F32(42.0))?;
let some_result = some.get();
println!("some value after `set`: {:?}", some_result);
```
{% endtab %}

{% tab title="Go" %}
```go
err = some.Set(float32(42.0), wasmer.F32)

if err != nil {
	panic(fmt.Sprintln("Failed to set the `some` global value:", err))
}
```
{% endtab %}
{% endtabs %}

### Running

We now have everything we need to run the WASM module, let's do it!

{% tabs %}
{% tab title="Rust" %}
You should be able to run it using the `cargo run` command. The output should look like this:

```text
Compiling module...
Instantiating module...
Getting globals types information...
one type: Const F32
some type: Var F32
Getting global values...
one value: 1.0
some value: F32(0.0)
Setting global values...
one value after `set`: F32(1.0)
some value after `set_some`: F32(21.0)
some value after `set`: F32(42.0)
```

{% hint style="info" %}
If you want to run the examples from the Wasmer [repository](https://github.com/wasmerio/wasmer/) codebase directly, you can also do:

```bash
git clone https://github.com/wasmerio/wasmer.git
cd wasmer
cargo run --example exported-global --release --features "cranelift"
```
{% endhint %}
{% endtab %}

{% tab title="Go" %}
You should be able to run it using the `go run main.go` command. The output should look like this:

```text
Compiling module...
Instantiating module...
Getting globals types information...
`one` type: const f32
`some` type: var f32
Getting global values...
`one` value: 1.0
`some` value: 0.0
Setting global values...
`one` value: 1.0
`some` value after `set_some`: 21.0
`some` value after `set`: 42.0
```

{% hint style="info" %}
If you want to run the examples from the Wasmer [repository](https://github.com/wasmerio/wasmer/) codebase directly, you can also do:

```text
git clone https://github.com/wasmerio/wasmer-go.git
cd wasmer-go
go test examples/example_exports_global_test.go
```
{% endhint %}
{% endtab %}
{% endtabs %}


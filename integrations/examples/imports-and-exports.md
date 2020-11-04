---
description: >-
  A Wasm module can import and export entities, like functions, memories,
  globals and tables. This example illustrates the basics of using these
  entities.
---

# Imports & exports

In this example we'll be using a sample Wasm module which exports some entities and requires us to also import some of them.

The goal here is to give you an idea of how to work with imports and exports. We won't go into the details of each entities, they'll be covered in more details in the other examples.

First we are going to want to initialize a new project. To do this we can navigate to our project folder, or create one. In this example, we will create a new project. Lets create it and navigate to it:

{% tabs %}
{% tab title="Rust" %}
{% hint style="info" %}
_Please take a look at the_ [_setup steps for Rust_](../rust/setup.md)_._
{% endhint %}

```bash
cargo new imports-exports
cd imports-exports
```

This should generate two important files for us, `Cargo.toml` and `src/main.rs`. The `Cargo.toml` is a file that describes your project and its dependencies. The `src/main.rs` is the entry point for your project, and contains the `fn main() { .. }` that is run when the project is executed.

We then modify the `Cargo.toml` to add the Wasmer dependencies as shown below:

{% code title="Cargo.toml" %}
```rust
[package]
name = "imports-exports"
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
The final code for this example can be found on [GitHub](https://github.com/wasmerio/wasmer-go/blob/master/examples/example_imports_exports_test.go).

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

## Declaring imports

When a Wasm modules declares imports you will have to make them available before you can instantiate the module. Our first task will be to create the required entities.

The module we are using needs two imports:

* A function named `host_function` in a namespace with an empty name;
* A global named `host_global` in the `env` namespace.

Let's create the import object:

{% tabs %}
{% tab title="Rust" %}
```rust
let import_object = imports! {
    "" => {
        "host_function" => host_function,
    },
    "env" => {
        "host_global" => host_global,
    }
}
```
{% endtab %}

{% tab title="Go" %}
```go
importObject := wasmer.NewImportObject()

importObject.Register(
    "",
    map[string]wasmer.IntoExtern{
        "host_function": hostFunction,
    },
)

importObject.Register(
    "env",
    map[string]wasmer.IntoExtern{
        "host_global":   hostGlobal,
    },
)
```
{% endtab %}
{% endtabs %}

Now that we have our import object ready, we'll need to use it when instantiating the module:

{% tabs %}
{% tab title="Rust" %}
```rust
let instance = Instance::new(&module, &import_object)?;
```
{% endtab %}

{% tab title="Go" %}
```go
instance, err := wasmer.NewInstance(module, importObject)
```
{% endtab %}
{% endtabs %}

That's it! Easy right?

We did not go into the details of how to create the imported entities, we encourage you to read other examples to know more about this:

{% page-ref page="host-functions.md" %}

## Fetching exports

Let's have a look at our module again: it exports some entities for us in our host program:

* A function named `guest_function`;
* A global name `guest_global`;
* A table name `guest_table`;
* A memory named `guest_memory`.

To get these entities we'll use the exports API:

{% tabs %}
{% tab title="Rust" %}
{% hint style="warning" %}
TODO: Write this section
{% endhint %}
{% endtab %}

{% tab title="Go" %}
```go
function, err := instance.Exports.GetFunction("guest_function")
if err != nil {
  panic(fmt.Sprintln("Failed to get the exported function:", err))
}

global, err := instance.Exports.GetGlobal("guest_global")
if err != nil {
    panic(fmt.Sprintln("Failed to get the exported global:", err))
}

memory, err := instance.Exports.GetMemory("guest_memory")
if err != nil {
    panic(fmt.Sprintln("Failed to get the exported memory:", err))
}

table, err := instance.Exports.GetTable("guest_table")
if err != nil {
    panic(fmt.Sprintln("Failed to get the exported table:", err))
}
```
{% endtab %}
{% endtabs %}

Again, we'll not cover how to use these entities here as this is the topic of other, more detailed, examples:

{% page-ref page="calling-guest-functions.md" %}

{% page-ref page="host-functions.md" %}

{% page-ref page="using-exported-globals.md" %}

{% page-ref page="memory.md" %}

## Running

We now have everything we need to run the Wasm module, let's do it!

{% tabs %}
{% tab title="Rust" %}
You should be able to run it using the `cargo run` command. The output should look like this:

{% hint style="warning" %}
TODO: Write this section
{% endhint %}
{% endtab %}

{% tab title="Go" %}
You should be able to run it using the `go run main.go` command. The output should look like this:

```text
Compiling module...
Creating the imported function...
Creating the imported global...
Instantiating module...
Getting the exported function...
Got the exported function: func(...interface {}) (interface {}, error)
Getting the exported global...
Got the exported global: *wasmer.Global
Getting the exported memory...
Got the exported memory: *wasmer.Memory
Getting the exported table...
Got the exported table: *wasmer.Table
```

{% hint style="info" %}
If you want to run the examples from the Wasmer [repository](https://github.com/wasmerio/wasmer/) codebase directly, you can also do:

```text
git clone https://github.com/wasmerio/wasmer-go.git
cd wasmer-go
go test examples/example_imports_exports_test.go
```
{% endhint %}
{% endtab %}
{% endtabs %}


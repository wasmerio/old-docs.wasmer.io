---
description: >-
  This example illustrates the basics of using Wasmer through a "Hello
  World"-like project.
---

# Instantiating a Wasm module

In this example we will be building a "Hello World"-like project. WebAssembly only supports passing integers and floats directly right now, thus to keep it simple we will be writing a host application that calls the `add_one` function of a guest Wasm module, which adds `1` to the value passed as a parameter, and returns the result.

The goal here is to show you the basics of using Wasmer, we'll focus on the steps required to get an instance out of a Wasm module.

First we are going to want to initialize a new project. To do this we can navigate to our project folder, or create one. In this example, we will create a new project. Lets create it and navigate to it:

{% tabs %}
{% tab title="Rust" %}
{% hint style="info" %}
The final code for this example can be found on [GitHub](https://github.com/wasmerio/wasmer/blob/master/examples/instance.rs).

_Please take a look at the_ [_setup steps for Rust_](../rust/setup.md)_._
{% endhint %}

```bash
cargo new instance
cd instance
```

This should generate two important files for us, `Cargo.toml` and `src/main.rs`. The `Cargo.toml` is a file that describes your project and its dependencies. The `src/main.rs` is the entry point for your project, and contains the `fn main() { .. }` that is run when the project is executed.

We then modify the `Cargo.toml` to add the Wasmer dependencies as shown below:

```rust
[package]
name = "instance"
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
The final code for this example can be found on [GitHub](https://github.com/wasmerio/wasmer-go/blob/master/examples/example_instance_test.go).

_Please take a look at the_ [_setup steps for Go_](../go/setup.md)_._
{% endhint %}

```bash
mkdir wasmer-example-instance
cd wasmer-example-instance
go mod init github.com/$USER/wasmer-example-instance
```
{% endtab %}

{% tab title="C/C++" %}
{% hint style="info" %}
The final code for this example can be found on [GitHub](https://github.com/wasmerio/wasmer/blob/master/lib/c-api/examples/instance.c).

_Please take a look at the_ [_setup steps for C/C++_](../c/setup.md)_._
{% endhint %}

```bash
mkdir wasmer-example-instance
cd wasmer-example-instance
vim Makefile
```

Let's create a simple `Makefile`:

```bash
CFLAGS = -g -I$(shell $(WASMER_C_API)/bin/wasmer config --includedir)
LDFLAGS = -Wl,-rpath,$(shell $(WASMER_C_API)/bin/wasmer config --libdir)
LDLIBS = $(shell $(WASMER_C_API)/bin/wasmer config --libs)

.SILENT: instance instance.o
instance: instance.o

.PHONY: clean
.SILENT: clean
clean:
	rm -f instance.o instance
```

Wasmer C API includes two header files: `wasm.h` and `wasmer_wasm.h`. The first one provides the standard Wasm C API while the second one adds some useful functions like `wat2wasm` and many others.
{% endtab %}
{% endtabs %}

Now that we have everything set up, let's go ahead and try it out!

## Loading the Wasm module

The first step will be to load the Wasm module we want to use. This is done by having its contents loaded as bytes:

{% tabs %}
{% tab title="Rust" %}
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

{% hint style="info" %}
Here we are using the text representation of the Wasm module. Wasmer wants to have a binary representation of the module so we have to use `wat2wasm` to do the translation.
{% endhint %}
{% endtab %}

{% tab title="Go" %}
```go
wasmBytes := []byte(`
(module
  (type $add_one_t (func (param i32) (result i32)))
  (func $add_one_f (type $add_one_t) (param $value i32) (result i32)
    local.get $value
    i32.const 1
    i32.add)
  (export "add_one" (func $add_one_f)))
`)
```
{% endtab %}

{% tab title="C/C++" %}
```c
const char *wat_string =
    "(module\n"
    "  (type $add_one_t (func (param i32) (result i32)))\n"
    "  (func $add_one_f (type $add_one_t) (param $value i32) (result i32)\n"
    "    local.get $value\n"
    "    i32.const 1\n"
    "    i32.add)\n"
    "  (export \"add_one\" (func $add_one_f)))";

wasm_byte_vec_t wat;
wasm_byte_vec_new(&wat, strlen(wat_string), wat_string);
wasm_byte_vec_t* wasm_bytes = wat2wasm(&wat);
```
{% endtab %}
{% endtabs %}

Let's assume we have the binary version of the module \(i.e the `.wasm` file\), here is how we would have loaded it:

{% tabs %}
{% tab title="Rust" %}
```rust
let wasm_bytes = std::fs::read("./path/to/module.wasm")?;
```
{% endtab %}

{% tab title="Go" %}
```go
wasmBytes, err := ioutil.ReadFile("./path/to/module.wasm")
```
{% endtab %}

{% tab title="C/C++" %}
```c
FILE* file = fopen("module.wasm", "rb");

if (!file) {
  printf("> Error loading module!\n");

  return 1;
}

fseek(file, 0L, SEEK_END);
size_t file_size = ftell(file);
fseek(file, 0L, SEEK_SET);

wasm_byte_vec_t wasm_bytes;
wasm_byte_vec_new_uninitialized(&wasm_bytes, file_size);

if (fread(wasm_bytes.data, file_size, 1, file) != 1) {
  printf("> Error loading module!\n");

  return 1;
}

fclose(file);
```
{% endtab %}
{% endtabs %}

## Compiling the Wasm module

The next step will be to compile the module. To do this, we'll need two things: the Wasm module as bytes and a `Store`.

The `Store` is a representation of the actual state of the module: it represents the state of every entities in the module during its lifecycle. It also holds the engine which is what will be used to actually compile the module.

Here is how we can create the store and compile the module:

{% tabs %}
{% tab title="Rust" %}
```rust
let store = Store::default();
let module = Module::new(&store, wasm_bytes)?;
```

{% hint style="info" %}
We are creating a store using the default settings provided by Wasmer. In some cases, you may want to use a specific engine or compiler. Here is how you would do:

```rust
let engine = JIT::new(&Cranelift::default()).engine();
let store = Store::new(&engine);
let module = Module::new(&store, wasm_bytes)?;
```

We created a store with the JIT engine and the Cranelift compiler with its default configuration. These are good defaults but it will be a good thing to adapt this configuration to your needs.
{% endhint %}
{% endtab %}

{% tab title="Go" %}
```go
engine := wasmer.NewEngine()
store := wasmer.NewStore(engine)
module, err := wasmer.NewModule(store, wasmBytes)

if err != nil {
  fmt.Println("Failed to compile module:", err)
}
```
{% endtab %}

{% tab title="C/C++" %}
```c
wasm_engine_t* engine = wasm_engine_new();
wasm_store_t* store = wasm_store_new(engine);
wasm_module_t* module = wasm_module_new(store, wasm_bytes);

if (!module) {
    printf("> Error compiling module!\n");

    return 1;
}
```
{% endtab %}
{% endtabs %}

## Creating an instance of the module

We are now close to having the module run in our Rust host.

The last step will be to create an `Instance` out of the Wasm module. As for the previous step, here we need more than just the compiled module: we also need to define imports.

In fact, Wasm modules can define entities they need to work properly. These are called imports. In this example we don't need any of them but we still need to define an empty set and use it to instantiate the module:

{% tabs %}
{% tab title="Rust" %}
```rust
let import_object = imports! {};
let instance = Instance::new(&module, &import_object)?;
```
{% endtab %}

{% tab title="Go" %}
```go
importObject := wasmer.NewImportObject()
instance, err := wasmer.NewInstance(module, importObject)
```
{% endtab %}

{% tab title="C/C++" %}
```c
wasm_extern_vec_t imports = WASM_EMPTY_VEC;
wasm_instance_t* instance = wasm_instance_new(store, module, &imports, NULL);

if (!instance) {
  printf("> Error instantiating module %d!\n");

  return 1;
}
```
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
Calling `add_one` function...
Results of `add_one`: 2
```

{% hint style="info" %}
If you want to run the examples from the Wasmer [repository](https://github.com/wasmerio/wasmer/) codebase directly, you can also do:

```bash
git clone https://github.com/wasmerio/wasmer.git
cd wasmer
cargo run --example instance --release --features "cranelift"
```
{% endhint %}
{% endtab %}

{% tab title="Go" %}
You should be able to run it using the `go run main.go` command. The output should look like this:

```text
Compiling module...
Instantiating module...
Calling `add_one` function...
Results of `add_one`: 2
```

{% hint style="info" %}
If you want to run the examples from the Wasmer [repository](https://github.com/wasmerio/wasmer/) codebase directly, you can also do:

```bash
git clone https://github.com/wasmerio/wasmer-go.git
cd wasmer-go
go test examples/example_instance_test.go
```
{% endhint %}
{% endtab %}

{% tab title="C/C++" %}
You should be able to run it using the `make clean instance && ./instance` command. The output should look like this:

```text
Creating the store...
Compiling module...
Creating imports...
Instantiating module...
Retrieving exports...
Calling `add_one` function...
Results of `add_one`: 2
```

{% hint style="info" %}
If you want to run the examples from the Wasmer [repository](https://github.com/wasmerio/wasmer/) codebase directly, you can also do:

```bash
git clone https://github.com/wasmerio/wasmer.git
cd wasmer/lib/c-api/examples/instance.c
make clean instance
./instance
```
{% endhint %}
{% endtab %}
{% endtabs %}


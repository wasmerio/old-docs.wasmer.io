---
description: >-
  A Wasm module can import and export entities, like functions, memories,
  globals and tables. This example illustrates the basics of using these
  entities.
---

# 🔁 Imports & exports

In this example we'll be using a sample Wasm module which exports some entities and requires us to also import some of them.

The goal here is to give you an idea of how to work with imports and exports. We won't go into the details of each entities, they'll be covered in more details in the other examples.

First we are going to want to initialize a new project. To do this we can navigate to our project folder, or create one. In this example, we will create a new project. Lets create it and navigate to it:

{% tabs %}
{% tab title="Rust" %}
{% hint style="info" %}
The final code for this example can be found on [GitHub](https://github.com/wasmerio/wasmer/blob/master/examples/imports_exports.rs).

_Please take a look at the_ [_setup steps for Rust_](../rust/setup.md)_._
{% endhint %}

```bash
cargo new imports-exports
cd imports-exports
```

We have to modify `Cargo.toml` to add the Wasmer dependencies as shown below:

```yaml
[dependencies]
# The Wasmer API
wasmer = "1.0"
```
{% endtab %}

{% tab title="Go" %}
{% hint style="info" %}
The final code for this example can be found on [GitHub](https://github.com/wasmerio/wasmer-go/blob/master/examples/example_imports_exports_test.go).

_Please take a look at the_ [_setup steps for Go_](../go/setup.md)_._
{% endhint %}

```bash
mkdir wasmer-example-imports-exports
cd wasmer-example-imports-exports
go mod init github.com/$USER/wasmer-example-imports-exports
```
{% endtab %}

{% tab title="Python" %}
{% hint style="info" %}
The final code for this example can be found on [GitHub](https://github.com/wasmerio/wasmer-python/blob/master/examples/imports_exports.go).

_Please take a look at the_ [_setup steps for Python_](../python/setup.md)_._
{% endhint %}

```bash
mkdir wasmer-example-imports-exports
cd wasmer-example-imports-exports
pip install wasmer
pip install wasmer_compiler_cranelift
```
{% endtab %}

{% tab title="C/C++" %}
{% hint style="info" %}
The final code for this example can be found on [GitHub](https://github.com/wasmerio/wasmer/blob/master/lib/c-api/examples/instance.c).

_Please take a look at the_ [_setup steps for C/C++_](../c/setup.md)_._
{% endhint %}

```bash
mkdir wasmer-example-imports-exports
cd wasmer-example-imports-exports
vim Makefile
```

Let's create a simple `Makefile`:

```bash
CFLAGS = -g -I$(shell $(WASMER_DIR)/bin/wasmer config --includedir)
LDFLAGS = -Wl,-rpath,$(shell $(WASMER_DIR)/bin/wasmer config --libdir)
LDLIBS = $(shell $(WASMER_DIR)/bin/wasmer config --libs)

.SILENT: imports-exports imports-exports.o
imports-exports: imports-exports.o

.PHONY: clean
.SILENT: clean
clean:
    rm -f imports-exports.o imports-exports
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
    },
};
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

{% tab title="Python" %}
```python
import_object = ImportObject()
import_object.register(
    "",
    {
        "host_function": host_function,
    }
)
import_object.register(
    "env",
    {
        "host_global": host_global,
    }
)
```
{% endtab %}

{% tab title="C/C++" %}
```c
wasm_functype_t* host_func_type = wasm_functype_new_0_1(wasm_valtype_new_i32());
wasm_func_t* host_func = wasm_func_new(store, host_func_type, host_func_callback);
wasm_functype_delete(host_func_type);

wasm_globaltype_t* host_global_type = wasm_globaltype_new(wasm_valtype_new(WASM_F32), WASM_CONST);
wasm_val_t host_global_val = WASM_I32_VAL(42);
wasm_global_t* host_global = wasm_global_new(store, host_global_type, &host_global_val);
wasm_globaltype_delete(host_global_type);

wasm_extern_t* externs[] = {
  wasm_func_as_extern(host_func),
  wasm_global_as_extern(host_global)
};

wasm_extern_vec_t import_object = WASM_ARRAY_VEC(externs);
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

{% tab title="Python" %}
```go
instance = Instance(module, import_object)
```
{% endtab %}

{% tab title="C/C++" %}
```c
wasm_instance_t* instance = wasm_instance_new(store, module, &import_object, NULL);
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
```rust
let function = instance.exports.get::<Function>("guest_function")?;

let global = instance.exports.get::<Global>("guest_global")?;

let memory = instance.exports.get::<Memory>("guest_memory")?;

let table = instance.exports.get::<Table>("guest_table")?;
```
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

table, err := instance.Exports.GetTable("guest_table")
if err != nil {
    panic(fmt.Sprintln("Failed to get the exported table:", err))
}

memory, err := instance.Exports.GetMemory("guest_memory")
if err != nil {
    panic(fmt.Sprintln("Failed to get the exported memory:", err))
}
```
{% endtab %}

{% tab title="Python" %}
```python
function = instance.exports.guest_function
global = instance.exports.guest_global
memory = instance.exports.guest_memory
table = instance.exports.guest_table
```
{% endtab %}

{% tab title="C/C++" %}
```c
wasm_func_t* func = wasm_extern_as_func(exports.data[0]);
if (func == NULL) {
    printf("> Failed to get the exported function!\n");

    return 1;
}

wasm_global_t* global = wasm_extern_as_global(exports.data[1]);
if (global == NULL) {
    printf("> Failed to get the exported global!\n");

    return 1;
}

wasm_table_t* table = wasm_extern_as_table(exports.data[2]);
if (table == NULL) {
    printf("> Failed to get the exported table!\n");

    return 1;
}

wasm_memory_t* memory = wasm_extern_as_memory(exports.data[3]);
if (memory == NULL) {
    printf("> Failed to get the exported memory!\n");

    return 1;
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

```text
Compiling module...
Creating the imported function...
Creating the imported global...
Instantiating module...
Getting the exported function...
Got exported function of type: FunctionType { params: [], results: [I32] }
Getting the exported global...
Got exported global of type: GlobalType { ty: I32, mutability: Const }
Getting the exported memory...
Got exported memory of type: MemoryType { minimum: 1 pages, maximum: None, shared: false }
Getting the exported table...
Got exported table of type: TableType { ty: FuncRef, minimum: 1, maximum: Some(1) }
```

{% hint style="info" %}
If you want to run the examples from the Wasmer [repository](https://github.com/wasmerio/wasmer/) codebase directly, you can also do:

```bash
git clone https://github.com/wasmerio/wasmer.git
cd wasmer
cargo run --example imports-exports --release --features "cranelift"
```
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
If you want to run the examples from the Wasmer [repository](https://github.com/wasmerio/wasmer-go) codebase directly, you can also do:

```bash
git clone https://github.com/wasmerio/wasmer-go.git
cd wasmer-go
go test examples/example_imports_exports_test.go
```
{% endhint %}
{% endtab %}

{% tab title="Python" %}
You should be able to run it using the `python main.py` command.

{% hint style="info" %}
If you want to run the examples from the Wasmer [repository](https://github.com/wasmerio/wasmer-python) codebase directly, you can also do:

```bash
git clone https://github.com/wasmerio/wasmer-python.git
cd wasmer-python
just prelude
source .env/bin/activate
just build-all $target
python examples/imports_exports.py
```
{% endhint %}
{% endtab %}

{% tab title="C/C++" %}
You should be able to run it using the `make clean imports-exports && ./imports-exports` command. The output should look like this:

```text
Creating the store...
Compiling module...
Creating the imported function...
Creating the imported global...
Instantiating module...
Retrieving exports...
Retrieving the exported function...
Got the exported function: 0x7f9317e05e00
Retrieving the exported global...
Got the exported global: 0x7f9317e05e90
Retrieving the exported table...
Got the exported table: 0x7f9317e05ec0
Retrieving the exported memory...
Got the exported memory: 0x7f9317e05ef0
```

{% hint style="info" %}
If you want to run the examples from the Wasmer [repository](https://github.com/wasmerio/wasmer/) codebase directly, you can also do:

```bash
git clone https://github.com/wasmerio/wasmer.git
cd wasmer/lib/c-api/examples/imports-exports.c
make clean imports-exports
./imports-exports
```
{% endhint %}
{% endtab %}
{% endtabs %}


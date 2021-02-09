---
description: >-
  This example illustrates the basics of the basics of interacting with Wasm
  module memory.
---

# 💾 Interacting with memory

A Wasm module can export its memory. With Wasmer you'll be able to interact with this memory.

In this example we'll illustrate the basics of interacting with the module memory:

* How to query information about the memory;
* How to load read from the memory;
* How to write to the memory.

There are mainly two ways of interacting with the memory, either through exported function or by calling the memory API directly. It really depends on how the memory is exported.

First we are going to want to initialize a new project. To do this we can navigate to our project folder, or create one. In this example, we will create a new project. Lets create it and navigate to it:

{% tabs %}
{% tab title="Rust" %}
{% hint style="info" %}
The final code for this example can be found on [GitHub](https://github.com/wasmerio/wasmer/blob/master/examples/memory.rs).

_Please take a look at the_ [_setup steps for Rust_](../rust/setup.md)_._
{% endhint %}

```bash
cargo new memory
cd memory
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
The final code for this example can be found on [GitHub](https://github.com/wasmerio/wasmer-go/blob/master/examples/example_memory_test.go).

_Please take a look at the_ [_setup steps for Go_](../go/setup.md)_._
{% endhint %}

```bash
mkdir wasmer-example-memory
cd wasmer-example-memory
go mod init github.com/$USER/wasmer-example-memory
```
{% endtab %}

{% tab title="C/C++" %}
{% hint style="info" %}
The final code for this example can be found on [GitHub](https://github.com/wasmerio/wasmer/blob/master/lib/c-api/examples/memory.c).

_Please take a look at the_ [_setup steps for C/C++_](../c/setup.md)_._
{% endhint %}

```bash
mkdir wasmer-example-memory
cd wasmer-example-memory
vim Makefile
```

Let's create a simple `Makefile`:

```bash
CFLAGS = -g -I$(shell $(WASMER_DIR)/bin/wasmer config --includedir)
LDFLAGS = -Wl,-rpath,$(shell $(WASMER_DIR)/bin/wasmer config --libdir)
LDLIBS = $(shell $(WASMER_DIR)/bin/wasmer config --libs)

.SILENT: memory memory.o
memory: memory.o

.PHONY: clean
.SILENT: clean
clean:
    rm -f memory.o memory
```
{% endtab %}
{% endtabs %}

Now that we have everything set up, let's go ahead and try it out!

## Querying memory information

The first interesting thing to do is to query information about the memory. To do that we must either have access to the memory \(i.e it has to be exported\) or we must have access to an exported function which is able to give us this information.

{% hint style="info" %}
One important thing to note: the size of the memory can be expressed as a number of pages or a number of bytes.

Each page of memory is 64 KiB in size.
{% endhint %}

{% tabs %}
{% tab title="Rust" %}
```rust
let mem_size: NativeFunc<(), i32> = instance
    .exports
    .get_native_function("mem_size")?;
let memory = instance.exports.get_memory("memory")?;

assert_eq!(memory.size(), Pages::from(1));
assert_eq!(memory.size().bytes(), Bytes::from(65536 as usize));
assert_eq!(memory.data_size(), 65536);

let result = mem_size.call()?;
assert_eq!(Pages::from(result as u32), memory.size());
```
{% endtab %}

{% tab title="Go" %}
```go
memSize, err := instance.Exports.GetFunction("mem_size")

if err != nil {
    panic(fmt.Sprintln("Failed to retrieve the `mem_size` function:", err))
}

memory, err := instance.Exports.GetMemory("memory")

if err != nil {
    panic(fmt.Sprintln("Failed to get the `memory` memory:", err))
}

size := memory.Size()
fmt.Println("Memory size (pages):", size)
fmt.Println("Memory size (pages as bytes):", size.ToBytes())
fmt.Println("Memory size (bytes):", memory.DataSize())

result, err := memSize()

if err != nil {
    panic(fmt.Sprintln("Failed to call the `mem_size` function:", err))
}

fmt.Println("Memory size (pages):", result)
```
{% endtab %}

{% tab title="C/C++" %}
```c
wasm_memory_pages_t pages = wasm_memory_size(memory);
size_t data_size = wasm_memory_data_size(memory);

printf("Memory size (pages): %d\n", pages);
printf("Memory size (bytes): %d\n", (int) data_size);
```
{% endtab %}
{% endtabs %}

Now that we know the size of our memory it's time to see how we can change this.

## Growing the memory

A memory can be grown to allow storing more things into it. This is easily done through

{% tabs %}
{% tab title="Rust" %}
```rust
memory.grow(2)?;
assert_eq!(memory.size(), Pages::from(3));
assert_eq!(memory.data_size(), 65536 * 3);
```
{% endtab %}

{% tab title="Go" %}
```go
memory.Grow(2)
fmt.Println("New memory size (pages):", memory.Size())
```
{% endtab %}

{% tab title="C/C++" %}
```c
if (!wasm_memory_grow(memory, 2)) {
    printf("> Error growing memory!\n");

    return 1;
}

wasm_memory_pages_t new_pages = wasm_memory_size(memory);
printf("New memory size (pages): %d\n", new_pages);
```
{% endtab %}
{% endtabs %}

To grow a memory you have to call the dedicated method and provide the number of pages, called the delta, you want to add to the memory.

## Reading from and writing to the memory

Now that we know how to query and adjust the size of the memory, let's see how we can write to it or read from it.

We'll only focus on how to do this using exported functions, the goal is to show how to work with memory addresses.

Let's start by using absolute memory addresses to write and read a value.

{% tabs %}
{% tab title="Rust" %}
```rust
let get_at: NativeFunc<i32, i32> = instance
    .exports
    .get_native_function("get_at")?;
let set_at: NativeFunc<(i32, i32), ()> = instance
    .exports
    .get_native_function("set_at")?;

let mem_addr = 0x2220;
let val = 0xFEFEFFE;

set_at.call(mem_addr, val)?;

let result = get_at.call(mem_addr)?;
println!("Value at {:#x?}: {:?}", mem_addr, result);
```
{% endtab %}

{% tab title="Go" %}
```go
getAt, err := instance.Exports.GetFunction("get_at")

if err != nil {
    panic(fmt.Sprintln("Failed to retrieve the `get_at` function:", err))
}

setAt, err := instance.Exports.GetFunction("set_at")

if err != nil {
    panic(fmt.Sprintln("Failed to retrieve the `set_at` function:", err))
}

memAddr := 0x2220
val := 0xFEFEFFE
_, err = setAt(memAddr, val)

if err != nil {
    panic(fmt.Sprintln("Failed to call the `set_at` function:", err))
}

result, err = getAt(memAddr)

if err != nil {
    panic(fmt.Sprintln("Failed to call the `get_at` function:", err))
}

fmt.Printf("Value at 0x%x: %d\n", memAddr, result)
```
{% endtab %}

{% tab title="C/C++" %}
```c
wasm_func_t* get_at = wasm_extern_as_func(exports.data[0]);
wasm_func_t* set_at = wasm_extern_as_func(exports.data[1]);

int mem_addr = 0x2220;
int val = 0xFEFEFFE;

wasm_val_t set_at_args_val[2] = { WASM_I32_VAL(mem_addr), WASM_I32_VAL(val) };
wasm_val_vec_t set_at_args = WASM_ARRAY_VEC(set_at_args_val);
wasm_val_vec_t set_at_results = WASM_EMPTY_VEC;
wasm_func_call(set_at, &set_at_args, &set_at_results);

wasm_val_t get_at_args_val[1] = { WASM_I32_VAL(mem_addr) };
wasm_val_vec_t get_at_args = WASM_ARRAY_VEC(get_at_args_val);
wasm_val_t get_at_results_val[1] = { WASM_INIT_VAL };
wasm_val_vec_t get_at_results = WASM_ARRAY_VEC(get_at_results_val);
wasm_func_call(get_at, &get_at_args, &get_at_results);

printf("Value at 0x%04x: %d\n", mem_addr, get_at_results_val[0].of.i32);
```
{% endtab %}
{% endtabs %}

Now assume we want to write a value at the end of the second memory page and the read it. Let's see how we can do that:

{% tabs %}
{% tab title="Rust" %}
```rust
let page_size = 0x1_0000;
let mem_addr = (page_size * 2) - mem::size_of_val(&val) as i32;
let val = 0xFEA09;
set_at.call(mem_addr, val)?;

let result = get_at.call(mem_addr)?;
println!("Value at {:#x?}: {:?}", mem_addr, result);
```
{% endtab %}

{% tab title="Go" %}
```go
pageSize := 0x1_0000
memAddr = (pageSize * 2) - int(unsafe.Sizeof(val))
val = 0xFEA09
_, err = setAt(memAddr, val)

if err != nil {
    panic(fmt.Sprintln("Failed to call the `set_at` function:", err))
}

result, err = getAt(memAddr)

if err != nil {
    panic(fmt.Sprintln("Failed to call the `get_at` function:", err))
}

fmt.Printf("Value at 0x%x: %d\n", memAddr, result)
```
{% endtab %}
{% endtabs %}

As you can see here we can use the size of a page and the size of the value we want to write to compute an address and write to it.

{% hint style="info" %}
Keep in mind that memory address can be any number you want as long as it is valid regarding the memory size.

In the previous examples, we used hexadecimal notation but you are free to use decimal notation if needed.
{% endhint %}

It enough for now: we only covered how to interact with the memory through exported functions. If you want to know more, see the following example:

{% page-ref page="memory-pointers.md" %}

## Running

We now have everything we need to run the Wasm module, let's do it!

{% tabs %}
{% tab title="Rust" %}
You should be able to run it using the `cargo run` command. The output should look like this:

```text
Compiling module...
Instantiating module...
Querying memory size...
Memory size: 1
Growing memory...
Value at 0x2220: 267382782
Value at 0x1fffc: 1042953
```

{% hint style="info" %}
If you want to run the examples from the Wasmer [repository](https://github.com/wasmerio/wasmer/) codebase directly, you can also do:

```bash
git clone https://github.com/wasmerio/wasmer.git
cd wasmer
cargo run --example memory --release --features "cranelift"
```
{% endhint %}
{% endtab %}

{% tab title="Go" %}
You should be able to run it using the `go run main.go` command. The output should look like this:

```text
Compiling module...
Instantiating module...
Querying memory size...
Memory size (pages): 1
Memory size (pages as bytes): 65536
Memory size (bytes): 65536
Memory size (pages): 1
Growing memory...
New memory size (pages): 3
Value at 0x2220: 267382782
Value at 0x1fff8: 1042953
```

{% hint style="info" %}
If you want to run the examples from the Wasmer [repository](https://github.com/wasmerio/wasmer/) codebase directly, you can also do:

```bash
git clone https://github.com/wasmerio/wasmer-go.git
cd wasmer-go
go test examples/example_memory_test.go
```
{% endhint %}
{% endtab %}

{% tab title="C/C++" %}
You should be able to run it using the `make clean memory && ./memory` command. The output should look like this:

```text
Creating the store...
Compiling module...
Creating imports...
Instantiating module...
Retrieving exports...
Querying memory size...
Memory size (pages): 1
Memory size (bytes): 65536
Growing memory...
New memory size (pages): 3
Value at 0x2220: 267382782
```

{% hint style="info" %}
If you want to run the examples from the Wasmer [repository](https://github.com/wasmerio/wasmer/) codebase directly, you can also do:

```bash
git clone https://github.com/wasmerio/wasmer.git
cd wasmer/lib/c-api/examples/memory
make clean memory
./memory
```
{% endhint %}
{% endtab %}
{% endtabs %}


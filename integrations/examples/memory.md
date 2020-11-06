---
description: >-
  This example illustrates the basics of the basics of interacting with WASM
  module memory.
---

# Interacting with memory

A WASM module can export its memory. With Wasmer you'll be able to interact with this memory.

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

This should generate two important files for us, `Cargo.toml` and `src/main.rs`. The `Cargo.toml` is a file that describes your project and its dependencies. The `src/main.rs` is the entry point for your project, and contains the `fn main() { .. }` that is run when the project is executed.

We then modify the `Cargo.toml` to add the Wasmer dependencies as shown below:

{% code title="Cargo.toml" %}
```rust
[package]
name = "memory"
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
The final code for this example can be found on [GitHub](https://github.com/wasmerio/wasmer-go/blob/master/examples/example_memory_test.go).

_Please take a look at the_ [_setup steps for Go_](../go/setup.md)_._
{% endhint %}

```text
mkdir wasmer-example-memory
cd wasmer-example-memory
go mod init github.com/$USER/wasmer-example-memory
```
{% endtab %}
{% endtabs %}

Now that we have everything set up, let's go ahead and try it out!

## Querying memory information

The first interesting thing to do is to query information about the memory. To do that we must either have access to the memory \(i.e it has to be exported\) or we must have access to an exported function which is able to give us this information.

{% hint style="info" %}
One important thing to note: the size of the memory can be expressed as a number of mages or a number of bytes.

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
memSize := utils.GetFunction(instance, "mem_size")
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

{% hint style="warning" %}
Note that here we used an helper function: `utils.GetFunction`. This is just to avoid repeating the boilerplate code required to handle errors.

**This helper function is not part of the Wasmer API.**

If you want to know how to fetch exported globals, have a look at the following example:

{% page-ref page="imports-and-exports.md" %}
{% endhint %}
{% endtab %}
{% endtabs %}

Now that we know the size of our memory it's time to see how we can change this.

## Growing the memory

A memory can be grown to allow storing more things into it. This is easily done through

{% tabs %}
{% tab %}

{% endtab %}
{% endtabs %}

To grow a memory you have to call the dedicated method and provide the number of pages, called the delta, you want to add to the memory.

## Reading from and writing to the memory

Now that we know how to query and adjust the size of the memory, let's see how we can write to it or read from it.

We'll only focus on how to do this using exported functions, the goal is to show how to work with memory addresses.

Let's start by using absolute memory addresses to write and read a value.

{% tabs %}

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

assert_eq!(result, val);
```

{% hint style="warning" %}

Note that here we used an helper function: `utils.GetFunction`. This is just to avoid repeating the boilerplate code required to handle errors.

**This helper function is not part of the Wasmer API.**

If you want to know how to fetch exported globals, have a look at the following example:

{% page-ref page="imports-and-exports.md" %}

Now assume we want to write a value at the end of the second memory page and the read it. Let's see how we can do that:

{% tabs %}
{% tab %}

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

We now have everything we need to run the WASM module, let's do it!

{% tabs %}
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

```text
git clone https://github.com/wasmerio/wasmer-go.git
cd wasmer-go
go test examples/example_memory_test.go
```
{% endhint %}
{% endtab %}
{% endtabs %}


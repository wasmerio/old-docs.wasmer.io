---
id: runtime-rust-integration-prepare-wasm-modules
title: Rust Integration: Prepare WASM Modules
sidebar_label: Prepare WASM Modules
---

For each of the following integration examples, we must have a WASM module available to call from Rust.  Although these WASM modules could have been compiled from source code written in wide a variety of languages, for these examples, we will create them by compiling small Rust programs to WebAssembly.

Each of the following Rust projects on Github can be used to create the WASM module required by the respective Rust project:

| Rust Integration Demo | Calls WASM Module | Rust Project for WASM Module |
|---|---|---|
| Hello World | `hello-world-guest` | [Hello World Guest](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/runtime/rust-integration/wasm/hello-world-guest/)
| Passing Data Between Rust and WASM | `passing-data-guest` | [Passing Data Guest](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/runtime/rust-integration/wasm/passing-data-guest/)
| Exposing Host Functions to WebAssembly | `host-functions-guest` | [Host Functions Guest](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/runtime/rust-integration/wasm/host-functions-guest/)
| Handling Errors | `handling-errors-guest` | [Handling Errors Guest](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/runtime/rust-integration/wasm/handling-errors-guest/)
| Interrupt Execution | `early-exit-guest` | [Early Exit Guest](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/runtime/rust-integration/wasm/early-exit-guest/)

## General Instructions

For each of the above guest modules, you need to follow the preparatory steps given below.

These instructions are given using the Rust host project name `hello-world`, but you should substitute this name as required.  All other steps are common.

1. ***Create a Local `wasm` Directory***  
    In the `hello-world` directory, create a subdirectory called `wasm` and change into it

    ```bash
    $ mkdir wasm
    $ cd wasm
    ```

1. ***Create a New Project***  
    Use the Rust Package Manager to create a new library project for the guest module, then change into the newly created directory:

    ```bash
    $ cargo new --lib hello-world-guest
     Created library `hello-world-guest` package
    $ cd hello-world-guest
    ```

1. ***Define Project Type and Dependencies***  
    Edit `Cargo.toml` to declare this project to be a library and that it depends on `wee_alloc`:

    ```toml
    [package]
    name = "hello-world-guest"
    version = "0.1.0"
    authors = ["The Wasmer Engineering Team <engineering@wasmer.io>"]
    edition = "2018"
    publish = false
    
    [lib]
    crate-type = ["cdylib", "rlib"]
    
    [dependencies]
    # `wee_alloc` is a tiny allocator for wasm that is only ~1K in code size
    # compared to the default allocator's ~10K. It is slower than the default
    # allocator, however.
    wee_alloc = "0.4.5"
    ```

    > ***IMPORTANT***
    >
    > Don't forget to add the `[lib]` section into `Cargo.toml`!  
    > This tells the Rust compiler that you are building a library, not an application

1. ***Write the Required Functionality in Rust***  
    Edit `src/lib.rs` to contain the source code appropriate for your particular WASM module:

    | WASM Guest Module | Rust Source Code to Create WASM Module |
    |---|---|
    | Hello World | [`hello-world-guest/src/lib.rs`](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/runtime/rust-integration/wasm/hello-world-guest/src/lib.rs)
    | Passing Data Between Rust and WASM | [`passing-data-guest/src/lib.rs`](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/runtime/rust-integration/wasm/passing-data-guest/src/lib.rs)
    | Exposing Host Functions to WebAssembly | [`host-functions-guest/src/lib.rs`](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/runtime/rust-integration/wasm/host-functions-guest/src/lib.rs)
    | Handling Errors | [`handling-errors-guest/src/lib.rs`](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/runtime/rust-integration/wasm/handling-errors-guest/src/lib.rs)
    | Interrupt Execution | [`early-exit-guest/src/lib.rs`](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/runtime/rust-integration/wasm/early-exit-guest/src/lib.rs)

1. ***Create the WASM Module***  
    Use `cargo` to compile a WebAssembly "release" version of the library project

    ```bash
    $ cargo build --release --target wasm32-unknown-unknown
        Updating crates.io index
       Compiling hello-world-guest v0.1.0
        Finished dev [unoptimized + debuginfo] target(s) in 3.50s
    ```

1. ***Copy the WASM Module into the Expected Directory for the Host Project***  
    Create a new `wasm32-unknown-unknown` under the host project's `target` directory, then copy the WASM module into this folder.

   ```bash
   $ mkdir -p ../../target/wasm32-unknown-unknown/release/
   $ cp ./target/wasm32-unknown-unknown/release/*.wasm ../../target/wasm32-unknown-unknown/release/
   ```

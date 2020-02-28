# Server

Wasmer-JS has multiple packages that enables using Wasm WASI Modules in Node.js.

### `@wasmer/cli`

It's a CLI that allows you to run Wasm WASI modules in your shell very easily, with the same interface as the [Wasmer WebAssembly Runtime](../../../../ecosystem/wasmer/).

### `@wasmer/wasi`

Provides an easy to use API to interact with WASI modules. It works in both the browser and the server

### `@wasmer/wasmfs`

It's a wrapper on top of [memfs](https://github.com/streamich/memfs). It provides a filesystem abstraction in the Browser, so you can use a normal "filesystem" in the browser memory \(not in a real filesystem, since it's not accessible in browser environments\)

### `@wasmer/wasm-transformer`

Sometimes the Wasm modules that you want to use in the browser can't run because some of it's import functions have BigInts as arguments, and browsers doesn't support it yet.  
This module helps enables running Wasm modules on Browsers.  
You can find more info here: [Module Transformation page](../../module-transformation.md).

{% embed url="https://github.com/wasmerio/wasmer-js" caption="" %}

## Examples

Here are some useful examples you can visit to learn how to use Wasmer-JS in your server!

{% page-ref page="examples/hello-world.md" %}

{% page-ref page="examples/transforming-modules.md" %}


# Browser

Wasmer-JS has multiple packages that enables using WASM WASI Modules in your Browser.

### `@wasmer/wasi`

Provides an easy to use API to interact with WASI modules. It works in both the browser and the server

### `@wasmer/wasmfs`

It's a wrapper on top of [memfs](https://github.com/streamich/memfs). It provides a filesystem abstraction in the Browser, so you can use a normal "filesystem" in the browser memory \(not in a real filesystem, since it's not accessible in browser environments\)

### `@wasmer/wasm-transformer`

Sometimes the WASM modules that you want to use in the browser can't run because some of it's import functions have BigInts as arguments, and browsers doesn't support it yet.  
This module helps enables running Wasm modules on Browsers.  
You can find more info here: [Module Transformation page](../../module-transformation.md).

### `@wasmer/wasm-terminal`

It's a terminal emulator based on [xTerm.js](https://xtermjs.org/).  
Is used in the online [WebAssembly shell](../../../../ecosystem/webassembly.sh.md) :\)

{% embed url="https://github.com/wasmerio/wasmer-js" %}

## Examples

Here are some useful examples you can visit to learn how to use Wasmer-JS in your project!

{% page-ref page="examples/hello-world.md" %}

{% page-ref page="examples/transforming-modules.md" %}

{% page-ref page="examples/handling-io.md" %}




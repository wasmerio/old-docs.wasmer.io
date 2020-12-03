# Javascript

![](../../.gitbook/assets/image%20%283%29%20%282%29%20%281%29.png)

WebAssembly is already available from Javascript for both the browser and the server \(Node.js\) 🤗

In this case, Wasmer provides a lot of useful packages to interact with WebAssembly modules in Javascript easily.

In this section we will see how to setup both your server-side and browser environments to use WebAssembly, and also how to use WebAssembly modules that interact with the Operating System such as [WASI](wasi/) using the different packages that Wasmer offer for Javascript.

## Published NPM Packages

Wasmer publishes various packages to NPM:

* [`@wasmer/wasi`](https://www.npmjs.com/package/@wasmer/wasi): The Package for using WASI easily from Node and the Browser
* [`@wasmer/wasm-transformer`](https://www.npmjs.com/package/@wasmer/wasm-transformer): it allows to run WebAssembly modules in browsers that doesn't have Wasm BigInt support 
* [`@wasmer/wasm-terminal`](https://www.npmjs.com/package/@wasmer/wasm-terminal): with WebAssembly files
* [`@wasmer/wasmfs`](https://www.npmjs.com/package/@wasmer/wasmfs): the filesystem for Wasm

Let's start!


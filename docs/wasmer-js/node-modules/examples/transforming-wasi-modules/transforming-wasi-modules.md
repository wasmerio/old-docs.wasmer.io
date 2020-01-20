---
id: wasmer-js-modules-transforming-wasi-modules
title: Wasmer-JS in the Browser
sidebar_label: Transforming WASI Modules
---

[Full Example Project Source Code](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/wasmer-js/node-modules/examples/transforming-wasi-modules)

# Why Is Transformation Necessary?

In the previous Hello World example, we showed you how to run the very basic `as-echo` WASM module that received a text string as an argument and simply echoed it back via standard out.  However, some WASI modules may be compiled in a way that means they can't immediately be run from a JavaScript environment such as a browser.

For example, any module that calls the [clock\_time\_get](https://github.com/WebAssembly/WASI/blob/master/phases/snapshot/docs.md#-clock_time_getid-clockid-precision-timestamp---errno-timestamp) WASI API, must be able to supply a 64-bit, signed integer.  However, passing a JavaScript `BigInt` to a WebAssembly `i64` is not yet supported &mdash; this detail is still at the proposal stage.  (See [here](https://github.com/WebAssembly/JS-BigInt-integration/issues/15) and [here](https://github.com/WebAssembly/proposals/issues/7) for details).

However, it is not impossible to run such a module; but before we can, we must first ***transform*** it using `@wasmer/wasm-transformer`.

> ### Under The Hood  
> Technically, this transformation adapts the interface so that a JavaScript `BigInt` (a 64-bit, signed integer) in can be transferred to and from WebAssembly in the form of a JavaScript `Uint8Array` containing 8, unsigned, 8-bit integers.


# Setup Instructions

Please following the step-by-step instructions given in the [Hello World](../hello-world/wasmer-js-modules-hello-world) example, but with the following changes:

1. Call you project `wasmer-js-transforming-wasi`
1. Download the WASM module [`clock_time_get.wasm`](https://github.com/wasmerio/docs.wasmer.io/raw/master/docs/wasmer-js/node-modules/examples/transforming-wasi-modules/static/clock_time_get.wasm) and store it in the `static` directory



## JavaScript Coding

The coding seen below is very similar to the coding used for the previous `hello-world` example with the following important difference.

Inside function `startWasiTask`, we fetch the WASM file contents and convert it to a `Uint8Array` as before, but then there is the additional line

```JavaScript
const loweredWasmBytes = await lowerI64Imports(wasmBytes)
```

Here, the `lowerI64Imports` function transforms the interface such that a JavaScript `BigInt` value can be transferred to a WebAssembly `i64` value as an array of 8, unsigned, 8-bit integers.

It is not until after this transformation has occurred that we can instantiate the WebAssembly module and invoke it as before.

```JavaScript
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Imports
import { WASI }            from '@wasmer/wasi'
import { WasmFs }          from '@wasmer/wasmfs'
import { lowerI64Imports } from "@wasmer/wasm-transformer"

const wasmFilePath = './clock_time_get.wasm'  // Path to our wasi module

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Instantiate new WASI and WasmFs Instances
// IMPORTANT:
// Instantiating WasmFs is only needed when running in a browser.
// When running on the server, NodeJS's native FS module is assigned by default
const wasmFs = new WasmFs()

let wasi = new WASI({
  // Arguments passed to the Wasm Module
  // The first argument is usually the filepath to the executable WASI module
  // we want to run.
  args: [wasmFilePath],

  // Environment variables that are accesible to the Wasi module
  env: {},

  // Bindings that are used by the Wasi Instance (fs, path, etc...)
  bindings: {
    ...WASI.defaultBindings,
    fs: wasmFs.fs
  }
})

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Preserve the original console.log functionality
const consoleLog = console.log

// Implement our own console.log functionality
console.log = (...args) =>
  (logTxt => {
    consoleLog(logTxt)
    document.body.appendChild(
      document.createTextNode(logTxt)
    )
  })
  (args.join(' '))

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Async Function to run our wasi module/instance
const startWasiTask =
  async () => {
    // Fetch our Wasm File
    const response  = await fetch(wasmFilePath)
    const wasmBytes = new Uint8Array(await response.arrayBuffer())

    // Lower the WebAssembly Module bytes
    // This will create trampoline functions for i64 parameters in function
    // calls such as: 
    // https://github.com/WebAssembly/WASI/blob/master/phases/old/snapshot_0/docs/wasi_unstable.md#clock_time_get
    // Allowing the Wasi module to work in the browser / node!
    const loweredWasmBytes = await lowerI64Imports(wasmBytes)

    // Instantiate the WebAssembly file
    let { instance } = await WebAssembly.instantiate(loweredWasmBytes, {
      wasi_unstable: wasi.wasiImport
    })

    wasi.start(instance)                      // Start the transformed WASI instance
    let stdout = await wasmFs.getStdOut()     // Get the contents of /dev/stdout
    console.log(`Standard Output: ${stdout}`) // Write wasi's stdout to the DOM
  }

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Everything starts here
startWasiTask()
```

Next, let's look at handling input and output via WASI.


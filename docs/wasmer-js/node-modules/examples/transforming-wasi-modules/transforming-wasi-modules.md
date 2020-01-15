---
id: wasmer-js-node-modules-transforming-wasi-modules
title: Wasmer-JS Node Modules Transforming WASI Modules
sidebar_label: Transforming WASI Modules
---

[Full Example Project Source Code](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/wasmer-js/node-modules/examples/transforming-wasi-modules)

## Why Is Transformation Necessary?

In the previous Hello World example, we showed you how to run a very basic "echo" wasm module that received a text string as an argument.  However, some WASI modules may be compiled in a way that means that can't immediately be run from a JavaScript environment such as a browser.

For example, modules that call the [clock\_time\_get](https://github.com/WebAssembly/WASI/blob/master/phases/old/snapshot_0/docs/wasi_unstable.md#clock_time_get) WASI API, must be able to receive a 64-bit integer (WebAssembly type `I64`, JavaScript `BigInt`) &mdash; which is not yet fully supported as it is still at the [proposal stage](https://github.com/WebAssembly/JS-BigInt-integration/issues/15). 

However, it is not impossible to run such a module; but before we can, we must first ***transform*** it using `@wasmer/wasm-transformer`.

## Example

Here, we will fetch a WASI module that we know returns an `I64` (or JavaScript `BigInt`) value.  Therefore, before attempting to call this module, we must first transform it using `lowerI64Imports` from `@wasmer/wasm-transformer`.  Once we have done this, we can then run it in the browser!


```javascript
// *****************************************************************************
// Imports
import { WASI }            from '@wasmer/wasi'
import { WasmFs }          from '@wasmer/wasmfs'
import { lowerI64Imports } from "@wasmer/wasm-transformer"

const wasmFilePath = './clock_time_get.wasm'  // Path to our wasi module

// *****************************************************************************
// Instantiate new WASI and WasmFs Instances
// NOTE:
// If running in NodeJS, WasmFs is not needed.  In this case, Node's native FS
// module is assigned by default.
// Here however, we want to show off how to use WasmFs within the browser.
// This also means that all our file system operations are sand-boxed.
// In other words, the wasi module running in the browser does not have any
// access to the file system of the machine running the browser
const wasmFs = new WasmFs()

let wasi = new WASI({
  // Arguments passed to the Wasm Module
  // The first argument is usually the filepath to the "executable wasi module"
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

// *****************************************************************************
// Preserve the original console.log functionality
const consoleLog = console.log

// Implement our own console.log functionality
console.log = (...args) =>
  (logTxt => {
    consoleLog(logTxt)
    document.body.appendChild(
      document.createTextNode(`JavaScript Console: ${logTxt}`)
    )
  })
  (args.join(' '))

// *****************************************************************************
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

// *****************************************************************************
// Everything starts here
startWasiTask()
```


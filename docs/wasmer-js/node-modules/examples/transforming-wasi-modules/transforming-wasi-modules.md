---
id: wasmer-js-node-modules-transforming-wasi-modules
title: Wasmer-JS Node Modules Transforming WASI Modules
sidebar_label: Transforming WASI Modules
---

[Full Example Project Source Code](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/wasmer-js/node-modules/examples/transforming-wasi-modules)

# Why Is Transformation Necessary?

In the previous Hello World example, we showed you how to run the very basic `as-echo` WASM module that received a text string as an argument and simply echoed it back via standard out.  However, some WASI modules may be compiled in a way that means they can't immediately be run from a JavaScript environment such as a browser.

For example, any module that calls the [clock\_time\_get](https://github.com/NuxiNL/cloudabi/blob/master/cloudabi.txt#L1230) WASI API, must be able to supply a 64-bit, signed integer.  However, passing a JavaScript `BigInt` to a WebAssembly `i64` is not yet supported &mdash; this detail is still at the proposal stage.  (See [here](https://github.com/WebAssembly/JS-BigInt-integration/issues/15) and [here](https://github.com/WebAssembly/proposals/issues/7) for details).

However, it is not impossible to run such a module; but before we can, we must first ***transform*** it using `@wasmer/wasm-transformer`.

> ### Under The Hood  
> Technically, this transformation adapts the interface so that JavaScript `BigInt`s (64-bit, signed integers) in can be transferred to and from WebAssembly in the form of arrays of 8, unsigned 8-bit integers (a JavaScript `Uint8Array`)

## Prerequisites

Make sure [Parcel](https://parceljs.org/) has been installed and is available from the command line

```bash
$ npm install -g parcel
```

> ### Mac users
> Before the installation of Parcel will work on a Mac, you must first install the [Xcode Command Line Tools](https://developer.apple.com/download/more/?=for%20Xcode)

## Run Example from Git Clone

If you have not already done the previous `hello-world` example, the simplest way to run this exercise is to clone the entire [`docs.wasmer.io`](https://github.com/wasmerio/docs.wasmer.io) repo:

1. Change into some development directory

    ```bash
    $ cd <some_development directory>
    ```

1. Clone this entire repo

    ```bash
    $ git clone https://github.com/wasmerio/docs.wasmer.io.git
    ```

1. Change into the `transforming-wasi-modules` directory

    ```bash
    $ cd docs.wasmer.io/docs/wasmer-js/node-modules/examples/transforming-wasi-modules
    ```

1. Install the required `npm` dependencies

    ```bash
    $ npm install
    ```

1. Start `parcel` 

   ```bash
   $ parcel index.html
   ```

1. Point your browser to [`http://localhost:1234`](http://localhost:1234) and you should see `Standard Output: Done!` appear both on the browser screen and in the JavaScript console

# The `clock_time_get` WebAssembly Module

In this example, we want to use the following call chain:

`JavaScript` --> `WebAssembly`  --> `Native "OS" function`

> ### As an Aside...  
> The term "OS" is in double quotes to indicate that the native function being called might not actually belong to the underlying operating system.  
> In reality, this function belongs to the host environment within which this WebAssembly module is running, and in this particular case, this is the environment provided by the browser, not the underlying operating system.  Nonetheless, from a WebAssembly point of view, we don't need to care about this detail.  
> All we need to know is that this function exists, and we can call it (if we're careful)!

In this case, the native "OS" function we want to call is `clock_time_get`.  To understand how we should call this function, we need look inside the WebAssembly module `clock_time_get.wasm`.  When converted to [WebAssembly Text](https://webassembly.github.io/spec/core/text/index.html) format, the first few lines of this module looks like this:

```WebAssemblyText
(module
  (type $t0 (func (param i32 i64 i32) (result i32)))
  (type $t1 (func (param i32 i32 i32 i32) (result i32)))
  (type $t2 (func))
  (import "wasi_unstable" "clock_time_get" (func $wasi_unstable.clock_time_get (type $t0)))
  (import "wasi_unstable" "fd_write" (func $wasi_unstable.fd_write (type $t1)))

  ;; snip...
```

On line 2, we can see the declaration of a type definition called `$t0`.  This type definition represents the interface to some `func`tion that takes three, signed integers as parameters and returns an integer.

```WebAssemblyText
(type $t0 (func (param i32 i64 i32) (result i32)))
```

Notice the data type of the second parameter; Uh oh! Its a 64-bit signed integer!

At the moment, we have no way to directly pass a JavaScript `BigInt` into WebAssembly; therefore, before calling this WebAssembly module, the interface to this function must be transformed

Then on line 5, we can see the declaration of the call to `clock_time_get`:

```WebAssemblyText
(import "wasi\_unstable" "clock\_time\_get" (func $wasi\_unstable.clock\_time\_get (type $t0)))
```

Two things are important to notice here:

1. The `import` keyword indicates that function `clock_time_get` lives in an external module called `wasi_unstable`

1. The interface to this function is described by the previously declared type definition `$t0`.  In other words, we know for certain that function `clock_time_get` must be passed an `i64` as its second parameter.

Having established exactly what the interface is to `clock_time_get`, we now know that we cannot call this WASM module without first transforming it.

### Important

This example is somewhat contrived because the WebAssembly module has been hard-coded to return the text string `Done!` rather than the value returned from `clock_time_get`.  This is because this module writes its output to standard out, which in turn, expects to receive printable strings followed by a carriage return character, not the raw `i32` value returned from `clock_time_get`.


## JavaScript Coding

The coding seen below is very similar to the coding used for the previous `hello-world` example with the following important difference.

Inside function `startWasiTask`, we fetch the WASM file contents and convert it to a `Uint8Array` as before, but then there is the additional line

```JavaScript
const loweredWasmBytes = await lowerI64Imports(wasmBytes)
```

Here, the `lowerI64Imports` function transforms the interface such that a JavaScript `BigInt` value can be transferred to a WebAssembly `i64` value as an array of 8, unsigned, 8-bit integers.

It is not until after this transformation has occurred that we can instantiate the WebAssembly module and invoke it as before.

```JavaScript
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
      document.createTextNode(logTxt)
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

Next, let's look at handling input and output via WASI.


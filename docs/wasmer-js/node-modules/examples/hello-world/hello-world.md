---
id: wasmer-js-node-modules-hello-world
title: Wasmer-JS Node Modules Hello World
sidebar_label: Hello World
---

[Full Example Project Source Code](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/wasmer-js/node-modules/examples/hello-world)

In this example, we will run the WASI module [as-echo](https://github.com/torch2424/as-echo), using `@wasmer/wasi`. 

This WASI module simply receives a text string (in our case `"Hello World!"`) and echoes it back via standard output `/dev/stdout` using the `@wasmer/wasmfs` module.

This example will be bundled and served by [Parcel](https://parceljs.org/) and run in the browser.  However, `@wasmer/wasi` and `@wasmer/wasmfs` also work in NodeJS, and the code examples from this guide can be used as a loose example &mdash; as long as the ES6 syntax is replaced with the equivalent NodeJS coding.

## Prerequisites

Make sure [Parcel](https://parceljs.org/) has been installed and is available from the command line

```bash
$ npm install -g parcel
```

> ### Mac users
> Before the installation of Parcel will work on a Mac, you must first install the [Xcode Command Line Tools](https://developer.apple.com/download/more/?=for%20Xcode)

## Run Example from Git Clone

The simplest way to run this exercise is to clone the entire [`docs.wasmer.io`](https://github.com/wasmerio/docs.wasmer.io) repo:

1. Change into some development directory

    ```bash
    $ cd <some_development_directory>
    ```

1. Clone this entire repo

    ```bash
    $ git clone https://github.com/wasmerio/docs.wasmer.io.git
    ```

1. Change into the `hello-world` example directory

    ```bash
    $ cd docs.wasmer.io/docs/wasmer-js/node-modules/examples/hello-world
    ```

1. Install the required `npm` dependencies

    ```bash
    $ npm install
    ```

1. Start `parcel` 

   ```bash
   $ parcel index.html
   ```

1. Point your browser to [`http://localhost:1234`](http://localhost:1234) and you should see `Standard Output: Hello World!` appear both on the browser screen and in the JavaScript console



## Using `wasmer-js`

This little demo uses the following two `wasmer-js` packages: 

| Package Name | Description
|---|---|
| `@wasmer/wasi` | A polyfill to implement any WebAssembly System Interface (`WASI`) bindings your browser might not support
| `@wasmer/wasmfs` | A sandboxed filesystem with which the `@wasmer/wasi` module can interact

These packages have already been listed in the `dependencies` section of `package.json`, but when needed in your own projects, can be added using the following command:

```bash
$ npm install --save @wasmer/wasi @wasmer/wasmfs
```

## JavaScript Coding

Seeing as this is demo code, it uses meaningful variable names and contains lots of explanatory comments (features that are often sadly missing from production code).  Please take some time to read and understand these comments as they explain how the functionality has been constructed.
    
Also, make a note of the comment explaining `@wasmer/wasm-transformer`; we will cover this very important detail in a later example.


```JavaScript
// *****************************************************************************
// Imports
import { WASI }   from '@wasmer/wasi'
import { WasmFs } from '@wasmer/wasmfs'

const wasmFilePath = './as-echo.wasm'  // Path to our WASI module
const echoStr      = 'Hello World!'    // Text string to echo

// *****************************************************************************
// Instantiate new WASI and WasmFs Instances
// NOTE:
// If running in NodeJS, WasmFs is not needed.  In this case, Node's native FS
// module is assigned by default.
// Here however, we want to show off how to use WasmFs within the browser.
// This also means that all our file system operations are sand-boxed.
// In other words, the WASI module running in the browser does not have any
// access to the file system of the machine running the browser
const wasmFs = new WasmFs()

let wasi = new WASI({
  // Arguments passed to the Wasm Module
  // The first argument is usually the filepath to the executable wasi module
  // we want to run.
  args: [wasmFilePath, echoStr],

  // Environment variables that are accesible to the Wasi module
  env: {},

  // Bindings that are used by the WASI Instance (fs, path, etc...)
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
// Async function to run our wasi module/instance
const startWasiTask =
  async pathToWasmFile => {
    // Fetch our WASM File
    let response  = await fetch(pathToWasmFile)
    let wasmBytes = new Uint8Array(await response.arrayBuffer())

    // IMPORTANT:
    // Some WASI module interfaces use datatypes that cannot yet be transferred
    // between environments (for example, you can't yet send a JavaScript BigInt
    // to a WebAssembly i64).  Therefore, the interface to such modules has to
    // be transformed using `@wasmer/wasm-transformer`, which we will cover in
    // a later example

    // Instantiate the WebAssembly file
    let { instance } = await WebAssembly.instantiate(wasmBytes, {
      wasi_unstable: wasi.wasiImport
    })

    wasi.start(instance)                      // Start the WASI instance
    let stdout = await wasmFs.getStdOut()     // Get the contents of /dev/stdout
    console.log(`Standard Output: ${stdout}`) // Write WASI's stdout to the DOM
  }

// *****************************************************************************
// Everything starts here
startWasiTask(wasmFilePath)
```


Next, let's take a look at transforming WASI modules that require transformations.

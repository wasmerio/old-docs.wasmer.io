# hello-world-server

[Full Example Project Source Code](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/wasmer-js/server/examples/hello-world)

## Hello World! \(Server\)

In this introductory example, we will develop a NodeJS-based application that calls a WebAssembly module that in turn, calls a native "OS" function. This is exactly the same call chain as was used in the client-side example:

`JavaScript` --&gt; `WebAssembly` --&gt; `Native "OS" function`

In this case, we will invoke the WASM module [`as-echo`](https://github.com/torch2424/as-echo) that receives a text string and echoes it back by writing it to standard out.

However, as we saw with the [client-side `hello-world` example](https://github.com/wasmerio/docs.wasmer.io/tree/ca2c9145ea511f3c00439b180be82cc5197a177f/docs/wasmer-js/client/examples/hello-world/wasmer-js-client-hello-world/README.md), file descriptors such as "standard in" and "standard out" are not normally available to a WebAssembly module since they belong to the underlying "OS". Therefore, we must again make use of the following package:

| Package Name | Description |
| :--- | :--- |
| `@wasmer/wasi` | A set of JavaScript polyfills that bridge the gap between the black-box world of a WebAssembly module and functionality available from the host environment |

> #### IMPORTANT
>
> Notice that for a server-side implementation, the `@wasmer/wasmfs` package is _**not**_ needed.

## Setup Instructions

### Step-By-Step Guide

1. Change into some development directory

   ```bash
    $ cd <some_development_directory>
   ```

2. Create and then change into a new project directory, then run `npm init`

   ```bash
    $ mkdir wasmer-js-node-hello-world
    $ cd wasmer-js-node-hello-world
    $ npm init
   ```

   After answering all the questions from `npm init`, you will have a configured `package.json` file.

3. Declare the use of package `@wasmer/wasi` as a runtime dependency by running the command:

   ```bash
   $ npm install --save @wasmer/wasi
   ```

4. Create a new directory called `wasm_lib`

   ```bash
    $ mkdir wasm_lib
   ```

5. Download the WebAssembly module [`as-echo.wasm`](https://github.com/wasmerio/docs.wasmer.io/raw/master/docs/wasmer-js/wasm_lib/as-echo.wasm) and store it in this directory
6. Create the file `server.js` and add the coding shown below.

   > #### Important Difference
   >
   > In contrast to running in the browser, the server-side implementation of the same WASM module is noticeably smaller.
   >
   > When running server-side, we do not need to write any code to obtain the contents of standard out after the `as-echo` WASM module has executed, since when running server-side, anything written to standard out by a WASM module appears directly in the console.

   ```javascript
    const fs       = require("fs")
    const { WASI } = require("@wasmer/wasi")

    const wasmFilePath = "./wasm_lib/as-echo.wasm"
    const echoStr      = "Hello World!"

    // Instantiate a new WASI Instance
    let wasi = new WASI({
      args : [wasmFilePath, echoStr]
    , env  : {}
    })

    // *****************************************************************************
    // Async function to run our WASM module/instance
    const startWasiTask =
      async pathToWasmFile => {
        // Fetch our WASM File
        let wasmBytes = new Uint8Array(fs.readFileSync(pathToWasmFile)).buffer

        // Instantiate the WebAssembly file
        let { instance } = await WebAssembly.instantiate(wasmBytes, {
          wasi_unstable: wasi.wasiImport
        })

        // Start the WASI instance
        wasi.start(instance)
      }

    // *****************************************************************************
    // Everything starts here
    startWasiTask(wasmFilePath)
   ```

7. Save `server.js` and run it using:

   ```bash
    $ node server.js
    Hello World!
   ```

Next, let's take a look at running WASM modules whose interfaces require transformation.


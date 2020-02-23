# Transforming WASI Modules

[Full Example Project Source Code](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/wasmer-js/server/examples/transforming-wasi-modules)

## Transforming WASI Modules in the Server

Irrespective of whether your JavaScript code runs on the client or the server, the statement shown below to [transform a WASI module](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/integrations/js/server/wasmer-js-module-transformation/README.md) will always be needed.

### Setup Instructions

Please repeat the step-by-step instructions given in the [Hello World](https://github.com/wasmerio/docs.wasmer.io/tree/master/integrations/js/server/examples/hello-world/wasmer-js-server-hello-world/README.md) example, but with the following changes:

1. Call your project `wasmer-js-transforming-wasi`
2. Download the Wasm module [`clock_time_get.wasm`](https://github.com/wasmerio/docs.wasmer.io/raw/master/docs/wasmer-js/wasm_lib/clock_time_get.wasm) and store it in the `wasm_lib` directory

### JavaScript Coding

The coding seen below is very similar to the coding used for the previous Hello World example — but with one very important difference!

Inside function `startWasiTask`, we fetch the Wasm file contents and convert it to a `Uint8Array` as before, but then there is the additional line:

```javascript
const loweredWasmBytes = await lowerI64Imports(wasmBytes)
```

The call to function `lowerI64Imports` performs the all-important transformation that allows a JavaScript `BigInt` to be transferred to a WebAssembly `i64`.

Now that the interface has been transformed, we can instantiate the WebAssembly module and invoke it as before.

```javascript
const fs  = require("fs")

const { WASI }            = require("@wasmer/wasi")
const { lowerI64Imports } = require("@wasmer/wasm-transformer")

const wasmFilePath = "./wasm_lib/clock_time_get.wasm"

// Instantiate a new WASI Instance
let wasi = new WASI({
  args : [wasmFilePath]
, env  : {}
})

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Async function to run our Wasm module/instance
const startWasiTask =
  async pathToWasmFile => {
    // Fetch the Wasm module and transform its interface
    let wasmBytes        = new Uint8Array(fs.readFileSync(pathToWasmFile))
    let loweredWasmBytes = lowerI64Imports(wasmBytes)

    // Instantiate the WebAssembly file
    let { instance } = await WebAssembly.instantiate(loweredWasmBytes, {
      wasi_unstable: wasi.wasiImport
    })

    // Start the WASI instance
    wasi.start(instance)
  }

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Everything starts here
startWasiTask(wasmFilePath)
```

Run the program:

```bash
$ node server.js
Done!
```


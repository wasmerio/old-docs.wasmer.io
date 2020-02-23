# Transforming Modules

{% hint style="success" %}
**Note**: The final code for this example can be found on [GitHub](https://github.com/wasmerio/docs.wasmer.io/tree/master/integrations/js/wasi/server/examples/transforming-modules).
{% endhint %}

Irrespective of whether your JavaScript code runs on the client or the server, the statement shown below to [transform a WASI module](/integrations/js/module-transformation) will be always needed until browsers land `BigInt` support in WebAssembly. 

### Setup Instructions

Please repeat the step-by-step instructions given in the [Hello World](/integrations/js/wasi/server/examples/hello-world) example, but with the following changes:

1. Call your project `wasmer-js-transforming-wasi`
2. Download the Wasm module [`clocktimeget.wasm`](https://github.com/wasmerio/docs.wasmer.io/raw/master/integrations/shared/wat/wasi/clocktimeget.wasm)

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
const { WASI } = require("@wasmer/wasi")
const nodeBindings = require("@wasmer/wasi/lib/bindings/node");
const { lowerI64Imports } = require("@wasmer/wasm-transformer")

const wasmFilePath = "./clocktimeget.wasm"

// Instantiate a new WASI Instance
let wasi = new WASI({
  args: [wasmFilePath],
  env: {},
  bindings: {
    ...nodeBindings,
    fs: fs
  }
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

{% hint style="info" %}
If you want to run the examples from the docs codebase directly, you can also do:

```bash
git clone https://github.com/wasmerio/docs.wasmer.io.git
cd docs.wasmer.io/integrations/js/wasi/server/examples/transforming-modules
npm run dev
```
{% endhint %}

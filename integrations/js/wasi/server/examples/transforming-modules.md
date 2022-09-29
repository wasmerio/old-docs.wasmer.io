# Transforming Modules

{% hint style="success" %}
**Note**: The final code for this example can be found on Github: [transforming-modules](https://github.com/wasmerio/docs.wasmer.io/tree/master/integrations/js/wasi/server/examples/transforming-modules).
{% endhint %}

Irrespective of whether your WebAssembly module is invoked from JavaScript code running in the client or the server, it is safest to assume that the statement shown below to [transform a WASI module](https://github.com/wasmerio/docs.wasmer.io/tree/e0f7639306bb4cf18cd0c23876b80f787d6b5876/integrations/js/module-transformation/README.md) will be always needed.

## Setup Instructions

Please repeat the step-by-step instructions given in the [Hello World](https://github.com/wasmerio/docs.wasmer.io/tree/e0f7639306bb4cf18cd0c23876b80f787d6b5876/integrations/js/wasi/server/examples/hello-world/README.md) example, but with the following changes:

1. Call your project `wasmer-js-transforming-wasi`
2. Download the Wasm module [`clocktimeget.wasm`](https://github.com/wasmerio/docs.wasmer.io/raw/master/integrations/shared/wat/wasi/clocktimeget.wasm)

## JavaScript Coding

The coding seen below is very similar to the coding used for the previous Hello World example — but with one very important difference!

Inside function `startWasiTask`, we fetch the Wasm file contents and convert it to a `Uint8Array` as before, but then there is the additional line:

```javascript
const loweredWasmBytes = await lowerI64Imports(wasmBytes)
```

The call to function `lowerI64Imports` performs the all-important transformation that allows a JavaScript `BigInt` to be transferred to a WebAssembly `i64`.

Now that the interface has been transformed, we can instantiate the WebAssembly module and invoke it as before.

```javascript
const fs = require("fs")
const { WASI } = require("@wasmer/wasi")
const { lowerI64Imports } = require("@wasmer/wasm-transformer")

const wasmFilePath = "./clocktimeget.wasm"

const testVal1 = "0xDEADBEEFDEADBEEF"
const testVal2 = "0xBADC0FFEE0DDF00D"

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Instantiate a new WASI Instance
let wasi = new WASI({
  args: [wasmFilePath],
  env: {},
})

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Async function to run WASI module/instance
const startWasiTask =
  async pathToWasmFile => {
    let { instance } = await WebAssembly.instantiate(
      // Transform the Wasm module so that i64 values can be used in function interfaces
      lowerI64Imports(
      // Fetch the WASM module
      new Uint8Array(fs.readFileSync(pathToWasmFile))
      ),
      // Grant access to the host functions imported by Wasm
      { wasi_unstable: wasi.wasiImport },
    )

    // Start the WASI instance
    wasi.start(instance)

    return instance.exports
  }

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Everything starts here
startWasiTask(wasmFilePath)
  .then(wasmFns => {
    // wasmFns.test_i64ToHexStr(BigInt(testVal1))
    // wasmFns.test_i64ToHexStr(BigInt(testVal2))

    wasmFns.writeTimeNanos()
  })
```

Run the program:

```bash
$ node server.js
0006acadc1dd8f18
```

{% hint style="info" %}
If you want to run the examples from the docs codebase directly, you can also do:

```bash
git clone https://github.com/wasmerio/docs.wasmer.io.git
cd docs.wasmer.io/integrations/js/wasi/server/examples/transforming-modules
npm run dev
```
{% endhint %}

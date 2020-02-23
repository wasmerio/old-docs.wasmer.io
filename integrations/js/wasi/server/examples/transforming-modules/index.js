const fs  = require("fs")
const { WASI } = require("@wasmer/wasi")
let nodeBindings = require("@wasmer/wasi/lib/bindings/node");
const { lowerI64Imports } = require("@wasmer/wasm-transformer")

nodeBindings = nodeBindings.default || nodeBindings;

const wasmFilePath = "../../../../../shared/wat/wasi/clocktimeget.wasm";

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

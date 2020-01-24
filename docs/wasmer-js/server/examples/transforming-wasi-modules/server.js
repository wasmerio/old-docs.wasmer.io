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
// Async function to run our WASI module/instance
const startWasiTask =
  async pathToWasmFile => {
    // Fetch the WASM module and transform its interface
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

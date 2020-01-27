// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Imports
import { WASI }            from '@wasmer/wasi'
import { WasmFs }          from '@wasmer/wasmfs'
import { lowerI64Imports } from "@wasmer/wasm-transformer"

const wasmFilePath = './clock_time_get.wasm'  // Path to our WASI module

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Instantiate new WASI and WasmFs Instances
// IMPORTANT:
// Instantiating WasmFs is only needed when running in a browser.
// When running on the server, NodeJS's native FS module is assigned by default
const wasmFs = new WasmFs()

let wasi = new WASI({
  // Arguments passed to the WASM Module
  // The first argument is usually the filepath to the executable WASI module
  // we want to run.
  args: [wasmFilePath],

  // Environment variables that are accesible to the WASI module
  env: {},

  // Bindings used by the WASI instance (fs, path, etc...)
  bindings: {
    ...WASI.defaultBindings,
    fs: wasmFs.fs
  }
})

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Preserve the original console.log functionality
const consoleLog = console.log

// Implement our own console.log functionality that also writes to the DOM
console.log = (...args) =>
  (logTxt => {
    consoleLog(logTxt)
    document.body.appendChild(
      document.createTextNode(logTxt)
    )
  })
  (args.join(' '))

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Async Function to run our WASI module/instance
const startWasiTask =
  async () => {
    // Fetch our WASM File
    const response  = await fetch(wasmFilePath)
    const wasmBytes = new Uint8Array(await response.arrayBuffer())

    // IMPORTANT EXTRA STEP!
    // We must transform the WebAssembly module interface!
    const loweredWasmBytes = await lowerI64Imports(wasmBytes)

    // Instantiate the WebAssembly file
    let { instance } = await WebAssembly.instantiate(loweredWasmBytes, {
      wasi_unstable: wasi.wasiImport
    })

    wasi.start(instance)                      // Start the transformed WASI instance
    let stdout = await wasmFs.getStdOut()     // Get the contents of stdout
    console.log(`Standard Output: ${stdout}`) // Write stdout to the DOM
  }

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Everything starts here
startWasiTask()

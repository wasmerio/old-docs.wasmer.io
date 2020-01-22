// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Imports
import { WASI }   from '@wasmer/wasi'
import { WasmFs } from '@wasmer/wasmfs'

const wasmFilePath = './as-echo.wasm'  // Path to our WASI module
const echoStr      = 'Hello World!'    // Text string to echo

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Instantiate new WASI and WasmFs Instances
// IMPORTANT:
// Instantiating WasmFs is only needed when running in a browser.
// When running on the server, NodeJS's native FS module is assigned by default
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

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
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

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
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

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Everything starts here
startWasiTask(wasmFilePath)
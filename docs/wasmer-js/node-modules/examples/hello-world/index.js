// *****************************************************************************
// Imports
import { WASI }   from '@wasmer/wasi'
import { WasmFs } from '@wasmer/wasmfs'

const wasmFilePath = './as-echo.wasm'  // Path to our wasi module
const echoStr      = 'Hello World!'    // Text string to echo

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
  args: [wasmFilePath, echoStr],

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
      document.createTextNode(`JavaScript Console: ${logTxt}`)
    )
  })
  (args.join(' '))

// *****************************************************************************
// Async function to run our wasi module/instance
const startWasiTask =
  async pathToWasmFile => {
    // Fetch our Wasm File
    let response  = await fetch(pathToWasmFile)
    let wasmBytes = new Uint8Array(await response.arrayBuffer())

    // IMPORTANT:
    // Some wasi modules import datatypes that cannot yet be supplied by all
    // JavaScript environments (for example, you can't yet import a JavaScript
    // BigInt into WebAssembly).  Therefore, the interface to such modules has
    // to be transformed using `@wasmer/wasm-transformer`, which we will cover
    // in later examples

    // Instantiate the WebAssembly file
    let { instance } = await WebAssembly.instantiate(wasmBytes, {
      wasi_unstable: wasi.wasiImport
    })

    wasi.start(instance)                      // Start the WASI instance
    let stdout = await wasmFs.getStdOut()     // Get the contents of /dev/stdout
    console.log(`Standard Output: ${stdout}`) // Write wasi's stdout to the DOM
  }

// *****************************************************************************
// Everything starts here
startWasiTask(wasmFilePath)

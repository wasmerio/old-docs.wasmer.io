const fs = require("fs");
const { WASI } = require("@wasmer/wasi");
let nodeBindings = require("@wasmer/wasi/lib/bindings/node");

const wasmFilePath = "../../../../../shared/wat/wasi/helloworld.wasm";

nodeBindings = nodeBindings.default || nodeBindings;

// Instantiate a new WASI Instance
let wasi = new WASI({
  args: [wasmFilePath],
  env: {},
  bindings: nodeBindings
});

// Async function to run our Wasm module/instance
const startWasiTask = async pathToWasmFile => {
  // Fetch our Wasm File
  let wasmBytes = new Uint8Array(fs.readFileSync(pathToWasmFile)).buffer;

  // Instantiate the WebAssembly file
  let wasmModule = await WebAssembly.compile(wasmBytes);
  let instance = await WebAssembly.instantiate(wasmModule, {
    ...wasi.getImports(wasmModule)
  });

  // Start the WASI instance
  wasi.start(instance);
};

// Everything starts here
startWasiTask(wasmFilePath);

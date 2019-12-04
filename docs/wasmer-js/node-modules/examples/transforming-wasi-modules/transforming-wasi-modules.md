---
id: wasmer-js-node-modules-transforming-wasi-modules
title: Wasmer-JS Node Modules Transforming WASI Modules
sidebar_label: Transforming WASI Modules
---

[Full Example Project Source Code](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/wasmer-js/node-modules/examples/transforming-wasi-modules)

In the Hello World example, we covered how to run a basic "echo" wasm module. 

However, some WASI modules may be compiled in a way that can't be yet run by the browser. For example, modules that call the [clock_time_get](https://github.com/WebAssembly/WASI/blob/master/phases/old/snapshot_0/docs/wasi_unstable.md#clock_time_get) WASI API, require passing a `BigInt` from Javascript as a `I64` into WebAssembly, which is currently not yet fully supported and still in a [proposal for the browser](https://github.com/WebAssembly/JS-BigInt-integration/issues/15). 

Thus, in order to run these modules, you would have to **transform the module** with `@wasmer/wasm-transformer`.

In this example, we will fetch a WASI module, transform it with `lowerI64Imports` from `@wasmer/wasm-transformer`, and then run it in the browser!

```javascript
// Imports
import { WASI } from '@wasmer/wasi';
import { WasmFs } from '@wasmer/wasmfs';
import { lowerI64Imports } from "@wasmer/wasm-transformer";

// The file path to the wasi module we want to run
const wasmFilePath = './qjs.wasm';

// Instantiate a new WASI and WasmFs Instance
// NOTE: For node WasmFs is not needed, and the native Fs module is assigned by default
// In this case, we want to show off WasmFs for the browser use case, and we want to
// "Sandbox" our file system operations
const wasmFs = new WasmFs();
let wasi = new WASI({
    // Arguments to pass to the Wasm Module
    // The first argument usually should be the filepath to the "executable wasi module"
    // That we want to run.
    args: [wasmFilePath],
    // Environment variables that are accesible to the Wasi module
    env: {},
    // Bindings that are used by the Wasi Instance (fs, path, etc...)
    bindings: {
      ...WASI.defaultBindings,
      fs: wasmFs.fs
    }
});

// Async Function to run our wasi module/instance
const startWasiTask = async () => {
  // Fetch our Wasm File
  const response = await fetch(wasmFilePath);
  const responseArrayBuffer = await response.arrayBuffer();
  const wasmBytes = new Uint8Array(responseArrayBuffer);

  // Lower the WebAssembly Module bytes
  // This will create trampoline functions for i64 parameters
  // in function calls like: 
  // https://github.com/WebAssembly/WASI/blob/master/phases/old/snapshot_0/docs/wasi_unstable.md#clock_time_get
  // Allowing the Wasi module to work in the browser / node!
  const loweredWasmBytes = await lowerI64Imports(wasmBytes);

  // Instantiate the WebAssembly file
  let { instance } = await WebAssembly.instantiate(loweredWasmBytes, {
    wasi_unstable: wasi.wasiImport
  });

  // Start the WebAssembly WASI instance!
  wasi.start(instance);

  // Output what's inside of /dev/stdout!
  const stdout = await wasmFs.getStdOut();
  // Add the Standard output to the dom
  console.log('Standard Output: ' + stdout);
};
startWasiTask();
```


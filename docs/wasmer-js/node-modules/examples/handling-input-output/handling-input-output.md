---
id: wasmer-js-node-modules-handling-input-output
title: Wasmer-JS Node Modules Handling Input and Output
sidebar_label: Handling Input and Output
---

In the Hello World example, we covered how to run a basic "echo" wasm module, and then read it's output. However, there may be times we want to interact with WASI modules that accept input as well!

In this example, we will be using the [duktape WASI module](https://wapm.io/package/duktape), to execute Javascript in the duktape runtime. To handle input, we will create our own `stdinRead` fucntion, that is bound to the zero-index file descriptor in WasmFS (`/dev/stdin`). This will allow us to intercept read requests, and send whatever input we would like to the wasi application. See the code below:

```javascript
// Imports
import { WASI } from '@wasmer/wasi';
import { WasmFs } from '@wasmer/wasmfs';
import { lowerI64Imports } from "@wasmer/wasm-transformer";

// The file path to the wasi module we want to run
const wasmFilePath = './duktape.wasm';

// A quick wrapper for console.log, to also output logs to the body
const consoleLog = console.log;
console.log = function() {
  const args = Array.prototype.slice.call(arguments);
  consoleLog(args);
  const log = args.join(' ');
  consoleLog(log);
  document.body.appendChild(document.createTextNode('JavaScript Console: ' + log));
}

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

// Assign all reads to fd 0 (in this case, /dev/stdin) to our custom function
// Handle read of stdin, similar to C read
// https://linux.die.net/man/2/read
// Implemented here within the WasmFs Dependancy, Memfs:
// https://github.com/streamich/memfs/blob/master/src/volume.ts#L1020
// NOTE: This function MUST BE SYNCHRONOUS, 
// per the C api. Otherwise, the Wasi module will error.
let readStdinCounter = 0
const stdinRead = (
    stdinBuffer, // Uint8Array of the buffer that is sent to the guest wasm module's standard input
    offset, // offset for the standard input
    length, // length of the standard input
    position // Position in the input
    ) => {

  // Per the C API, first read should be the string
  // Second read would be the end of the string
  if (readStdinCounter % 2 !== 0) {
    readStdinCounter++;
    return 0;
  }

  // Use window.prompt to synchronously get input from the user
  // This will block the entire main thread until this finishes.
  // To do this more clean-ly, it would be best to use a Web Worker
  // and Shared Array Buffer. And use prompt as a fallback
  // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer
  // https://github.com/wasmerio/wasmer-js/blob/master/packages/wasm-terminal/src/process/process.ts#L174
  let responseStdin = prompt(
      `Please enter standard input to the duktape prompt\n`
      );

  // When the user cancels, throw an error to get out of the standard input read loop
  // From the guest wasm modules (duktape)
  if (responseStdin === null) {
    const userError = new Error("Process killed by Prompt Cancellation");
    userError.user = true;
    throw userError;
    return -1;
  }
  responseStdin += "\n";

  // Encode the string into bytes to be placed into the buffer for standard input
  const buffer = new TextEncoder().encode(responseStdin);
  for (let x = 0; x < buffer.length; ++x) {
    stdinBuffer[x] = buffer[x];
  }

  // Return the current stdin, per the C API
  return buffer.length;
}

// Assign all reads to fd 0 (in this case, /dev/stdin) to our custom function
wasmFs.volume.fds[0].node.read = stdinRead;

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
  try {
    wasi.start(instance);
  } catch(e) {
    // Catch errors, and if it is not a forced user error (User cancelled the prompt)
    // Log the error and end the process
    if (!e.user) {
      console.error(e);
      return;
    } 
  }


  // User cancelled the prompt!

  // Output what's inside of /dev/stdout!
  const stdout = await wasmFs.getStdOut();
  // Add the Standard output to the dom
  console.log('Standard Output: ' + stdout);
};
startWasiTask();
```



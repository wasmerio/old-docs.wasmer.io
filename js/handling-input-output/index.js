// Imports
import { WASI } from '@wasmer/wasi';
import { WasmFs } from '@wasmer/wasmfs';
import { lowerI64Imports } from "@wasmer/wasm-transformer";

// The file path to the wasi module we want to run
const wasmFilePath = './qjs.wasm';

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
const stdinRead = (
  stdinBuffer,
  offset,
  length,
  position
) => {

  // Per the C API, first read should be the string
  // Second read would be the end of the string
  if (this.readStdinCounter % 2 !== 0) {
    this.readStdinCounter++;
    return 0;
  }

  responseStdin = prompt(
    `Please enter text for stdin:\n${this.stdinPrompt}`
  );
  if (responseStdin === null) {
    const userError = new Error("Process killed by Prompt Cancellation");
    userError.user = true;
    throw userError;
    return -1;
  }
  responseStdin += "\n";

  const buffer = new TextEncoder().encode(responseStdin);
  for (let x = 0; x < buffer.length; ++x) {
    stdinBuffer[x] = buffer[x];
  }

  // Return the current stdin
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
  const loweredWasmBytes = await lowerI64Imports(wasmBytes);

  // Instantiate the WebAssembly file
  let { instance } = await WebAssembly.instantiate(loweredWasmBytes, {
    wasi_unstable: wasi.wasiImport
  });

  // Start the WebAssembly WASI instance!
  try {
    wasi.start(instance);
  } catch(e) {
    if (!e.user) {
      console.error(e);
      return;
    } 
  }

  // Output what's inside of /dev/stdout!
  const stdout = await wasmFs.getStdOut();
  // Add the Standard output to the dom
  console.log('Standard Output: ' + stdout);
};
startWasiTask();

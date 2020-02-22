# @wasmer/wasmfs

## WasmFS

`import { WasmFs } from "@wasmer/wasmfs"`.

### Constructor

`new WasmFs(): WasmFs`

Constructor that returns a `WasmFs` instance. Please see the Instance properties to see the properties on the returned instance

### Instance Properties

#### fs

`wasmFs.fs: MemFs`

[memfs](https://github.com/streamich/memfs)' [node fs](https://nodejs.org/api/fs.html) implementation object. See the [node fs documentation](https://nodejs.org/api/fs.html) for API usage.

{% hint style="info" %}
**NOTE:** The functions on this `fs` implementation can easily be overriden to provide custom functionality when your Wasm module \(running with [`@wasmer/wasi`](https://github.com/wasmerio/wasmer-js/tree/master/packages/wasi)\) tries to do file system operations

```javascript
const wasmFs = new WasmFs();

const originalWriteFileSync = wasmFs.fs.writeFileSync;
wasmFs.fs.writeFileSync = (path, text) => {
  console.log("File written:", path);
  originalWriteFileSync(path, text);
};

wasmFs.fs.writeFileSync("/dev/stdout", "Quick Start!");

// Would log: "File written: /dev/stdout"
```
{% endhint %}

#### getStdOut

`wasmFs.getStdOut(): string`

Function that returns the current standard output \(`/dev/stdout`\) of the filesystem.

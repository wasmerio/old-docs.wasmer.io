---
id: wasmer-js-node-modules-reference-api-wasmfs
title: Wasmer-JS Node Modules @wasmer/wasmfs Reference API
sidebar_label: @wasmer/wasmfs Reference API
---

# WasmFS

The default export of `@wasmer/wasmfs`, also available as `import { WasmFs } from "@wasmer/wasmfs"`.

## Constructor

`new WasmFs(): WasmFs`

Constructor that returns a WasmFs instance. Please see the Instance properties to see the properties on the returned instance

## Instance Properties

### fs

`wasmFs.fs: MemFs`

[memfs](https://github.com/streamich/memfs)' [node fs](https://nodejs.org/api/fs.html) implementation object. See the [node fs documentation](https://nodejs.org/api/fs.html) for API usage.

**NOTE:** The functions on this `fs` implementation can easily be overriden to provide custom functionality when your wasm module (running with `[@wasmer/wasi](https://github.com/wasmerio/wasmer-js/tree/master/packages/wasi)`) tries to do file system operations. For example:

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

### getStdOut

`wasmFs.getStdOut(): string`

Function that returns the current standard output (`/dev/stdout`) of the filesystem.

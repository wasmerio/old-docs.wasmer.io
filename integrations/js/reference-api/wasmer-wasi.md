---
description: The Package for using WASI easily from Node and the Browser
---

# @wasmer/wasi

## WASI

`import { WASI } from "@wasmer/wasi"`

### Constructor

`new WASI(wasiConfigObject): WASI`

Constructs a new WASI instance.

The `wasiConfigObject` is is as follows:

```javascript
let myWASIInstance = new WASI({
  // OPTIONAL: The pre-opened dirctories
  preopenDirectories: {},

  // OPTIONAL: The environment vars
  env: {},

  // OPTIONAL: The arguments provided
  args: [],

  // OPTIONAL: The environment bindings (fs, path),
  // useful for using WASI in diferent environments
  // such as Node.js, Browsers, ...
  bindings: {
    // hrtime: (time?: [number, number]) -> number
    // exit: (code?: number) -> void
    // kill: (pid: number, signal?: string | number) -> void
    // randomFillSync: (buffer: Buffer, offset?: number, size?: number) -> Buffer
    // isTTY: () -> bool
    // fs: Filesystem (with similar API interface as Node 'fs' module)
    // path: Path  (with similar API Interface as Node 'path' module)
    ...WASI.defaultBindings,
  }
});
```

This returns a WASI instance. Please see the Instance properties section to learn about the WASI instance.

### Class Properties

### Instance Properties

#### memory

`wasiInstance.memory: WebAssembly.Memory`

[`WebAssembly.memory`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WebAssembly/Memory) object, that is a view into the Wasm Module's linear memory.

#### view

`wasiInstance.view: DataView`

[DataView](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView) object, that is a view into the Wasm Module's linear memory.

#### FD\_MAP

`wasiInstance.FD_MAP: Map<number, File>`

[Javascript Map](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map), where the key is the file descriptor, and the value is a [Javascript File](https://developer.mozilla.org/en-US/docs/Web/API/File).

#### exports

`wasiInstance.exports: Exports`

WASI API to be imported in the importObject on instantiation.

#### bindings

`wasiInstance.bindings: WASIBindings`

The bindings for common node like objects, such as `fs` for filesystem, these should work by default, but are applied depending on the platform. You can view the source code for your respective platform's bindings here.

#### start

`wasiInstance.start(wasmInstance: WebAssembly.Instance): void`

Function that takes in a WASI WebAssembly Instance and starts it.

#### getImports

`wasiInstance.getImports(wasmModule: WebAssembly.Module): Exports`

Function that returns the map of corresponding imports for the WASI module. It will throw an error in case the `wasmModule` is not a WASI Module, or it have an incompatible version.


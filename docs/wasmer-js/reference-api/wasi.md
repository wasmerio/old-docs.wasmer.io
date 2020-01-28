---
id: wasmer-js-reference-api-wasi
title: Wasmer-JS Reference API @wasmer/wasi 
sidebar_label: @wasmer/wasi
---

# WASI

The default exported ES6 class of `@wasmer/wasi`, also available as `import { WASI } from "@wasmer/wasi"` 

## Constructor

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
    // hrtime: WASI.defaultBindings.hrtime,
    // exit: WASI.defaultBindings.exit,
    // kill: WASI.defaultBindings.kill,
    // randomFillSync: WASI.defaultBindings.randomFillSync,
    // isTTY: WASI.defaultBindings.isTTY,
    // fs: WASI.defaultBindings.fs,
    // path: WASI.defaultBindings.path,
    ...WASI.defaultBindings
  }
});
```

This returns a WASI instance. Please see the Instance properties section to learn about the WASI instance.

## Class Properties

### defaultBindings

`WASI.defaultBindings: WASIBindings`

The [default bindings](https://github.com/wasmerio/wasmer-js/tree/master/packages/wasi/src/bindings) for the environment that are set on the `bindings` property of the constructor config object. This is useful for use cases like, you want to plugin in your own file system. For example:

    const myFs = require("fs");
    
    let wasi = new WASI({
      preopenDirectories: {},
      env: {},
      args: [],
      bindings: {
        fs: myFs,
        ...WASI.defaultBindings
      }
    });

## Instance Properties

### memory

`wasiInstance.memory: WebAssembly.Memory`

[`WebAssembly.memory`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WebAssembly/Memory) object, that is a view into the Wasm Module's linear memory.

### view

`wasiInstance.view: DataView` 

[DataView](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView) object, that is a view into the Wasm Module's linear memory.

### FD_MAP

`wasiInstance.FD_MAP: Map<number, File>` 

[Javascript Map](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map), where the key is the file descriptor, and the value is a [JavaScript File](https://developer.mozilla.org/en-US/docs/Web/API/File).

### exports

`wasiInstance.exports: Exports`

WASI API to be imported in the importObject on instantiation.

### bindings

`wasiInstance.bindings: WASIBindings`

The bindings for common node like objects, such as `fs` for filesystem, these should work by default, but are applied depending on the platoform. You can view the source code for your respective platform's bindings here.

### start

`wasiInstance.start(wasmInstance: WebAssembly.Instance): void`

Function that takes in a WASI WebAssembly Instance and starts it.

---
id: wasmer-js-reference-api-wasm-transformer
title: Wasmer-JS Reference API @wasmer/wasm-transformer
sidebar_label: @wasmer/wasm-transformer
---

# version

`version(): string`

Exported function that returns a string of the current version of the package.

```javascript
import { version } from "wasm-transformer";
console.log(version()) // x.x.x
```

# wasmTransformerInit

## Node / Unoptimized

**This export is not needed, and should be skipped**

## Optimized

`wasmTransformerInit(wasmTransformerUrl: string): Promise`

Exported function that takes in the url to the wasm file for that the wasm-transformer uses. This is usually within your `node_modules` with the path: `"node_modules/@wasmer/wasm-transformer/wasm-transformer.wasm"`. But this path will be different depending on where you host the wasm file.

```javascript
import { wasmTransformerInit } from "wasm-transformer/optimized/wasm-transformer.esm";
wasmTransformerInit(
  // IMPORTANT: This URL points to wherever the wasm-transformer.wasm is hosted
  "node_modules/@wasmer/wasm-transformer/wasm-transformer.wasm"
).then(() => {
  // The wasm-transformer is now ready, and all exported transformations can be run.
})
```

# lowerI64Imports

## Node / Unoptimized

`lowerI64Imports(wasmBinaryWithI64Imports: Uint8Array): Promise<Uint8Array>`

## Optimized

`lowerI64Imports(wasmBinaryWithI64Imports: Uint8Array): Uint8Array`

Exported Function to insert trampoline functions for imports that have i64 params or returns. This is useful for running Wasm modules in browsers that [do not support JavaScript BigInt -> Wasm i64 integration](https://github.com/WebAssembly/proposals/issues/7). Especially in the case for [i64 WASI Imports](https://github.com/CraneStation/wasmtime/blob/master/docs/WASI-api.md#clock_time_get). Returns or resolves the lowered wasm binary as a Uint8Array.

```javascript
import { lowerI64Imports } from "@wasmer/wasm-transformer";
const transformWasmModuleBytes = async () => {
  const myWasmModuleBytes = new Uint8Array([ ... ]);
  // NOTE: Optimized Bundles must import/call wasmInit, 
  // and this function does not return a promise
  const loweredWasmModuleBytes = await lowerI64Imports(myWasmModulesBytes);
};
transformWasmModuleBytes();
```


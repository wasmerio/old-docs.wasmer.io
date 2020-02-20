# @wasmer/wasm-transformer

## version

`version(): string`

Exported function that returns a string of the current version of the package.

```javascript
import { version } from "wasm-transformer";

console.log(version()) // x.x.x
```

## lowerI64Imports

`lowerI64Imports(wasmBinaryWithI64Imports: Uint8Array): Promise<Uint8Array>`

Exported Function to insert trampoline functions for imports that have i64 params or returns. This is useful for running Wasm modules in browsers that [do not support JavaScript BigInt -&gt; Wasm i64 integration](https://github.com/WebAssembly/proposals/issues/7). Especially in the case for [i64 WASI Imports](https://github.com/WebAssembly/WASI/blob/master/phases/snapshot/docs.md#-clock_time_getid-clockid-precision-timestamp---errno-timestamp). Returns or resolves the lowered wasm binary as a `Uint8Array`.

```javascript
import { lowerI64Imports } from "@wasmer/wasm-transformer";

const transformWasmModuleBytes = async () => {
  const myWasmModuleBytes = new Uint8Array([ ... ]);
  const loweredWasmModuleBytes = await lowerI64Imports(myWasmModulesBytes);
};
transformWasmModuleBytes();
```

